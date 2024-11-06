<script lang="ts">
	export let epoch = 70;
	export let days = 0;
	export let hours = 12;
	export let minutes = 51;
	export let seconds = 42;
</script>

<div class="epoch-timer">
	<p>Current Epoch <span>{epoch}</span> ends in</p>
	<div class="timer">
		<span>{days} DAYS</span>
		<span>{hours} HRS</span>
		<span>{minutes} MIN</span>
		<span>{seconds} SEC</span>
	</div>
</div>

<style>
	.epoch-timer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background-color: #eaf3fc;
		border-radius: 10px;
		margin-bottom: 1rem;
	}
	.timer span {
		margin: 0 0.25rem;
		font-weight: bold;
	}
</style>
