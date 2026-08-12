import { useState } from "react";
import "./App.css";

import type { EntryDto, GroupDto } from "./types/types";

import { HomePage } from "./pages/HomePage";
import { VaultPage } from "./pages/VaultPage";
import { EditEntryPage } from "./pages/EditEntryPage";
import { CreateEntryPage } from "./pages/CreateEntryPage";
import { CreateGroupPage } from "./pages/CreateGroupPage";
import { EditGroupPage } from "./pages/EditGroupPage";
import { CreateVaultPage } from "./pages/CreateVaultPage";
import { TopBar } from "./components/TopBar";
import { clearVault, saveVault } from "./api/vault";

type Page =
  | { type: "home" }
  | { type: "vault" }
  | { type: "create-vault" }
  | { type: "edit-entry"; entry: EntryDto }
  | { type: "create-entry"; parent: string | null }
  | { type: "create-group"; parent: string | null }
  | { type: "edit-group"; group: GroupDto };

function App() {
  const [page, setPage] = useState<Page>({
    type: "home",
  });

  const [dirty, setDirty] = useState(false);

  async function handleSave() {
    try {
      await saveVault();
      setDirty(false);
    } catch (error) {
      console.error("Failed to save vault:", error);
    }
  }

  async function handleLock() {
    try {
      await clearVault();

      setDirty(false);
      setPage({ type: "home" });
    } catch (error) {
      console.error("Failed to lock vault:", error);
    }
  }

  switch (page.type) {
    case "home":
      return (
        <HomePage
          onOpened={() => setPage({ type: "vault" })}
          onCreateVault={() => setPage({ type: "create-vault" })}
        />
      );

    case "vault":
      return (
        <main>
          <TopBar dirty={dirty} onSave={handleSave} onLock={handleLock} />
          <VaultPage
            onDirty={() => setDirty(true)}
            onEditEntry={(entry) =>
              setPage({
                type: "edit-entry",
                entry,
              })
            }
            onCreateEntry={(parent) =>
              setPage({
                type: "create-entry",
                parent,
              })
            }
            onCreateGroup={(parent) =>
              setPage({
                type: "create-group",
                parent: parent,
              })
            }
            onEditGroup={(group) =>
              setPage({
                type: "edit-group",
                group,
              })
            }
          />
        </main>
      );

    case "create-entry":
      return (
        <CreateEntryPage
          parent={page.parent}
          onCreated={() => setPage({ type: "vault" })}
          onCancel={() => setPage({ type: "vault" })}
          onDirty={() => setDirty(true)}
        />
      );

    case "edit-entry":
      return (
        <EditEntryPage
          entry={page.entry}
          onDirty={() => setDirty(true)}
          onBack={() => setPage({ type: "vault" })}
        />
      );

    case "create-group":
      return (
        <CreateGroupPage
          parent={page.parent}
          onCreated={() => setPage({ type: "vault" })}
          onDirty={() => setDirty(true)}
          onCancel={() => setPage({ type: "vault" })}
        />
      );

    case "create-vault":
      return (
        <CreateVaultPage
          onCreated={() => setPage({ type: "vault" })}
          onCancel={() => setPage({ type: "home" })}
        />
      );

    case "edit-group":
      return (
        <EditGroupPage
          group={page.group}
          onSaved={() => setPage({ type: "vault" })}
          onDirty={() => setDirty(true)}
          onDeleted={() => setPage({ type: "vault" })}
          onCancel={() => setPage({ type: "vault" })}
        />
      );
  }
}

export default App;
