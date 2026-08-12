import { useState } from "react";
import { openVault } from "../api/vault";
import { open } from "@tauri-apps/plugin-dialog";
import type { SubmitEvent } from "react";

interface OpenVaultFormProps {
  onOpened: () => void;
}

export function OpenVaultForm({ onOpened }: OpenVaultFormProps) {
  const [path, setPath] = useState("");
  const [masterPassword, setMasterPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function selectVault() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "KeePass Database",
          extensions: ["kdbx"],
        },
      ],
    });

    if (typeof selected === "string") {
      setPath(selected);
      setError(null);
    }
  }

  async function handleSubmit(e: SubmitEvent<HTMLFormElement>) {
    e.preventDefault();

    if (!path || !masterPassword) {
      return;
    }

    setLoading(true);
    setError(null);

    try {
      await openVault(path, masterPassword);
      onOpened();
    } catch (error) {
      setError(String(error));
    } finally {
      setLoading(false);
    }
  }

  return (
    <form className="open-vault-form" onSubmit={handleSubmit}>
      <div>
        <label htmlFor="vault-path">Vault</label>

        <div className="vault-path-input">
          <input
            id="vault-path"
            type="text"
            value={path}
            readOnly
            placeholder="Select a .kdbx file"
          />

          <button type="button" onClick={selectVault}>
            Browse
          </button>
        </div>
      </div>

      <div className="form-field">
        <label htmlFor="master-password">Master password</label>

        <input
          id="master-password"
          type="password"
          value={masterPassword}
          onChange={(e) => setMasterPassword(e.target.value)}
        />
      </div>

      {error && <p className="form-error">{error}</p>}

      <button
        className="primary-button"
        type="submit"
        disabled={!path || !masterPassword || loading}
      >
        {loading ? "Opening..." : "Open Vault"}
      </button>
    </form>
  );
}
