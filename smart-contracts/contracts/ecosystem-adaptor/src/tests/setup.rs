use crate::{
    contract::instantiate,
    msg::{EcosystemInfo, InstantiateMsg, PolytoneInfo},
};
use cosmwasm_std::{
    testing::{
        mock_dependencies_with_balances, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    },
    Coin, Empty, OwnedDeps,
};

pub const VAULT: &str = "babylon_vault";
pub const USER: &str = "user";
pub const DENOM0: &str = "uatom";
pub const DENOM1: &str = "uosmo";
pub const DENOM2: &str = "ujuno";

fn basic_setup(
    deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = deps;
    let env = mock_env();
    let info = mock_info(VAULT, &[]);

    assert!(instantiate(
        deps.as_mut(),
        env.clone(),
        info,
        InstantiateMsg {
            babylon_vault: VAULT.to_string(),
            ecosystem_info: EcosystemInfo {
                deposit_denoms: vec![DENOM0.to_string(), DENOM1.to_string(), DENOM2.to_string(),],
                deposit_ecosystem: "cosmoshub".to_string(),
                transfer_channel: "channel-0".to_string(),
                connection: "connection-0".to_string(),
                return_source_channel: "channel-0".to_string(),
                destination_chain_denom: "ibc/denom".to_string(),
            },
            polytone_info: PolytoneInfo {
                polyton_note_contract: "polytone".to_string(),
            },
        },
    )
    .is_ok());

    deps
}

pub fn setup_with_balances(
    balances: &[(&str, &[Coin])],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies_with_balances(balances);
    basic_setup(deps)
}
