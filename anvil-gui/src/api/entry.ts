import { invoke } from "@tauri-apps/api/core";
import { EntryDto } from "../types/types";

export async function listEntries(): Promise<EntryDto[]> {
  return invoke<EntryDto[]>("list_entries");
}

export async function createEntry(data: {
  parent: string | null;
  title: string | null;
  username: string | null;
  password: string;
  url: string | null;
  notes: string | null;
  totp: string | null;
}): Promise<void> {
  await invoke("create_entry", data);
}

export async function updateEntry(data: {
  id: string;
  title: string | null;
  username: string | null;
  password: string | null;
  url: string | null;
  notes: string | null;
  totp: string | null;
}): Promise<void> {
  await invoke("update_entry", data);
}

export async function deleteEntry(entryId: string): Promise<void> {
  await invoke("delete_entry", { entryId });
}

export async function moveEntry(data: {
  entryId: string;
  destinationId: string;
}): Promise<void> {
  await invoke("delete_entry", data);
}
