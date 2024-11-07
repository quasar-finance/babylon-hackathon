<script lang="ts">
	import { onMount } from 'svelte';
	import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
	import VoteHeader from '$lib/components/voting/VoteHeader.svelte';
	import PieChart from '$lib/components/voting/PieChart.svelte';
	import VotingIncentives from '$lib/components/voting/VotingIncentives.svelte';
	import EpochTimer from '$lib/components/voting/EpochTimer.svelte';
	import VoteActions from '$lib/components/voting/VoteActions.svelte';
	import contractAddresses from '$lib/config/contract_addresses.json';
	import type { Coin } from '@cosmjs/stargate';

	export let tvl = "$37.91M";
	let activeTab = "Vault allocation";

	const VAULT_CONTRACT_ADDRESS = contractAddresses.babylon_vault;
	
	interface DestinationInfo {
		destination: string;
		adaptor: string;
	}

	let pieData: { label: string; value: number }[] = [{"label": "hardcoded-id1", "value": 0.55500}, {"label": "hardcoded-id2", "value": 0.33}];
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
			const response: DestinationInfo[] = await client.queryContractSmart(VAULT_CONTRACT_ADDRESS, {
				destinations: {}
			});
			
			console.log('Contract response:', response);

			// Query balance for each destination's adaptor
			const balancePromises = response.map(async (destination) => {
                console.log("querying", destination.adaptor);
				const balanceResponse: { balance: Coin[] } = await client.queryContractSmart(destination.adaptor, {
					balance_query: {}
				});
				return {
					destination: destination.destination,
					balance: balanceResponse.balance
				};
			});

			const balances = await Promise.all(balancePromises);
			console.log('Balances:', balances);

			// Transform the response into pie chart data
			pieData = response.map((destination, index) => ({
				label: destination.destination,
				value: parseFloat(balances[index].balance[0]?.amount || '0')
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
		totalDistribution={tvl}
		{activeTab}
		tabs={["Vault allocation"]}
		onTabChange={handleTabChange}
	/>

	<div class="content">
		{#if isLoading}
			<div class="loading">Loading allocations...</div>
		{:else}
			<PieChart data={pieData} unit='BBN'/>
		{/if}

		<div class="info-section">
			<VotingIncentives title="Active Allocations" text="active allocations of the vault to BSNs and BVSes. These allocations are updates every epoch in rolling updates. Earned yield is compounded into the allocation on the destination. Rewards are paid out seperately"/>
			<EpochTimer epoch={70} days={0} hours={12} minutes={51} seconds={42} />
		</div>

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
