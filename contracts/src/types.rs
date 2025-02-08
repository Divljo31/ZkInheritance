use stylus_sdk::prelude::*;


#[derive(Default)]
pub struct Testament {
    pub owner: Address,           // Address of the will owner
    pub lawyer: Address,          // Lawyer's address
    pub is_active: bool,          // is testament active
    pub heirs: Vec<Heir>,         // List of heirs
}

#[derive(Default)]
pub struct Heir {
    pub address: Address,         // Heir's address
    pub percentage: u64,          // Inheritance percentage (crypto)
    pub document_hash: String,    // Hash of the JSON file with the will
}