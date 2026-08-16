import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { EntryDto, GroupDto } from "./types/types";

function App() {
  const [groups, setGroups] = useState<GroupDto[]>([]);
  const [entries, setEntries] = useState<EntryDto[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadVault() {
      try {
        await invoke("open_a_vault", {
          path: "C:\\Users\\arelf\\Downloads\\testing.kdbx",
          masterPassword: "password",
        });

        const groups = await invoke<GroupDto[]>("list_groups");
        const entries = await invoke<EntryDto[]>("list_entries");

        setGroups(groups);
        setEntries(entries);
      } catch (error) {
        console.error(error);
        setError(String(error));
      }
    }

    loadVault();
  }, []);

  if (error) {
    return (
      <main className="container">
        <h1>Error</h1>
        <pre>{error}</pre>
      </main>
    );
  }

  return (
    <main className="container">
      <h1>Anvil</h1>

      <h2>Groups</h2>

      {groups.length === 0 ? (
        <p>No groups found.</p>
      ) : (
        groups.map((group) => (
          <div key={group.id}>
            {group.name}
          </div>
        ))
      )}

      <h2>Entries</h2>

      {entries.length === 0 ? (
        <p>No entries found.</p>
      ) : (
        entries.map((entry) => (
          <div key={entry.id}>
            <h3>{entry.title}</h3>
            <p>Username: {entry.username}</p>
            <p>Password: {entry.password}</p>
            <p>URL: {entry.url}</p>
          </div>
        ))
      )}
    </main>
  );
}

export default App;
