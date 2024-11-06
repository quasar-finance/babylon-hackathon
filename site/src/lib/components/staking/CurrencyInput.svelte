<script lang="ts">
	export let label;
	export let value = '';
	export let onInput;
	export let onHalf;
	export let onMax;
	export let options: string[] = ['Bitcoin', 'WBTC', 'BTC LST'];
</script>

<div class="currency-input">
	<select>
		{#each options as option}
			<option>{option}</option>
		{/each}
	</select>
	<div class="input-wrapper">
		<input
			type="text"
			bind:value
			on:input={(e) => onInput(e.target.value)}
		/>
		<div class="buttons">
			<button on:click={onHalf}>Half</button>
			<button on:click={onMax}>Max</button>
		</div>
	</div>
</div>

<style>
	.currency-input {
		display: flex;
		align-items: center;
		background-color: #eaeaea;
		padding: 0.5rem;
		border-radius: 8px;
		margin-bottom: 1rem;
		gap: 0.5rem;
	}
	.input-wrapper {
		position: relative;
		flex: 1;
		display: flex;
	}
	.buttons {
		position: absolute;
		right: 0;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		gap: 0.5rem;
		padding-right: 0.5rem;
	}
	select, input {
		padding: 0.25rem;
		border: none;
		background: none;
	}
	input {
		width: 100%;
		padding-right: 8rem; /* Make space for buttons */
	}
	button {
		padding: 0.25rem;
		border: none;
		background: none;
		cursor: pointer;
		font-size: 0.9rem;
		font-weight: bold;
	}
</style>
