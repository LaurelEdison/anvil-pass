import { open } from "@tauri-apps/plugin-dialog";

import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

interface HomeProps {
  onOpened: () => void;
}

export function Home({ onOpened }: HomeProps) {
  const [path, setPath] = useState("");
  const [masterPassword, setMasterPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [opening, setOpening] = useState(false);

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

  async function openVault() {
    if (!path || !masterPassword || opening) {
      return;
    }

    setOpening(true);
    setError(null);

    try {
      await invoke("open_a_vault", {
        path,
        masterPassword,
      });

      onOpened();
    } catch (error) {
      setError(String(error));
    } finally {
      setOpening(false);
    }
  }

  return (
    <main className="home">
      <div className="home-card">
        <h1>Anvil</h1>
        <p>Open your password vault</p>

        <div>
          <label htmlFor="vault-path">Vault</label>

          <div>
            <input
              id="vault-path"
              value={path}
              readOnly
              placeholder="Select a .kdbx file"
            />

            <button type="button" onClick={selectVault}>
              Browse
            </button>
          </div>
        </div>

        <div>
          <label htmlFor="master-password">Master password</label>

          <input
            id="master-password"
            type="password"
            value={masterPassword}
            onChange={(e) => setMasterPassword(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                openVault();
              }
            }}
          />
        </div>

        {error && <p className="error">{error}</p>}

        <button
          type="button"
          disabled={!path || !masterPassword || opening}
          onClick={openVault}
        >
          {opening ? "Opening..." : "Open Vault"}
        </button>
      </div>
    </main>
  );
}
