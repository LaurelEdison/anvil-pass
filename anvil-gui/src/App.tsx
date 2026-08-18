import { useState } from "react";
import { Home } from "./components/Home";
import { Vault } from "./components/Vault";
import "./App.css";

function App() {
  const [vaultOpened, setVaultOpened] = useState(false);

  if (!vaultOpened) {
    return <Home onOpened={() => setVaultOpened(true)} />;
  }

  return <Vault />;
}

export default App;
