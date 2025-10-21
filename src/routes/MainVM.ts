import type { TUIResult } from "$lib/features/result/TUIResult";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { get, writable } from "svelte/store";

export type MainScreenAction = { action: "arrow-up" } | { action: "arrow-down" } | { action: "search-input", text: string } | { action: "enter-press" } | { action: "escape-press" } | { action: "result-hover", index: number } | { action: "result-click" };

export class MainVM {
	state = writable<{
		text: string;
		currentIndex: number;
		results: TUIResult[];
	}>({
		text: "",
		currentIndex: 0,
		results: []
	});

	constructor() {
		listen<{ results: TUIResult[] }>("show-plugin-results", (e) => {
			let results: TUIResult[] = e.payload.results;
			this.state.update(state => ({ ...state, currentIndex: 0, results: results }));
		})
	}

	onAction(action: MainScreenAction) {
		switch (action.action) {
			case "arrow-up": {
				this.onArrowUp();
				break;
			}

			case "arrow-down": {
				this.onArrowDown();
				break;
			}

			case "search-input": {
				this.onSearchInput(action.text);
				break;
			}

			case "enter-press": {
				this.onEnterPress();
				break;
			}

			case "escape-press": {
				this.onEscapePress();
				break;
			}

			case "result-hover": {
				this.onResultHover(action.index)
				break;
			}

			case "result-click": {
				this.onResultClick();
				break;
			}
		}
	}

	private onArrowUp() {
		let newIndex = get(this.state).currentIndex - 1;

		if (newIndex < 0) {
			return;
		}

		this.state.update(state => ({ ...state, currentIndex: newIndex }));

		this.scrollIntoResult(newIndex, false);
	}

	private onArrowDown() {
		let newIndex = get(this.state).currentIndex + 1;

		if (newIndex >= get(this.state).results.length) {
			return;
		}

		this.state.update(state => ({ ...state, currentIndex: newIndex }));

		this.scrollIntoResult(newIndex, true);
	}

	private scrollIntoResult(index: number, goingDown: boolean) {
		const resultsDiv = document.getElementById("results-div")!! as HTMLDivElement;
		const resultDiv = document.getElementById(`result-${index}`)!! as HTMLDivElement;

		const resultsRect = resultsDiv.getBoundingClientRect();
		const resultRect = resultDiv.getBoundingClientRect();

		const isFullyVisible = goingDown ? resultRect.bottom <= resultsRect.bottom : resultRect.top >= resultsRect.top;

		if (!isFullyVisible) {
			resultDiv.scrollIntoView({
				block: goingDown ? "end" : "start"
			});
		}
	}

	private async onSearchInput(text: string) {
		let results: TUIResult[] = await invoke("invoke_search", { text: text });

		this.state.update(state => ({ ...state, results: results, text: text, currentIndex: 0 }));
	}

	private onEnterPress() {
		this.runResult();
	}

	private runResult() {
		let state = get(this.state);
		let index = state.currentIndex;
		let result = state.results[index];

		if (result.action) {
			this.resetState();
			invoke("invoke_result_action", { action: result.action });
		}
	}

	private onEscapePress() {
		getCurrentWindow().close();
		this.resetState();
	}

	private resetState() {
		this.state.set({ text: "", results: [], currentIndex: 0 });
	}

	private onResultHover(index: number) {
		this.state.update(state => ({ ...state, currentIndex: index }));
	}

	private onResultClick() {
		this.runResult();
	}
}
