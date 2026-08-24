mod common;

use anvil_core::{
    self,
    vault::{
        DatabaseProcessingError,
        entries::{NewEntry, UpdateEntry},
        groups::NewGroup,
    },
};
use common::create_test_vault;
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

    assert_eq!(retrieved.title, Some("Complete Entry".to_string()));
    assert_eq!(retrieved.username, Some("complete_user".to_string()));
    assert_eq!(retrieved.password, Some("password123".to_string()));
    assert_eq!(retrieved.url, Some("https://complete.com".to_string()));
    assert_eq!(retrieved.notes, Some("Complete notes".to_string()));
    assert_eq!(retrieved.totp, Some("123456".to_string()));
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
        .with_parent_group(group);

    let id = vault.add_entry(entry).unwrap();
    let retrieved = vault.get_entry(id).unwrap();

    assert_eq!(retrieved.title, Some("Complete Entry".to_string()));
    assert_eq!(retrieved.username, Some("complete_user".to_string()));
    assert_eq!(retrieved.password, Some("password123".to_string()));
    assert_eq!(retrieved.url, Some("https://complete.com".to_string()));
    assert_eq!(retrieved.notes, Some("Complete notes".to_string()));
    assert_eq!(retrieved.totp, Some("123456".to_string()));
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
        .with_parent_group(Uuid::new_v4());

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

#[test]
fn test_update_entry_new() {
    let update = UpdateEntry::new();

    assert!(update.title.is_none());
    assert!(update.username.is_none());
    assert!(update.password.is_none());
    assert!(update.url.is_none());
    assert!(update.notes.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_with_password() {
    let password = "new_password123";
    let update = UpdateEntry::new().with_password(password);

    assert_eq!(update.password, Some(password.to_string()));
    // Other fields should still be None
    assert!(update.title.is_none());
    assert!(update.username.is_none());
    assert!(update.url.is_none());
    assert!(update.notes.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_with_title() {
    let title = "New Title";
    let update = UpdateEntry::new().with_title(title);

    assert_eq!(update.title, Some(title.to_string()));
    assert!(update.password.is_none());
    assert!(update.username.is_none());
    assert!(update.url.is_none());
    assert!(update.notes.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_with_username() {
    let username = "new_user";
    let update = UpdateEntry::new().with_username(username);

    assert_eq!(update.username, Some(username.to_string()));
    assert!(update.title.is_none());
    assert!(update.password.is_none());
    assert!(update.url.is_none());
    assert!(update.notes.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_with_url() {
    let url = "https://example.com";
    let update = UpdateEntry::new().with_url(url);

    assert_eq!(update.url, Some(url.to_string()));
    assert!(update.title.is_none());
    assert!(update.password.is_none());
    assert!(update.username.is_none());
    assert!(update.notes.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_with_notes() {
    let notes = "Some notes here";
    let update = UpdateEntry::new().with_notes(notes);

    assert_eq!(update.notes, Some(notes.to_string()));
    assert!(update.title.is_none());
    assert!(update.password.is_none());
    assert!(update.username.is_none());
    assert!(update.url.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_with_totp() {
    let totp = "123456";
    let update = UpdateEntry::new().with_totp(totp);

    assert_eq!(update.totp, Some(totp.to_string()));
    assert!(update.title.is_none());
    assert!(update.password.is_none());
    assert!(update.username.is_none());
    assert!(update.url.is_none());
    assert!(update.notes.is_none());
}

#[test]
fn test_update_entry_chaining_all_fields() {
    let update = UpdateEntry::new()
        .with_title("My Title")
        .with_username("my_username")
        .with_password("secret123")
        .with_url("https://example.com")
        .with_notes("Some notes")
        .with_totp("987654");

    assert_eq!(update.title, Some("My Title".to_string()));
    assert_eq!(update.username, Some("my_username".to_string()));
    assert_eq!(update.password, Some("secret123".to_string()));
    assert_eq!(update.url, Some("https://example.com".to_string()));
    assert_eq!(update.notes, Some("Some notes".to_string()));
    assert_eq!(update.totp, Some("987654".to_string()));
}

#[test]
fn test_update_entry_chaining_subset() {
    let update = UpdateEntry::new()
        .with_title("Updated Title")
        .with_password("new_password");

    assert_eq!(update.title, Some("Updated Title".to_string()));
    assert_eq!(update.password, Some("new_password".to_string()));
    assert!(update.username.is_none());
    assert!(update.url.is_none());
    assert!(update.notes.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_with_different_string_types() {
    // Test with String
    let password = String::from("string_password");
    let update = UpdateEntry::new().with_password(password);
    assert_eq!(update.password, Some("string_password".to_string()));

    // Test with &str
    let title = "str_title";
    let update = UpdateEntry::new().with_title(title);
    assert_eq!(update.title, Some("str_title".to_string()));

    // Test with &String
    let username = &String::from("ref_string_user");
    let update = UpdateEntry::new().with_username(username);
    assert_eq!(update.username, Some("ref_string_user".to_string()));
}

#[test]
fn test_update_entry_overwrite_fields() {
    let update = UpdateEntry::new()
        .with_password("first_password")
        .with_password("second_password");

    // The last call should overwrite the previous
    assert_eq!(update.password, Some("second_password".to_string()));

    let update = UpdateEntry::new()
        .with_title("First Title")
        .with_title("Second Title")
        .with_username("user");

    assert_eq!(update.title, Some("Second Title".to_string()));
    assert_eq!(update.username, Some("user".to_string()));
}

#[test]
fn test_update_entry_empty_strings() {
    let update = UpdateEntry::new()
        .with_title("")
        .with_password("")
        .with_username("");

    assert_eq!(update.title, Some("".to_string()));
    assert_eq!(update.password, Some("".to_string()));
    assert_eq!(update.username, Some("".to_string()));
    assert!(update.url.is_none());
    assert!(update.notes.is_none());
    assert!(update.totp.is_none());
}

#[test]
fn test_update_entry_multiple_calls_same_field_ordering() {
    let update = UpdateEntry::new()
        .with_notes("note1")
        .with_url("url1")
        .with_notes("note2")
        .with_url("url2");

    assert_eq!(update.notes, Some("note2".to_string()));
    assert_eq!(update.url, Some("url2".to_string()));
}
