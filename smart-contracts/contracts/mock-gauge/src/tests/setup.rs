use crate::{contract::instantiate, msg::InstantiateMsg};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Empty, OwnedDeps,
};

pub const OWNER: &str = "owner";
pub const USER: &str = "user";

pub const DESTINATION_IDS: [&str; 2] = ["id1", "id2"];

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
            destinations: DESTINATION_IDS.iter().map(|&s| s.to_string()).collect(), // this is really ugly and should be done simpler
        },
    )
    .is_ok());

    deps
}

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies();
    basic_setup(deps)
}
