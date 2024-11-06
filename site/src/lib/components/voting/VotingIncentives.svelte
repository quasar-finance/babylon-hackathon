<script lang="ts">
	export let text = `
		Voting Incentives can be added from any chain for any market. Voters get all voting rewards from different chains for the pools they vote.
		Only vlPNP in lock state can vote. Penpie casts votes every Wednesday around 2:00 p.m UTC to Pendle finance.
	`;
</script>

<div class="voting-incentives">
	<h4>Voting Incentives and Votes</h4>
	<p>{text}</p>
</div>

<style>
	.voting-incentives {
		padding: 1rem;
		background-color: #eaf3fc;
		border-radius: 10px;
		margin-bottom: 1rem;
	}
	h4 {
		margin-top: 0;
	}
</style>
