use std::fs;

use anvil_core::{
    fields,
    vault::{
        DatabaseProcessingError,
        database::{create_vault, open_vault},
        entries::NewEntry,
    },
};
use tempfile::tempdir;

// tests/backup_tests.rs
mod common;

#[test]
fn test_create_backup() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();
    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup_path.clone()).unwrap();

    assert!(backup_path.exists());

    let backup_vault = open_vault(password, backup_path).unwrap();
    assert_eq!(backup_vault.list_entries().len(), 1);
    let retrieved = backup_vault.get_entry_by_title("Test Entry").unwrap();
    assert_eq!(retrieved.get(fields::PASSWORD), Some("password123"));
}

#[test]
fn test_create_backup_fails_when_dirty() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path).unwrap();

    // Add entry but don't save (vault is dirty)
    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    assert!(vault.dirty);

    let result = vault.create_backup(backup_path);
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::DirtyDatabase)
    ));
}

#[test]
fn test_create_backup_after_save() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();
    assert!(!vault.dirty);

    let result = vault.create_backup(backup_path);
    assert!(result.is_ok());
}

#[test]
fn test_create_backup_overwrites_existing() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry1 = NewEntry::new("password1").with_title("Entry 1");
    vault.add_entry(entry1).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup_path.clone()).unwrap();

    let backup1 = open_vault(password, backup_path.clone()).unwrap();
    assert_eq!(backup1.list_entries().len(), 1);

    let entry2 = NewEntry::new("password2").with_title("Entry 2");
    vault.add_entry(entry2).unwrap();
    vault.save(password).unwrap();

    // Create backup again (overwrite)
    vault.create_backup(backup_path.clone()).unwrap();

    let backup2 = open_vault(password, backup_path).unwrap();
    assert_eq!(backup2.list_entries().len(), 2);
}

#[test]
fn test_create_backup_in_nonexistent_directory() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("nonexistent").join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    let result = vault.create_backup(backup_path);
    assert!(matches!(result, Err(DatabaseProcessingError::Io(_))));
}

#[test]
fn test_create_backup_when_vault_file_missing() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("nonexistent_vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    // Create vault but don't save (file doesn't exist on disk)
    let vault = create_vault(password, vault_path.clone()).unwrap();

    std::fs::remove_file(&vault_path).unwrap();
    assert!(!vault_path.exists());

    let result = vault.create_backup(backup_path);
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::VaultFileMissing)
    ));
}

#[test]
fn test_backup_preserves_all_data() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123")
        .with_title("Full Entry")
        .with_username("full_user")
        .with_url("https://full.com")
        .with_notes("Full notes")
        .with_totp("123456");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup_path.clone()).unwrap();

    let backup = open_vault(password, backup_path).unwrap();
    let retrieved = backup.get_entry_by_title("Full Entry").unwrap();
    assert_eq!(retrieved.get(fields::TITLE), Some("Full Entry"));
    assert_eq!(retrieved.get(fields::USERNAME), Some("full_user"));
    assert_eq!(retrieved.get(fields::PASSWORD), Some("password123"));
    assert_eq!(retrieved.get(fields::URL), Some("https://full.com"));
    assert_eq!(retrieved.get(fields::NOTES), Some("Full notes"));
    assert_eq!(retrieved.get(fields::OTP), Some("123456"));
}

#[test]
fn test_multiple_backups() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup1_path = temp_dir.path().join("backup1.kdbx");
    let backup2_path = temp_dir.path().join("backup2.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    // Save initial state
    let entry1 = NewEntry::new("password1").with_title("Entry 1");
    vault.add_entry(entry1).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup1_path.clone()).unwrap();

    let entry2 = NewEntry::new("password2").with_title("Entry 2");
    vault.add_entry(entry2).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup2_path.clone()).unwrap();

    // Verify both backups
    let backup1 = open_vault(password, backup1_path).unwrap();
    assert_eq!(backup1.list_entries().len(), 1);

    let backup2 = open_vault(password, backup2_path).unwrap();
    assert_eq!(backup2.list_entries().len(), 2);
}

#[test]
fn test_backup_read_only_vault() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    // Make vault read-only
    #[cfg(unix)]
    {
        use std::{fs, os::unix::fs::PermissionsExt};
        let perm = fs::Permissions::from_mode(0o444);
        fs::set_permissions(&vault_path, perm).unwrap();
    }

    // Try to create backup (should still work - we're reading the file)
    let result = vault.create_backup(backup_path);
    assert!(result.is_ok());
}

