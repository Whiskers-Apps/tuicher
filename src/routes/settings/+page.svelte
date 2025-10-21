<script lang="ts">
	import { configRepo } from "$lib/features/config/ConfigRepo";
	import { onMount } from "svelte";

	let cssVars = $state("");
	let loaded = $state(false);

	onMount(() => {
		$configRepo.init(() => {
			cssVars = $configRepo.getCss();
			loaded = true;
		});
	});
</script>

{#if loaded}
	{@html cssVars}
	<div class="h-screen w-full flex bg text"></div>
{/if}
