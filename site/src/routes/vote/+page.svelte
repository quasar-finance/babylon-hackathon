<script lang="ts">
	import VoteHeader from '$lib/components/voting/VoteHeader.svelte';
	import PieChart from '$lib/components/voting/PieChart.svelte';
	import VotingIncentives from '$lib/components/voting/VotingIncentives.svelte';
	import EpochTimer from '$lib/components/voting/EpochTimer.svelte';
	import VoteActions from '$lib/components/voting/VoteActions.svelte';

	let totalDistribution = "$37.91M";
	let activeTab = "Vault allocation";

	function handleTabChange(tab) {
		activeTab = tab;
	}

	function handleVote() {
		alert("Allocation action triggered!");
	}

	function handleAddIncentives() {
		alert("Add Voting Incentives action triggered!");
	}

	// Example data for the pie chart (replace with actual data)
	let pieData = [
		{ label: "Label 1", value: 21.35 },
		{ label: "Label 2", value: 11.13 },
		// More data as needed
	];
</script>

<main class="voting-page">
	<VoteHeader
		{totalDistribution}
		{activeTab}
		tabs={["Vault allocation", "vePendle Voting"]}
		onTabChange={handleTabChange}
	/>

	<div class="content">
		<PieChart {pieData} />

		<div class="info-section">
			<VotingIncentives />
			<EpochTimer epoch={70} days={0} hours={12} minutes={51} seconds={42} />
		</div>

		<VoteActions onVote={handleVote} onAddIncentives={handleAddIncentives} />
	</div>
</main>

<style>
	.voting-page {
		max-width: 800px;
		margin: auto;
		padding: 2rem;
		background-color: #f9f9f9;
		border-radius: 10px;
	}
	.content {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.info-section {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
	}
</style>
