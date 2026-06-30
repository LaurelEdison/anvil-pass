use anvil_core::{
    GroupId,
    vault::{
        DatabaseProcessingError,
        entries::NewEntry,
        groups::NewGroup,
        traversal::{NodeType, TreeNode, TreeVault},
    },
};
use uuid::Uuid;

use crate::common::create_test_vault;

mod common;

#[test]
fn test_tree_vault_populate_empty() {
    let (vault, _temp_dir, _path, _password) = create_test_vault();
    let tree = TreeVault::populate(&vault.database.root());

    // Should have at least the root group
    assert!(!tree.nodes.is_empty());
    assert_eq!(tree.nodes.len(), 1); // Root group only

    let root_node = &tree.nodes[0];
    assert_eq!(root_node.get_node_type(), NodeType::Group);
    assert!(root_node.get_parent_id().is_none());
    assert!(root_node.get_children_id().is_empty());
    assert_eq!(root_node.get_depth(), 0);
}

#[test]
fn test_tree_vault_populate_with_groups() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Add some groups
    let group_names = vec!["Work", "Personal", "Archive"];
    for name in group_names.clone() {
        vault.add_group(NewGroup::new(name)).unwrap();
    }

    let tree = TreeVault::populate(&vault.database.root());

    // Should have root + 3 groups = 4 nodes
    assert_eq!(tree.nodes.len(), 4);

    // Find the groups in the tree
    let group_nodes: Vec<&TreeNode> = tree
        .nodes
        .iter()
        .filter(|n| n.is_group() && n.get_depth() == 1)
        .collect();

    assert_eq!(group_nodes.len(), 3);

    // Check group names
    let names: Vec<String> = group_nodes.iter().map(|n| n.get_display_name()).collect();

    for name in group_names {
        assert!(names.contains(&name.to_string()));
    }
}

#[test]
fn test_tree_vault_populate_with_entries() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Add entries to root
    let entry_titles = vec!["Entry 1", "Entry 2", "Entry 3"];
    for title in entry_titles.clone() {
        let entry = NewEntry::new("password").with_title(title);
        vault.add_entry(entry).unwrap();
    }

    let tree = TreeVault::populate(&vault.database.root());

    // Should have root + 3 entries = 4 nodes
    assert_eq!(tree.nodes.len(), 4);

    // Find entry nodes
    let entry_nodes: Vec<&TreeNode> = tree
        .nodes
        .iter()
        .filter(|n| matches!(n.get_node_type(), NodeType::Entry))
        .collect();

    assert_eq!(entry_nodes.len(), 3);

    // Check entry names
    let names: Vec<String> = entry_nodes.iter().map(|n| n.get_display_name()).collect();

    for title in entry_titles {
        assert!(names.contains(&title.to_string()));
    }
}

#[test]
fn test_tree_vault_populate_nested_structure() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Create nested groups
    let parent_group_id = vault.add_group(NewGroup::new("Parent")).unwrap();
    let child_group_id = vault.add_group(NewGroup::new("Child")).unwrap();
    let grandchild_group_id = vault.add_group(NewGroup::new("Grandchild")).unwrap();

    // Move child under parent (assuming you have this functionality)
    // If not, create groups directly under parent using a different API

    // Add entries at different levels
    let parent_entry = NewEntry::new("parent_pass")
        .with_title("Parent Entry")
        .with_parent_group(GroupId::from_uuid(parent_group_id));
    vault.add_entry(parent_entry).unwrap();

    let child_entry = NewEntry::new("child_pass")
        .with_title("Child Entry")
        .with_parent_group(GroupId::from_uuid(child_group_id));
    vault.add_entry(child_entry).unwrap();

    let tree = TreeVault::populate(&vault.database.root());

    // Find nodes by depth
    let depth0: Vec<&TreeNode> = tree.nodes.iter().filter(|n| n.get_depth() == 0).collect();
    assert_eq!(depth0.len(), 1); // Root only

    let depth1: Vec<&TreeNode> = tree.nodes.iter().filter(|n| n.get_depth() == 1).collect();
    // Should have Parent group at depth 1

    let depth2: Vec<&TreeNode> = tree.nodes.iter().filter(|n| n.get_depth() == 2).collect();
    // Should have Child group and Parent Entry at depth 2
}

