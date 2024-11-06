<script lang="ts">
	let claimableAmount = 0.2; // This should be fetched from your backend/contract
</script>

<main class="claiming-page">
	<div class="outer-box">
		<div class="amount-display">
			<span class="amount">{claimableAmount}BTC</span>
			<span class="label">Available to Claim</span>
		</div>
		
		<div 
			class="outer-box clickable bg-black"
			on:click={() => alert("claiming click")}
			on:keydown={(e) => e.key === 'Enter' && alert("claiming enter")}
			role="button"
			tabindex="0"
		>
			<button>
				Claim
			</button>
		</div>
    
	</div>
</main>

<style>
	.outer-box {
		max-width: 400px;
		margin: 2rem auto;
		padding: 1.5rem;
		background-color: #f3f3f3;
		border-radius: 10px;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		text-align: center;
	}

	.amount-display {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
	}

	.amount {
		font-size: 2rem;
		font-weight: bold;
		color: #333;
	}

	.label {
		font-size: 0.9rem;
		color: #666;
	}

	.claim-button {
		width: 100%;
		padding: 0.75rem 1.5rem;
		color: white;
		border: none;
		border-radius: 5px;
		font-weight: bold;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.claim-button:hover {
		background-color: #45a049;
	}

	.clickable {
		cursor: pointer;
		outline: none;
	}
</style>
