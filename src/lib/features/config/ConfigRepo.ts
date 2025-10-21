import { invoke } from "@tauri-apps/api/core";
import type { Config } from "./Config";
import { writable } from "svelte/store";

export class ConfigRepo {
	config: Config | undefined = undefined

	async init(onSettingsLoad: (() => void)) {
		this.config = await invoke("invoke_get_config");
		onSettingsLoad();
	}

	getCss(): string {
		let config = this.config!!;

		return `<style>
:root{
	--background: ${config.theme.background};
	--secondary: ${config.theme.secondary};
	--tertiary: ${config.theme.tertiary};
	--disabled: ${config.theme.disabled};
	--text: ${config.theme.text};
	--text_secondary: ${config.theme.text_secondary};
	--text_tertiary: ${config.theme.text_tertiary};
	--on_text: ${config.theme.on_text};
	--accent: #FFDE72;
	--warning: ${config.theme.warning};
}

.bg {
	background-color: var(--background);
}

.bg-text {
	background-color: var(--text);
}

.bg-secondary {
	background-color: var(--secondary);
}

.bg-tertiary {
	background-color: var(--tertiary);
}

.bg-accent {
	background-color: var(--accent);
}

.text {
	color: var(--text);
}

.text_secondary {
	color: var(--text_secondary);
}

.custom-scroll::-webkit-scrollbar {
	width: 4px;
}

.custom-scroll::-webkit-scrollbar-track {
	background: transparent;
}

.custom-scroll::-webkit-scrollbar-thumb {
	background-color: var(--tertiary);
	border-radius: 0px;
}

input::placeholder {
	color: var(--text_tertiary);
}

</style>`}
}

export const configRepo = writable(new ConfigRepo());