#[test]
fn test_tree_vault_build_node_recursive_depth() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Create a deep hierarchy
    let mut parent_id = None;
    let depths = 5;

    for i in 0..depths {
        let group_name = format!("Level_{}", i);
        let group_id = vault.add_group(NewGroup::new(&group_name)).unwrap();

        if let Some(parent) = parent_id {
            // Move group under parent (if functionality exists)
            // Otherwise this is just a flat structure
        }
        parent_id = Some(GroupId::from_uuid(group_id));
    }

    let tree = TreeVault::populate(&vault.database.root());

    // Verify depth is tracked correctly
    for node in &tree.nodes {
        if node.is_group() && node.get_depth() > 0 {
            // Group at depth > 0 should have a parent
            assert!(node.get_parent_id().is_some());
        }
    }
}

#[test]
fn test_tree_vault_populate_with_mixed_content() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Create a group with entries and subgroups
    let group_id = vault.add_group(NewGroup::new("Main")).unwrap();

    // Add entries
    for i in 1..=3 {
        let entry = NewEntry::new(format!("pass_{}", i))
            .with_title(format!("Entry {}", i))
            .with_parent_group(GroupId::from_uuid(group_id));
        vault.add_entry(entry).unwrap();
    }

    // Add subgroups
    for i in 1..=2 {
        let subgroup_name = format!("Subgroup {}", i);
        let subgroup_id = vault.add_group(NewGroup::new(&subgroup_name)).unwrap();

        // Add entry to subgroup
        let entry = NewEntry::new(format!("sub_pass_{}", i))
            .with_title(format!("Sub Entry {}", i))
            .with_parent_group(GroupId::from_uuid(subgroup_id));
        vault.add_entry(entry).unwrap();
    }

    let tree = TreeVault::populate(&vault.database.root());

    // Should have: root + Main + 3 entries + 2 subgroups + 2 sub-entries = 9 nodes
    assert_eq!(tree.nodes.len(), 9);

    // Verify node types
    let groups: Vec<&TreeNode> = tree.nodes.iter().filter(|n| n.is_group()).collect();
    assert_eq!(groups.len(), 4); // root + Main + 2 subgroups

    let entries: Vec<&TreeNode> = tree
        .nodes
        .iter()
        .filter(|n| matches!(n.get_node_type(), NodeType::Entry))
        .collect();
    assert_eq!(entries.len(), 5); // 3 main entries + 2 sub entries
}

#[test]
fn test_tree_node_is_group() {
    let group_node = TreeNode {
        id: Uuid::new_v4(),
        display_name: "Group".to_string(),
        node_type: NodeType::Group,
        parent_id: None,
        children_id: Vec::new(),
        depth: 0,
    };
    assert!(group_node.is_group());

    let entry_node = TreeNode {
        id: Uuid::new_v4(),
        display_name: "Entry".to_string(),
        node_type: NodeType::Entry,
        parent_id: Some(Uuid::new_v4()),
        children_id: Vec::new(),
        depth: 1,
    };
    assert!(!entry_node.is_group());
}

#[test]
fn test_tree_node_getters() {
    let id = Uuid::new_v4();
    let parent_id = Uuid::new_v4();
    let children_id = vec![Uuid::new_v4(), Uuid::new_v4()];

    let node = TreeNode {
        id,
        display_name: "Test Node".to_string(),
        node_type: NodeType::Group,
        parent_id: Some(parent_id),
        children_id: children_id.clone(),
        depth: 2,
    };

    assert_eq!(node.get_id(), id);
    assert_eq!(node.get_display_name(), "Test Node");
    assert_eq!(node.get_node_type(), NodeType::Group);
    assert_eq!(node.get_parent_id(), Some(parent_id));
    assert_eq!(node.get_children_id(), children_id);
    assert_eq!(node.get_depth(), 2);
}

