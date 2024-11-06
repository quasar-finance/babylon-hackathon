use crate::error::{assert_deposit_funds, assert_withdraw_funds, VaultError};
use crate::msg::{DestinationInfo, ExecuteMsg, InstantiateMsg, OracleQueryMsg, QueryMsg};
use crate::state::{DESTINATIONS, GAUGE, LSTS, ORACLE, OWNER};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult, Storage, Uint128,
};
use cw2::set_contract_version;
use cw20_base::contract::{execute_burn, execute_mint, query_balance, query_token_info};
use cw20_base::enumerable::query_all_accounts;
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};
use std::collections::{HashMap, HashSet};

const CONTRACT_NAME: &str = "quasar:babylon-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type VaultResult<T = Response> = Result<T, VaultError>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> VaultResult {
    OWNER.initialize(
        deps.storage,
        deps.api,
        mars_owner::OwnerInit::SetInitialOwner { owner: msg.owner },
    )?;
    ORACLE.save(deps.storage, &deps.api.addr_validate(&msg.oracle)?)?;
    LSTS.save(deps.storage, &HashSet::new())?;
    GAUGE.save(deps.storage, &deps.api.addr_validate(&msg.gauge)?)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // store token info using cw20-base format
    let data = TokenInfo {
        name: msg.subdenom.clone(),
        symbol: msg.subdenom,
        decimals: 6,
        total_supply: Uint128::zero(),
        // set self as minter, so we can properly execute mint and burn
        mint: Some(MinterData {
            minter: env.contract.address.clone(),
            cap: None,
        }),
    };
    TOKEN_INFO.save(deps.storage, &data)?;

    Ok(Response::new().add_attribute("vault_token", env.contract.address.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> VaultResult {
    match msg {
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        ExecuteMsg::RegisterLst { denom } => register_lst(deps, info, denom),
        ExecuteMsg::UnregisterLst { denom } => unregister_lst(deps, info, denom),
        ExecuteMsg::RegisterDestination {
            destination,
            adaptor,
        } => register_destination(deps, info, destination, adaptor),
        ExecuteMsg::UnregisterDestination { destination } => {
            unregister_destination(deps, info, destination)
        }
        ExecuteMsg::SetOracle { oracle } => set_oracle(deps, info, oracle),
        ExecuteMsg::Deposit {} => deposit(deps, env, info),
        ExecuteMsg::Withdraw { amount } => withdraw(deps, env, info, amount),
        _ => Ok(Response::default()),
    }
}

fn register_lst(deps: DepsMut, info: MessageInfo, denom: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    LSTS.update(deps.storage, |lsts| -> StdResult<HashSet<String>> {
        let mut lsts = lsts;
        lsts.insert(denom);
        Ok(lsts)
    })?;
    Ok(Response::default())
}

fn unregister_lst(deps: DepsMut, info: MessageInfo, denom: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    LSTS.update(deps.storage, |lsts| -> StdResult<HashSet<String>> {
        let mut lsts = lsts;
        if lsts.remove(&denom) {
            Ok(lsts)
        } else {
            Err(StdError::generic_err("Denom not found"))
        }
    })?;
    Ok(Response::default())
}

fn register_destination(
    deps: DepsMut,
    info: MessageInfo,
    destination: String,
    adaptor: String,
) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    DESTINATIONS.save(
        deps.storage,
        destination,
        &deps.api.addr_validate(&adaptor)?,
    )?;
    Ok(Response::default())
}

fn unregister_destination(deps: DepsMut, info: MessageInfo, destination: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    if DESTINATIONS.has(deps.storage, destination.clone()) {
        DESTINATIONS.remove(deps.storage, destination);
    } else {
        return Err(VaultError::DestinationNotFound { destination });
    }
    Ok(Response::default())
}

fn set_oracle(deps: DepsMut, info: MessageInfo, oracle: String) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    ORACLE.save(deps.storage, &deps.api.addr_validate(&oracle)?)?;
    Ok(Response::default())
}

fn get_lst_denoms(storage: &dyn Storage) -> StdResult<Vec<String>> {
    Ok(LSTS.load(storage)?.into_iter().collect())
}

fn get_prices(deps: &Deps, denoms: &[String]) -> VaultResult<HashMap<String, Decimal>> {
    let oracle = ORACLE.load(deps.storage)?;
    let denoms_with_prices: StdResult<Vec<_>> = denoms
        .iter()
        .map(|denom| -> StdResult<(String, Decimal)> {
            let price = deps.querier.query_wasm_smart(
                oracle.to_string(),
                &OracleQueryMsg::Price {
                    denom: denom.clone(),
                },
            )?;
            Ok((denom.clone(), price))
        })
        .collect();
    let denoms_with_prices = denoms_with_prices?;
    Ok(denoms_with_prices.into_iter().collect())
}

