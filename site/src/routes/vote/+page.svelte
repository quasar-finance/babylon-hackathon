<script lang="ts">
	import { onMount } from 'svelte';
	import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
	import VoteHeader from '$lib/components/voting/VoteHeader.svelte';
	import PieChart from '$lib/components/voting/PieChart.svelte';
	import VotingIncentives from '$lib/components/voting/VotingIncentives.svelte';
	import EpochTimer from '$lib/components/voting/EpochTimer.svelte';
	import VoteActions from '$lib/components/voting/VoteActions.svelte';
	import contractAddresses from '$lib/config/contract_addresses.json';

	let totalDistribution = "$37.91M";
	let activeTab = "Vault allocation";

	const GAUGE_CONTRACT = contractAddresses.mock_gauge;
	
	let pieData: { label: string; value: number }[] = [{"label": "hardcoded-id1", "value": 0.43}, {"label": "hardcoded-id2", "value": 0.57}];
	let isLoading = true;

	function calculatePercentages(data: { label: string; value: number }[]): { label: string; value: number }[] {
		const total = data.reduce((sum, item) => sum + item.value, 0);
		return data.map(item => ({
			label: item.label,
			value: (item.value / total) * 100
		}));
	}

	async function fetchAllocations() {
		try {
			const rpcEndpoint = "https://rpc-euphrates.devnet.babylonlabs.io";
			console.log(rpcEndpoint);
			if (!rpcEndpoint) {
				throw new Error('RPC endpoint not configured');
			}

			const client = await CosmWasmClient.connect(rpcEndpoint);
			const response = await client.queryContractSmart(GAUGE_CONTRACT, {
				get_allocations: {}
			});
			
			console.log('Contract response:', response);
			
			// Transform the response into pie chart data
			pieData = response.allocations.map((allocation: { destination_id: any; amount: string; }) => ({
				label: allocation.destination_id,
				value: parseFloat(allocation.amount)
			}));
			
			isLoading = false;
		} catch (error) {
			console.error('Failed to fetch allocations:', error);
			isLoading = false;
		}
	}

	onMount(() => {
		fetchAllocations();
	});

	function handleTabChange(tab: string) {
		activeTab = tab;
	}

	function handleVote() {
		alert("Allocation action triggered!");
	}

	function handleAddIncentives() {
		alert("Add Voting Incentives action triggered!");
	}
</script>

<main class="voting-page">
	<VoteHeader
		{totalDistribution}
		{activeTab}
		tabs={["Vault allocation"]}
		onTabChange={handleTabChange}
	/>

	<div class="content">
		{#if isLoading}
			<div class="loading">Loading allocations...</div>
		{:else}
			<PieChart data={calculatePercentages(pieData)} />
		{/if}

		<div class="info-section">
			<VotingIncentives />
			<EpochTimer epoch={70} days={0} hours={12} minutes={51} seconds={42} />
		</div>

		<!-- <VoteActions onVote={handleVote} onAddIncentives={handleAddIncentives} /> -->
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
	.loading {
		text-align: center;
		padding: 2rem;
	}
</style>
