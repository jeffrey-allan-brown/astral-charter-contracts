use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Accounts {
  accounts: LookupMap<String, String>,
}

impl Default for Accounts {
  fn default() -> Self {
    Self {
      accounts: LookupMap::new(b"r".to_vec()),
    }
  }
}

#[near_bindgen]
impl Accounts {
  pub fn set_status(&mut self, message: String) {
    let account_id = env::signer_account_id();
    self.accounts.insert(&account_id, &message);
  }

  pub fn get_status(&self, account_id: String) -> Option<String> {
    return self.accounts.get(&account_id);
  }
}