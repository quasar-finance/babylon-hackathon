use cosmwasm_std::{coin, Coin};

#[derive(PartialEq, Debug)]
pub struct Changes {
    pub add: Vec<Coin>,
    pub remove: Vec<Coin>,
}

impl Changes {
    pub fn new(add: Vec<Coin>, remove: Vec<Coin>) -> Self {
        Self { add, remove }
    }
}

impl Default for Changes {
    fn default() -> Self {
        Self::new(vec![], vec![])
    }
}

pub fn compute_changes(current: &[Coin], desired: &[Coin]) -> Changes {
    let add = desired
        .iter()
        .map(|d| {
            let current_allocations: Vec<&Coin> =
                current.iter().filter(|c| c.denom == d.denom).collect();
            let mut result = coin(d.amount.into(), d.denom.clone());
            if !current_allocations.is_empty() {
                result.amount -= std::cmp::min(current_allocations[0].amount, result.amount);
            }
            result
        })
        .filter(|x| !x.amount.is_zero())
        .collect();
    let remove = current
        .iter()
        .map(|c| {
            let desired_allocations: Vec<&Coin> =
                desired.iter().filter(|d| d.denom == c.denom).collect();
            let mut result = coin(c.amount.into(), c.denom.clone());
            if !desired_allocations.is_empty() {
                result.amount -= std::cmp::min(desired_allocations[0].amount, result.amount);
            }
            result
        })
        .filter(|x| !x.amount.is_zero())
        .collect();

    Changes::new(add, remove)
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
