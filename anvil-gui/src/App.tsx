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
        <VaultPage
          onEditEntry={(entry) =>
            setPage({
              type: "edit-entry",
              entry,
            })
          }
          onCreateEntry={() =>
            setPage({
              type: "create-entry",
              parent: null,
            })
          }
          onCreateGroup={() =>
            setPage({
              type: "create-group",
              parent: null,
            })
          }
          onEditGroup={(group) =>
            setPage({
              type: "edit-group",
              group,
            })
          }
        />
      );

    case "create-entry":
      return (
        <CreateEntryPage
          parent={page.parent}
          onCreated={() => setPage({ type: "vault" })}
          onCancel={() => setPage({ type: "vault" })}
        />
      );

    case "edit-entry":
      return (
        <EditEntryPage
          entry={page.entry}
          onBack={() => setPage({ type: "vault" })}
        />
      );

    case "create-group":
      return (
        <CreateGroupPage
          parent={page.parent}
          onCreated={() => setPage({ type: "vault" })}
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
          onDeleted={() => setPage({ type: "vault" })}
          onCancel={() => setPage({ type: "vault" })}
        />
      );
  }
}

export default App;
