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

```
bash store.sh dummy_oracle.wasm
```

```
bash store.sh ecosystem_adaptor.wasm
```

instantiate.sh instantiates contracts. Refer to the contract by name of the wasm file. the .wasm can be added or not added. The callers needs to set the json of the corresponding contract correctly
```
bash scripts/smart-contracts/instantiate.sh mock_gauge '{"owner":"bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3","destinations":["bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3"]}'
```

```
bash scripts/smart-contracts/instantiate.sh dummy_oracle '{"owner":"bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3"}'
```

```
bash scripts/smart-contracts/instantiate.sh babylon_vault '{"owner":"bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3", "subdenom":"quasar-awesome-baby;on-vault-token-for-shared-security", "oracle" :"bbn17hnyglhmmzaatasaqcunee2uzdxs5294w5x2mglv469nqgj8h2lspdzzap", "gauge": "bbn15az6elygde95w38yqwkg0g98na8x5qgt4agrtmpw5jtcfg246nssnfpn8s"}'
```

```
bash scripts/smart-contracts/instantiate.sh ecosystem_adaptor '{"babylon_vault": "bbn13eu6ctese2zrvjfjnuqrk2rw654s4n2qee2eqkdtm4j9ycem52vqd4rlrp","ecosystem_info": {"connection": "connection-0","deposit_denoms": ["ubbn"],"deposit_ecosystem": "cosmoshub","destination_chain_denom": "ibc/denom","return_source_channel": "channel-0","transfer_channel": "channel-0"}, "polytone_info": {"polyton_note_contract": "bbn13eu6ctese2zrvjfjnuqrk2rw654s4n2qee2eqkdtm4j9ycem52vqd4rlrp"}}'
```