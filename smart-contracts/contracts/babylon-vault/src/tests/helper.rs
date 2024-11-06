use cosmwasm_std::{DepsMut, Env, MessageInfo, Uint128};
use cw20_base::contract::execute_mint;

pub(crate) fn mint_shares(deps: DepsMut, env: Env, receiver: String, fund_shares: Uint128) {
    let info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };
    execute_mint(deps, env.clone(), info, receiver, fund_shares).unwrap();
}
