import { EntryDto } from "../types/types";

interface EntryListProps {
  entries: EntryDto[];
  selectedEntry: EntryDto | null;
  onSelectEntry: (entry: EntryDto) => void;
  onCreateEntry: () => void;
  onDeleteEntry: (entry: EntryDto) => void;
}

export function EntryList({
  entries,
  onSelectEntry,
  onCreateEntry,
  onDeleteEntry,
}: EntryListProps) {
  return (
    <section className="entry-list">
      <div>
        <h2>Entries</h2>

        <button type="button" onClick={onCreateEntry}>
          New Entry
        </button>
      </div>

      {entries.map((entry) => (
        <div key={entry.id}>
          <button type="button" onClick={() => onSelectEntry(entry)}>
            {entry.title || "Untitled"}
          </button>

          <button type="button" onClick={() => onDeleteEntry(entry)}>
            Delete
          </button>
        </div>
      ))}
    </section>
  );
}
