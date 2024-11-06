<script lang="ts">
    import CurrencyInput from './CurrencyInput.svelte';


	let unstakeAmount = '';
	let outputValue = '4.2 BTC';
	let outputUSD = '$294,000';

	function handleInput(value) {
		unstakeAmount = value;
	}

	function setHalf() {
		unstakeAmount = (parseFloat(unstakeAmount) / 2).toString();
	}

	function setMax() {
		unstakeAmount = '4.2'; // Example max value, replace as needed
	}
</script>

<CurrencyInput
	label="BLT"
	value={unstakeAmount}
	onInput={handleInput}
	onHalf={setHalf}
	onMax={setMax}
    options={["BLT"]}
/>

<div class="output">
	<p>You will get</p>
	<div>
		<span>{outputValue}</span>
		<span>{outputUSD}</span>
	</div>
</div>

<button class="withdraw-button">Withdraw</button>

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
