<script lang="ts">
	// Import the Keplr connection function
	import { connectKeplr } from '$lib/keplr';

	// Define the wallet address and connection logic
	let walletAddress = '';
	async function connectWallet() {
		const wallet = await connectKeplr();

		if (wallet.isOk()) {
			walletAddress = wallet.value.address;
		}
	}
</script>

<nav class="navbar">
	<div class="navbar-logo">Quasar</div>
	<div class="navbar-links">
		<a href="/stake" sveltekit:prefetch>Stake</a>
		<a href="/claim">Claim</a>
		<a href="/vote">Gauge</a>
		<a href="/defi">DeFi</a>
	</div>
	<div class="navbar-wallet">
		{#if walletAddress}
			<p>Connected Wallet: {walletAddress}</p>
		{:else}
			<button on:click={connectWallet}>Connect Wallet</button>
		{/if}
	</div>
</nav>

<style>
	.navbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background-color: #f3f3f3;
		border-bottom: 1px solid #ccc;
	}
	.navbar-logo {
		font-weight: bold;
		font-size: 1.2rem;
	}
	.navbar-links a {
		margin: 0 1rem;
		text-decoration: none;
		color: #000;
	}
	.navbar-wallet button {
		background-color: #000;
		color: #fff;
		padding: 0.5rem 1rem;
		border: none;
		cursor: pointer;
	}
</style>
