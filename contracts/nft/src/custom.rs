use crate::*;

#[near_bindgen]
impl Contract {

    pub fn batch_transfer(&mut self, receiver_id: ValidAccountId, token_ids: Vec<TokenId>,) {
        let sender_id = env::predecessor_account_id();
        for token_id in token_ids.iter() {
            let (_, _) = self.internal_transfer(
                &sender_id, receiver_id.as_ref(), &token_id, None, None,
            );
        }
    }

    pub fn list_miner_types(&self, from_index: u64, limit: u64) ->HashMap<TokenMetadataId, TokenMetadata> {
        let keys = self.metadata_by_id.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len())).map(
            |index| (
                keys.get(index).unwrap(), 
                self.metadata_by_id.get(&keys.get(index).unwrap()).unwrap())
        ).collect::<HashMap<_,_>>()
    }

    pub fn list_miner_types_by_owner(&self, owner_id: ValidAccountId, 
        from_index: u64, limit: u64) ->HashMap<TokenMetadataId, TokenMetadata> {

        let own_tokens = self.miners_per_owner.get(
            owner_id.as_ref()).unwrap_or(UnorderedMap::new(unique_prefix(owner_id.as_ref())));
            
        let keys = own_tokens.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len())).map(
            |index| (
                keys.get(index).unwrap(), 
                self.metadata_by_id.get(&keys.get(index).unwrap()).unwrap())
        ).collect::<HashMap<_,_>>()
    }

    pub fn list_miners_by_owner_and_type(&self, owner_id: ValidAccountId, type_id: TokenMetadataId,
        from_index: u64, limit: u64) ->HashMap<TokenId, Token> {
        let all_tokens = self.miners_per_owner.get(owner_id.as_ref())
            .unwrap_or(UnorderedMap::new(unique_prefix(owner_id.as_ref())));
        
        let tokens = all_tokens.get(&type_id)
            .unwrap_or(UnorderedSet::new(unique_prefix(&String::from("no-matter"))));
        
        let tokenids = tokens.to_vec();

        (from_index..std::cmp::min(from_index + limit, tokenids.len() as u64)).map(
            |index| (
                (*tokenids.get(index as usize).unwrap()).clone(), 
                self.tokens_by_id.get(tokenids.get(index as usize).unwrap()).unwrap())
        ).collect::<HashMap<_,_>>()
    }

}
