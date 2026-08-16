import { EntryDto } from "../types/types";

interface EntryDetailProps {
	entry: EntryDto | null;
}

export function EntryDetail({ entry }: EntryDetailProps) {
	if (entry === null) {
		return (
			<section className="entry-detail">
				<p>Select an entry</p>
			</section>
		);
	}

	return (
		<section className="entry-detail">
			<h2>{entry.title}</h2>

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
