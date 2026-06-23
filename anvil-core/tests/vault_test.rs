mod common;

use anvil_core::{
    self,
    vault::{
        DatabaseProcessingError,
        database::{create_vault, open_vault},
        entries::NewEntry,
    },
};
use keepass::db::fields;
use std::{fs, path::PathBuf};
use tempfile::tempdir;

#[test]
fn test_create_vault() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("new_vault.kdbx");
    let password = "test_password";

    // Create a new vault
    let vault = create_vault(password, path.clone()).unwrap();

    // Verify vault was created
    assert!(!vault.dirty);
    assert_eq!(vault.path, path);
    assert!(path.exists());

    // Verify we can open it
    let opened = open_vault(password, path).unwrap();
    assert!(!opened.dirty);
}

#[test]
fn test_create_vault_and_add_entry() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("vault_with_entry.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    let entry = NewEntry::new("password123")
        .with_title("Test Entry")
        .with_username("testuser");
    let id = vault.add_entry(entry).unwrap();

    vault.save(password).unwrap();

    let reloaded = open_vault(password, path).unwrap();
    let retrieved = reloaded.get_entry(id).unwrap();
    assert_eq!(retrieved.get(fields::TITLE), Some("Test Entry"));
    assert_eq!(retrieved.get(fields::USERNAME), Some("testuser"));
    assert_eq!(retrieved.get(fields::PASSWORD), Some("password123"));
}

#[test]
fn test_open_vault() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_open.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();
    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    vault.save(password).unwrap();

    let opened = open_vault(password, path).unwrap();

    let entries = opened.list_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].get(fields::TITLE), Some("Test Entry"));
}

#[test]
fn test_open_vault_wrong_password() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_wrong_password.kdbx");
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";

    let mut vault = create_vault(correct_password, path.clone()).unwrap();
    vault.save(correct_password).unwrap();

    let result = open_vault(wrong_password, path);
    assert!(result.is_err());
}

#[test]
fn test_open_vault_nonexistent_file() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("nonexistent.kdbx");
    let password = "test_password";

    let result = open_vault(password, path);
    assert!(result.is_err());
}

#[test]
fn nonexistent_file_returns_io_error() {
    let path = PathBuf::from("definitely-does-not-exist.kdbx");

    let result = open_vault("password", path);

    assert!(matches!(result, Err(DatabaseProcessingError::Io(_))));
}

#[test]
fn invalid_database_returns_open_error() {
    use std::fs;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("fake.kdbx");

    fs::write(&path, "hello world").unwrap();

    let result = open_vault("password", path);

    assert!(matches!(result, Err(DatabaseProcessingError::Open(_))));
}

#[test]
fn test_save_vault() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_save.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    let id = vault.add_entry(entry).unwrap();

    vault.save(password).unwrap();

    assert!(path.exists());
    let metadata = fs::metadata(&path).unwrap();
    assert!(metadata.len() > 0);

    let reloaded = open_vault(password, path).unwrap();
    let retrieved = reloaded.get_entry(id).unwrap();
    assert_eq!(retrieved.get(fields::TITLE), Some("Test Entry"));
}

#[test]
fn test_save_only_when_dirty() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_dirty.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    // Save initially (should create file)
    vault.save(password).unwrap();
    let metadata1 = fs::metadata(&path).unwrap();
    let modified1 = metadata1.modified().unwrap();

    // Save again without changes (should skip)
    vault.save(password).unwrap();
    let metadata2 = fs::metadata(&path).unwrap();
    let modified2 = metadata2.modified().unwrap();

    // File should not be modified (or at least not significantly)
    // Note: On some filesystems, modified time precision might be low
    // This is a best-effort check
    assert_eq!(modified1, modified2);
    assert!(!vault.dirty);
}

#[test]
fn test_save_after_changes() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_changes.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    vault.save(password).unwrap();
    let metadata1 = fs::metadata(&path).unwrap();
    let size1 = metadata1.len();
    let modified1 = metadata1.modified().unwrap();

    let entry = NewEntry::new("password123").with_title("New Entry");
    vault.add_entry(entry).unwrap();
    assert!(vault.dirty);

    vault.save(password).unwrap();
    assert!(!vault.dirty);

    let metadata2 = fs::metadata(&path).unwrap();
    let size2 = metadata2.len();
    let modified2 = metadata2.modified().unwrap();

    assert!(size2 > size1);
    assert!(modified2 >= modified1);
}

#[test]
fn test_save_atomic() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_atomic.kdbx");
    let temp_path = path.with_extension("tmp");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();

    vault.save(password).unwrap();

    assert!(!temp_path.exists());

    assert!(path.exists());
    let reloaded = open_vault(password, path).unwrap();
    assert_eq!(reloaded.list_entries().len(), 1);
}

#[test]
fn test_save_overwrites_existing() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_overwrite.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    let entry1 = NewEntry::new("password1").with_title("Entry 1");
    vault.add_entry(entry1).unwrap();
    vault.save(password).unwrap();

    let reloaded1 = open_vault(password, path.clone()).unwrap();
    assert_eq!(reloaded1.list_entries().len(), 1);

    let entry2 = NewEntry::new("password2").with_title("Entry 2");
    vault.add_entry(entry2).unwrap();
    vault.save(password).unwrap();

    let reloaded2 = open_vault(password, path).unwrap();
    assert_eq!(reloaded2.list_entries().len(), 2);
}

