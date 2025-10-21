export interface TUIResult {
	icon_path: string | null,
	text: string,
	secondary_text: string | null,
	action: OpenApp | OpenFile | OpenURL | CopyText | CopyImage | ShowResults | Custom | OpenSettings | Session | Bookmark | null,
	info: string
}

export interface OpenApp {
	type: string,
	path: string
}

export interface OpenFile {
	type: string,
	path: string
}

export interface OpenURL {
	type: string,
	url: string
}

export interface CopyText {
	type: string,
	text: string
}

export interface CopyImage {
	type: string,
	path: string
}

export interface ShowResults {
	type: string,
	results: TUIResult[]
}

export interface Custom {
	type: string,
	plugin_id: string,
	action: string,
	info: string[]
}

export interface OpenSettings {
	type: string
}

export interface Session {
	type: string
}

export type Bookmark = { type: string, name: string; url: string } | { type: string, id: number }