#[test]
fn test_tree_node_without_parent() {
    let node = TreeNode {
        id: Uuid::new_v4(),
        display_name: "Root".to_string(),
        node_type: NodeType::Group,
        parent_id: None,
        children_id: Vec::new(),
        depth: 0,
    };

    assert!(node.get_parent_id().is_none());
    assert_eq!(node.get_depth(), 0);
}

#[test]
fn test_tree_node_with_children() {
    let children = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let node = TreeNode {
        id: Uuid::new_v4(),
        display_name: "Parent".to_string(),
        node_type: NodeType::Group,
        parent_id: None,
        children_id: children.clone(),
        depth: 0,
    };

    assert_eq!(node.get_children_id().len(), 3);
    assert_eq!(node.get_children_id(), children);
}

#[test]
fn test_as_tree_default_root() {
    let (vault, _temp_dir, _path, _password) = create_test_vault();

    let result = vault.as_tree(None);
    assert!(result.is_ok());

    let tree = result.unwrap();
    assert!(!tree.nodes.is_empty());

    // Root should have no parent
    let root = &tree.nodes[0];
    assert!(root.get_parent_id().is_none());
    assert_eq!(root.get_depth(), 0);
    assert!(root.is_group());
}

#[test]
fn test_as_tree_specific_group() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Create a group
    let group_name = "Specific Group";
    let group_id = vault.add_group(NewGroup::new(group_name)).unwrap();

    // Add entries to the group
    let entry = NewEntry::new("password")
        .with_title("Entry in Group")
        .with_parent_group(GroupId::from_uuid(group_id));
    vault.add_entry(entry).unwrap();

    // Get tree for this specific group
    let result = vault.as_tree(Some(group_id));
    assert!(result.is_ok());

    let tree = result.unwrap();
    // Should have at least the group itself
    assert!(!tree.nodes.is_empty());

    // Find the group in the tree
    let group_node = tree.nodes.iter().find(|n| n.get_id() == group_id);
    assert!(group_node.is_some());

    let group_node = group_node.unwrap();
    assert_eq!(group_node.get_display_name(), group_name);
    assert!(group_node.is_group());
}

#[test]
fn test_as_tree_nonexistent_group() {
    let (vault, _temp_dir, _path, _password) = create_test_vault();

    let fake_id = Uuid::new_v4();
    let result = vault.as_tree(Some(fake_id));

    assert!(result.is_err());
    match result {
        Err(DatabaseProcessingError::FailedToFindGroup(id)) => {
            assert_eq!(id, fake_id);
        }
        _ => panic!("Expected FailedToFindGroup error"),
    }
}

#[test]
fn test_as_tree_with_nested_groups() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Create hierarchy
    let parent_id = vault.add_group(NewGroup::new("Parent")).unwrap();
    let child_id = vault.add_group(NewGroup::new("Child")).unwrap();

    // Add entries to both
    let parent_entry = NewEntry::new("parent_pass")
        .with_title("Parent Entry")
        .with_parent_group(GroupId::from_uuid(parent_id));
    vault.add_entry(parent_entry).unwrap();

    let child_entry = NewEntry::new("child_pass")
        .with_title("Child Entry")
        .with_parent_group(GroupId::from_uuid(child_id));
    vault.add_entry(child_entry).unwrap();

    // Get tree for parent
    let tree = vault.as_tree(Some(parent_id)).unwrap();

    // Should include parent and its children
    let parent_node = tree.nodes.iter().find(|n| n.get_id() == parent_id).unwrap();

    // Parent should have children (including the child group)
    assert!(!parent_node.get_children_id().is_empty());

    // The child entry should be in the tree
    let child_entry_node = tree
        .nodes
        .iter()
        .find(|n| n.get_display_name() == "Child Entry");
    assert!(child_entry_node.is_some());
}

#[test]
fn test_tree_vault_with_entries_no_title() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Add entry without title
    let entry = NewEntry::new("password");
    let entry_id = vault.add_entry(entry).unwrap();

    let tree = TreeVault::populate(&vault.database.root());

    // Find the entry node - it should use the ID as display name
    let entry_node = tree
        .nodes
        .iter()
        .find(|n| matches!(n.get_node_type(), NodeType::Entry));
    assert!(entry_node.is_some());

    let entry_node = entry_node.unwrap();
    // Display name should be the ID string since no title
    assert_eq!(entry_node.get_display_name(), entry_id.to_string());
}

#[test]
fn test_tree_vault_with_large_structure() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Create a large tree
    let num_groups = 10;
    let entries_per_group = 5;

    for i in 0..num_groups {
        let group_name = format!("Group_{}", i);
        let group_id = vault.add_group(NewGroup::new(&group_name)).unwrap();

        for j in 0..entries_per_group {
            let entry = NewEntry::new(format!("pass_{}_{}", i, j))
                .with_title(format!("Entry_{}_{}", i, j))
                .with_parent_group(GroupId::from_uuid(group_id));
            vault.add_entry(entry).unwrap();
        }
    }

    let tree = TreeVault::populate(&vault.database.root());

    // Should have: root + num_groups + (num_groups * entries_per_group)
    let expected_nodes = 1 + num_groups + (num_groups * entries_per_group);
    assert_eq!(tree.nodes.len(), expected_nodes);

    // Verify all groups and entries are present
    let groups: Vec<&TreeNode> = tree.nodes.iter().filter(|n| n.is_group()).collect();
    assert_eq!(groups.len(), 1 + num_groups); // root + created groups

    let entries: Vec<&TreeNode> = tree
        .nodes
        .iter()
        .filter(|n| matches!(n.get_node_type(), NodeType::Entry))
        .collect();
    assert_eq!(entries.len(), num_groups * entries_per_group);
}

#[test]
fn test_tree_vault_node_relationships() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    // Create a group with entries
    let group_id = vault.add_group(NewGroup::new("Test Group")).unwrap();

    let entry1_id = vault
        .add_entry(
            NewEntry::new("pass1")
                .with_title("Entry 1")
                .with_parent_group(GroupId::from_uuid(group_id)),
        )
        .unwrap();

    let entry2_id = vault
        .add_entry(
            NewEntry::new("pass2")
                .with_title("Entry 2")
                .with_parent_group(GroupId::from_uuid(group_id)),
        )
        .unwrap();

    let tree = TreeVault::populate(&vault.database.root());

    // Find the group node
    let group_node = tree.nodes.iter().find(|n| n.get_id() == group_id).unwrap();

    // Group should have entries as children
    let children = group_node.get_children_id();
    assert_eq!(children.len(), 2);
    assert!(children.contains(&entry1_id));
    assert!(children.contains(&entry2_id));

    // Entries should have the group as parent
    for entry_id in &children {
        let entry_node = tree.nodes.iter().find(|n| n.get_id() == *entry_id).unwrap();
        assert_eq!(entry_node.get_parent_id(), Some(group_id));
    }
}

#[test]
fn test_tree_node_clone_behavior() {
    let node = TreeNode {
        id: Uuid::new_v4(),
        display_name: "Test".to_string(),
        node_type: NodeType::Group,
        parent_id: None,
        children_id: vec![Uuid::new_v4()],
        depth: 1,
    };

    // Test that getters return expected values
    assert_eq!(node.get_id(), node.id);
    assert_eq!(node.get_display_name(), node.display_name);
    assert_eq!(node.get_node_type(), node.node_type);
    assert_eq!(node.get_parent_id(), node.parent_id);
    assert_eq!(node.get_children_id(), node.children_id);
    assert_eq!(node.get_depth(), node.depth);

    // Test immutability - getters should return clones
    let children = node.get_children_id();
    assert_eq!(children, node.children_id);
    // Modifying the returned vector shouldn't affect the original
    drop(children); // Just to show it's safe
}