fn get_balances(deps: &Deps, address: String, denoms: &[String]) -> StdResult<Vec<Coin>> {
    denoms
        .iter()
        .map(|denom| -> StdResult<Coin> {
            deps.querier.query_balance(address.clone(), denom.clone())
        })
        .collect()
}

fn vault_value(balances: &[Coin], prices: &HashMap<String, Decimal>) -> VaultResult<Uint128> {
    let values: Result<Vec<Uint128>, _> = balances
        .iter()
        .map(|balance| -> VaultResult<Uint128> {
            let value = balance
                .amount
                .checked_mul_floor(*prices.get(&balance.denom).unwrap())?;
            Ok(value)
        })
        .collect();
    let value = values?.iter().sum();
    Ok(value)
}

fn deposit(deps: DepsMut, env: Env, info: MessageInfo) -> VaultResult {
    assert_deposit_funds(deps.storage, &info.funds)?;

    let token_info = query_token_info(deps.as_ref())?;
    let existing_shares = token_info.total_supply;

    let contract_address = env.contract.address.to_string();
    let lst_denoms = get_lst_denoms(deps.storage)?;
    let prices = get_prices(&deps.as_ref(), &lst_denoms)?;
    let balances = get_balances(&deps.as_ref(), contract_address.clone(), &lst_denoms)?;
    let contract_value = vault_value(&balances, &prices)?;
    let deposit_value = info.funds[0]
        .amount
        .checked_mul_floor(*prices.get(&info.funds[0].denom).unwrap())?;

    let new_shares = if existing_shares.is_zero() {
        info.funds[0].amount
    } else {
        existing_shares.checked_mul_floor(Decimal::from_ratio(
            deposit_value,
            contract_value.checked_sub(deposit_value)?,
        ))?
    };

    execute_mint(
        deps,
        env.clone(),
        MessageInfo {
            sender: env.contract.address,
            funds: vec![],
        },
        info.sender.to_string(),
        new_shares,
    )?;

    Ok(Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("amount", new_shares))
}

fn withdraw(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> VaultResult {
    assert_withdraw_funds(deps.storage, &info.funds)?;

    if amount > query_balance(deps.as_ref(), info.sender.to_string())?.balance || amount.is_zero() {
        return Err(VaultError::InvalidFunds {});
    }

    let token_info = query_token_info(deps.as_ref())?;
    let existing_shares = token_info.total_supply;
    let lst_denoms = get_lst_denoms(deps.storage)?;
    let contract_address = env.contract.address.to_string();
    let balances = get_balances(&deps.as_ref(), contract_address.clone(), &lst_denoms)?;
    let claim_ratio = Decimal::from_ratio(amount, existing_shares);

    let claim_funds: Result<Vec<_>, _> = balances
        .into_iter()
        .map(|balance| -> VaultResult<Coin> {
            Ok(Coin {
                amount: balance.amount.checked_mul_floor(claim_ratio)?,
                denom: balance.denom,
            })
        })
        .collect();
    let mut claim_funds = claim_funds?;
    claim_funds.sort_by(|a, b| a.denom.cmp(&b.denom));

    execute_burn(deps, env, info.clone(), amount)?;

    let send_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: claim_funds,
    };

    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("action", "withdraw")
        .add_attribute("amount", amount))
}

fn get_destinations(storage: &dyn Storage) -> VaultResult<Vec<DestinationInfo>> {
    let destinations: Result<Vec<_>, _> = DESTINATIONS
        .range(storage, None, None, Order::Ascending)
        .collect();
    Ok(destinations?
        .into_iter()
        .map(|(destination, adaptor)| DestinationInfo {
            destination,
            adaptor,
        })
        .collect())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> VaultResult<Binary> {
    match msg {
        QueryMsg::Value {} => Ok(to_json_binary(&query_value(deps, env)?)?),
        QueryMsg::Owner {} => Ok(to_json_binary(&OWNER.query(deps.storage)?)?),
        QueryMsg::Lsts {} => Ok(to_json_binary(&get_lst_denoms(deps.storage)?)?),
        QueryMsg::Destinations {} => Ok(to_json_binary(&get_destinations(deps.storage)?)?),
        QueryMsg::Balance { address } => Ok(to_json_binary(&query_balance(deps, address)?)?),
        QueryMsg::TokenInfo {} => Ok(to_json_binary(&query_token_info(deps)?)?),
        QueryMsg::AllAccounts { start_after, limit } => Ok(to_json_binary(&query_all_accounts(
            deps,
            start_after,
            limit,
        )?)?),
    }
}

fn query_value(deps: Deps, env: Env) -> VaultResult<Uint128> {
    let lst_denoms = get_lst_denoms(deps.storage)?;
    let prices = get_prices(&deps, &lst_denoms)?;
    let balances = get_balances(&deps, env.contract.address.to_string(), &lst_denoms)?;
    vault_value(&balances, &prices)
}
