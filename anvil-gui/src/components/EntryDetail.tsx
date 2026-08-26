import { useState } from "react";
import { EntryDto } from "../types/types";
import { openUrl } from "@tauri-apps/plugin-opener";

interface EntryDetailProps {
  entry: EntryDto | null;
  onEdit: (entry: EntryDto) => void;
}
function normalizeUrl(url: string): string {
  if (/^[a-z][a-z0-9+.-]*:/i.test(url)) {
    return url;
  }

  return `https://${url}`;
}
export function EntryDetail({ entry, onEdit }: EntryDetailProps) {
  const [showPassword, setShowPassword] = useState(false);

  if (entry === null) {
    return (
      <section className="entry-detail">
        <p>Select an entry</p>
      </section>
    );
  }

  async function copyToClipboard(value: string) {
    await navigator.clipboard.writeText(value);
  }

  async function openUrlInBrowser(url: string) {
    await openUrl(normalizeUrl(url));
  }

  return (
    <section className="entry-detail">
      <div>
        <h2>{entry.title || "Untitled"}</h2>

        <button type="button" onClick={() => onEdit(entry)}>
          Edit
        </button>
      </div>

      {entry.username && (
        <div>
          <strong>Username</strong>

          <div>
            <p>{entry.username}</p>

            <button
              type="button"
              onClick={() => copyToClipboard(entry.username)}
            >
              Copy
            </button>
          </div>
        </div>
      )}

      {entry.password && (
        <div>
          <strong>Password</strong>

          <div>
            <p>{showPassword ? entry.password : "••••••••"}</p>

            <button
              type="button"
              onClick={() => setShowPassword((value) => !value)}
            >
              {showPassword ? "Hide" : "Show"}
            </button>

            <button
              type="button"
              onClick={() => copyToClipboard(entry.password)}
            >
              Copy
            </button>
          </div>
        </div>
      )}

      {entry.url && (
        <div>
          <strong>URL</strong>

          <div>
            <p>{entry.url}</p>

            <button type="button" onClick={() => copyToClipboard(entry.url)}>
              Copy
            </button>
            <button type="button" onClick={() => openUrlInBrowser(entry.url)}>
              Open
            </button>
          </div>
        </div>
      )}

      {entry.totp && (
        <div>
          <strong>TOTP</strong>
          <p>{entry.totp}</p>
        </div>
      )}

      {entry.notes && (
        <div>
          <strong>Notes</strong>
          <p>{entry.notes}</p>
        </div>
      )}
    </section>
  );
}
