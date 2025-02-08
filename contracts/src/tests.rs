use crate::{TestamentContract, types::{Heir, Testament}};
use stylus_sdk::prelude::*;

#[test]
fn test_create_testament() {
    let mut contract = TestamentContract::new();
    let testament_id = contract.create_testament(Address::default());

    assert_eq!(testament_id, 0);
    assert_eq!(contract.testaments.len(), 1);
}

#[test]
fn test_add_heir() {
    let mut contract = TestamentContract::new();
    let testament_id = contract.create_testament(Address::default());

    contract.add_heir(
        testament_id,
        Address::default(),
        50,
        "test-hash".to_string(),
    );

    let testament = &contract.testaments[testament_id as usize];
    assert_eq!(testament.heirs.len(), 1);
    assert_eq!(testament.heirs[0].percentage, 50);
}

#[test]
fn test_activate_testament() {
    let mut contract = TestamentContract::new();
    let lawyer = Address::default();

    let testament_id = contract.create_testament(lawyer);

    contract.activate_testament(testament_id);
    assert!(contract.testaments[testament_id as usize].is_active);
}
