#!/bin/bash
set -eo pipefail

REPO_ROOT=$(git rev-parse --show-toplevel)

cp "$REPO_ROOT/scripts/smart-contracts/contract_addresses.json" "$REPO_ROOT/site/src/"
