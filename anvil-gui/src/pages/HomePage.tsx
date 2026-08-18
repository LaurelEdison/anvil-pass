import { OpenVaultForm } from "../components/OpenVaultForm";

interface HomePageProps {
  onOpened: () => void;
}

export function HomePage({ onOpened }: HomePageProps) {
  return (
    <main>
      <h1>Anvil</h1>
      <p>Open a password vault</p>

      <OpenVaultForm onOpened={onOpened} />
    </main>
  );
}
