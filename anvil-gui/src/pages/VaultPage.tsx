import { useEffect, useState } from "react";
import { EntryDto, GroupDto } from "../types/types";
import { listEntries, deleteEntry } from "../api/entry";
import { listGroups } from "../api/group";
import { Sidebar } from "../components/Sidebar";
import { EntryList } from "../components/EntryList";
import { EntryDetail } from "../components/EntryDetail";
import { DeleteEntryConfirmation } from "../components/DeleteEntryConfirmation";

interface VaultPageProps {
	onEditEntry: (entry: EntryDto) => void;
	onCreateEntry: (parent: string | null) => void;
	onCreateGroup: (parent: string | null) => void;
	onEditGroup: (group: GroupDto) => void;
}

export function VaultPage({
	onEditEntry,
	onCreateEntry,
	onCreateGroup,
	onEditGroup,
}: VaultPageProps) {
	const [entries, setEntries] = useState<EntryDto[]>([]);
	const [groups, setGroups] = useState<GroupDto[]>([]);
	const [selectedEntry, setSelectedEntry] = useState<EntryDto | null>(null);
	const [selectedGroup, setSelectedGroup] = useState<string | null>(null);

	const [error, setError] = useState<string | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<EntryDto | null>(null);

	useEffect(() => {
		async function loadVault() {
			try {
				const [entries, groups] = await Promise.all([
					listEntries(),
					listGroups(),
				]);

				setEntries(entries);
				setGroups(groups);
			} catch (error) {
				setError(String(error));
			}
		}

		loadVault();
	}, []);

	async function handleDeleteEntry() {
		if (!deleteTarget) {
			return;
		}

		try {
			await deleteEntry(deleteTarget.id);

			setEntries((entries) =>
				entries.filter((entry) => entry.id !== deleteTarget.id)
			);

			if (selectedEntry?.id === deleteTarget.id) {
				setSelectedEntry(null);
			}

			setDeleteTarget(null);
		} catch (error) {
			setError(String(error));
		}
	}

	if (error) {
		return <p>{error}</p>;
	}

	return (
		<main className="vault-page">
			<Sidebar
				groups={groups}
				selectedGroup={selectedGroup}
				onSelectGroup={setSelectedGroup}
				onCreateGroup={() => onCreateGroup(selectedGroup)}
				onEditGroup={onEditGroup}
			/>

			<EntryList
				entries={entries}
				selectedEntry={selectedEntry}
				onSelectEntry={setSelectedEntry}
				onCreateEntry={() => onCreateEntry(selectedGroup)}
				onDeleteEntry={setDeleteTarget}
			/>

			<EntryDetail
				entry={selectedEntry}
				onEdit={onEditEntry}
			/>

			{deleteTarget && (
				<DeleteEntryConfirmation
					entry={deleteTarget}
					onConfirm={handleDeleteEntry}
					onCancel={() => setDeleteTarget(null)}
				/>
			)}
		</main>
	);
}
