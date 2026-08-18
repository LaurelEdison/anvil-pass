import { useState } from "react";
import "./App.css";
import { HomePage } from "./pages/HomePage";

function App() {
  const [vaultOpened, setVaultOpened] = useState(false);

  if (!vaultOpened) {
    return <HomePage onOpened={() => setVaultOpened(true)} />;
  }
}

export default App;
