use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId};

mod pledge;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub fundraiser: AccountId,
    pub pledges: UnorderedMap<AccountId, u128>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            fundraiser: "emilbob.testnet".parse().unwrap(),
            pledges: UnorderedMap::new(b"p"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn init(fundraiser_init: AccountId) -> Self {
        assert!(!env::state_exists(), "Contract already made!");

        Self {
            fundraiser: fundraiser_init,
            pledges: UnorderedMap::new(b"p"),
        }
    }

    pub fn get_fundraiser(&self) -> AccountId {
        self.fundraiser.clone()
    }

    #[private]
    pub fn set_fundraiser(&mut self, new_fundraiser: AccountId) {
        self.fundraiser = new_fundraiser;
    }
}
