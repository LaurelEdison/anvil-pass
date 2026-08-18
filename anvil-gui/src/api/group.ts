import { invoke } from "@tauri-apps/api/core";
import { GroupDto } from "../types/types";
export async function createGroup(data: {
  name: string;
  tags?: string[] | null;
  notes?: string | null;
  parent?: string | null;
}): Promise<void> {
  await invoke("create_group", data);
}

export async function updateGroup(data: {
  groupId: string;
  name?: string | null;
  tags?: string[] | null;
  notes?: string | null;
}): Promise<void> {
  await invoke("update_group", data);
}

export async function deleteGroup(groupId: string): Promise<void> {
  await invoke("delete_group", {
    groupId,
  });
}
export async function listGroups(): Promise<GroupDto[]> {
  return invoke<GroupDto[]>("list_groups");
}
