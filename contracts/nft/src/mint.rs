use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata) {
        let initial_storage_usage = env::storage_usage();
        self.assert_owner();
        let token = Token {
            owner_id: self.owner_id.clone(),
            metadata,
            approved_account_ids: Default::default(),
        };
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        let new_token_size_in_bytes = env::storage_usage() - initial_storage_usage;
        let required_storage_in_bytes =
            self.extra_storage_in_bytes_per_token + new_token_size_in_bytes;

        deposit_refund(required_storage_in_bytes);
    }

    #[payable]
    pub fn nft_batch_mint(&mut self, token_owner: ValidAccountId, 
        token_id_prefix: TokenId, start_sn: u32, copys: u32, metadata: TokenMetadata) {

        let initial_storage_usage = env::storage_usage();
        self.assert_owner();

        for sn in start_sn..(start_sn+copys) {
            let token = Token {
                owner_id: token_owner.as_ref().clone(),
                metadata: metadata.clone(),
                approved_account_ids: Default::default(),
            };
            let token_id: String = format!("{}#{}", token_id_prefix, sn);
            assert!(
                self.tokens_by_id.insert(&token_id, &token).is_none(),
                "Token already exists"
            );
            self.internal_add_token_to_owner(&token.owner_id, &token_id);
        }

        let new_token_size_in_bytes = env::storage_usage() - initial_storage_usage;
        let required_storage_in_bytes =
            self.extra_storage_in_bytes_per_token * copys as u64 + new_token_size_in_bytes;

        deposit_refund(required_storage_in_bytes);
    }
}
