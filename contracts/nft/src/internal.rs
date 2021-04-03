use crate::*;

use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

const YOCTO_MULTIPLIER: f32 = 0.000000000000000000000001;

/// Price per 1 byte of storage from mainnet config after `1.18.0` release and protocol version `42`.
/// It's 10 times lower than the genesis price.
pub(crate) const STORAGE_PRICE_PER_BYTE: Balance = 10_000_000_000_000_000_000;

pub(crate) fn unique_prefix(account_id: &AccountId) -> Vec<u8> {
    let mut prefix = Vec::with_capacity(33);
    prefix.push(b'o');
    prefix.extend(env::sha256(account_id.as_bytes()));
    prefix
}

pub(crate) fn unique_power_prefix(account_id: &AccountId) -> Vec<u8> {
    let mut prefix = Vec::with_capacity(33);
    prefix.push(b'p');
    prefix.extend(env::sha256(account_id.as_bytes()));
    prefix
}

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoⓃ ({} Ⓝ )",
        YOCTO_MULTIPLIER
    )
}

pub(crate) fn assert_self() {
    assert_eq!(
        env::predecessor_account_id(),
        env::current_account_id(),
        "Method is private"
    );
}

pub(crate) fn deposit_refund(storage_used: u64) {
    let required_cost = STORAGE_PRICE_PER_BYTE * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoⓃ to cover storage ({} Ⓝ )",
        required_cost,
        required_cost as f32  * YOCTO_MULTIPLIER
    );

    let refund = attached_deposit - required_cost;
    if refund > 0 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.len() as u64 + 4
}

pub(crate) fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashSet<AccountId>,
) -> Promise {
    let storage_released: u64 = approved_account_ids
        .iter()
        .map(bytes_for_approved_account_id)
        .sum();
    Promise::new(account_id).transfer(Balance::from(storage_released) * STORAGE_PRICE_PER_BYTE)
}


impl Contract {

    pub(crate) fn make_random_value(&self) -> Thash {

        let randomness = env::random_seed();
        let ptr: *const u8 = randomness.as_ptr();
        let ptr: *const u128 = ptr as *const u128;
        let big_rand: u128 = unsafe { *ptr };

        let value = U256::from(self.current_total_thash) * U256::from(big_rand) 
            / (U256::from(u128::max_value()) + U256::from(1));
        
        env::log(format!("Random number is {} in epoch {}.", value.as_u128(), self.current_mining_epoch).as_bytes());

        value.as_u128() as Thash
    }

    pub(crate) fn find_block_producer(&self, value: Thash) -> AccountId {
        let keys = self.mining_entities.keys_as_vector();
        let mut border: Thash = 0;
        let mut ret = self.owner_id.clone();
        for index in 0..keys.len() {
            let entity = keys.get(index).unwrap();
            let thash = self.mining_entities.get(&entity).unwrap();
            border += thash;
            if border > value {
                ret = entity;
                break;
            }
        }
        env::log(format!("{} produced rbtc block in {}.", ret, self.current_mining_epoch).as_bytes());
        ret
    }

    pub(crate) fn get_miner_metadata(&self, token: &Token) -> MinerMetadata {
        let extra = self.metadata_by_id.get(&token.metadata_id)
            .expect("Internal Error: No metadata").extra.expect("Internal Error: No extra");
        near_sdk::serde_json::from_str(&extra).unwrap()
    }

    pub(crate) fn get_power_endline(&self, power_left: u32, metadata: &MinerMetadata) -> (u32, MiningEpoch) {
        let hours = power_left / metadata.w;
        let remain = power_left - hours * metadata.w;
        (remain, self.current_mining_epoch + hours)
    }

    pub(crate) fn get_power_refund(&self, epoch_diff: u32, metadata: &MinerMetadata) -> u32 {
        let hours = epoch_diff;
        hours * metadata.w
    }

    pub(crate) fn internal_thash_reduce(&mut self, owner_id: &AccountId, metadata: &MinerMetadata) {
        // update total thash
        self.current_total_thash -= metadata.thash;
        let owner_thash = self.mining_entities.get(owner_id).expect("Internal Error: no this mining entity");
        let thash_leftover = owner_thash - metadata.thash;
        if thash_leftover > 0 {
            self.mining_entities.insert(owner_id, &thash_leftover);
        } else {
            self.mining_entities.remove(owner_id);
        }
    }

    /// called in the end of mining settlement each epoch,
    /// to update power consume
    pub(crate) fn settle_power_for_individuals(&mut self) {
        env::log(format!("settle_power_for_individuals.").as_bytes());
        let miners = self.power_events.get(&self.current_mining_epoch)
            .unwrap_or(UnorderedSet::new(b"non-relevant".to_vec()));
        for token_id in miners.iter() {
            let mut miner = self.tokens_by_id.get(&token_id).expect("Internal Error: Miner not exist.");
            miner.switch = PW_OFF;
            self.tokens_by_id.insert(&token_id, &miner);

            let extra = self.metadata_by_id.get(&miner.metadata_id)
                .expect("Internal Error: No metadata").extra.expect("Internal Error: No extra");
            let metadata: MinerMetadata = near_sdk::serde_json::from_str(&extra).unwrap();
            self.internal_thash_reduce(&miner.owner_id, &metadata);
        }
        self.power_events.remove(&self.current_mining_epoch);
    }
    
    pub(crate) fn settle_power_for_pools(&mut self) {
        env::log(format!("settle_power_for_pools, under construction.").as_bytes());
    }

    pub(crate) fn settle_random_failures(&mut self) {
        env::log(format!("settle_random_failures, under construction.").as_bytes());
    }

    //*********************************************************

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );
    }

    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            .unwrap_or_else(|| UnorderedSet::new(unique_prefix(account_id)));
        tokens_set.insert(token_id);
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            .expect("Token should be owned by the sender");
        tokens_set.remove(token_id);
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }

    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        enforce_owner_id: Option<&ValidAccountId>,
        memo: Option<String>,
    ) -> (AccountId, HashSet<AccountId>) {
        let Token {
            sn,
            owner_id,
            metadata_id,
            operator,
            switch,
            status,
            power_left,
            power_deadline,
            approved_account_ids,
        } = self.tokens_by_id.get(token_id).expect("Token not found");
        if sender_id != &owner_id && !approved_account_ids.contains(sender_id) {
            env::panic(b"Unauthorized");
        }

        if let Some(enforce_owner_id) = enforce_owner_id {
            assert_eq!(
                &owner_id,
                enforce_owner_id.as_ref(),
                "The token owner is different from enforced"
            );
        }

        assert_ne!(
            &owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        env::log(
            format!(
                "Transfer {} from @{} to @{}",
                token_id, &owner_id, receiver_id
            )
            .as_bytes(),
        );

        self.internal_remove_token_from_owner(&owner_id, token_id);
        self.internal_add_token_to_owner(receiver_id, token_id);

        let token = Token {
            sn,
            owner_id: receiver_id.clone(),
            metadata_id,
            operator,
            switch,
            status,
            power_left,
            power_deadline,
            approved_account_ids: Default::default(),
        };
        self.tokens_by_id.insert(token_id, &token);

        if let Some(memo) = memo {
            env::log(format!("Memo: {}", memo).as_bytes());
        }

        (owner_id, approved_account_ids)
    }
}
