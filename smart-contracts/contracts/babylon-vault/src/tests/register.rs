use cosmwasm_std::{
    from_json,
    testing::{mock_env, mock_info},
    Addr, StdError,
};
use mars_owner::OwnerError;

use crate::{
    contract::{execute, query},
    msg::{DestinationInfo, ExecuteMsg, QueryMsg},
    tests::setup::{setup, OWNER, USER},
    VaultError,
};

#[test]
fn register_lst_fails_for_non_owner() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let denom = "lst".to_string();
    let msg = ExecuteMsg::RegisterLst { denom };
    let result = execute(deps.as_mut(), env, info, msg);
    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        VaultError::Owner(OwnerError::NotOwner {})
    );
}

#[test]
fn unregister_lst_fails_for_non_owner() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let denom = "lst".to_string();
    let msg = ExecuteMsg::UnregisterLst { denom };
    let result = execute(deps.as_mut(), env, info, msg);
    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        VaultError::Owner(OwnerError::NotOwner {})
    );
}

#[test]
fn register_and_unregister_lst() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(OWNER, &[]);

    let denom = "lst".to_string();
    let msg = ExecuteMsg::RegisterLst {
        denom: denom.clone(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let lsts: Vec<String> =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Lsts {}).unwrap()).unwrap();
    assert_eq!(lsts.len(), 1);
    assert_eq!(lsts[0], denom);

    let msg = ExecuteMsg::UnregisterLst { denom };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let lsts: Vec<String> =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Lsts {}).unwrap()).unwrap();
    assert_eq!(lsts.len(), 0);
}

#[test]
fn unregister_fails_if_denom_is_not_registered() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(OWNER, &[]);

    let denom = "lst".to_string();
    let msg = ExecuteMsg::RegisterLst {
        denom: denom.clone(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let lsts: Vec<String> =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Lsts {}).unwrap()).unwrap();
    assert_eq!(lsts.len(), 1);
    assert_eq!(lsts[0], denom);

    let other_denom = "other_lst".to_string();
    let msg = ExecuteMsg::UnregisterLst {
        denom: other_denom.clone(),
    };
    let result = execute(deps.as_mut(), env, info, msg);
    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        VaultError::Std(StdError::GenericErr {
            msg: "Denom not found".to_string()
        })
    );
}

#[test]
fn register_destination_fails_for_non_owner() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let destination = "dest1".to_string();
    let adaptor = "adaptor1".to_string();
    let msg = ExecuteMsg::RegisterDestination {
        destination,
        adaptor,
    };
    let result = execute(deps.as_mut(), env, info, msg);
    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        VaultError::Owner(OwnerError::NotOwner {})
    );
}

#[test]
fn unregister_destination_fails_for_non_owner() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let destination = "dest1".to_string();
    let msg = ExecuteMsg::UnregisterDestination { destination };
    let result = execute(deps.as_mut(), env, info, msg);
    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        VaultError::Owner(OwnerError::NotOwner {})
    );
}

#[test]
fn register_and_unregister_destination() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(OWNER, &[]);

    let destination = "dest1".to_string();
    let adaptor = "adaptor1".to_string();
    let msg = ExecuteMsg::RegisterDestination {
        destination: destination.clone(),
        adaptor: adaptor.clone(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let destinations: Vec<DestinationInfo> =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Destinations {}).unwrap()).unwrap();
    assert_eq!(destinations.len(), 1);
    assert_eq!(
        destinations[0],
        DestinationInfo {
            destination: destination.clone(),
            adaptor: Addr::unchecked(adaptor),
        }
    );

    let msg = ExecuteMsg::UnregisterDestination { destination };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());
    let destinations: Vec<String> =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Destinations {}).unwrap()).unwrap();
    assert_eq!(destinations.len(), 0);
}

#[test]
fn unregister_fails_if_destination_is_not_registered() {
    let mut deps = setup();
    let env = mock_env();
    let info = mock_info(OWNER, &[]);

    let destination = "dest1".to_string();
    let adaptor = "adaptor1".to_string();
    let msg = ExecuteMsg::RegisterDestination {
        destination: destination.clone(),
        adaptor: adaptor.clone(),
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok());

    let destinations: Vec<DestinationInfo> =
        from_json(query(deps.as_ref(), env.clone(), QueryMsg::Destinations {}).unwrap()).unwrap();
    assert_eq!(destinations.len(), 1);
    assert_eq!(
        destinations[0],
        DestinationInfo {
            destination: destination.clone(),
            adaptor: Addr::unchecked(adaptor),
        }
    );

    let other_destination = "dest2".to_string();
    let msg = ExecuteMsg::UnregisterDestination {
        destination: other_destination.clone(),
    };
    let result = execute(deps.as_mut(), env, info, msg);
    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        VaultError::DestinationNotFound {
            destination: other_destination
        }
    );
}
