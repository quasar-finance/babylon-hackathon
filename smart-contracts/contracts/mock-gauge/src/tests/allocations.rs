use crate::contract::{execute_upsert_weight, instantiate};
use crate::{contract::query_allocations, msg::InstantiateMsg};

use super::*;
use cosmwasm_std::Uint128;
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Decimal,
};

#[test]
fn test_query_allocations_initial_state() {
    let mut deps = mock_dependencies();

    // Initialize contract with two destinations
    let msg = InstantiateMsg {
        owner: "owner".to_string(),
        destinations: vec!["dest1".to_string(), "dest2".to_string()],
    };
    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Query allocations
    let res = query_allocations(deps.as_ref()).unwrap();

    // Both destinations should have equal weights (1:1)
    assert_eq!(res.allocations.len(), 2);
    assert_eq!(res.allocations[0].destination_id, "dest1");
    assert_eq!(res.allocations[0].amount, Decimal::from_ratio(1u128, 2u128));
    assert_eq!(res.allocations[1].destination_id, "dest2");
    assert_eq!(res.allocations[1].amount, Decimal::from_ratio(1u128, 2u128));
}

#[test]
fn test_query_allocations_after_single_update() {
    let mut deps = mock_dependencies();

    // Initialize contract with two destinations
    let msg = InstantiateMsg {
        owner: "owner".to_string(),
        destinations: vec!["dest1".to_string(), "dest2".to_string()],
    };
    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update weight for dest1
    execute_upsert_weight(deps.as_mut(), "dest1".to_string(), Uint128::new(3)).unwrap();

    // Query allocations
    let res = query_allocations(deps.as_ref()).unwrap();

    // Weights should be 3:1
    assert_eq!(res.allocations.len(), 2);
    assert_eq!(res.allocations[0].destination_id, "dest1");
    assert_eq!(res.allocations[0].amount, Decimal::from_ratio(3u128, 4u128));
    assert_eq!(res.allocations[1].destination_id, "dest2");
    assert_eq!(res.allocations[1].amount, Decimal::from_ratio(1u128, 4u128));
}

#[test]
fn test_query_allocations_multiple_updates() {
    let mut deps = mock_dependencies();

    // Initialize contract with three destinations
    let msg = InstantiateMsg {
        owner: "owner".to_string(),
        destinations: vec![
            "dest1".to_string(),
            "dest2".to_string(),
            "dest3".to_string(),
        ],
    };
    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update weights for all destinations
    execute_upsert_weight(deps.as_mut(), "dest1".to_string(), Uint128::new(5)).unwrap();
    execute_upsert_weight(deps.as_mut(), "dest2".to_string(), Uint128::new(3)).unwrap();
    execute_upsert_weight(deps.as_mut(), "dest3".to_string(), Uint128::new(2)).unwrap();

    // Query allocations
    let res = query_allocations(deps.as_ref()).unwrap();

    // Weights should be 5:3:2
    assert_eq!(res.allocations.len(), 3);
    assert_eq!(res.allocations[0].destination_id, "dest1");
    assert_eq!(
        res.allocations[0].amount,
        Decimal::from_ratio(5u128, 10u128)
    );
    assert_eq!(res.allocations[1].destination_id, "dest2");
    assert_eq!(
        res.allocations[1].amount,
        Decimal::from_ratio(3u128, 10u128)
    );
    assert_eq!(res.allocations[2].destination_id, "dest3");
    assert_eq!(
        res.allocations[2].amount,
        Decimal::from_ratio(2u128, 10u128)
    );
}

#[test]
fn test_query_allocations_with_zero_weight() {
    let mut deps = mock_dependencies();

    // Initialize contract with two destinations
    let msg = InstantiateMsg {
        owner: "owner".to_string(),
        destinations: vec!["dest1".to_string(), "dest2".to_string()],
    };
    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Set weight of dest1 to zero
    execute_upsert_weight(deps.as_mut(), "dest1".to_string(), Uint128::zero()).unwrap();

    // Query allocations
    let res = query_allocations(deps.as_ref()).unwrap();

    // Only dest2 should be present with 100% allocation
    assert_eq!(res.allocations.len(), 1);
    assert_eq!(res.allocations[0].destination_id, "dest2");
    assert_eq!(res.allocations[0].amount, Decimal::one());
}
