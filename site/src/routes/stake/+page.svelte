<script lang="ts">
	import ToggleButton from '$lib/components/staking/ToggleButton.svelte';
	import StakeView from '$lib/components/staking/StakeView.svelte';
	import UnstakeView from '$lib/components/staking/UnstakeView.svelte';
</script>

<main class="staking-page">
	<ToggleButton
		leftLabel="Stake"
		rightLabel="Unstake"
		leftComponent={StakeView}
		rightComponent={UnstakeView}
	/>
</main>

<style>
	.staking-page {
		display: flex;
		justify-content: center;
		padding: 2rem;
		background-color: #f9f9f9;
	}
</style>
