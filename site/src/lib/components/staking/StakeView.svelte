<script lang="ts">
	import CurrencyInput from './CurrencyInput.svelte';

	let inputValue = '';
	let outputValue = '4.2 BLT';
	let outputUSD = '$294,000';

	function handleInput(value) {
		inputValue = value;
	}

	function setHalf() {
		inputValue = (parseFloat(inputValue) / 2).toString();
	}

	function setMax() {
		inputValue = '4.2'; // Example max value, replace as needed
	}
</script>

<CurrencyInput
	label="Bitcoin"
	value={inputValue}
	onInput={handleInput}
	onHalf={setHalf}
	onMax={setMax}
/>

<div class="output">
	<p>You will get</p>
	<div>
		<span>{outputValue}</span>
		<span>{outputUSD}</span>
	</div>
</div>

<button class="deposit-button">Deposit</button>

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
