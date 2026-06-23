mod common;

use anvil_core::{
    self, GroupId,
    vault::{
        DatabaseProcessingError,
        entries::{NewEntry, UpdateEntry},
        groups::NewGroup,
    },
};
use common::create_test_vault;
use keepass::db::fields;
use uuid::Uuid;

use crate::common::create_test_group;

#[test]
fn test_new_entry_builder() {
    let entry = NewEntry::new("password123")
        .with_title("My Entry")
        .with_username("user")
        .with_url("https://example.com")
        .with_notes("Some notes")
        .with_totp("123456");

    assert_eq!(entry.password, "password123");
    assert_eq!(entry.title, Some("My Entry".to_string()));
    assert_eq!(entry.username, Some("user".to_string()));
    assert_eq!(entry.url, Some("https://example.com".to_string()));
    assert_eq!(entry.notes, Some("Some notes".to_string()));
    assert_eq!(entry.totp, Some("123456".to_string()));
}

#[test]
fn test_update_entry_default() {
    let update = UpdateEntry {
        title: Some("New Title".to_string()),
        ..Default::default()
    };

    assert_eq!(update.title, Some("New Title".to_string()));
    assert_eq!(update.username, None);
    assert_eq!(update.password, None);
}

#[test]
fn test_add_entry_with_all_fields() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    let entry = NewEntry::new("password123")
        .with_title("Complete Entry")
        .with_username("complete_user")
        .with_url("https://complete.com")
        .with_notes("Complete notes")
        .with_totp("123456");

    let id = vault.add_entry(entry).unwrap();
    let retrieved = vault.get_entry(id).unwrap();

    assert_eq!(retrieved.get(fields::TITLE), Some("Complete Entry"));
    assert_eq!(retrieved.get(fields::USERNAME), Some("complete_user"));
    assert_eq!(retrieved.get(fields::PASSWORD), Some("password123"));
    assert_eq!(retrieved.get(fields::URL), Some("https://complete.com"));
    assert_eq!(retrieved.get(fields::NOTES), Some("Complete notes"));
    assert_eq!(retrieved.get(fields::OTP), Some("123456"));
}

#[test]
fn test_add_entry_with_all_fields_with_group() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    let group = vault.add_group(NewGroup::new("group")).unwrap();
    let entry = NewEntry::new("password123")
        .with_title("Complete Entry")
        .with_username("complete_user")
        .with_url("https://complete.com")
        .with_notes("Complete notes")
        .with_totp("123456")
        .with_parent_group(GroupId::from_uuid(group));

    let id = vault.add_entry(entry).unwrap();
    let retrieved = vault.get_entry(id).unwrap();

    assert_eq!(retrieved.get(fields::TITLE), Some("Complete Entry"));
    assert_eq!(retrieved.get(fields::USERNAME), Some("complete_user"));
    assert_eq!(retrieved.get(fields::PASSWORD), Some("password123"));
    assert_eq!(retrieved.get(fields::URL), Some("https://complete.com"));
    assert_eq!(retrieved.get(fields::NOTES), Some("Complete notes"));
    assert_eq!(retrieved.get(fields::OTP), Some("123456"));
}

#[test]
fn test_add_entry_with_all_fields_with_err() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    let entry = NewEntry::new("password123")
        .with_title("Complete Entry")
        .with_username("complete_user")
        .with_url("https://complete.com")
        .with_notes("Complete notes")
        .with_totp("123456")
        .with_parent_group(GroupId::from_uuid(Uuid::new_v4()));

    let result = vault.add_entry(entry);

    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindGroup(_))
    ));
}

#[test]
fn test_update_nonexistent_entry() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    let update = UpdateEntry {
        title: Some("New Title".to_string()),
        ..Default::default()
    };

    let result = vault.update_entry(Uuid::new_v4(), update);
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindEntry(_))
    ));
}

#[test]
fn test_delete_nonexistent_entry() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    let result = vault.delete_entry(Uuid::new_v4());
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindEntry(_))
    ));
}

#[test]
fn test_move_nonexistent_entry() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    let group_id = create_test_group(&mut vault, "Test Group");
    let result = vault.move_entry(Uuid::new_v4(), group_id.uuid());
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToFindEntry(_))
    ));
}

#[test]
fn test_move_entry_to_nonexistent_group() {
    let (mut vault, _temp_dir, _path, _password) = create_test_vault();

    let entry = NewEntry::new("password123").with_title("Test Entry");
    let id = vault.add_entry(entry).unwrap();

    let result = vault.move_entry(id, Uuid::new_v4());
    assert!(matches!(
        result,
        Err(DatabaseProcessingError::FailedToMoveEntry { .. })
    ));
}
