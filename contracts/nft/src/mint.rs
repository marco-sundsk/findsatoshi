use crate::*;

const TGAS: f32 = 0.000000000001;

#[near_bindgen]
impl Contract {

    /// Find relations of iterator and gas
    pub fn see_gas(&mut self, copies: U64) -> U64 {
        let quantity: u64 = copies.into();
        
        self.current_total_thash = quantity as u32 * 100;
        let value = self.make_random_value();
        let biggest_value = self.current_total_thash as u32 - 1;
        env::log(
            format!(
                "random is {}, we take {} as max.", value, biggest_value
            ).as_bytes());

        let keys = self.miner_metadata_by_id.keys_as_vector();

        let mut border: Thash = 0;

        for index in 0..quantity {
            let non_relevant1 = keys.get(0).unwrap();
            let non_relevant2 = self.miner_metadata_by_id.get(&String::from("mid-001")).unwrap();
            let thash = 100;
            border += thash;
            if border > biggest_value {
                env::log(
                    format!(
                        "Found : #{} as winner.", index
                    ).as_bytes());
                break;
            }
        }

        env::log(
            format!(
                "used gas: {}.", env::used_gas() as f32 * TGAS
            ).as_bytes());
        
        (value as u64).into()
    }

    /// mint miner nft
    pub fn create_new_miners(&mut self, token_owner: ValidAccountId, 
        metadata_id: TokenMetadataId, metadata: TokenMetadata
    ) {
        
        self.assert_owner();

        let quantity: u64 = metadata.copies.unwrap_or(1.into()).into();

        assert!(
            self.token_metadata_by_id.insert(&metadata_id, &metadata).is_none(),
            "This ID already exists"
        );

        let miner_metadata: MinerMetadata = near_sdk::serde_json::from_str(&metadata.extra.unwrap()).expect("extra msg illegal!");
        assert!(
            self.miner_metadata_by_id.insert(&metadata_id, &miner_metadata).is_none(),
            "This ID already exists"
        );

        for sn_number in 0..quantity {
            let token = Token {
                sn: format!("{}", sn_number),
                owner_id: token_owner.as_ref().clone(),
                metadata_id: metadata_id.clone(),
                miner_metadata_id: metadata_id.clone(),

                operator: token_owner.as_ref().clone(),
                status: 0,
                switch: 0,

                power_left: 0,
                power_deadline: 0,
                approved_account_ids: Default::default(),
            };
            let token_id: String = format!("{}#{}", token.metadata_id, token.sn);
            assert!(
                self.miners_by_id.insert(&token_id, &token).is_none(),
                "Miner already exists"
            );
            self.internal_add_token_to_owner(&token.owner_id, &token_id);
        }
    }

    /// mint power card
    pub fn issue_power_cards(&mut self, power_owner: ValidAccountId, 
        metadata_id: TokenMetadataId, metadata: TokenMetadata
    ) {

        self.assert_owner();

        let quantity: u64 = metadata.copies.unwrap_or(1.into()).into();

        assert!(
            self.token_metadata_by_id.insert(&metadata_id, &metadata).is_none(),
            "This ID already exists"
        );

        let mut power_map = self.powers_per_owner.get(power_owner.as_ref())
            .unwrap_or_else(|| UnorderedMap::new(unique_power_prefix(power_owner.as_ref())));
        power_map.insert(&metadata_id, &(quantity as u32));
        self.powers_per_owner.insert(power_owner.as_ref(), &power_map);
    }

}
