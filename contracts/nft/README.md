# TBD


```shell
export OWNERID=findsatoshi-owner.testnet
export NFTID=findsatoshi-nft.testnet
```


### mint token

```shell
```

### build and deploy

```shell
source build.sh

near deploy $NFTID res/nft.wasm --account_id=$NFTID

near call $NFTID new '{"owner_id": "findsatoshi-owner.testnet"}' --account_id=$NFTID

near view $NFTID nft_metadata
```