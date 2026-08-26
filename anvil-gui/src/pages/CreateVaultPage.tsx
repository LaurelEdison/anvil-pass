import { useState } from "react";
import { createVault } from "../api/vault";

interface CreateVaultPageProps {
  onCreated: () => void;
  onCancel: () => void;
}

export function CreateVaultPage({ onCreated, onCancel }: CreateVaultPageProps) {
  const [path, setPath] = useState("");
  const [masterPassword, setMasterPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");

  const [showPassword, setShowPassword] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const passwordsMatch = masterPassword === confirmPassword;

  async function handleCreate() {
    if (
      !path ||
      !masterPassword ||
      !confirmPassword ||
      !passwordsMatch ||
      saving
    ) {
      return;
    }

    setSaving(true);
    setError(null);

    try {
      await createVault(path, masterPassword);
      onCreated();
    } catch (error) {
      setError(String(error));
    } finally {
      setSaving(false);
    }
  }

  return (
    <main className="create-vault-page">
      <h1>Create Vault</h1>

      <div>
        <label htmlFor="vault-path">Vault path</label>

        <input
          id="vault-path"
          type="text"
          value={path}
          onChange={(e) => setPath(e.target.value)}
          placeholder="Path to new .kdbx file"
        />
      </div>

      <div>
        <label htmlFor="master-password">Master password</label>

        <input
          id="master-password"
          type={showPassword ? "text" : "password"}
          value={masterPassword}
          onChange={(e) => setMasterPassword(e.target.value)}
        />
      </div>

      <div>
        <label htmlFor="confirm-password">Confirm master password</label>

        <input
          id="confirm-password"
          type={showPassword ? "text" : "password"}
          value={confirmPassword}
          onChange={(e) => setConfirmPassword(e.target.value)}
        />
      </div>

      <label>
        <input
          type="checkbox"
          checked={showPassword}
          onChange={(e) => setShowPassword(e.target.checked)}
        />
        Show password
      </label>

      {confirmPassword && !passwordsMatch && <p>Passwords do not match.</p>}

      {error && <p>{error}</p>}

      <div>
        <button type="button" onClick={onCancel} disabled={saving}>
          Cancel
        </button>

        <button
          type="button"
          disabled={
            !path ||
            !masterPassword ||
            !confirmPassword ||
            !passwordsMatch ||
            saving
          }
          onClick={handleCreate}
        >
          {saving ? "Creating..." : "Create Vault"}
        </button>
      </div>
    </main>
  );
}
