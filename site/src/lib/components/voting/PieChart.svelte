<script lang="ts">
	import { arc, pie } from 'd3-shape';
	import { interpolateRainbow } from 'd3-scale-chromatic';

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
</script>

<div class="pie-chart">
	<svg {width} {height}>
		<g transform={`translate(${width / 2}, ${height / 2})`}>
			{#each arcs as arc, i}
				<path
					d={arcGenerator(arc)}
					fill={interpolateRainbow(i / data.length)}
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

<style>
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
</style>
