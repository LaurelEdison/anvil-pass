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
