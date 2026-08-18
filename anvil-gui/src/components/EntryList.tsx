import { EntryDto } from "../types/types";

interface EntryListProps {
  entries: EntryDto[];
  selectedEntry: EntryDto | null;
  onSelectEntry: (entry: EntryDto) => void;
  onCreateEntry: () => void;
}

export function EntryList({
  entries,
  onSelectEntry,
  onCreateEntry,
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
        <button
          type="button"
          key={entry.id}
          onClick={() => onSelectEntry(entry)}
        >
          {entry.title || "Untitled"}
        </button>
      ))}
    </section>
  );
}
