#!/bin/bash
set -eo pipefail

# Check if artifact name is provided
if [ -z "$1" ]; then
    echo "Error: Artifact name is required"
    echo "Usage: $0 <artifact-name>"
    exit 1
fi

artifact="$1"

for cmd in babylond jq git; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is required but not installed."
        exit 1
    fi
done

REPO_ROOT=$(git rev-parse --show-toplevel)

source ${REPO_ROOT}/scripts/smart-contracts/env_euphrates.sh

cd ${REPO_ROOT}/smart-contracts/artifacts

echo "Storing $(basename "$artifact")..."
res=$(babylond tx wasm store "$artifact" $keyringBackend --from $userKey --chain-id $chainId --gas 50000000 --gas-prices 0.01u$feeToken --node $nodeUrl -y -b sync -o "json")
txhash=$(echo "$res" | jq -r '.txhash')
echo "Transaction hash: $txhash"
sleep 45
code_id=$(babylond q tx $txhash -o json --node $nodeUrl | jq -r '.events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')
echo "Code ID: $code_id"

json_file="${REPO_ROOT}/scripts/smart-contracts/code_ids.json"

if [ ! -f "$json_file" ]; then
    echo "{}" > "$json_file"
fi

# Set the code id in the json file
filename=$(basename "$artifact" .wasm)
jq --arg name "$filename" --arg id "$code_id" \
    '. + {($name): $id}' "$json_file" > "$json_file.tmp" && mv "$json_file.tmp" "$json_file"
