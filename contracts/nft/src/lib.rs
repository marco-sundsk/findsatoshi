use std::collections::{HashSet, HashMap};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, Base64VecU8, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, StorageUsage};

use crate::internal::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::custom::*;
pub use crate::mining::*;
use crate::nft_metadata::{TokenMetadata, NFTMetadata};

mod internal;
mod mint;
mod nft_core;
mod nft_metadata;
mod custom;
mod mining;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

pub type TokenId = String;
pub type TokenSeqNum = String;
pub type TokenMetadataId = String;

/// 0 - normal, 1 - malfunction
pub const ST_NORMAL: u8 = 0;
pub const ST_MALFUNCTION: u8 = 1;
pub type TokenStatus = u8;
/// 0 - poweroff, 1 - poweron
pub const PW_OFF: u8 = 0;
pub const PW_ON: u8 = 1;
pub type PowerSwitch = u8;



#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MinerMetadata {
    pub category: String,
    pub thash: u32,
    pub w: u32,
}

/// the miner machine, 
/// metadata_id is its batch-type,
/// sn is its serialnum in this batch,
/// power_left is the remaining power, participate in mining would consume this one.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub sn: TokenSeqNum,
    pub owner_id: AccountId,
    pub metadata_id: TokenMetadataId,
    
    pub operator: AccountId,  // miningpoolId or owner_id itself.
    pub status: TokenStatus,
    pub switch: PowerSwitch,
    
    pub power_left: u32,
    pub power_deadline: MiningEpoch,
    // used for compatible with standards
    pub approved_account_ids: HashSet<AccountId>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// TODO: to be eliminate, update to miners_per_owner
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    
    /// each miner in one miner-type is individual, record as Token structure. 
    pub miners_per_owner: LookupMap<AccountId, UnorderedMap<TokenMetadataId, UnorderedSet<TokenId>>>,
    
    /// unlike miners, the copies of one power-nft is identical, we only record copy amount.
    pub powers_per_owner: LookupMap<AccountId, UnorderedMap<TokenMetadataId, u32>>,

    /// TokenId is formed as TokenMetadataId + "#" + TokenSeqNum
    pub tokens_by_id: UnorderedMap<TokenId, Token>,
    /// TokenMetadataId is designated by owner when mint
    pub metadata_by_id: UnorderedMap<TokenMetadataId, TokenMetadata>,

    pub owner_id: AccountId,

    /// The storage size in bytes for one account.
    pub extra_storage_in_bytes_per_token: StorageUsage,

    // pub metadata: NFTMetadata

    // mining relative member
    pub current_mining_epoch: MiningEpoch,
    pub current_total_thash: Thash,
    pub mining_pools: UnorderedMap<AccountId, MiningPool>,
    pub mining_entities: UnorderedMap<AccountId, Thash>,
    pub power_events: LookupMap<MiningEpoch, UnorderedSet<TokenId>>,

}

#[near_bindgen]
impl Contract {
    // #[init]
    // pub fn new(owner_id: ValidAccountId, metadata: NFTMetadata) -> Self {
    //     assert!(!env::state_exists(), "Already initialized");
    //     let mut this = Self {
    //         tokens_per_owner: LookupMap::new(b"a".to_vec()),
    //         tokens_by_id: UnorderedMap::new(b"t".to_vec()),
    //         owner_id: owner_id.into(),
    //         extra_storage_in_bytes_per_token: 0,
    //         metadata
    //     };

    //     this.measure_min_token_storage_cost();

    //     this
    // }

    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut this = Self {
            tokens_per_owner: LookupMap::new(b"a".to_vec()),
            miners_per_owner: LookupMap::new(b"u".to_vec()),
            powers_per_owner: LookupMap::new(b"p".to_vec()),
            tokens_by_id: UnorderedMap::new(b"t".to_vec()),
            metadata_by_id: UnorderedMap::new(b"m".to_vec()),
            owner_id: owner_id.into(),
            extra_storage_in_bytes_per_token: 0,
            current_mining_epoch: 0,
            current_total_thash: 0,
            mining_pools: UnorderedMap::new(b"mp".to_vec()),
            mining_entities: UnorderedMap::new(b"me".to_vec()),
            power_events: LookupMap::new(b"pe".to_vec())
        };

        this.measure_min_token_storage_cost();

        this
    }

    fn measure_min_token_storage_cost(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = "a".repeat(64);
        let u = UnorderedSet::new(unique_prefix(&tmp_account_id));
        self.tokens_per_owner.insert(&tmp_account_id, &u);

        let tokens_per_owner_entry_in_bytes = env::storage_usage() - initial_storage_usage;
        let owner_id_extra_cost_in_bytes = (tmp_account_id.len() - self.owner_id.len()) as u64;

        self.extra_storage_in_bytes_per_token =
            tokens_per_owner_entry_in_bytes + owner_id_extra_cost_in_bytes;

        self.tokens_per_owner.remove(&tmp_account_id);
    }
}
