use std::collections::{HashSet, HashMap};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, Base64VecU8, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, BlockHeight};

use crate::internal::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::custom::*;
pub use crate::mining::*;
pub use crate::mining_internal::*;
use crate::nft_metadata::{TokenMetadata, MinerMetadata};

mod internal;
mod mint;
mod nft_core;
mod nft_metadata;
mod custom;
mod mining;
mod mining_internal;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

pub type TokenId = String;
pub type TokenSeqNum = String;
pub type TokenMetadataId = String;
pub type MinerMetadataId = String;

/// 0 - normal, 1 - malfunction
pub const ST_NORMAL: u8 = 0;
pub const ST_MALFUNCTION: u8 = 1;
pub type TokenStatus = u8;
/// 0 - poweroff, 1 - poweron
pub const PW_OFF: u8 = 0;
pub const PW_ON: u8 = 1;
pub type PowerSwitch = u8;


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
    pub miner_metadata_id: MinerMetadataId,
    
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
    /// owner of this contract
    pub owner_id: AccountId,

    //*********************
    //  NFT PARTS
    //*********************
    /// TokenMetadataId is designated by owner when mint
    /// Token includes miners and e-powers
    pub token_metadata_by_id: UnorderedMap<TokenMetadataId, TokenMetadata>,

    /// only miners have this metadata
    pub miner_metadata_by_id: UnorderedMap<MinerMetadataId, MinerMetadata>,

    /// each miner is a token,
    /// TokenId is formed as TokenMetadataId + "#" + TokenSeqNum
    pub miners_by_id: UnorderedMap<TokenId, Token>,

    /// user -> MinerMetadataId -> UnorderedSet<TokenId>
    pub miners_per_owner: LookupMap<AccountId, UnorderedMap<MinerMetadataId, UnorderedSet<TokenId>>>,
    
    /// user -> TokenMetadataId -> copys
    /// unlike miners, the copies of one power-nft is identical, we only record copy amount.
    pub powers_per_owner: LookupMap<AccountId, UnorderedMap<TokenMetadataId, u32>>,

    // pub metadata: NFTMetadata

    // *****************************************
    //        MINING PARTS
    // *****************************************
    
    /// each epoch, we distribute epoch_award to some miner or pool
    pub current_mining_epoch: MiningEpoch,
    pub epoch_award: Balance,
    
    /// we can only settle epoch after interval blocks from previous epoch
    pub min_interval_of_epoch: BlockHeight,
    pub current_epoch_start_at: BlockHeight,

    /// The total thash that participate in settlement.
    /// And we choose winner from entities according to their Thash.
    /// current_total_thash = Sum(mining_entities.Thash)
    pub current_total_thash: Thash,    
    pub mining_entities: UnorderedMap<AccountId, Thash>,

    /// e-power consuming when participate in settlement
    pub power_events: LookupMap<MiningEpoch, UnorderedSet<TokenId>>,

    /// mining pool supports
    pub mining_pools: UnorderedMap<AccountId, MiningPool>,

}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            miners_per_owner: LookupMap::new(b"a".to_vec()),
            powers_per_owner: LookupMap::new(b"b".to_vec()),

            miners_by_id: UnorderedMap::new(b"c".to_vec()),
            token_metadata_by_id: UnorderedMap::new(b"d".to_vec()),
            miner_metadata_by_id: UnorderedMap::new(b"e".to_vec()),
            
            owner_id: owner_id.into(),
            
            current_mining_epoch: 0,
            current_epoch_start_at: env::block_index(),
            current_total_thash: 0,
            epoch_award: 2500000000,
            min_interval_of_epoch: 3600,
            mining_pools: UnorderedMap::new(b"f".to_vec()),
            mining_entities: UnorderedMap::new(b"g".to_vec()),
            power_events: LookupMap::new(b"h".to_vec())
        }
    }
}
