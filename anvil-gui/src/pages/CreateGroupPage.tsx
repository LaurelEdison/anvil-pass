import { useState } from "react";
import { createGroup } from "../api/group";

interface CreateGroupPageProps {
	parent: string | null;
	onCreated: () => void;
	onCancel: () => void;
	onDirty: () => void;
}

export function CreateGroupPage({
	parent,
	onCreated,
	onCancel,
	onDirty,
}: CreateGroupPageProps) {
	const [name, setName] = useState("");
	const [notes, setNotes] = useState("");
	const [saving, setSaving] = useState(false);
	const [error, setError] = useState<string | null>(null);

	async function handleCreate() {
		if (!name || saving) {
			return;
		}

		setSaving(true);
		setError(null);

		try {
			await createGroup({
				name,
				notes: notes || null,
				parent,
			});
			onDirty();
			onCreated();
		} catch (error) {
			setError(String(error));
		} finally {
			setSaving(false);
		}
	}

	return (
		<main className="create-group-page">
			<h2>Create Group</h2>

			<div>
				<label htmlFor="group-name">Name</label>

				<input
					id="group-name"
					value={name}
					onChange={(e) => setName(e.target.value)}
				/>
			</div>

			<div>
				<label htmlFor="group-notes">Notes</label>

				<textarea
					id="group-notes"
					value={notes}
					onChange={(e) => setNotes(e.target.value)}
				/>
			</div>

			{error && <p>{error}</p>}

			<div>
				<button type="button" onClick={onCancel}>
					Cancel
				</button>

				<button
					type="button"
					disabled={!name || saving}
					onClick={handleCreate}
				>
					{saving ? "Creating..." : "Create Group"}
				</button>
			</div>
		</main>
	);
}
