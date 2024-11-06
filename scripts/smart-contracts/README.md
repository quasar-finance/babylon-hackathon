# Scripts
These scripts simplify build and deploying of the project. Due to 30 second block times, some of the scripts are very slow

you should have babylond, jq and git installed to run all scripts.

`setup.sh` sets up a prefunded key. If the funds run out, use the faucet with
```
curl $faucetUrl/claim \  -H "Content-Type: multipart/form-data" \                            
  -d '{ "address": "bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3"}'
```

To setup local env vars
```
source env_euphrates.sh
```

store.sh stores codes. The argument needs to be the wasm file located in smart-contracts/artifacts
```
bash store.sh babylon_vault.wasm
```

instantiate.sh instantiates contracts. Refer to the contract by name of the wasm file. the .wasm can be added or not added. The callers needs to set the json of the corresponding contract correctly
```
bash scripts/smart-contracts/instantiate.sh mock_gauge '{"owner":"bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3","destinations":["bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3"]}'
```
