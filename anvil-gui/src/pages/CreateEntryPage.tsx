import { CreateEntry } from "../components/CreateEntry";

interface CreateEntryPageProps {
	parent: string | null;
	onCreated: () => void;
	onCancel: () => void;
	onDirty: () => void;
}

export function CreateEntryPage({
	parent,
	onCreated,
	onCancel,
	onDirty,
}: CreateEntryPageProps) {
	return (
		<main className="create-entry-page">
			<CreateEntry
				parent={parent}
				onCreated={onCreated}
				onCancel={onCancel}
				onDirty={onDirty}
			/>
		</main>
	);
}
