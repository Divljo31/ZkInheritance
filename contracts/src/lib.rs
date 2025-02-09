use stylus_sdk::{msg, prelude::*};
use crate::types::{Testament, Heir};


#[stylus::contract]
pub struct TestamentContract {
    pub testaments: Vec<Testament>, // Lista svih testamenata
}

#[stylus::external]
impl TestamentContract {
    // Inicijalizacija ugovora
    pub fn new() -> Self {
        Self {
            testaments: Vec::new(),
        }
    }

    // Kreiranje testamenta
    pub fn create_testament(
        &mut self,
        lawyer: Address,
    ) -> u64 {
        let testament_id = self.testaments.len() as u64;
        self.testaments.push(Testament {
            owner: msg::sender(),
            lawyer,
            is_active: false,
            heirs: Vec::new(),
        });
        testament_id
    }

    // Dodavanje naslednika u testament
    pub fn add_heir(
        &mut self,
        testament_id: u64,
        heir_address: Address,
        percentage: u64,
        document_hash: String,
    ) {
        let caller = msg::sender();
        let testament = &mut self.testaments[testament_id as usize];

        // Provera da li samo vlasnik može dodati naslednika
        assert!(caller == testament.owner, "Only the owner can add heirs");

        testament.heirs.push(Heir {
            address: heir_address,
            percentage,
            document_hash,
        });
    }

    // Aktivacija testamenta od strane advokata
    pub fn activate_testament(&mut self, testament_id: u64) {
        let caller = msg::sender();
        let testament = &mut self.testaments[testament_id as usize];

        // Samo advokat može aktivirati testament
        assert!(caller == testament.lawyer, "Only the assigned lawyer can activate the testament");
        testament.is_active = true;
    }

    // Naslednik proverava svoje nasledstvo
    pub fn claim_inheritance(&self, testament_id: u64, proof: Vec<u8>) -> (u64, String) {
        let caller = msg::sender();
        let testament = &self.testaments[testament_id as usize];

        // Provera da li je testament aktiviran
        assert!(testament.is_active, "Testament is not active");

        // Provera naslednika i vraćanje podataka
        for heir in &testament.heirs {
            if heir.address == caller {
                // TODO: Proveriti zkProof ovde koristeći zkVerify.rs
                return (heir.percentage, heir.document_hash.clone());
            }
        }
        panic!("You are not listed as an heir");
    }
}
