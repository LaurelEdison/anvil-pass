import type { GroupDto } from "../types/types";

interface SidebarProps {
  groups: GroupDto[];
  selectedGroup: string | null;
  onSelectGroup: (groupId: string | null) => void;
  onCreateGroup: () => void;
  onEditGroup: (group: GroupDto) => void;
}

export function Sidebar({
  groups,
  selectedGroup,
  onSelectGroup,
  onCreateGroup,
  onEditGroup,
}: SidebarProps) {
  return (
    <aside className="sidebar">
      <div>
        <h2>Groups</h2>

        <button type="button" onClick={onCreateGroup}>
          +
        </button>
      </div>

      <button
        type="button"
        onClick={() => onSelectGroup(null)}
      >
        All Entries
      </button>

      {groups.map((group) => (
        <button
          type="button"
          key={group.id}
          onClick={() => onSelectGroup(group.id)}
          onDoubleClick={() => onEditGroup(group)}
        >
          {group.name}
        </button>
      ))}
    </aside>
  );
}
