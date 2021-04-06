use crate::*;

/// This structure is from NEAR-NFT-Standard 
/// to indicate top-level infomation of NFT managed by this contract
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTMetadata {
    spec: String, // required, essentially a version like "nft-1.0.0"
    name: String, // required, ex. "Mosaics"
    symbol: String, // required, ex. "MOSIAC"
    icon: Option<String>, // Data URL
    base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    reference: Option<String>, // URL to a JSON file with more info
    reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

/// This structure is from NEAR-NFT-Standard 
/// to indicate NFT token's metadata
/// Custom information are store in extra field with json-str 
/// And copies is the token amount that belongs to this metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    title: Option<String>, // used as Category: Miner or Power;
    description: Option<String>, // used as Sub-category: Miner types, Power types,
    media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<U64>, // number of copies of this kind of nft.
    issued_at: Option<String>, // ISO 8601 datetime when token was issued or minted
    expires_at: Option<String>, // ISO 8601 datetime when token expires
    starts_at: Option<String>, // ISO 8601 datetime when token starts being valid
    updated_at: Option<String>, // ISO 8601 datetime when token was last updated
    pub extra: Option<String>, // JSON-string: {"Thash": nnnn, "W": nnn} for Miner, {"class": "fire/water/nulcear", ...}
    reference: Option<String>, // URL to an off-chain JSON file with more info.
    reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

/// custom metadata of Miner Machine, parsed from TokenMetadata::extra
/// This is the actual Miner Type structure we used allthrough this contract
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MinerMetadata {
    pub producer: String,
    pub category: String,
    pub thash: u32,
    pub w: u32,
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self) -> NFTMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTMetadata {
        // self.metadata.clone()
        NFTMetadata {
            spec: String::from("findsatoshi-nft-1.0.0"),
            name: String::from("FindsatoshiNft"),
            symbol: String::from("FST"),
            icon: Some(String::from("")),
            base_uri: Some(String::from("")),
            reference: None,
            reference_hash: None,
        }
    }
}