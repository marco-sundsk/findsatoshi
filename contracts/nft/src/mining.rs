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

pub struct MiningDistribution {
    pub mine_entity_id: AccountId,
    pub left_border: u32,
    pub right_border: u32,  
}

#[near_bindgen]
impl Contract {

    pub fn settle_mining_epoch(&mut self) {

        let value = self.make_random_value();

        let block_producer = self.find_block_producer(value);

        env::log(format!("Send vBTC to {} in epoch {}.", block_producer.clone(), self.current_mining_epoch).as_bytes());

        self.current_mining_epoch += 1;
        self.settle_power_for_individuals();
        self.settle_power_for_pools();
        self.settle_random_failures();
    }

    pub fn batch_poweron_miners(&mut self, token_ids: Vec<TokenId>,) {
        let owner_id = env::predecessor_account_id();
        for token_id in token_ids.iter() {
            let mut miner = self.tokens_by_id.get(token_id).expect("Miner doesn't exist");
            if miner.owner_id != owner_id || miner.owner_id != miner.operator {
                env::panic("No control of this miner.".as_bytes())
            }
            let metadata = self.get_miner_metadata(&miner);
            if miner.status == ST_NORMAL && miner.switch != PW_ON {
                miner.switch = PW_ON;
                // update total thash
                self.current_total_thash += metadata.thash;
                let owner_thash = self.mining_entities.get(&owner_id).unwrap_or(0);
                self.mining_entities.insert(&owner_id, &(owner_thash + metadata.thash));
                // update power events
                let (remaining, mining_epoch) = self.get_power_endline(miner.power_left, &metadata);
                if remaining == miner.power_left {
                    env::panic("Not enough power to use.".as_bytes())
                }
                let mut miners_set = self.power_events.get(&mining_epoch)
                    .unwrap_or(UnorderedSet::new(format!("{}", mining_epoch).as_bytes().to_vec()));
                miners_set.insert(&token_id);
                self.power_events.insert(&mining_epoch, &miners_set);
                miner.power_left = remaining;
                // udpate miner itself
                self.tokens_by_id.insert(&token_id, &miner);
            }
        }
    }

    pub fn batch_poweroff_miners(&mut self, token_ids: Vec<TokenId>,) {
        let owner_id = env::predecessor_account_id();
        for token_id in token_ids.iter() {
            let mut miner = self.tokens_by_id.get(token_id).expect("Miner doesn't exist");
            if miner.owner_id != owner_id || miner.owner_id != miner.operator {
                env::panic("No control of this miner.".as_bytes())
            }
            let metadata = self.get_miner_metadata(&miner);
            if miner.status == ST_NORMAL && miner.switch != PW_OFF {
                miner.switch = PW_OFF;
                // update total thash
                self.internal_thash_reduce(&miner.owner_id, &metadata);
                // update power events
                let mut miner_set = self.power_events.get(&miner.power_deadline).expect("Internal Error: no this power event");
                miner_set.remove(&token_id);
                if miner_set.len() > 0 {
                    self.power_events.insert(&miner.power_deadline, &miner_set);
                } else {
                    self.power_events.remove(&miner.power_deadline);
                }
                // refund power
                miner.power_left += self.get_power_refund(miner.power_deadline - self.current_mining_epoch, &metadata);
                // udpate miner itself
                self.tokens_by_id.insert(&token_id, &miner);
            }
        }
    }

    pub fn batch_add_miners_to_pool(&mut self, token_ids: Vec<TokenId>, mining_pool: AccountId) {
        let owner_id = env::predecessor_account_id();

    }

    pub fn batch_retrieve_miners_from_pool(&mut self, token_ids: Vec<TokenId>, mining_pool: AccountId) {
        let owner_id = env::predecessor_account_id();

    }

}