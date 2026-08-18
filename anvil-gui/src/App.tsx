import { useState } from "react";
import "./App.css";
import { EntryDto } from "./types/types";
import { HomePage } from "./pages/HomePage";
import { VaultPage } from "./pages/VaultPage";
import { EditEntryPage } from "./pages/EditEntryPage";


type Page =
  | { type: "home" }
  | { type: "vault" }
  | { type: "edit-entry"; entry: EntryDto }
  | { type: "create-entry"; parent: string | null };
function App() {
  const [page, setPage] = useState<Page>({
    type: "home",
  });

  switch (page.type) {
    case "home":
      return (
        <HomePage
          onOpened={() => setPage({ type: "vault" })}
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
        />
      );

    case "edit-entry":
      return (
        <EditEntryPage
          entry={page.entry}
          onBack={() => setPage({ type: "vault" })}
        />
      );
  }
}

export default App;
