interface TopBarProps {
  dirty: boolean;
  onSave: () => void;
  onLock: () => void;
}

export function TopBar({ dirty, onSave, onLock }: TopBarProps) {
  return (
    <nav className="top-bar">
      <strong>Anvil</strong>

      <div>
        <button type="button" disabled={!dirty} onClick={onSave}>
          {dirty ? "Save*" : "Save"}
        </button>

        <button type="button" onClick={onLock}>
          Lock
        </button>
      </div>
    </nav>
  );
}
