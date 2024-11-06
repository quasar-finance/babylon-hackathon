<script lang="ts">
	export let onVote;
	export let onAddIncentives;
</script>

<div class="vote-actions">
	<button class="vote-button" on:click={onVote}>Vote</button>
	<button class="add-incentives-button" on:click={onAddIncentives}>Add Voting Incentives</button>
</div>

<style>
	.vote-actions {
		display: flex;
		gap: 1rem;
	}
	.vote-button, .add-incentives-button {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		font-weight: bold;
	}
	.vote-button {
		background-color: #2c82c9;
		color: white;
	}
	.add-incentives-button {
		background-color: #5fc0a6;
		color: white;
	}
</style>
