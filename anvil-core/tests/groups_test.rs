use anvil_core::{
    GroupId,
    vault::{
        DatabaseProcessingError,
        groups::{NewGroup, UpdateGroup},
    },
};
use uuid::Uuid;

use crate::common::{create_test_vault, create_vault_with_entries};

mod common;
#[test]
fn add_group_creates_root_group() {
    let (mut vault, _, _, _) = create_test_vault();

    let id = vault.add_group(NewGroup::new("Personal")).unwrap();

    let group = vault.get_group(id).unwrap();

    assert_eq!(group.name, "Personal");
}

#[test]
fn add_group_under_parent() {
    let (mut vault, _, _, _) = create_test_vault();

    let parent = vault.add_group(NewGroup::new("Parent")).unwrap();

    let child = vault
        .add_group(NewGroup::new("Child").with_parent_group(GroupId::from_uuid(parent)))
        .unwrap();

    let child_group = vault.get_group(child).unwrap();

    assert_eq!(child_group.name, "Child");
}
#[test]
fn add_group_under_parent_not_exist() {
    let (mut vault, _, _, _) = create_test_vault();

    let result = vault
        .add_group(NewGroup::new("Child").with_parent_group(GroupId::from_uuid(Uuid::new_v4())));

    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindGroup(_))
    ));
}

#[test]
fn add_group_sets_optional_fields() {
    let (mut vault, _, _, _) = create_test_vault();

    let id = vault
        .add_group(
            NewGroup::new("Docs")
                .with_notes("secret")
                .with_tags(vec!["work".into(), "linux".into()]),
        )
        .unwrap();

    let group = vault.get_group(id).unwrap();

    assert_eq!(group.notes.as_deref(), Some("secret"));
    assert_eq!(group.tags, vec!["work", "linux"]);
}

#[test]
fn list_groups_contains_created_groups() {
    let (mut vault, _, _, _) = create_test_vault();

    vault.add_group(NewGroup::new("A")).unwrap();
    vault.add_group(NewGroup::new("B")).unwrap();

    let groups = vault.list_groups();

    assert!(groups.iter().any(|g| g.name == "A"));
    assert!(groups.iter().any(|g| g.name == "B"));
}

#[test]
fn get_group_returns_group() {
    let (mut vault, _, _, _) = create_test_vault();

    let id = vault.add_group(NewGroup::new("Passwords")).unwrap();

    let group = vault.get_group(id).unwrap();

    assert_eq!(group.name, "Passwords");
}

#[test]
fn get_group_invalid_id_returns_error() {
    let (vault, _, _, _) = create_test_vault();

    let result = vault.get_group(Uuid::new_v4());

    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindGroup(_))
    ));
}
#[test]
fn update_group_updates_fields() {
    let (mut vault, _, _, _) = create_test_vault();

    let id = vault.add_group(NewGroup::new("Old")).unwrap();

    vault
        .update_group(
            id,
            UpdateGroup {
                name: None,
                notes: Some("updated".into()),
                tags: None,
            },
        )
        .unwrap();

    let group = vault.get_group(id).unwrap();

    assert_eq!(group.name, "Old");

    vault
        .update_group(
            id,
            UpdateGroup {
                name: Some("New".into()),
                notes: Some("updated".into()),
                tags: Some(vec!["one".into(), "two".into()]),
            },
        )
        .unwrap();

    let group = vault.get_group(id).unwrap();

    assert_eq!(group.name, "New");
    assert_eq!(group.notes.as_deref(), Some("updated"));
    assert_eq!(group.tags, vec!["one", "two"]);
}

#[test]
fn update_group_() {
    let (mut vault, _, _, _) = create_test_vault();

    let result = vault.update_group(
        Uuid::new_v4(),
        UpdateGroup {
            name: Some("New".into()),
            notes: Some("updated".into()),
            tags: Some(vec!["one".into(), "two".into()]),
        },
    );
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindGroup(_))
    ));
}

#[test]
fn update_group_partial_update_keeps_other_fields() {
    let (mut vault, _, _, _) = create_test_vault();

    let id = vault
        .add_group(
            NewGroup::new("Old")
                .with_notes("note")
                .with_tags(vec!["tag".into()]),
        )
        .unwrap();

    vault
        .update_group(
            id,
            UpdateGroup {
                name: Some("New".into()),
                notes: None,
                tags: None,
            },
        )
        .unwrap();

    let group = vault.get_group(id).unwrap();

    assert_eq!(group.name, "New");
    assert_eq!(group.notes.as_deref(), None);
    assert_eq!(group.tags, vec!["tag"]);
}

#[test]
fn delete_group_removes_group() {
    let (mut vault, _, _, _) = create_test_vault();

    let id = vault.add_group(NewGroup::new("Temp")).unwrap();

    vault.delete_group(id).unwrap();

    assert!(vault.get_group(id).is_err());
}

#[test]
fn delete_group_invalid_id_returns_error() {
    let (mut vault, _, _, _) = create_test_vault();

    let result = vault.delete_group(Uuid::new_v4());

    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindGroup(_))
    ));
}

#[test]
fn move_group_changes_parent() {
    let (mut vault, _, _, _) = create_test_vault();

    let parent1 = vault.add_group(NewGroup::new("One")).unwrap();
    let parent2 = vault.add_group(NewGroup::new("Two")).unwrap();

    let child = vault
        .add_group(NewGroup::new("Child").with_parent_group(GroupId::from_uuid(parent1)))
        .unwrap();

    vault.move_group(child, parent2).unwrap();

    // TODO verify child now belongs to parent2 if the keepass API exposes parent/group relationships.
}

#[test]
fn move_group_parent_not_exist() {
    let (mut vault, _, _, _) = create_test_vault();

    let parent1 = vault.add_group(NewGroup::new("One")).unwrap();
    //parent 2 not exist
    let parent2 = Uuid::new_v4();

    let child = vault
        .add_group(NewGroup::new("Child").with_parent_group(GroupId::from_uuid(parent1)))
        .unwrap();

    let result = vault.move_group(child, parent2);

    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToMoveGroup {
            src_id: _,
            dst_id: _,
            err: _
        })
    ));
}

#[test]
fn move_group_child_not_exist() {
    let (mut vault, _, _, _) = create_test_vault();

    let parent2 = vault.add_group(NewGroup::new("Two")).unwrap();

    let result = vault.move_group(Uuid::new_v4(), parent2);

    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindGroup(_))
    ));
}

#[test]
fn test_get_group_by_name_found() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Create a group with a specific name
    let group_name = "Test Group";
    let group_id = vault.add_group(NewGroup::new(group_name)).unwrap();

    // Search for the group by name
    let result = vault.get_group_by_name(group_name);

    assert!(result.is_ok());
    let group = result.unwrap();
    assert_eq!(group.name, group_name);
    assert_eq!(group.id(), GroupId::from_uuid(group_id));
}

#[test]
fn test_get_group_by_name_not_found() {
    let (vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Search for a group that doesn't exist
    let result = vault.get_group_by_name("Nonexistent Group");

    assert!(result.is_err());
    match result {
        Err(DatabaseProcessingError::FailedToFindGroupByName(name)) => {
            assert_eq!(name, "Nonexistent Group");
        }
        _ => panic!("Expected FailedToFindGroupByName error"),
    }
}

#[test]
fn test_get_group_by_name_case_sensitive() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let group_name = "CaseSensitive";
    vault.add_group(NewGroup::new(group_name)).unwrap();

    // Search with exact case (should find)
    let result = vault.get_group_by_name("CaseSensitive");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "CaseSensitive");

    // Search with different case (should not find)
    let result = vault.get_group_by_name("casesensitive");
    assert!(result.is_err());

    let result = vault.get_group_by_name("CASESENSITIVE");
    assert!(result.is_err());
}

#[test]
fn test_get_group_by_name_with_whitespace() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let group_name = "Group With Spaces";
    vault.add_group(NewGroup::new(group_name)).unwrap();

    // Search with exact whitespace
    let result = vault.get_group_by_name("Group With Spaces");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, group_name);

    // Search with different whitespace
    let result = vault.get_group_by_name("Group  With  Spaces");
    assert!(result.is_err());

    let result = vault.get_group_by_name("GroupWithSpaces");
    assert!(result.is_err());
}
