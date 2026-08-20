import { EntryDto } from "../types/types";

interface DeleteEntryConfirmationProps {
	entry: EntryDto;
	onConfirm: () => void;
	onCancel: () => void;
}

export function DeleteEntryConfirmation({
	entry,
	onConfirm,
	onCancel,
}: DeleteEntryConfirmationProps) {
	return (
		<div className="delete-confirmation">
			<h2>Delete Entry</h2>

			<p>
				Are you sure you want to delete{" "}
				<strong>{entry.title || "Untitled"}</strong>?
			</p>

			<p>This action cannot be undone.</p>

			<div>
				<button type="button" onClick={onCancel}>
					Cancel
				</button>

				<button type="button" onClick={onConfirm}>
					Delete
				</button>
			</div>
		</div>
	);
}
