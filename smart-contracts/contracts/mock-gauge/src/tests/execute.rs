use crate::contract::execute_upsert_weight;
use crate::error::VaultError;
use crate::state::{DESTINATIONS, WEIGHTS};
use crate::tests::setup::DESTINATION_IDS;
use crate::{
    contract::execute_add_destination,
    tests::setup::{setup, OWNER},
};
use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Response, Uint128};

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

#[test]
fn test_upsert_weight_new_destination() {
    let mut deps = setup();
    let destination_id = "new_destination".to_string();
    let amount = Uint128::new(100);

    let res = execute_upsert_weight(deps.as_mut(), destination_id.clone(), amount).unwrap();
    assert_eq!(res, Response::new());

    assert!(DESTINATIONS.has(deps.as_ref().storage, destination_id.clone()));

    let stored_weight = WEIGHTS.get(deps.as_ref().storage, &destination_id).unwrap();
    assert_eq!(stored_weight.destination_id, destination_id);
    assert_eq!(stored_weight.amount, amount);
}

#[test]
fn test_upsert_weight_existing_destination() {
    let mut deps = setup();
    let destination_id = DESTINATION_IDS[0].to_string();
    let amount = Uint128::new(100);

    let res = execute_upsert_weight(deps.as_mut(), destination_id.clone(), amount).unwrap();
    assert_eq!(res, Response::new());

    let stored_weight = WEIGHTS.get(deps.as_ref().storage, &destination_id).unwrap();
    assert_eq!(stored_weight.destination_id, destination_id);
    assert_eq!(stored_weight.amount, amount);
}

#[test]
fn test_upsert_weight_updates_existing_weight() {
    let mut deps = setup();
    let destination_id = DESTINATION_IDS[0].to_string();

    let initial_amount = Uint128::new(100);
    execute_upsert_weight(deps.as_mut(), destination_id.clone(), initial_amount).unwrap();

    let new_amount = Uint128::new(200);
    let res = execute_upsert_weight(deps.as_mut(), destination_id.clone(), new_amount).unwrap();
    assert_eq!(res, Response::new());

    let stored_weight = WEIGHTS.get(deps.as_ref().storage, &destination_id).unwrap();
    assert_eq!(stored_weight.destination_id, destination_id);
    assert_eq!(stored_weight.amount, new_amount);
}
