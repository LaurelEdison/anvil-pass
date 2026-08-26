import { OpenVaultForm } from "../components/OpenVaultForm";

interface HomePageProps {
  onOpened: () => void;
  onCreateVault: () => void;
}

export function HomePage({ onOpened, onCreateVault }: HomePageProps) {
  return (
    <main>
      <h1>Anvil</h1>
      <p>Open a password vault</p>

      <OpenVaultForm onOpened={onOpened} />
      <button type="button" onClick={onCreateVault}>
        Create New Vault
      </button>
    </main>
  );
}
