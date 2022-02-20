# What's that

That are my first steps in creating blockchain smart-contracts using the
[NEAR](https://near.org/) protocol.

# Where is it?



# How to deploy and test

Please replace the `ACCOUNT_NAME` placeholder with your (testnet) account.

0. Make sure you've got `wasm32-unknown-unknown` target installed:
```shell
$ rustup target add wasm32-unknown-unknown
```
1. Build
```shell
$ cargo build --target wasm32-unknown-unknown --release
```
2. Deploy
```shell
$ near dev-deploy -f --wasmFile target/wasm32-unknown-unknown/release/near_smart_contract_coinmarkercap.wasm
```
3. Add prices
```shell
$ . neardev/dev-account.env # This loads a CONTRACT_NAME variable
$ near call "$CONTRACT_NAME" record_price --accountId "$CONTRACT_NAME" --args '{"price": 1}'
$ near call "$CONTRACT_NAME" record_price --accountId "$CONTRACT_NAME" --args '{"price": 2}'
$ near call "$CONTRACT_NAME" record_price --accountId "$CONTRACT_NAME" --args '{"price": 3}'
$ near call "$CONTRACT_NAME" record_price --accountId "$CONTRACT_NAME" --args '{"price": 4}'
$ near call "$CONTRACT_NAME" record_price --accountId "$CONTRACT_NAME" --args '{"price": 5}'
```
4. Verify that the average is something about `3.0`:
```shell
$ near view "$CONTRACT_NAME" get_average --accountId "$CONTRACT_NAME"
```
5. You can also view the average from any account:
```shell
$ near view "$CONTRACT_NAME" get_average --accountId ACCOUNT_NAME
```
