import { useState } from "react";
import type { EntryDto } from "../types/types";
import { updateEntry } from "../api/entry";
import { PasswordGenerator } from "./PasswordGenerator";

interface EntryEditProps {
  entry: EntryDto;
  onSaved: (entry: EntryDto) => void;
  onCancel: () => void;
}

export function EntryEdit({ entry, onSaved, onCancel }: EntryEditProps) {
  const [title, setTitle] = useState(entry.title);
  const [username, setUsername] = useState(entry.username);
  const [password, setPassword] = useState(entry.password);
  const [url, setUrl] = useState(entry.url);
  const [notes, setNotes] = useState(entry.notes);
  const [totp, setTotp] = useState(entry.totp);

  const [showGenerator, setShowGenerator] = useState(false);
  const [saving, setSaving] = useState(false);

  const changed =
    title !== entry.title ||
    username !== entry.username ||
    password !== entry.password ||
    url !== entry.url ||
    notes !== entry.notes ||
    totp !== entry.totp;

  async function handleSave() {
    if (!changed || saving) {
      return;
    }

    setSaving(true);

    try {
      await updateEntry({
        id: entry.id,
        title,
        username,
        password,
        url,
        notes,
        totp,
      });

      onSaved({
        ...entry,
        title,
        username,
        password,
        url,
        notes,
        totp,
      });
    } finally {
      setSaving(false);
    }
  }

  return (
    <section className="entry-edit">
      <h2>Edit Entry</h2>

      <div>
        <label>Title</label>
        <input value={title} onChange={(e) => setTitle(e.target.value)} />
      </div>

      <div>
        <label>Username</label>
        <input value={username} onChange={(e) => setUsername(e.target.value)} />
      </div>

      <div>
        <label>Password</label>

        <div>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />

          <button type="button" onClick={() => setShowGenerator(true)}>
            Generate
          </button>
        </div>
      </div>

      <div>
        <label>URL</label>
        <input value={url} onChange={(e) => setUrl(e.target.value)} />
      </div>

      <div>
        <label>TOTP</label>
        <input value={totp} onChange={(e) => setTotp(e.target.value)} />
      </div>

      <div>
        <label>Notes</label>
        <textarea value={notes} onChange={(e) => setNotes(e.target.value)} />
      </div>

      <div>
        <button type="button" onClick={onCancel}>
          Cancel
        </button>

        <button
          type="button"
          disabled={!changed || saving}
          onClick={handleSave}
        >
          {saving ? "Saving..." : "Save"}
        </button>
      </div>

      <PasswordGenerator
        onUse={(generatedPassword) => {
          setPassword(generatedPassword);
          setShowGenerator(false);
        }}
        onClose={() => setShowGenerator(false)}
      />
    </section>
  );
}
