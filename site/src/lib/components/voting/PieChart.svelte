<script lang="ts">
	import { arc, pie } from 'd3-shape';
	import { 
		schemeCategory10,     // Fixed set of 10 distinct colors
		// or choose from these alternatives:
		// schemeAccent,      // 8 colors
		// schemeDark2,       // 8 colors
		// schemePaired,      // 12 colors
		// schemePastel1,     // 9 colors
		// schemePastel2,     // 8 colors
		// schemeSet1,        // 9 colors
		// schemeSet2,        // 8 colors
		// schemeSet3,        // 12 colors
		// schemeTableau10    // 10 colors
	} from 'd3-scale-chromatic';
	import { scaleOrdinal } from 'd3-scale';

	export let data = [
		{ label: 'Option A', value: 30 },
		{ label: 'Option B', value: 25 },
		{ label: 'Option C', value: 20 },
		{ label: 'Option D', value: 15 },
		{ label: 'Option E', value: 10 }
	]; // Array of objects with `label` and `value` keys
	export let width = 300;
	export let height = 300;
	export let unit = '%';

	const radius = Math.min(width, height) / 2;

	// Create the pie layout
	const pieGenerator = pie()
		.value(d => d.value)
		.sort(null);

	// Create the arc generator
	const arcGenerator = arc()
		.innerRadius(0)
		.outerRadius(radius)
		.cornerRadius(3)
		.padAngle(0.01);

	// Generate the arcs
	$: arcs = pieGenerator(data);

	let tooltipData = null;
	let tooltipX = 0;
	let tooltipY = 0;

	function showTooltip(event, d) {
		tooltipData = d.data;
		tooltipX = event.clientX;
		tooltipY = event.clientY;
	}

	function hideTooltip() {
		tooltipData = null;
	}

	const colorScale = scaleOrdinal(schemeCategory10);
</script>

<div class="chart-container">
	<div class="pie-chart">
		<svg {width} {height}>
			<g transform={`translate(${width / 2}, ${height / 2})`}>
				{#each arcs as arc, i}
					<path
						d={arcGenerator(arc)}
						fill={colorScale(i)}
						on:mouseover={(e) => showTooltip(e, arc)}
						on:mouseout={hideTooltip}
					/>
				{/each}
			</g>
		</svg>

		{#if tooltipData}
			<div 
				class="tooltip"
				style="left: {tooltipX}px; top: {tooltipY}px"
			>
				<strong>{tooltipData.label}</strong>: {tooltipData.value}{unit}
			</div>
		{/if}
	</div>

	<div class="legend">
		{#each data as item, i}
			<div class="legend-item">
				<svg width="20" height="20">
					<rect width="20" height="20" fill={colorScale(i)} />
				</svg>
				<span>{item.label}: {item.value}{unit}</span>
			</div>
		{/each}
	</div>
</div>

<style>
	.chart-container {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2rem;
		width: 100%;
	}

	.pie-chart {
		position: relative;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.tooltip {
		position: fixed;
		background: white;
		padding: 8px;
		border-radius: 4px;
		box-shadow: 0 2px 4px rgba(0,0,0,0.1);
		pointer-events: none;
		transform: translate(-50%, -100%);
		margin-top: -8px;
	}

	.legend {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		transform: scale(0.8);
		transform-origin: left center;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
</style>
