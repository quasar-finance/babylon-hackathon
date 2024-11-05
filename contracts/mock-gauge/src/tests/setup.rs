use crate::{
    contract::{instantiate, reply},
    msg::InstantiateMsg,
    state::Destination,
};
use cosmwasm_std::{
    from_json,
    testing::{
        mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MockApi,
        MockQuerier, MockStorage,
    },
    to_json_binary, Coin, ContractResult, Decimal, Empty, OwnedDeps, QuerierResult, Reply,
    SubMsgResponse, SubMsgResult, SystemError, SystemResult, WasmQuery,
};
use prost::Message;
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::MsgCreateDenomResponse;

pub const OWNER: &str = "owner";
pub const USER: &str = "user";
pub const SUBDENOM: &str = "subdenom";
pub const DEPOSIT_DENOM: &str = "denom1";
pub const OTHER_DEPOSIT_DENOM: &str = "denom2";
pub const VAULT_DENOM: &str = "vault_denom";

pub const DESTINATIONS: [&str; 2] = ["id1", "id2"];

fn basic_setup(
    deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = deps;
    let env = mock_env();
    let info = mock_info(USER, &[]);

    assert!(instantiate(
        deps.as_mut(),
        env.clone(),
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
            destinations: DESTINATIONS.iter().map(|&s| s.to_string()).collect(), // this is really ugly and should be done simpler
        },
    )
    .is_ok());

    deps
}

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies();
    basic_setup(deps)
}

pub fn setup_with_balances(
    balances: &[(&str, &[Coin])],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies_with_balances(balances);
    basic_setup(deps)
}
