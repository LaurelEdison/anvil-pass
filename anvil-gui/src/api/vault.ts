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
