<script lang="ts">
	import { configRepo } from "$lib/features/config/ConfigRepo";
	import { onMount } from "svelte";
	import SearchIcon from "$lib/icons/search.svg?component";
	import AppsIcon from "$lib/icons/apps.svg?component";
	import SettingsIcon from "$lib/icons/settings.svg?component";
	import LogoutIcon from "$lib/icons/logout.svg?component";
	import SuspendIcon from "$lib/icons/suspend.svg?component";
	import ShutdownIcon from "$lib/icons/shutdown.svg?component";
	import RebootIcon from "$lib/icons/reboot.svg?component";
	import GlobeIcon from "$lib/icons/globe.svg?component";
	import { MainVM } from "./MainVM";
	import { listen } from "@tauri-apps/api/event";
	import { convertFileSrc } from "@tauri-apps/api/core";

	let cssVars = $state("");
	let loaded = $state(false);
	let vm = new MainVM();
	let uiState = vm.state;

	onMount(() => {
		$configRepo.init(() => {
			cssVars = $configRepo.getCss();
			loaded = true;
		});

		listen("window-show", (_e) => {
			(document.getElementById(
				"search-input",
			) as HTMLInputElement)!!.focus();
		});
	});

	function onKeyDown(e: KeyboardEvent) {
		if (e.key === "ArrowDown") {
			e.preventDefault();
			vm.onAction({ action: "arrow-down" });
		}

		if (e.key === "ArrowUp") {
			e.preventDefault();
			vm.onAction({ action: "arrow-up" });
		}

		if (e.key === "Enter") {
			e.preventDefault();
			vm.onAction({ action: "enter-press" });
		}

		if (e.key === "Escape") {
			e.preventDefault();
			vm.onAction({ action: "escape-press" });
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

{#if loaded}
	{@html cssVars}

	<div class="bg h-screen w-full p-5 text flex flex-col">
		<div class="flex bg-secondary items-center p-3 pl-4 pr-4 rounded-full">
			<SearchIcon class="h-5 w-5" />
			<!-- svelte-ignore a11y_autofocus -->
			<input
				value={$uiState.text}
				class="w-full ml-2 outline-none text-[1rem]"
				autofocus
				placeholder="Search"
				oninput={(e) => {
					vm.onAction({
						action: "search-input",
						text: (e.target as HTMLInputElement)!!.value,
					});
				}}
				id="search-input"
			/>
		</div>

		<div
			class="flex-grow overflow-auto space-y-1 mt-4 custom-scroll"
			id="results-div"
		>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			{#each $uiState.results as result, index}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div
					class={`flex p-3 pl-4 pr-4 rounded-full min-h-[50px] items-center ${index === $uiState.currentIndex ? "bg-secondary" : ""}`}
					id={`result-${index}`}
					onfocus={() => {}}
					aria-roledescription="Hover result"
					onclick={() => {
						vm.onAction({ action: "result-click" });
					}}
				>
					{#if result.icon_path}
						<img
							class="h-[40px] w-[40px] mr-4"
							src={convertFileSrc(result.icon_path)}
							alt="icon"
						/>
					{/if}

					{#if !result.icon_path}
						{#if result.info === "app"}
							<AppsIcon class="h-[40px] w-[40px] mr-4" />
						{/if}

						{#if result.info === "settings"}
							<SettingsIcon class="h-[40px] w-[40px] mr-4" />
						{/if}

						{#if result.info === "search-engine" || result.info === "url-search"}
							<GlobeIcon class="h-[40px] w-[40px] mr-4" />
						{/if}

						{#if result.info === "shutdown"}
							<ShutdownIcon class="h-[40px] w-[40px] mr-4" />
						{/if}

						{#if result.info === "restart"}
							<RebootIcon class="h-[40px] w-[40px] mr-4" />
						{/if}

						{#if result.info === "suspend"}
							<SuspendIcon class="h-[40px] w-[40px] mr-4" />
						{/if}

						{#if result.info === "logout"}
							<LogoutIcon class="h-[40px] w-[40px] mr-4" />
						{/if}
					{/if}

					<div class=" flex flex-col justify-center">
						<p class="text-[1rem]">{result.text}</p>

						{#if result.secondary_text}
							<div>
								<p class="text_secondary text-[0.9rem]">
									{result.secondary_text}
								</p>
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
</style>
