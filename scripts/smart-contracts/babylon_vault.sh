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

DUMMY_ORACLE_ADDRESS=$(jq -r '.dummy_oracle' "${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json")
MOCK_GAUGE_ADDRESS=$(jq -r '.mock_gauge' "${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json")


bash $CONTRACT_SCRIPTS/store.sh babylon_vault.wasm
bash $CONTRACT_SCRIPTS/instantiate.sh babylon_vault '{"owner":"bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3", "subdenom":"quasar-awesome-babylon-vault-token-for-shared-security", "oracle" :"'$DUMMY_ORACLE_ADDRESS'", "gauge": "'$MOCK_GAUGE_ADDRESS'"}'

sleep 10

BABYLON_VAULT_ADDRESS=$(jq -r '.babylon_vault' "${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json")
babylond tx wasm execute $BABYLON_VAULT_ADDRESS '{"register_lst":{"denom":"ubbn"}}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl
