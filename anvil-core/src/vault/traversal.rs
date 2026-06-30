use keepass::db::GroupRef;
use uuid::Uuid;

use crate::vault::{DatabaseProcessingError, Vault};

// TODO right now just flat dfs, eventually implement to parent/child tree to support drag and drop
// and copy paste blah blah

pub struct TreeVault {
    pub nodes: Vec<TreeNode>,
}
impl TreeVault {
    pub fn populate(group_ref: &GroupRef) -> Self {
        let mut nodes = Vec::new();

        TreeVault::build_node_recursive(group_ref, None, &mut nodes, 0);

        Self { nodes }
    }
    fn build_node_recursive(
        current_group: &GroupRef,
        parent_id: Option<Uuid>,
        nodes: &mut Vec<TreeNode>,
        depth: usize,
    ) -> Uuid {
        let index = nodes.len();
        let current_group_id = current_group.id().uuid();
        let curr_group_node = TreeNode {
            id: current_group_id,
            display_name: current_group.name.clone(),
            children_id: Vec::new(),
            parent_id: parent_id,
            node_type: NodeType::Group,
            depth,
        };

        nodes.push(curr_group_node);

        let mut children_ids = Vec::new();

        for entry in current_group.entries() {
            let entry_id = entry.id().uuid();
            let display_name = match entry.get_title() {
                Some(s) => s,
                None => &entry_id.clone().to_string(),
            };
            let entry_node = TreeNode {
                children_id: Vec::new(),
                parent_id: Some(current_group_id),
                id: entry_id,
                display_name: display_name.into(),
                node_type: NodeType::Entry,
                depth: depth + 1,
            };
            nodes.push(entry_node);
            children_ids.push(entry_id);
        }
        for subgroup in current_group.groups() {
            let subgroup_id = TreeVault::build_node_recursive(
                &subgroup,
                Some(current_group_id),
                nodes,
                depth + 1,
            );
            children_ids.push(subgroup_id);
        }

        nodes[index].children_id = children_ids;
        current_group_id
    }
}

// I think we're only gonna store metadata for now
pub struct TreeNode {
    pub id: Uuid,
    pub display_name: String,
    pub node_type: NodeType,
    pub parent_id: Option<Uuid>,
    pub children_id: Vec<Uuid>,
    pub depth: usize,
}
impl TreeNode {
    pub fn is_group(&self) -> bool {
        if matches!(self.node_type, NodeType::Group) {
            return true;
        }
        false
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }
    pub fn get_display_name(&self) -> String {
        return self.display_name.clone();
    }
    pub fn get_node_type(&self) -> NodeType {
        return self.node_type;
    }
    pub fn get_parent_id(&self) -> Option<Uuid> {
        return self.parent_id;
    }
    pub fn get_children_id(&self) -> Vec<Uuid> {
        return self.children_id.clone();
    }
    pub fn get_depth(&self) -> usize {
        return self.depth;
    }
}
// Hence why these structs are only to differentiate between node types, I'll think about what data
// to store in the future
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeType {
    Entry,
    Group,
}
pub struct EntryData;
pub struct GroupData;

impl Vault {
    pub fn as_tree(&self, group_name: Option<Uuid>) -> Result<TreeVault, DatabaseProcessingError> {
        let group = match group_name {
            Some(s) => self.get_group(s)?,
            None => self.database.root(),
        };
        let tree = TreeVault::populate(&self.database.root());
        Ok(tree)
    }
}
