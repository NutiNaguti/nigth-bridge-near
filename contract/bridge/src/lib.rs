use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::predecessor_account_id;
use near_sdk::{self, collections::UnorderedSet, AccountId};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault, Promise};

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    AdminList,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Bridge {
    token_address: AccountId,
    lite_client_address: AccountId,
    admin_list: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Bridge {
    #[init]
    #[private]
    pub fn init(token_address: AccountId, lite_client_address: AccountId) -> Self {
        Self {
            token_address,
            lite_client_address,
            admin_list: UnorderedSet::new(StorageKey::AdminList),
        }
    }

    pub fn validate_transfer(&self, block_number: u64, account_address: String, amount: u128) {
        // let logs_filter = Promise::new(self.lite_client_address.clone());
    }

    pub fn check_admin(&self) -> bool {
        self.admin_list.contains(&predecessor_account_id())
    }

    pub fn set_token_address(&mut self, token_address: AccountId) {
        self.token_address = token_address;
    }

    pub fn mint(&mut self, address: String, amount: u128) {
        unimplemented!()
    }

    pub fn burn(&mut self, amount: u128) {
        unimplemented!()
    }
}
