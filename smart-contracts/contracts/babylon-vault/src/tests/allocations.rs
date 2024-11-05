use crate::contract::{compute_changes, Changes};
use cosmwasm_std::coin;

#[test]
fn compute_allocation_changes_for_coinciding_allcations() {
    let initial_allocations = vec![coin(123, "lst1"), coin(234, "lst2")];
    let desired_allocations = vec![coin(123, "lst1"), coin(234, "lst2")];

    let changes = compute_changes(&initial_allocations, &desired_allocations);
    assert_eq!(changes, Changes::default());
}

#[test]
fn compute_allocation_changes_from_empty_allocation() {
    let initial_allocations = vec![];
    let desired_allocations = vec![coin(123, "lst1"), coin(234, "lst2")];

    let changes = compute_changes(&initial_allocations, &desired_allocations);
    assert_eq!(changes, Changes::new(desired_allocations, vec![]));
}

#[test]
fn compute_allocation_changes_to_empty_allocation() {
    let initial_allocations = vec![coin(123, "lst1"), coin(234, "lst2")];
    let desired_allocations = vec![];

    let changes = compute_changes(&initial_allocations, &desired_allocations);
    assert_eq!(changes, Changes::new(vec![], initial_allocations));
}

#[test]
fn compute_allocation_changes() {
    let initial_allocations = vec![coin(123, "lst1"), coin(234, "lst2")];
    let desired_allocations = vec![coin(234, "lst1"), coin(123, "lst2")];

    let changes = compute_changes(&initial_allocations, &desired_allocations);
    assert_eq!(
        changes,
        Changes::new(vec![coin(111, "lst1")], vec![coin(111, "lst2")])
    );
}
