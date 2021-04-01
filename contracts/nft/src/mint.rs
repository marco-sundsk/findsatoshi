use crate::*;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn create_new_miners(&mut self, token_owner: ValidAccountId, 
        metadata_id: TokenMetadataId, quantity: u32, metadata: TokenMetadata) {

        let initial_storage_usage = env::storage_usage();
        self.assert_owner();

        assert!(
            self.metadata_by_id.insert(&metadata_id, &metadata).is_none(),
            "miners already exists"
        );

        for sn_number in 0..quantity {
            let token = Token {
                sn: format!("{}", sn_number),
                owner_id: token_owner.as_ref().clone(),
                metadata_id: metadata_id.clone(),
                status: 0,
                approved_account_ids: Default::default(),
            };
            let token_id: String = format!("{}#{}", token.metadata_id, token.sn);
            assert!(
                self.tokens_by_id.insert(&token_id, &token).is_none(),
                "Token already exists"
            );
            self.internal_add_token_to_owner(&token.owner_id, &token_id);
        }

        let new_token_size_in_bytes = env::storage_usage() - initial_storage_usage;
        let required_storage_in_bytes =
            self.extra_storage_in_bytes_per_token * quantity as u64 + new_token_size_in_bytes;

        deposit_refund(required_storage_in_bytes);
    }

}
