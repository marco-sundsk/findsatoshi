use crate::*;

#[near_bindgen]
impl Contract {

    pub fn batch_transfer_miners(&mut self, receiver_id: ValidAccountId, token_ids: Vec<TokenId>,) {
        let sender_id = env::predecessor_account_id();
        for token_id in token_ids.iter() {
            let (_, _) = self.internal_transfer(
                &sender_id, receiver_id.as_ref(), &token_id, None, None,
            );
        }
    }

    pub fn batch_transfer_powers(&mut self, receiver_id: ValidAccountId, power_card: TokenMetadataId, amount: u32) {
        let sender_id = env::predecessor_account_id();
        // check sender has enough cards to transfer
        let mut power_map = self.powers_per_owner.get(&sender_id)
            .expect("Sender has insurficient card to transfer.");

        let num_sender_has = power_map.get(&power_card)
            .expect("Sender has insurficient card to transfer.");
        
        if num_sender_has < amount {
            env::panic("Sender has insurficient card to transfer.".as_bytes());
        }
        
        // change sender's count
        let remain = num_sender_has - amount;
        if remain > 0 {
            power_map.insert(&power_card, &remain);
            self.powers_per_owner.insert(&sender_id, &power_map);
        } else {
            power_map.remove(&power_card);
            if power_map.keys_as_vector().len() > 0 {
                self.powers_per_owner.insert(&sender_id, &power_map);
            } else {
                self.powers_per_owner.remove(&sender_id);
            }
        }
        // change receiver's count
        let mut recv_power_map = self.powers_per_owner.get(receiver_id.as_ref())
            .unwrap_or(UnorderedMap::new(unique_prefix(receiver_id.as_ref())));
        let num_recv_has = recv_power_map.get(&power_card).unwrap_or(0);
        recv_power_map.insert(&power_card, &(num_recv_has + amount));
        self.powers_per_owner.insert(receiver_id.as_ref(), &recv_power_map);
        
    }

    pub fn consume_powers_by_tokens(&mut self, power_card: TokenMetadataId, token_ids: Vec<TokenId>
    ) {

    }

    pub fn consume_powers_by_tokenmetadata(&mut self, 
        power_card: TokenMetadataId, 
        tokenmetadata_id: TokenMetadataId
    ) {
        

    }

    //**********************
    //**** VIEW FUNCTIONS **
    //**********************

    /// list all miner types created in this contract
    pub fn list_miner_types(&self, from_index: u64, limit: u64
    ) ->HashMap<TokenMetadataId, TokenMetadata> {
        let keys = self.metadata_by_id.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len())).map(
            |index| (
                keys.get(index).unwrap(), 
                self.metadata_by_id.get(&keys.get(index).unwrap()).unwrap())
        ).collect::<HashMap<_,_>>()
    }

    /// list all miner types of the user's miners
    pub fn list_miner_types_by_owner(&self, owner_id: ValidAccountId, 
        from_index: u64, limit: u64
    ) ->HashMap<TokenMetadataId, TokenMetadata> {

        let own_tokens = self.miners_per_owner.get(
            owner_id.as_ref()).unwrap_or(UnorderedMap::new(unique_prefix(owner_id.as_ref())));
            
        let keys = own_tokens.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len())).map(
            |index| (
                keys.get(index).unwrap(), 
                self.metadata_by_id.get(&keys.get(index).unwrap()).unwrap())
        ).collect::<HashMap<_,_>>()
    }

    /// list all miners belongs to the miner type of the user 
    pub fn list_miners_by_owner_and_type(&self, owner_id: ValidAccountId, type_id: TokenMetadataId,
        from_index: u64, limit: u64
    ) ->HashMap<TokenId, Token> {
        let all_tokens = self.miners_per_owner.get(owner_id.as_ref())
            .unwrap_or(UnorderedMap::new(unique_prefix(owner_id.as_ref())));
        
        let tokens_of_this_type = all_tokens.get(&type_id)
            .unwrap_or(UnorderedSet::new(unique_prefix(&String::from("no-matter"))));
        
        let tokenids = tokens_of_this_type.to_vec();

        (from_index..std::cmp::min(from_index + limit, tokenids.len() as u64)).map(
            |index| (
                (*tokenids.get(index as usize).unwrap()).clone(), 
                self.tokens_by_id.get(tokenids.get(index as usize).unwrap()).unwrap())
        ).collect::<HashMap<_,_>>()
    }

    pub fn list_miners_by_onwer(&self, owner_id: ValidAccountId, 
        from_index: u64, limit: u64
    ) ->HashMap<TokenId, Token> {
        let all_tokens = self.miners_per_owner.get(owner_id.as_ref())
            .unwrap_or(UnorderedMap::new(unique_prefix(owner_id.as_ref())));
        
        let types = all_tokens.keys_as_vector();

        let mut tokenids = Vec::<TokenId>::new();

        for type_index in 0..types.len() {
            let tokens_of_this_type = all_tokens.get(&types.get(type_index).unwrap()).unwrap();
            tokenids.extend(tokens_of_this_type.iter());
        }

        let retids = tokenids;

        (from_index..std::cmp::min(from_index + limit, retids.len() as u64)).map(
            |index| (
                (*retids.get(index as usize).unwrap()).clone(), 
                self.tokens_by_id.get(retids.get(index as usize).unwrap()).unwrap())
        ).collect::<HashMap<_,_>>()
    }

}
