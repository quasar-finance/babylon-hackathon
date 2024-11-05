use crate::error::VaultError;
use crate::{
    contract::execute_add_destination,
    tests::setup::{setup, OWNER},
};
use cosmwasm_std::testing::{mock_env, mock_info};

#[test]
fn test_unauthorized_add_destination() {
    let mut deps = setup();
    let unauthorized_info = mock_info("unauthorized", &[]);

    let err = execute_add_destination(
        deps.as_mut(),
        unauthorized_info,
        "new_destination".to_string(),
    )
    .unwrap_err();

    assert_eq!(err, VaultError::Owner(mars_owner::OwnerError::NotOwner {}));
}

#[test]
fn test_successful_add_destination() {
    let mut deps = setup();
    let owner_info = mock_info(OWNER, &[]);

    let res =
        execute_add_destination(deps.as_mut(), owner_info, "new_destination".to_string()).unwrap();

    assert_eq!(res.attributes.len(), 2);
    assert_eq!(res.attributes[0].key, "action");
    assert_eq!(res.attributes[0].value, "add_destination");
    assert_eq!(res.attributes[1].key, "destination");
    assert_eq!(res.attributes[1].value, "new_destination");
}

#[test]
fn test_duplicate_add_destination() {
    let mut deps = setup();
    let owner_info = mock_info(OWNER, &[]);

    execute_add_destination(
        deps.as_mut(),
        owner_info.clone(),
        "new_destination".to_string(),
    )
    .unwrap();

    let err = execute_add_destination(deps.as_mut(), owner_info, "new_destination".to_string())
        .unwrap_err();

    assert!(matches!(
        err,
        VaultError::DestinationAlreadyExists { id } if id == "new_destination"
    ));
}
