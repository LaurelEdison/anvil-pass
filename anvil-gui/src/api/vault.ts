import { invoke } from "@tauri-apps/api/core";

export async function openVault(
  path: string,
  masterPassword: string,
): Promise<void> {
  await invoke("open_a_vault", {
    path,
    masterPassword,
  });
}

export async function createVault(
  path: string,
  masterPassword: string,
): Promise<void> {
  await invoke("create_vault", {
    path,
    masterPassword,
  });
}

export async function saveVault(): Promise<void> {
  await invoke("save_vault");
}

export async function clearVault(): Promise<void> {
  await invoke("clear_vault");
}

export async function isDirty(): Promise<boolean> {
  return await invoke<boolean>("is_dirty");
}
