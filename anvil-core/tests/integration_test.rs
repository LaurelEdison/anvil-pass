mod common;

use anvil_core::{
    self, GroupId, fields,
    vault::{
        entries::{NewEntry, UpdateEntry},
        groups::NewGroup,
    },
};
use common::create_test_vault;

use crate::common::reload_vault;

#[test]
fn test_complete_workflow() {
    let (mut vault, _temp_dir, path, password) = create_test_vault();

    //Create groups
    let work_group = GroupId::from_uuid(vault.add_group(NewGroup::new("Work")).unwrap());
    assert!(vault.dirty);
    vault.save(&password).unwrap();
    assert!(!vault.dirty);

    let personal_group = GroupId::from_uuid(vault.add_group(NewGroup::new("Personal")).unwrap());
    assert!(vault.dirty);
    vault.save(&password).unwrap();
    assert!(!vault.dirty);

    // Add entries
    let work_entry = NewEntry::new("work123")
        .with_title("Work Email")
        .with_username("work@company.com")
        .with_parent_group(work_group);
    let work_id = vault.add_entry(work_entry).unwrap();
    assert!(vault.dirty);
    vault.save(&password).unwrap();
    assert!(!vault.dirty);

    let personal_entry = NewEntry::new("personal123")
        .with_title("Personal Email")
        .with_username("personal@gmail.com")
        .with_parent_group(personal_group);
    let personal_id = vault.add_entry(personal_entry).unwrap();
    assert!(vault.dirty);
    vault.save(&password).unwrap();
    assert!(!vault.dirty);

    // TODO Search

    // Update
    let update = UpdateEntry {
        username: Some("work@newcompany.com".to_string()),
        ..Default::default()
    };
    vault.update_entry(work_id, update).unwrap();
    assert!(vault.dirty);
    vault.save(&password).unwrap();
    assert!(!vault.dirty);

    // Move
    vault.move_entry(personal_id, work_group.uuid()).unwrap();
    assert!(vault.dirty);
    vault.save(&password).unwrap();
    assert!(!vault.dirty);

    // Verify final state
    let work_entries = vault.get_entries_by_group(work_group);
    assert_eq!(work_entries.len(), 2);

    let personal_entries = vault.get_entries_by_group(personal_group);
    assert_eq!(personal_entries.len(), 0);

    // Save
    vault.save(&password).unwrap();

    // Reload and verify
    let reloaded = reload_vault(path, &password);
    assert_eq!(reloaded.list_entries().len(), 2);

    // Verify specific entries
    let reloaded_work = reloaded.get_entry(work_id).unwrap();
    assert_eq!(
        reloaded_work.get(fields::USERNAME),
        Some("work@newcompany.com")
    );
    assert_eq!(
        reloaded_work.get(fields::USERNAME),
        Some("work@newcompany.com")
    );
}

#[test]
fn test_save_and_load_multiple_times() {
    let (mut vault, _temp_dir, path, password) = create_test_vault();

    // Add entries and save
    for i in 0..5 {
        let entry = NewEntry::new(format!("pass{}", i)).with_title(format!("Entry {}", i));
        vault.add_entry(entry).unwrap();
    }
    vault.save(&password).unwrap();

    // Load and add more
    let mut vault = reload_vault(path.clone(), &password);
    for i in 5..10 {
        let entry = NewEntry::new(format!("pass{}", i)).with_title(format!("Entry {}", i));
        vault.add_entry(entry).unwrap();
    }
    vault.save(&password).unwrap();

    // Final verify
    let final_vault = reload_vault(path, &password);
    assert_eq!(final_vault.list_entries().len(), 10);
}
