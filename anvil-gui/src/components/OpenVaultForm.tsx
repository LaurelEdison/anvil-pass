import { useState } from "react";
import { openVault } from "../api/vault";
import type { SubmitEvent } from "react";

interface OpenVaultFormProps {
  onOpened: () => void;
}

export function OpenVaultForm({ onOpened }: OpenVaultFormProps) {
  const [path, setPath] = useState("");
  const [masterPassword, setMasterPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

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
    <form onSubmit={handleSubmit}>
      <div>
        <label htmlFor="vault-path">Vault</label>

        <input
          id="vault-path"
          type="text"
          value={path}
          onChange={(e) => setPath(e.target.value)}
          placeholder="Path to .kdbx file"
        />
      </div>

      <div>
        <label htmlFor="master-password">Master password</label>

        <input
          id="master-password"
          type="password"
          value={masterPassword}
          onChange={(e) => setMasterPassword(e.target.value)}
        />
      </div>

      {error && <p>{error}</p>}

      <button type="submit" disabled={!path || !masterPassword || loading}>
        {loading ? "Opening..." : "Open Vault"}
      </button>
    </form>
  );
}
