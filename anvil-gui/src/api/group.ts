import { invoke } from "@tauri-apps/api/core";
import { GroupDto } from "../types/types";

export async function listGroups(): Promise<GroupDto[]> {
	return invoke<GroupDto[]>("list_groups");
}
