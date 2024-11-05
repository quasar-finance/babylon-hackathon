<script lang="ts">
	import { connectKeplr } from '$lib/keplr';
	import { SigningStargateClient } from '@cosmjs/stargate';
  
	let walletAddress = '';
	let signer: SigningStargateClient;
  
	async function connectWallet() {
		const wallet = await connectKeplr();

		if (wallet.isOk()) {
			walletAddress = wallet.value.address;
			signer = wallet.value.signer;
		}
	}
  </script>
  
  <main>
	<h1>Cosmos SDK Transaction Generator</h1>
  
	{#if walletAddress}
	  <p>
		All transactions are served from the Connected wallet.
		This means that newly created tokens will also be created under 
		the namespace of this token.
	</p>
	  <p>Connected Wallet: {walletAddress}</p>
	{:else}
	  <button on:click={connectWallet}>Connect Keplr Wallet</button>
	{/if}
  </main>
  
  <style>
	main {
	  text-align: center;
	  padding: 2rem;
	}
  </style>
  