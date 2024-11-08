#!/bin/bash
set -eo pipefail

# Add check for required argument
if [ -z "$1" ]; then
    echo "Error: Destination name argument is required"
    echo "Usage: $0 <destination_name>"
    exit 1
fi

DESTINATION_NAME=$1

for cmd in babylond jq git; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is required but not installed."
        exit 1
    fi
done

REPO_ROOT=$(git rev-parse --show-toplevel)
CONTRACT_SCRIPTS=${REPO_ROOT}/scripts/smart-contracts

source ${REPO_ROOT}/scripts/smart-contracts/env_euphrates.sh

bash $CONTRACT_SCRIPTS/store.sh ecosystem_adaptor.wasm

BABYLON_VAULT_ADDRESS=$(jq -r '.babylon_vault' "${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json")
bash $CONTRACT_SCRIPTS/instantiate.sh ecosystem_adaptor '{"babylon_vault": "'$BABYLON_VAULT_ADDRESS'","ecosystem_info": {"connection": "connection-0","deposit_denoms": ["ubbn"],"deposit_ecosystem": "'$DESTINATION_NAME'","destination_chain_denom": "ibc/denom","return_source_channel": "channel-0","transfer_channel": "channel-0"}, "polytone_info": {"polyton_note_contract": "'$BABYLON_VAULT_ADDRESS'"}}'

sleep 45

ECOSYSTEM_ADAPTOR_ADDRESS=$(jq -r '.ecosystem_adaptor' "${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json")

babylond tx wasm execute $BABYLON_VAULT_ADDRESS '{"register_destination":{"destination":"'$DESTINATION_NAME'","adaptor":"'$ECOSYSTEM_ADAPTOR_ADDRESS'"}}' $keyringBackend --from=$userKey --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl
