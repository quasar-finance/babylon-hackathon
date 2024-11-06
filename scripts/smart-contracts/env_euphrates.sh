export binary="babylond"
export chainId="euphrates-0.5.0"
export homeDir="~/.babylond"

export userKey="user"
export keyringBackend="--keyring-backend=test"
export feeToken="bbn"

export rpcUrl="https://rpc-euphrates.devnet.babylonlabs.io"
export nodeUrl="$rpcUrl"
export grpcUrl="grpc-euphrates.devnet.babylonlabs.io:443"
export faucetUrl="https://faucet-euphrates.devnet.babylonlabs.io"
export address="bbn1knv468atwzjk4v0d22jwa497v0sd0zez3lh7g3"

alias babylond='babylond --home $homeDir'
