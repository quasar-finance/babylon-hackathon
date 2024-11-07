// import { OfflineSigner } from '@cosmjs/proto-signing';
// import { SigningStargateClient } from '@cosmjs/stargate';

// import { OfflineSigner } from '@cosmjs/proto-signing';
// import { SigningStargateClient } from '@cosmjs/stargate';
import type { Window as KeplrWindow, OfflineAminoSigner, OfflineDirectSigner } from "@keplr-wallet/types";
import { err, ok, Result } from 'neverthrow';

declare global {
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  interface Window extends KeplrWindow {}
}

export async function connectKeplr(): Promise<Result<{"address": string, "signer": OfflineAminoSigner & OfflineDirectSigner}, string>>{
    if (!window.keplr) {
        alert('Please install Keplr extension');
        return err("Keplr was not installed");
    }
    const offlineSigner = window.keplr?.getOfflineSigner("euphrates-0.5.0");
    if (!offlineSigner) {
        return err("Failed to get offline signer");
    }
    const accounts = await offlineSigner.getAccounts();

    await addKeplr();
    await window.keplr.enable("euphrates-0.5.0");

    return ok({
        address: accounts[0].address,
        signer: offlineSigner
    });
}

async function addKeplr() {
    window.keplr.experimentalSuggestChain({
        "rpc": "https://rpc-euphrates.devnet.babylonlabs.io:443",
        "rest": "https://lcd-euphrates.devnet.babylonlabs.io:443",
        "chainId": "euphrates-0.5.0",
        "chainName": "Babylon Euphrates devnet",
        "stakeCurrency": {
            "coinDenom": "bbn",
            "coinMinimalDenom": "ubbn",
            "coinDecimals": 6,
            "coinImageUrl": "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/bbn-dev/chain.png"
        },
        "bech32Config": {
            "bech32PrefixAccAddr": "bbn",
            "bech32PrefixAccPub": "bbnpub",
            "bech32PrefixValAddr": "bbnvaloper",
            "bech32PrefixValPub": "bbnvaloperpub",
            "bech32PrefixConsAddr": "bbnvalcons",
            "bech32PrefixConsPub": "bbnvalconspub"
        },
        "bip44": {
            "coinType": 118
        },
        "currencies": [
            {
                "coinDenom": "bbn",
                "coinMinimalDenom": "ubbn",
                "coinDecimals": 6,
                "coinImageUrl": "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/bbn-dev/chain.png"
            }
        ],
        "feeCurrencies": [
            {
                "coinDenom": "bbn",
                "coinMinimalDenom": "ubbn",
                "coinDecimals": 6,
                "coinImageUrl": "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/bbn-dev/chain.png",
                "gasPriceStep": {
                    "low": 0.007,
                    "average": 0.007,
                    "high": 0.01
                }
            }
        ],
        "gasPriceStep": {
            "low": 0.05,
            "average": 0.125,
            "high": 0.2
        },
        "features": [
            "cosmwasm"
        ]
    });
}
