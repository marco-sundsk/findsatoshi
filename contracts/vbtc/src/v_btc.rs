use crate::*;
use near_sdk::{env, log, Balance};

#[near_bindgen]
impl Contract {
    /// mint vbtc.
    /// Requirements:
    /// * The predecessor account must be minter.
    /// * No need to deposit near, cause minter would care about the storage fee of this contract.
    /// * If account is not registered, will be auto registered.
    pub fn mint(&mut self, amount: Balance, receiver_id: ValidAccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.minter_id,
             "Only minter can mint vBTC");
        if !self.ft.accounts.contains_key(receiver_id.as_ref()) {
            // Not registered, register
            self.ft.internal_register_account(receiver_id.as_ref());
        }
        self.ft.internal_deposit(receiver_id.as_ref(), amount);
        log!("Mint {} sa-vBTC to {}", amount, receiver_id);
    }

}
