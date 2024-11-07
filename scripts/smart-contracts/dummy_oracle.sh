#!/bin/bash
set -eo pipefail

for cmd in babylond jq git; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is required but not installed."
        exit 1
    fi
done

REPO_ROOT=$(git rev-parse --show-toplevel)
CONTRACT_SCRIPTS=${REPO_ROOT}/scripts/smart-contracts

source ${REPO_ROOT}/scripts/smart-contracts/env_euphrates.sh

bash $CONTRACT_SCRIPTS/store.sh dummy_oracle.wasm
bash $CONTRACT_SCRIPTS/instantiate.sh dummy_oracle '{"owner":"bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3"}'

sleep 45

DUMMY_ORACLE_ADDRESS=$(jq -r '.dummy_oracle' "${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json")


babylond tx wasm execute $DUMMY_ORACLE_ADDRESS '{
    "set_price": {
        "denom": "ubbn",
        "price": "100"
    }
}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl
