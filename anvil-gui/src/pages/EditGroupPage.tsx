import { useState } from "react";
import type { GroupDto } from "../types/types";
import { updateGroup, deleteGroup } from "../api/group";

interface EditGroupPageProps {
	group: GroupDto;
	onSaved: (group: GroupDto) => void;
	onDeleted: () => void;
	onCancel: () => void;
}

export function EditGroupPage({
	group,
	onSaved,
	onDeleted,
	onCancel,
}: EditGroupPageProps) {
	const [name, setName] = useState(group.name);
	const [notes, setNotes] = useState(group.notes);

	const [saving, setSaving] = useState(false);
	const [deleting, setDeleting] = useState(false);
	const [error, setError] = useState<string | null>(null);

	const changed =
		name !== group.name ||
		notes !== group.notes;

	async function handleSave() {
		if (!changed || saving || deleting) {
			return;
		}

		setSaving(true);
		setError(null);

		try {
			await updateGroup({
				groupId: group.id,
				name,
				notes,
			});

			onSaved({
				...group,
				name,
				notes,
			});
		} catch (error) {
			setError(String(error));
		} finally {
			setSaving(false);
		}
	}

	async function handleDelete() {
		if (deleting || saving) {
			return;
		}

		const confirmed = window.confirm(
			`Delete group "${group.name}"?`,
		);

		if (!confirmed) {
			return;
		}

		setDeleting(true);
		setError(null);

		try {
			await deleteGroup(group.id);
			onDeleted();
		} catch (error) {
			setError(String(error));
		} finally {
			setDeleting(false);
		}
	}

	return (
		<main className="edit-group-page">
			<h2>Edit Group</h2>

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
				<button
					type="button"
					disabled={saving || deleting}
					onClick={onCancel}
				>
					Cancel
				</button>

				<button
					type="button"
					disabled={!changed || saving || deleting}
					onClick={handleSave}
				>
					{saving ? "Saving..." : "Save"}
				</button>
			</div>

			<div>
				<button
					type="button"
					disabled={saving || deleting}
					onClick={handleDelete}
				>
					{deleting ? "Deleting..." : "Delete Group"}
				</button>
			</div>
		</main>
	);
}
