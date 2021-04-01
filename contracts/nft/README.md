# TBD


```shell
export OWNERID=findsatoshi-owner.testnet
export NFTID=findsatoshi-nft.testnet
```


### mint token

```shell
near call $NFTID nft_mint '{"token_id": "testtoken01", 
    "metadata": {"title": "熊猫-T80S", 
        "description": "nft-description", 
        "media": null, 
        "media_hash": null, 
        "copies": "500", 
        "issued_at": "2021-04-01T14:00:01Z", 
        "expires_at": null, 
        "starts_at": "2021-04-01T14:00:01Z", 
        "updated_at": "2021-04-01T14:00:01Z", 
        "extra": "{\"id\": \"miner001\", \"type\": \"熊猫-T80S\", \"Thash\": 110, \"W\": 3250}", 
        "reference": null,
        "reference_hash": null}}' --account_id=$OWNERID --amount=1

near call $NFTID nft_mint '{"token_id": "testtoken02", 
    "metadata": {"title": "nft-title2", 
        "description": "nft-description2", 
        "copies": "10", 
        "issued_at": "2021-04-01T14:00:01Z", 
        "expires_at": null, 
        "starts_at": "2021-04-01T14:00:01Z", 
        "updated_at": "2021-04-01T14:00:01Z", 
        "extra": "no-extra", 
        "reference": null,
        "reference_hash": null}}' --account_id=$OWNERID --amount=1        
```

### build and deploy

```shell
source build.sh

near deploy $NFTID res/nft.wasm --account_id=$NFTID

near call $NFTID new '{"owner_id": "findsatoshi-owner.testnet"}' --account_id=$NFTID

near view $NFTID nft_metadata
```