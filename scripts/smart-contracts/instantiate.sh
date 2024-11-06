#!/bin/bash
set -eo pipefail

for cmd in babylond jq git; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is required but not installed."
        exit 1
    fi
done

REPO_ROOT=$(git rev-parse --show-toplevel)

source ${REPO_ROOT}/scripts/smart-contracts/env_euphrates.sh

if [ $# -ne 2 ]; then
    echo "Usage: $0 <filename> <init_json>"
    exit 1
fi

INPUT=$1
INIT_JSON=$2

get_code_id() {
    local filename=$1
    local contract_name=${filename%.wasm}  # Remove .wasm extension
    local code_id=$(jq -r ".[\"$contract_name\"]" "${REPO_ROOT}/scripts/smart-contracts/code_ids.json")
    
    if [ "$code_id" == "null" ]; then
        echo "Error: Contract $contract_name not found in code_ids.json"
        exit 1
    fi
    echo $code_id
}


CODE_ID=$(get_code_id "$INPUT")
echo "Using Code ID: $CODE_ID"


res=$(babylond tx wasm instantiate $CODE_ID "$INIT_JSON" $keyringBackend --from=$userKey --admin=$address --label="test-label" --gas=auto --gas-prices 0.01u$feeToken --gas-adjustment=1.3 --chain-id=$chainId -b=sync -y --log_format=json -o "json" --node $nodeUrl)
txhash=$(echo "$res" | jq -r '.txhash')
echo "Transaction hash: $txhash"
sleep 45
address=$(babylond q tx $txhash -o json --node $nodeUrl | jq -r '.events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')
echo "Contract address: $address"

json_file="${REPO_ROOT}/scripts/smart-contracts/contract_addresses.json"

if [ ! -f "$json_file" ]; then
    echo "{}" > "$json_file"
fi

# Set the code id in the json file
filename=$(basename "$INPUT" .wasm)
jq --arg name "$filename" --arg id "$address" \
    '. + {($name): $id}' "$json_file" > "$json_file.tmp" && mv "$json_file.tmp" "$json_file"