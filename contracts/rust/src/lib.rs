#![no_std]

extern crate alloc;

use stylus_sdk::prelude::*;
use alloc::{string::String, vec::Vec};

// Definišemo naslednika
#[derive(Default, Encode, Decode)]
pub struct Heir {
    pub heir_address: Address,
    pub percentage: u64,
    pub document_hash: String,
}

// Struktura testamenta
#[derive(Default, Encode, Decode)]
pub struct TestamentData {
    pub owner: Address,
    pub lawyer: Address,
    pub is_active: bool,
    pub heirs: Vec<Heir>,
}

#[stylus::contract]
pub struct Testament {
    testaments: Mapping<u64, TestamentData>,
    testament_count: u64,
}

#[stylus::external]
impl Testament {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            testaments: Mapping::default(),
            testament_count: 0,
        }
    }

    // Kreiranje testamenta
    pub fn create_testament(&mut self, lawyer: Address) -> u64 {
        self.testament_count += 1;
        let testament_id = self.testament_count;

        let testament = TestamentData {
            owner: msg::sender(),
            lawyer,
            is_active: false,
            heirs: Vec::new(),
        };

        self.testaments.insert(testament_id, testament);

        event!(TestamentCreated {
            testament_id,
            owner: msg::sender(),
            lawyer,
        });

        testament_id
    }

    // Dodavanje naslednika
    pub fn add_heir(
        &mut self,
        testament_id: u64,
        heir_address: Address,
        percentage: u64,
        document_hash: String,
    ) -> Result<(), String> {
        let sender = msg::sender();
        let mut testament = self.testaments.get(testament_id).ok_or("Testament not found")?;

        if sender != testament.owner {
            return Err("Only the owner can add heirs".into());
        }

        testament.heirs.push(Heir {
            heir_address,
            percentage,
            document_hash,
        });

        self.testaments.insert(testament_id, testament);
        Ok(())
    }

    // Pregled naslednika
    pub fn get_heirs(&self, testament_id: u64) -> Result<Vec<Heir>, String> {
        let testament = self.testaments.get(testament_id).ok_or("Testament not found")?;
        Ok(testament.heirs)
    }

    // Aktivacija testamenta od strane advokata
    pub fn activate_testament(&mut self, testament_id: u64) -> Result<(), String> {
        let sender = msg::sender();
        let mut testament = self.testaments.get(testament_id).ok_or("Testament not found")?;

        if sender != testament.lawyer {
            return Err("Only assigned lawyer can activate the testament".into());
        }

        testament.is_active = true;
        self.testaments.insert(testament_id, testament);

        event!(TestamentActivated { testament_id });

        Ok(())
    }

    // Naslednik može videti šta nasledjuje pre aktivacije
    pub fn view_inheritance(&self, testament_id: u64) -> Result<(u64, String), String> {
        let sender = msg::sender();
        let testament = self.testaments.get(testament_id).ok_or("Testament not found")?;

        for heir in &testament.heirs {
            if heir.heir_address == sender {
                return Ok((heir.percentage, heir.document_hash.clone()));
            }
        }
        Err("You are not listed as an heir".into())
    }

    // Naslednik može povući sredstva nakon aktivacije testamenta
    pub fn claim_inheritance(&mut self, testament_id: u64) -> Result<(), String> {
        let sender = msg::sender();
        let mut testament = self.testaments.get(testament_id).ok_or("Testament not found")?;

        if !testament.is_active {
            return Err("Testament is not active".into());
        }

        for heir in &testament.heirs {
            if heir.heir_address == sender {
                event!(InheritanceClaimed {
                    testament_id,
                    heir: sender,
                    amount: heir.percentage
                });
                return Ok(());
            }
        }
        Err("You are not listed as an heir".into())
    }
}

// Događaji
#[stylus::event]
struct TestamentCreated {
    testament_id: u64,
    owner: Address,
    lawyer: Address,
}

#[stylus::event]
struct TestamentActivated {
    testament_id: u64,
}

#[stylus::event]
struct InheritanceClaimed {
    testament_id: u64,
    heir: Address,
    amount: u64,
}
