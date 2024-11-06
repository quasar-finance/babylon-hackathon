<script lang="ts">
	import type { SvelteComponent } from 'svelte';

	// Props for labels and components to display in each toggle
	export let leftLabel: string;
	export let rightLabel: string;
	export let leftComponent: typeof SvelteComponent;
	export let rightComponent: typeof SvelteComponent;

	// State for the active tab
	let activeTab = 'left';

	function setTab(tab: string) {
		activeTab = tab;
	}
</script>

<div class="outer-box">
	<div class="toggle">
		<button
			class:active={activeTab === 'left'}
			on:click={() => setTab('left')}
		>
			{leftLabel}
		</button>
		<button
			class:active={activeTab === 'right'}
			on:click={() => setTab('right')}
		>
			{rightLabel}
		</button>
	</div>

	<!-- Conditionally render the active component -->
	{#if activeTab === 'left'}
		<svelte:component this={leftComponent} />
	{:else}
		<svelte:component this={rightComponent} />
	{/if}
</div>

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
	.toggle {
		display: flex;
		background-color: #f3f3f3;
		border-radius: 8px;
		margin-bottom: 1rem;
	}
	button {
		flex: 1;
		padding: 0.5rem 1rem;
		border: none;
		background: none;
		cursor: pointer;
		font-weight: bold;
	}
	.active {
		background-color: #d3d3d3;
		border-radius: 8px;
	}
</style>
