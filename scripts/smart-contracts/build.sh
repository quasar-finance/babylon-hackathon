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

cd ${REPO_ROOT}/smart-contracts

ARTIFACTS_DIR="./artifacts"
mkdir -p "$ARTIFACTS_DIR"

# we need to use the docker builder do to wasm artifact size
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0