import { invoke } from "@tauri-apps/api/core";
import { EntryDto } from "../types/types";

export async function createEntry(
	parent: string | null,
	title: string | null,
	username: string | null,
	password: string,
	url: string | null,
	notes: string | null,
	totp: string | null,
): Promise<void> {
	await invoke("create_entry", {
		parent,
		title,
		username,
		password,
		url,
		notes,
		totp,
	});
}

export async function updateEntry(
	id: string,
	title: string | null,
	username: string | null,
	password: string | null,
	url: string | null,
	notes: string | null,
	totp: string | null,
): Promise<void> {
	await invoke("update_entry", {
		id,
		title,
		username,
		password,
		url,
		notes,
		totp,
	});
}

export async function deleteEntry(entryId: string): Promise<void> {
	await invoke("delete_entry", {
		entryId,
	});
}

export async function listEntries(): Promise<EntryDto[]> {
	return await invoke<EntryDto[]>("list_entries");
}

