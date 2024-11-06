#!/bin/bash
set -e

bash env_euphrates.sh

REPO_ROOT=$(git rev-parse --show-toplevel)
cd ${REPO_ROOT}/smart-contracts && cargo wasm


# Create artifacts directory if it doesn't exist
ARTIFACTS_DIR="./artifacts"
mkdir -p "$ARTIFACTS_DIR"

echo "dang"
for contract_dir in contracts/*/; do
    echo $contract_dir
    echo "dir"
    if [ -f "${contract_dir}Cargo.toml" ]; then
        contract_name=$(basename "$contract_dir")
        
        artifact_name="${contract_name//-/_}"
        
        # Build the contract
        echo "Building $contract_name..."
        (cd "$contract_dir" && cargo build --release --target wasm32-unknown-unknown)
        
        # Copy the compiled wasm to artifacts directory
        cp "target/wasm32-unknown-unknown/release/${artifact_name}.wasm" "$ARTIFACTS_DIR/"
    fi
done