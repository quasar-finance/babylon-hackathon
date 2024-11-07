use cosmwasm_std::{
    testing::{mock_dependencies, MockStorage},
    Uint128,
};

use crate::state::{Weight, Weights};

#[test]
fn test_weights_operations() {
    let mut deps = mock_dependencies();
    let store = deps.as_mut().storage;
    let weights = Weights::new("weights", "total_weight");

    let weight1 = Weight {
        destination_id: "dest1".to_string(),
        amount: Uint128::new(100),
    };
    weights.add(store, weight1.clone()).unwrap();

    let stored_weight = weights.get(store, "dest1").unwrap();
    assert_eq!(stored_weight.amount, Uint128::new(100));
    assert_eq!(weights.total(store).unwrap(), Uint128::new(100));

    let weight1_updated = Weight {
        destination_id: "dest1".to_string(),
        amount: Uint128::new(150),
    };
    weights.add(store, weight1_updated).unwrap();

    let stored_weight = weights.get(store, "dest1").unwrap();
    assert_eq!(stored_weight.amount, Uint128::new(150));
    assert_eq!(weights.total(store).unwrap(), Uint128::new(150));

    let weight2 = Weight {
        destination_id: "dest2".to_string(),
        amount: Uint128::new(200),
    };
    weights.add(store, weight2).unwrap();

    assert_eq!(weights.total(store).unwrap(), Uint128::new(350));
}

#[test]
fn test_weight_not_found() {
    let storage = MockStorage::new();
    let weights = Weights::new("weights", "total_weight");

    // Test getting non-existent weight
    let result = weights.get(&storage, "nonexistent");
    assert!(result.is_err());
}