#[test]
fn test_backup_to_existing_file_with_different_content() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    fs::write(&backup_path, b"dummy backup content").unwrap();

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup_path.clone()).unwrap();

    let backup = open_vault(password, backup_path).unwrap();
    assert_eq!(backup.list_entries().len(), 1);
}

#[test]
fn test_backup_with_groups() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let group1_id = {
        let mut root = vault.database.root_mut();
        let mut group = root.add_group();
        group.name = String::from("Group 1");
        group.id()
    };

    let group2_id = {
        let mut root = vault.database.root_mut();
        let mut group = root.add_group();
        group.name = String::from("Group 2");
        group.id()
    };

    let entry1 = NewEntry::new("pass1")
        .with_title("Entry in Group 1")
        .with_parent_group(group1_id);
    vault.add_entry(entry1).unwrap();

    let entry2 = NewEntry::new("pass2")
        .with_title("Entry in Group 2")
        .with_parent_group(group2_id);
    vault.add_entry(entry2).unwrap();

    vault.save(password).unwrap();

    vault.create_backup(backup_path.clone()).unwrap();

    let backup = open_vault(password, backup_path).unwrap();

    let group1_entries = backup.get_entries_by_group(group1_id);
    assert_eq!(group1_entries.len(), 1);
    assert_eq!(
        group1_entries[0].get(fields::TITLE),
        Some("Entry in Group 1")
    );

    let group2_entries = backup.get_entries_by_group(group2_id);
    assert_eq!(group2_entries.len(), 1);
    assert_eq!(
        group2_entries[0].get(fields::TITLE),
        Some("Entry in Group 2")
    );
}

#[test]
fn test_backup_with_large_entry() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let large_content = "x".repeat(1024 * 1024); // 1MB
    let entry = NewEntry::new("password123")
        .with_title("Large Entry")
        .with_notes(&large_content);
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup_path.clone()).unwrap();

    let backup = open_vault(password, backup_path).unwrap();
    let retrieved = backup.get_entry_by_title("Large Entry").unwrap();
    assert_eq!(retrieved.get(fields::NOTES), Some(large_content.as_str()));
}

#[test]
fn test_backup_preserves_dirty_state() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();
    assert!(!vault.dirty);

    vault.create_backup(backup_path).unwrap();

    assert!(!vault.dirty);

    // Add entry without saving (becomes dirty)
    let entry2 = NewEntry::new("password456").with_title("Entry 2");
    vault.add_entry(entry2).unwrap();
    assert!(vault.dirty);

    // Should not be able to create backup while dirty
    let result = vault.create_backup(temp_dir.path().join("backup2.kdbx"));
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::DirtyDatabase)
    ));
}

#[test]
fn test_backup_binary_identical() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    vault.create_backup(backup_path.clone()).unwrap();

    let vault_content = fs::read(&vault_path).unwrap();
    let backup_content = fs::read(&backup_path).unwrap();
    assert_eq!(vault_content, backup_content);
}

#[test]
fn test_backup_after_moves_and_deletes() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let group_id = {
        let mut root = vault.database.root_mut();
        let mut group = root.add_group();
        group.name = String::from("Test Group");
        group.id()
    };

    let entry1 = NewEntry::new("pass1").with_title("Entry 1");
    let id1 = vault.add_entry(entry1).unwrap();

    let entry2 = NewEntry::new("pass2")
        .with_title("Entry 2")
        .with_parent_group(group_id);
    let id2 = vault.add_entry(entry2).unwrap();

    vault.save(password).unwrap();

    // Make changes: move entry1 to group, delete entry2
    vault.move_entry(id1, group_id.uuid()).unwrap();
    vault.delete_entry(id2).unwrap();
    vault.save(password).unwrap();

    // Create backup
    vault.create_backup(backup_path.clone()).unwrap();

    // Verify backup reflects final state
    let backup = open_vault(password, backup_path).unwrap();

    // Entry 1 should be in group
    let group_entries = backup.get_entries_by_group(group_id);
    assert_eq!(group_entries.len(), 1);
    assert_eq!(group_entries[0].id().uuid(), id1);

    // Entry 2 should be gone
    assert!(backup.get_entry(id2).is_err());
}

#[test]
fn test_backup_while_dirty_returns_correct_error() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path).unwrap();

    // Make vault dirty
    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    assert!(vault.dirty);

    let result = vault.create_backup(backup_path);
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::DirtyDatabase)
    ));
}

#[test]
fn test_backup_io_error() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("vault.kdbx");
    let backup_path = temp_dir.path().join("nonexistent_dir").join("backup.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, vault_path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    let result = vault.create_backup(backup_path);
    assert!(matches!(result, Err(DatabaseProcessingError::Io(_))));
}
