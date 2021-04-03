use crate::*;

use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
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

    pub(crate) fn get_power_consume(&self, power_left: u32, metadata: &MinerMetadata) -> (u32, MiningEpoch) {
        let hours = power_left / metadata.w;
        (hours * metadata.w, self.current_mining_epoch + hours)
    }

    pub(crate) fn get_power_refund(&self, epoch_diff: u32, metadata: &MinerMetadata) -> u32 {
        let hours = epoch_diff;
        hours * metadata.w
    }

    pub(crate) fn internal_increase_thash(&mut self, owner_id: &AccountId, metadata: &MinerMetadata) {
        self.current_total_thash += metadata.thash;
        let owner_thash = self.mining_entities.get(owner_id).unwrap_or(0);
        self.mining_entities.insert(owner_id, &(owner_thash + metadata.thash));
    }

    pub(crate) fn internal_reduce_thash(&mut self, owner_id: &AccountId, metadata: &MinerMetadata) {
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

    pub(crate) fn internal_add_to_power_event(&mut self, token_id: &TokenId, deadline: &MiningEpoch) {
        let mut miners_set = self.power_events.get(deadline)
            .unwrap_or(UnorderedSet::new(format!("w{}", deadline).as_bytes().to_vec()));
        miners_set.insert(&token_id);
        self.power_events.insert(deadline, &miners_set);
    }

    pub(crate) fn internal_remove_from_power_event(&mut self, token_id: &TokenId, deadline: &MiningEpoch) {
        let mut miner_set = self.power_events.get(deadline).expect("Internal Error: no this power event");
        miner_set.remove(token_id);
        if miner_set.len() > 0 {
            self.power_events.insert(deadline, &miner_set);
        } else {
            self.power_events.remove(deadline);
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

            let miner_metadata: MinerMetadata = self.miner_metadata_by_id.get(&miner.miner_metadata_id)
                .expect("Internal Error: No miner_metadata");

            self.internal_reduce_thash(&miner.owner_id, &miner_metadata);
        }
        self.power_events.remove(&self.current_mining_epoch);
    }
    
    pub(crate) fn settle_power_for_pools(&mut self) {
        env::log(format!("settle_power_for_pools, under construction.").as_bytes());
    }

    pub(crate) fn settle_random_failures(&mut self) {
        env::log(format!("settle_random_failures, under construction.").as_bytes());
    }
}