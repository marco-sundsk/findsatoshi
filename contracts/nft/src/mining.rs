use crate::*;

pub type MiningEpoch = u32;
/// 1Ehash = 10**6 Thash, so u32 is enough to indicate all mining compute power
pub type Thash = u32;
/// 0 - normal, 1 - maintaining
pub type PoolStatus = u8;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MiningPool {
    pub owner_id: AccountId,
    pub name: String,
    pub status: PoolStatus,
    pub switch: PowerSwitch,
    pub total_thash: Thash,
    pub miners: UnorderedSet<TokenId>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableMiningPool {
    pub owner_id: AccountId,
    pub name: String,
    pub status: PoolStatus,
    pub switch: PowerSwitch,
    pub total_thash: Thash,
    pub miners: Vec<TokenId>,
}


#[near_bindgen]
impl Contract {

    pub fn settle_mining_epoch(&mut self) {

        self.assert_owner();

        if env::block_index() < self.current_epoch_start_at + self.min_interval_of_epoch {
            env::panic("not long from last settlement.".as_bytes())
        }
        self.current_epoch_start_at = env::block_index();

        let value = self.make_random_value();

        let block_producer = self.find_block_producer(value);

        env::log(
            format!(
                "Send vBTC to {} in epoch {}.", block_producer.clone(), self.current_mining_epoch
            ).as_bytes());

        self.current_mining_epoch += 1;
        self.settle_power_for_individuals();
        self.settle_power_for_pools();
        self.settle_random_failures();
    }

    pub fn batch_poweron_miners(&mut self, token_ids: Vec<TokenId>,) {
        let owner_id = env::predecessor_account_id();
        for token_id in token_ids.iter() {
            let mut miner = self.miners_by_id.get(token_id).expect("Miner doesn't exist");
            if miner.owner_id != owner_id || miner.owner_id != miner.operator {
                env::panic("No control of this miner.".as_bytes())
            }
            let metadata = self.miner_metadata_by_id.get(&miner.miner_metadata_id)
                .expect("Internal Error: no miner_metadata of this miner");
            if miner.status == ST_NORMAL && miner.switch != PW_ON {
                miner.switch = PW_ON;
                // update total thash
                self.internal_increase_thash(&miner.owner_id, &metadata);
                // consume power
                let (used, mining_epoch) = self.get_power_consume(miner.power_left, &metadata);
                if used == 0 {
                    env::panic("Not enough power to use.".as_bytes())
                }
                miner.power_left -= used;
                // udapte power events
                self.internal_add_to_power_event(&token_id, &mining_epoch);
                // udpate miner itself
                self.miners_by_id.insert(&token_id, &miner);
            }
        }
    }

    pub fn batch_poweroff_miners(&mut self, token_ids: Vec<TokenId>,) {
        let owner_id = env::predecessor_account_id();
        for token_id in token_ids.iter() {
            let mut miner = self.miners_by_id.get(token_id).expect("Miner doesn't exist");
            if miner.owner_id != owner_id || miner.owner_id != miner.operator {
                env::panic("No control of this miner.".as_bytes())
            }
            let metadata = self.miner_metadata_by_id.get(&miner.miner_metadata_id)
                .expect("Internal Error: no miner_metadata of this miner");
            if miner.status == ST_NORMAL && miner.switch != PW_OFF {
                miner.switch = PW_OFF;
                // update total thash
                self.internal_reduce_thash(&miner.owner_id, &metadata);
                // update power events
                self.internal_remove_from_power_event(&token_id, &miner.power_deadline);
                // refund power
                miner.power_left += self.get_power_refund(miner.power_deadline - self.current_mining_epoch, &metadata);
                // udpate miner itself
                self.miners_by_id.insert(&token_id, &miner);
            }
        }
    }

    pub fn batch_add_miners_to_pool(&mut self, token_ids: Vec<TokenId>, mining_pool: AccountId) {
        env::log(
            format!(
                "This function is underconstruction. {}, {}", token_ids.len(), mining_pool
            ).as_bytes());
    }

    pub fn batch_retrieve_miners_from_pool(&mut self, token_ids: Vec<TokenId>, mining_pool: AccountId) {
        env::log(
            format!(
                "This function is underconstruction. {}, {}", token_ids.len(), mining_pool
            ).as_bytes());

    }

}