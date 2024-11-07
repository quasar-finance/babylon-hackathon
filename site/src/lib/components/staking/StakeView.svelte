<script lang="ts">
	import { connectKeplr } from '$lib/keplr';
	import { error } from '@sveltejs/kit';
	import CurrencyInput from './CurrencyInput.svelte';
	import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
	import { coin } from '@cosmjs/stargate';
	import { Decimal } from "@cosmjs/math";
	import contractAddresses from '$lib/config/contract_addresses.json';


	let inputValue = '1';
	let inputDenom = 'ubbn'
	$: outputValue = inputValue;
	$: outputUSD = `$${(parseFloat(inputValue) * 9.34).toLocaleString()}`;
	
	const VAULT_CONTRACT_ADDRESS = contractAddresses.babylon_vault;

	function convertToMicroUnits(value: string): string {
		if (!value) return '0';
		return (parseFloat(value) * 1_000_000).toString();
	}

	async function handleDeposit() {
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
					deposit: {} // Empty object as per the contract's ExecuteMsg
				};

				const response = await client.execute(
					address,
					VAULT_CONTRACT_ADDRESS,
					msg,
					'auto',
					"",
					[coin(convertToMicroUnits(inputValue), inputDenom)]
				);

				console.log('Deposit successful:', response);
			}

			// Handle success (maybe clear input, show success message, etc.)
		} catch (error) {
			console.error('Deposit failed:', error);
			// Handle error (show error message to user)
		}
	}

	function handleInput(value) {
		inputValue = value;
		outputUSD = `$${(parseFloat(value) * 100000).toLocaleString()}`;
	}

	function setHalf() {
		inputValue = (parseFloat(inputValue) / 2).toString();
	}

	async function setMax() {
		inputValue = "10";
	}
</script>

<CurrencyInput
	label="BBN"
	value={inputValue}
	onInput={handleInput}
	onHalf={setHalf}
	onMax={setMax}
	options={["BBN"]}
/>

<div class="output">
	<p>You will get</p>
	<div>
		<span>{outputValue}</span>
		<span class="text-sm text-gray-600">{outputUSD}</span>
	</div>
</div>

<button class="deposit-button" on:click={handleDeposit}>Deposit</button>

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
	.deposit-button {
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
