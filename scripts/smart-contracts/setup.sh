
set -e

REPO_ROOT=$(git rev-parse --show-toplevel)

babylond keys add user --recover --source ${REPO_ROOT}/scripts/smart-contracts/mnemonic.key --keyring-backend test