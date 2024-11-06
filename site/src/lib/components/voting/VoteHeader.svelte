<script lang="ts">
	export let totalDistribution = "$37.91M";
	export let activeTab = "vlPNP Voting";
	export let tabs = ["vlPNP Voting", "vePendle Voting"];
	export let onTabChange;

	function handleTabClick(tab) {
		onTabChange(tab);
	}
</script>

<div class="vote-header">
	<div class="total-incentive">Total distribution: {totalDistribution}</div>
	<div class="tabs">
		{#each tabs as tab}
			<button
				class:selected={tab === activeTab}
				on:click={() => handleTabClick(tab)}
			>
				{tab}
			</button>
		{/each}
	</div>
</div>

<style>
	.vote-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}
	.total-incentive {
		font-weight: bold;
	}
	.tabs button {
		background: none;
		border: none;
		padding: 0.5rem 1rem;
		cursor: pointer;
		font-weight: bold;
	}
	.tabs .selected {
		color: white;
		background-color: #2c82c9;
		border-radius: 5px;
	}
</style>
