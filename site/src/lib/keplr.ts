// import { OfflineSigner } from '@cosmjs/proto-signing';
// import { SigningStargateClient } from '@cosmjs/stargate';

// import { OfflineSigner } from '@cosmjs/proto-signing';
// import { SigningStargateClient } from '@cosmjs/stargate';
import type { Window as KeplrWindow } from "@keplr-wallet/types";
import { err, ok, Result } from 'neverthrow';

declare global {
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  interface Window extends KeplrWindow {}
}

export async function connectKeplr(): Promise<Result<unknown, string>>{
    if (!window.keplr) {
        alert('Please install Keplr extension');
        return err("Keplr was not installed");
    }
    await window.keplr?.enable("osmosis-1");
    const offlineSigner = window.keplr?.getOfflineSigner("osmosis-1");
    if (!offlineSigner) {
        return err("Failed to get offline signer");
    }
    const accounts = await offlineSigner.getAccounts();

    return ok({
        address: accounts[0].address,
        signer: offlineSigner
    });
}

