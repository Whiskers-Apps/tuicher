export interface Config {
	plugins: PluginConfig[];
	search_engines: SearchEngine[];
	theme: Theme;
	height: number;
	width: number;
}

export interface PluginConfig {
	id: number;
	keyword: string
}

export interface SearchEngine {
	id: number;
	keyword: string;
	name: string;
	url: string
}

export interface Theme {
	background: string;
	secondary: string;
	tertiary: string;
	disabled: string;
	text: string;
	text_secondary: string;
	text_tertiary: string;
	on_text: string;
	warning: string
}
