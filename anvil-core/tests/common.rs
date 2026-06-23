#![allow(dead_code)]

use anvil_core::{
    self,
    vault::{Vault, entries::NewEntry},
};
use keepass::db::{Database, GroupId};
use std::path::PathBuf;
use tempfile::tempdir;
use uuid::Uuid;

pub fn create_test_vault() -> (Vault, tempfile::TempDir, PathBuf, String) {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path().join("test.kdbx");
    let password = "test_password".to_string();

    let vault = anvil_core::vault::Vault {
        database: Database::new(),
        path: path.clone(),
        dirty: false,
    };

    (vault, temp_dir, path, password)
}

pub fn create_vault_with_entries() -> (Vault, tempfile::TempDir, PathBuf, String, Vec<Uuid>) {
    let (mut vault, temp_dir, path, password) = create_test_vault();
    let mut ids = Vec::new();

    // Add some test entries
    for i in 0..5 {
        let entry = NewEntry::new(format!("password{}", i))
            .with_title(format!("Test Entry {}", i))
            .with_username(format!("user{}", i));
        let id = vault.add_entry(entry).unwrap();
        ids.push(id);
    }

    (vault, temp_dir, path, password, ids)
}

pub fn create_test_group(vault: &mut Vault, name: &str) -> GroupId {
    let mut root = vault.database.root_mut();
    let mut group = root.add_group();
    group.name = String::from(name);
    group.id()
}

pub fn reload_vault(path: PathBuf, password: &str) -> Vault {
    let mut file = std::fs::File::open(path.clone()).unwrap();
    let key = keepass::DatabaseKey::new().with_password(password);
    let db = Database::open(&mut file, key).unwrap();

    Vault {
        database: db,
        path: path,
        dirty: false,
    }
}

/// Assert that an entry exists with the given title
#[macro_export]
macro_rules! assert_entry_exists {
    ($vault:expr, $title:expr) => {
        let result = $vault.get_entry_by_title($title);
        assert!(result.is_ok(), "Entry with title '{}' should exist", $title);
    };
}

/// Assert that an entry does not exist with the given title
#[macro_export]
macro_rules! assert_entry_not_exists {
    ($vault:expr, $title:expr) => {
        let result = $vault.get_entry_by_title($title);
        assert!(
            result.is_err(),
            "Entry with title '{}' should not exist",
            $title
        );
    };
}
