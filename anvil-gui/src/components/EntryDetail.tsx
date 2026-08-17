import { EntryDto } from "../types/types";

interface EntryDetailProps {
	entry: EntryDto | null;
	onEdit: (entry: EntryDto) => void;
}

export function EntryDetail({ entry, onEdit }: EntryDetailProps) {
	if (entry === null) {
		return (
			<section className="entry-detail">
				<p>Select an entry</p>
			</section>
		);
	}

	return (
		<section className="entry-detail">
			<div>
				<h2>{entry.title || "Untitled"}</h2>

				<button onClick={() => onEdit(entry)}>
					Edit
				</button>
			</div>

			<div>
				<strong>Username</strong>
				<p>{entry.username}</p>
			</div>

			<div>
				<strong>Password</strong>
				<p>{entry.password}</p>
			</div>

			<div>
				<strong>URL</strong>
				<p>{entry.url}</p>
			</div>

			<div>
				<strong>TOTP</strong>
				<p>{entry.totp}</p>
			</div>

			<div>
				<strong>Notes</strong>
				<p>{entry.notes}</p>
			</div>
		</section>
	);
}