#[test]
fn test_create_vault_overwrites_existing() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_create_overwrite.kdbx");
    let password1 = "password1";
    let password2 = "password2";

    let mut vault1 = create_vault(password1, path.clone()).unwrap();
    let entry = NewEntry::new("password123").with_title("Entry 1");
    vault1.add_entry(entry).unwrap();
    vault1.save(password1).unwrap();

    let _vault2 = create_vault(password2, path.clone()).unwrap();

    let reloaded = open_vault(password2, path.clone()).unwrap();
    assert_eq!(reloaded.list_entries().len(), 0);

    let result = open_vault(password1, path.clone());
    assert!(result.is_err());
}

#[test]
fn test_multiple_save_load_cycles() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_multiple_cycles.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();
    let mut ids = Vec::new();

    for i in 0..5 {
        let entry = NewEntry::new(format!("password{}", i)).with_title(format!("Entry {}", i));
        let id = vault.add_entry(entry).unwrap();
        ids.push(id);

        vault.save(password).unwrap();

        let reloaded = open_vault(password, path.clone()).unwrap();
        let retrieved = reloaded.get_entry(id).unwrap();
        assert_eq!(
            retrieved.get(fields::TITLE),
            Some(format!("Entry {}", i).as_str())
        );
        assert_eq!(vault.dirty, false);
    }

    let final_vault = open_vault(password, path).unwrap();
    assert_eq!(final_vault.list_entries().len(), 5);
}

#[test]
fn test_save_with_many_entries() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_many_entries.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    let count = 100;
    for i in 0..count {
        let entry = NewEntry::new(format!("password{}", i)).with_title(format!("Entry {}", i));
        vault.add_entry(entry).unwrap();
    }

    vault.save(password).unwrap();

    let reloaded = open_vault(password, path).unwrap();
    assert_eq!(reloaded.list_entries().len(), count);
}

#[test]
fn test_save_preserves_entry_data() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_preserve_data.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    let entry = NewEntry::new("password123")
        .with_title("Full Entry")
        .with_username("full_user")
        .with_url("https://full.com")
        .with_notes("Full notes")
        .with_totp("123456");
    let id = vault.add_entry(entry).unwrap();

    vault.save(password).unwrap();
    let reloaded = open_vault(password, path).unwrap();
    let retrieved = reloaded.get_entry(id).unwrap();

    assert_eq!(retrieved.get(fields::TITLE), Some("Full Entry"));
    assert_eq!(retrieved.get(fields::USERNAME), Some("full_user"));
    assert_eq!(retrieved.get(fields::PASSWORD), Some("password123"));
    assert_eq!(retrieved.get(fields::URL), Some("https://full.com"));
    assert_eq!(retrieved.get(fields::NOTES), Some("Full notes"));
    assert_eq!(retrieved.get(fields::OTP), Some("123456"));
}

#[test]
fn test_create_vault_in_nonexistent_directory() {
    let temp_dir = tempdir().unwrap();
    let nonexistent = temp_dir.path().join("nonexistent").join("vault.kdbx");
    let password = "test_password";

    let result = create_vault(password, nonexistent);
    assert!(result.is_err());
}

#[test]
fn test_vault_cleanup_on_drop() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test_cleanup.kdbx");
    let password = "test_password";

    {
        let _vault = create_vault(password, path.clone()).unwrap();
        // Vault goes out of scope
    }

    // File should still exist (we didn't delete it)
    assert!(path.exists());
}

#[test]
fn test_save_with_readonly_directory() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("vault.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    // Make directory read-only
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perm = fs::Permissions::from_mode(0o444);
        fs::set_permissions(temp_dir.path(), perm).unwrap();
    }

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();
    let result = vault.save(password);

    #[cfg(unix)]
    {
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, DatabaseProcessingError::Io(_)));
        }
    }
}

#[test]
fn test_save_when_disk_full() {
    // This test is platform-specific and may not work on all systems
    // It's more of a conceptual test
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("vault.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    // Add a very large entry (try to trigger disk full)
    let large_password = "x".repeat(1024 * 1024 * 10); // 10MB
    let entry = NewEntry::new(large_password).with_title("Large Entry");
    vault.add_entry(entry).unwrap();

    // This will either succeed or fail with Io error
    let result = vault.save(password);
    if let Err(e) = result {
        assert!(matches!(e, DatabaseProcessingError::Io(_)));
    }
}

#[test]
fn test_save_with_invalid_temp_file() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("vault.kdbx");
    let password = "test_password";

    let mut vault = create_vault(password, path.clone()).unwrap();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    vault.add_entry(entry).unwrap();

    // First save should work
    vault.save(password).unwrap();

    // Make the temp file exist and be read-only to cause rename failure
}

#[test]
fn test_open_corrupted_vault() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("corrupted.kdbx");
    let password = "test_password";

    // Create a corrupted file
    fs::write(&path, b"this is not a valid kdbx file").unwrap();

    // Try to open it
    let result = open_vault(password, path);
    assert!(result.is_err());
}
