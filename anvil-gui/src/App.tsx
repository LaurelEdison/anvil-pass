import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { EntryDto, GroupDto } from "./types/types";
import { Sidebar } from "./components/Sidebar";
import { EntryList } from "./components/EntryList";
import { EntryDetail } from "./components/EntryDetail";

function App() {
  const [groups, setGroups] = useState<GroupDto[]>([]);
  const [entries, setEntries] = useState<EntryDto[]>([]);
  const [selectedGroup, setSelectedGroup] = useState<string | null>(null);
  const [selectedEntry, setSelectedEntry] = useState<EntryDto | null>(null);
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

  const visibleEntries =
    selectedGroup === null
      ? entries
      : entries.filter((entry) => entry.parent === selectedGroup);

  if (error) {
    return (
      <main className="app">
        <h1>Error</h1>
        <pre>{error}</pre>
      </main>
    );
  }

  return (
    <main className="app">
      <Sidebar
        groups={groups}
        selectedGroup={selectedGroup}
        onSelectGroup={(groupId) => {
          setSelectedGroup(groupId);
          setSelectedEntry(null);
        }}
      />

      <EntryList
        entries={visibleEntries}
        selectedEntry={selectedEntry}
        onSelectEntry={setSelectedEntry}
      />

      <EntryDetail entry={selectedEntry} />
    </main>
  );
}

export default App;
