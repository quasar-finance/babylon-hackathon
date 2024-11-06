use cosmwasm_std::{
    coin,
    testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR},
    Decimal,
};
use mars_owner::OwnerError;

use crate::{
    contract::{execute, get_deposit_msg, get_withdraw_msg},
    msg::ExecuteMsg,
    tests::setup::{
        mock_wasm_querier_with_gauge, setup, setup_with_balances, DEPOSIT_DENOM, DEST1, DEST2,
        OWNER, USER,
    },
    VaultError,
};

#[test]
fn rebalance_fails_for_non_owner() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let msg = ExecuteMsg::Rebalance {};
    let result = execute(deps.as_mut(), env, info, msg);
    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        VaultError::Owner(OwnerError::NotOwner {})
    );
}

#[test]
fn rebalance_from_unallocated_assets() {
    let deposits = 10000u128;
    let mut deps = setup_with_balances(&[(MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)])]);
    let dest1_adaptor = deps.api.addr_make(DEST1);
    let dest2_adaptor = deps.api.addr_make(DEST2);

    deps.querier.update_wasm(mock_wasm_querier_with_gauge(
        "oracle".to_string(),
        "gauge".to_string(),
        dest1_adaptor.to_string(),
        Decimal::percent(30),
        vec![],
        dest2_adaptor.to_string(),
        Decimal::percent(70),
        vec![],
        Decimal::percent(123),
        Decimal::percent(246),
    ));
    let env = mock_env();
    let info = mock_info(OWNER, &[]);

    let msg = ExecuteMsg::RegisterDestination {
        destination: DEST1.to_string(),
        adaptor: dest1_adaptor.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let msg = ExecuteMsg::RegisterDestination {
        destination: DEST2.to_string(),
        adaptor: dest2_adaptor.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let msg = ExecuteMsg::RegisterLst {
        denom: DEPOSIT_DENOM.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let msg = ExecuteMsg::Rebalance {};
    let result = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(result.messages.len(), 2);
    assert_eq!(
        result.messages[0].msg,
        get_deposit_msg(dest1_adaptor, vec![coin(3000, DEPOSIT_DENOM)]).unwrap()
    );
    assert_eq!(
        result.messages[1].msg,
        get_deposit_msg(dest2_adaptor, vec![coin(7000, DEPOSIT_DENOM)]).unwrap()
    );
}

#[test]
fn rebalance_between_adaptors() {
    let mut deps = setup();
    let dest1_adaptor = deps.api.addr_make(DEST1);
    let dest2_adaptor = deps.api.addr_make(DEST2);

    deps.querier.update_wasm(mock_wasm_querier_with_gauge(
        "oracle".to_string(),
        "gauge".to_string(),
        dest1_adaptor.to_string(),
        Decimal::percent(30),
        vec![coin(7000, DEPOSIT_DENOM)],
        dest2_adaptor.to_string(),
        Decimal::percent(70),
        vec![coin(3000, DEPOSIT_DENOM)],
        Decimal::percent(123),
        Decimal::percent(246),
    ));
    let env = mock_env();
    let info = mock_info(OWNER, &[]);

    let msg = ExecuteMsg::RegisterDestination {
        destination: DEST1.to_string(),
        adaptor: dest1_adaptor.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let msg = ExecuteMsg::RegisterDestination {
        destination: DEST2.to_string(),
        adaptor: dest2_adaptor.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let msg = ExecuteMsg::RegisterLst {
        denom: DEPOSIT_DENOM.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let msg = ExecuteMsg::Rebalance {};
    let result = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(result.messages.len(), 2);
    assert_eq!(
        result.messages[0].msg,
        get_withdraw_msg(dest1_adaptor, vec![coin(4000, DEPOSIT_DENOM)]).unwrap()
    );
    assert_eq!(
        result.messages[1].msg,
        get_deposit_msg(dest2_adaptor, vec![coin(4000, DEPOSIT_DENOM)]).unwrap()
    );
}

#[test]
fn rebalance_between_adaptors_with_unallocated_assets() {
    let deposits = 10000u128;
    let mut deps = setup_with_balances(&[(MOCK_CONTRACT_ADDR, &[coin(deposits, DEPOSIT_DENOM)])]);
    let dest1_adaptor = deps.api.addr_make(DEST1);
    let dest2_adaptor = deps.api.addr_make(DEST2);

    deps.querier.update_wasm(mock_wasm_querier_with_gauge(
        "oracle".to_string(),
        "gauge".to_string(),
        dest1_adaptor.to_string(),
        Decimal::percent(30),
        vec![coin(7000, DEPOSIT_DENOM)],
        dest2_adaptor.to_string(),
        Decimal::percent(70),
        vec![coin(3000, DEPOSIT_DENOM)],
        Decimal::percent(123),
        Decimal::percent(246),
    ));
    let env = mock_env();
    let info = mock_info(OWNER, &[]);

    let msg = ExecuteMsg::RegisterDestination {
        destination: DEST1.to_string(),
        adaptor: dest1_adaptor.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let msg = ExecuteMsg::RegisterDestination {
        destination: DEST2.to_string(),
        adaptor: dest2_adaptor.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let msg = ExecuteMsg::RegisterLst {
        denom: DEPOSIT_DENOM.to_string(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let msg = ExecuteMsg::Rebalance {};
    let result = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(result.messages.len(), 2);
    assert_eq!(
        result.messages[0].msg,
        get_withdraw_msg(dest1_adaptor, vec![coin(1000, DEPOSIT_DENOM)]).unwrap()
    );
    assert_eq!(
        result.messages[1].msg,
        get_deposit_msg(dest2_adaptor, vec![coin(11000, DEPOSIT_DENOM)]).unwrap()
    );
}
