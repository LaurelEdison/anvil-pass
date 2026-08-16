import { EntryDto } from "../types/types";

interface EntryListProps {
	entries: EntryDto[];
	selectedEntry: EntryDto | null;
	onSelectEntry: (entry: EntryDto) => void;
}

export function EntryList({
	entries,
	selectedEntry,
	onSelectEntry,
}: EntryListProps) {
	return (
		<section className="entry-list">
			<h2>Entries</h2>

			{entries.map((entry) => (
				<button
					className="entry-item"
					key={entry.id}
					onClick={() => onSelectEntry(entry)}
					disabled={selectedEntry?.id === entry.id}
				>
					<strong>{entry.title}</strong>
					<span>{entry.username}</span>
				</button>
			))}
		</section>
	);
}
