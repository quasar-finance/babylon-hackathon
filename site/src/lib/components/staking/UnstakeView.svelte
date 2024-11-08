<script lang="ts">
    import { connectKeplr } from '$lib/keplr';
    import CurrencyInput from './CurrencyInput.svelte';
    import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
    import { Decimal } from "@cosmjs/math";
    import contractAddresses from '$lib/config/contract_addresses.json';

    let inputValue = '1';
    let inputDenom = 'BLT'
    $: outputValue = inputValue;
    $: outputUSD = `$${(parseFloat(inputValue) * 9.34).toLocaleString()}`;
    
    const VAULT_CONTRACT_ADDRESS = contractAddresses.babylon_vault;

    function convertToMicroUnits(value: string): string {
        if (!value) return '0';
        return (parseFloat(value) * 1_000_000).toString();
    }

    async function handleWithdraw() {
        try {
            const wallet = await connectKeplr();
            if (wallet.isOk()) {
                const signer = wallet.value.signer;
                const address = wallet.value.address;

                const client = await SigningCosmWasmClient.connectWithSigner("https://rpc-euphrates.devnet.babylonlabs.io:443", signer, {
                    gasPrice: {
                        amount: Decimal.fromUserInput("0.1", 1),
                        denom: "ubbn"
                    }
                });

                const msg = {
                    withdraw: {
                        amount: convertToMicroUnits(inputValue)
                    }
                };

                const response = await client.execute(
                    address,
                    VAULT_CONTRACT_ADDRESS,
                    msg,
                    'auto',
                    ""
                );

                console.log('Withdrawal successful:', response);
            }
        } catch (error) {
            console.error('Withdrawal failed:', error);
        }
    }

    function handleInput(value) {
        inputValue = value;
        outputUSD = `$${(parseFloat(value) * 100000).toLocaleString()}`;
    }

    function setHalf() {
        inputValue = (parseFloat(inputValue) / 2).toString();
    }

    function setMax() {
        inputValue = '4.2'; // Example max value, replace as needed
    }
</script>

<CurrencyInput
    label="BLT"
    value={inputValue}
    onInput={handleInput}
    onHalf={setHalf}
    onMax={setMax}
    options={["BLT"]}
/>

<div class="output">
    <p>You will get</p>
    <div>
        <span>{outputValue}</span>
        <span class="text-sm text-gray-600">{outputUSD}</span>
    </div>
</div>

<button class="withdraw-button" on:click={handleWithdraw}>Withdraw</button>

<style>
	.output {
		display: flex;
		justify-content: space-between;
		align-items: center;
		background-color: #eaeaea;
		padding: 0.5rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}
	.withdraw-button {
		background-color: #000;
		color: #fff;
		padding: 0.75rem;
		border: none;
		border-radius: 8px;
		font-weight: bold;
		cursor: pointer;
		width: 100%;
		margin-bottom: 1rem;
	}
</style>
