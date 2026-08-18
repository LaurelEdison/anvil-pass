import { CreateEntry } from "../components/CreateEntry";

interface CreateEntryPageProps {
	parent: string | null;
	onCreated: () => void;
	onCancel: () => void;
}

export function CreateEntryPage({
	parent,
	onCreated,
	onCancel,
}: CreateEntryPageProps) {
	return (
		<main className="create-entry-page">
			<CreateEntry
				parent={parent}
				onCreated={onCreated}
				onCancel={onCancel}
			/>
		</main>
	);
}
