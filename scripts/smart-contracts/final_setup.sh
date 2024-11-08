
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


sh $CONTRACT_SCRIPTS/mock_gauge.sh "satlayer:50,rooch:60,ethereum:20,base:30,stacks:10,babylon:40"
sleep 45

# sh $CONTRACT_SCRIPTS/dummy_oracle.sh

# sleep 45

sh $CONTRACT_SCRIPTS/babylon_vault.sh

sleep 45

sh $CONTRACT_SCRIPTS/ecosystem_adaptor.sh satlayer
sh $CONTRACT_SCRIPTS/ecosystem_adaptor.sh rooch
sh $CONTRACT_SCRIPTS/ecosystem_adaptor.sh ethereum
sh $CONTRACT_SCRIPTS/ecosystem_adaptor.sh base
sh $CONTRACT_SCRIPTS/ecosystem_adaptor.sh stacks
sh $CONTRACT_SCRIPTS/ecosystem_adaptor.sh babylon

sleep 45