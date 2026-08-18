import { invoke } from "@tauri-apps/api/core";
import { PasswordOptions, PasswordResult } from "../types/types";

export async function generatePassword(
  options: PasswordOptions = {},
): Promise<PasswordResult> {
  return await invoke<PasswordResult>("generate_password", {
    withNumber: options.withNumber,
    withUppercase: options.withUppercase,
    withLowercase: options.withLowercase,
    withSymbols: options.withSymbols,
    withExtendedAscii: options.withExtendedAscii,
  });
}
