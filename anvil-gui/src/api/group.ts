import { invoke } from "@tauri-apps/api/core";
import { GroupDto } from "../types/types";

export async function createGroup(name: string): Promise<void> {
	await invoke("create_group", {
		name,
	});
}

export async function updateGroup(groupId: string): Promise<void> {
	await invoke("update_group", {
		groupId,
	});
}

export async function deleteGroup(groupId: string): Promise<void> {
	await invoke("delete_group", {
		groupId,
	});
}

export async function listGroups(): Promise<GroupDto[]> {
	return await invoke<GroupDto[]>("list_groups");
}

