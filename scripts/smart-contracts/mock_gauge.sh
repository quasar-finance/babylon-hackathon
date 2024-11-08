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

bash $CONTRACT_SCRIPTS/store.sh mock_gauge.wasm
bash $CONTRACT_SCRIPTS/instantiate.sh mock_gauge '{"owner":"bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3","destinations":["Rooch","Satlayer","Ethereum","Stacks","Merlin","Base"]}'

sleep 45

MOCK_GAUGE_ADDRESS=$(jq -r '.mock_gauge' "${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json")


babylond tx wasm execute $MOCK_GAUGE_ADDRESS '{
    "custom": {
        "upsert_allocation": {
            "destination_id": "Rooch",
            "amount": "40"
        }
    }
}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl

sleep 45

babylond tx wasm execute $MOCK_GAUGE_ADDRESS '{
    "custom": {
        "upsert_allocation": {
            "destination_id": "Satlayer",
            "amount": "40"
        }
    }
}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl

sleep 45

babylond tx wasm execute $MOCK_GAUGE_ADDRESS '{
    "custom": {
        "upsert_allocation": {
            "destination_id": "Ethereum",
            "amount": "40"
        }
    }
}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl

sleep 45

babylond tx wasm execute $MOCK_GAUGE_ADDRESS '{
    "custom": {
        "upsert_allocation": {
            "destination_id": "Stacks",
            "amount": "40"
        }
    }
}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl

sleep 45

babylond tx wasm execute $MOCK_GAUGE_ADDRESS '{
    "custom": {
        "upsert_allocation": {
            "destination_id": "Merlin",
            "amount": "40"
        }
    }
}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl

sleep 45

babylond tx wasm execute $MOCK_GAUGE_ADDRESS '{
    "custom": {
        "upsert_allocation": {
            "destination_id": "Base",
            "amount": "40"
        }
    }
}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl
