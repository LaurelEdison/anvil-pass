import { GroupDto } from "../types/types";

interface SidebarProps {
	groups: GroupDto[];
	selectedGroup: string | null;
	onSelectGroup: (groupId: string | null) => void;
}

export function Sidebar({
	groups,
	selectedGroup,
	onSelectGroup,
}: SidebarProps) {
	return (
		<aside className="sidebar">
			<h2>Logo</h2>

			<button
				onClick={() => onSelectGroup(null)}
				disabled={selectedGroup === null}
			>
				All Items
			</button>

			<h3>Groups</h3>

			{groups.map((group) => (
				<button
					key={group.id}
					onClick={() => onSelectGroup(group.id)}
					disabled={selectedGroup === group.id}
				>
					{group.name}
				</button>
			))}
		</aside>
	);
}
