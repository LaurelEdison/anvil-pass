mod common;

use anvil_core::{
    self, GroupId,
    vault::{
        DatabaseProcessingError,
        entries::{NewEntry, UpdateEntry},
        groups::NewGroup,
        traversal::NodeType::Group,
    },
};
use common::create_test_vault;
use keepass::db::fields;
use uuid::Uuid;

use crate::common::{create_test_group, create_vault_with_entries};

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

#[test]
fn test_search_entry_exact_found_in_specific_group() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Create a group and add an entry with a specific title
    let group_id = vault.add_group(NewGroup::new("Test Group")).unwrap();
    let entry = NewEntry::new("password123")
        .with_title("Unique Title")
        .with_parent_group(GroupId::from_uuid(group_id));
    vault.add_entry(entry).unwrap();

    // Search for the entry in the specific group
    let result = vault.search_entry_exact("Unique Title", Some(group_id));

    assert!(result.is_some());
    assert_eq!(result.unwrap().get_title().unwrap(), "Unique Title");
}

#[test]
fn test_search_entry_exact_not_found_in_specific_group() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Create a group with an entry
    let group_id = vault.add_group(NewGroup::new("Test Group")).unwrap();
    let entry = NewEntry::new("password123")
        .with_title("Different Title")
        .with_parent_group(GroupId::from_uuid(group_id));
    vault.add_entry(entry).unwrap();

    // Search for a title that doesn't exist in this group
    let result = vault.search_entry_exact("Nonexistent Title", Some(group_id));

    assert!(result.is_none());
}

#[test]
fn test_search_entry_exact_found_in_all_groups() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Add an entry to the root group
    let entry = NewEntry::new("root_password").with_title("Root Entry");
    vault.add_entry(entry).unwrap();

    // Search without specifying a group (search all)
    let result = vault.search_entry_exact("Root Entry", None);

    assert!(result.is_some());
    assert_eq!(result.unwrap().get_title().unwrap(), "Root Entry");
}

#[test]
fn test_search_entry_exact_not_found_in_all_groups() {
    let (vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Search for a title that doesn't exist anywhere
    let result = vault.search_entry_exact("Nonexistent Entry", None);

    assert!(result.is_none());
}

#[test]
fn test_search_entry_exact_case_sensitive() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Add an entry with a title
    let entry = NewEntry::new("password123").with_title("CaseSensitive");
    vault.add_entry(entry).unwrap();

    // Search with exact case (should find)
    let result = vault.search_entry_exact("CaseSensitive", None);
    assert!(result.is_some());

    // Search with different case (should not find)
    let result = vault.search_entry_exact("casesensitive", None);
    assert!(result.is_none());

    let result = vault.search_entry_exact("CASESENSITIVE", None);
    assert!(result.is_none());
}

#[test]
fn test_search_entry_exact_with_whitespace() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let title = "Entry With Spaces";
    let entry = NewEntry::new("password123").with_title(title);
    vault.add_entry(entry).unwrap();

    // Search with exact whitespace
    let result = vault.search_entry_exact("Entry With Spaces", None);
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_title().unwrap(), title);

    // Search with different whitespace
    let result = vault.search_entry_exact("Entry  With  Spaces", None);
    assert!(result.is_none());
}

#[test]
fn test_search_entry_exact_with_special_characters() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let title = "Entry!@#$%^&*()";
    let entry = NewEntry::new("password123").with_title(title);
    vault.add_entry(entry).unwrap();

    let result = vault.search_entry_exact("Entry!@#$%^&*()", None);
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_title().unwrap(), title);
}

#[test]
fn test_search_entry_exact_with_unicode() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let title = "你好世界 🌍";
    let entry = NewEntry::new("password123").with_title(title);
    vault.add_entry(entry).unwrap();

    let result = vault.search_entry_exact("你好世界 🌍", None);
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_title().unwrap(), title);
}

#[test]
fn test_search_entry_exact_in_specific_group_with_multiple_entries() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let group_id = vault.add_group(NewGroup::new("Test Group")).unwrap();

    // Add multiple entries to the same group
    let entry1 = NewEntry::new("password1")
        .with_title("Title A")
        .with_parent_group(GroupId::from_uuid(group_id));
    vault.add_entry(entry1).unwrap();

    let entry2 = NewEntry::new("password2")
        .with_title("Title B")
        .with_parent_group(GroupId::from_uuid(group_id));
    vault.add_entry(entry2).unwrap();

    // Search for each entry
    let result = vault.search_entry_exact("Title A", Some(group_id));
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_password().unwrap(), "password1");

    let result = vault.search_entry_exact("Title B", Some(group_id));
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_password().unwrap(), "password2");

    // Search for non-existent title
    let result = vault.search_entry_exact("Title C", Some(group_id));
    assert!(result.is_none());
}

#[test]
fn test_search_entry_exact_with_nonexistent_group() {
    let (vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let fake_group_id = Uuid::new_v4();

    // This should panic because group doesn't exist
    let result = std::panic::catch_unwind(|| {
        vault.search_entry_exact("Any Title", Some(fake_group_id));
    });

    assert!(result.is_err()); // Should panic when unwrapping the group
}

#[test]
fn test_search_entry_exact_empty_title() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let entry = NewEntry::new("password123").with_title("");
    vault.add_entry(entry).unwrap();

    let result = vault.search_entry_exact("", None);
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_title().unwrap(), "");
}

#[test]
fn test_search_entry_exact_entry_without_title() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Create an entry without a title
    let entry = NewEntry::new("password123");
    vault.add_entry(entry).unwrap();

    // Search for something that shouldn't match
    let result = vault.search_entry_exact("Some Title", None);
    assert!(result.is_none());
}

#[test]
fn test_search_entry_exact_in_group_with_mixed_entries() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let group_id = vault.add_group(NewGroup::new("Test Group")).unwrap();

    // Add entries with various titles
    let titles = vec!["Alpha", "Beta", "Gamma", "Delta"];
    for title in titles.clone() {
        let entry = NewEntry::new(format!("password_{}", title))
            .with_title(title)
            .with_parent_group(GroupId::from_uuid(group_id));
        vault.add_entry(entry).unwrap();
    }

    // Search for each title
    for title in titles {
        let result = vault.search_entry_exact(title, Some(group_id));
        assert!(result.is_some());
        assert_eq!(result.unwrap().get_title().unwrap(), title);
    }

    // Search for a title not in the group
    let result = vault.search_entry_exact("Omega", Some(group_id));
    assert!(result.is_none());
}

#[test]
fn test_search_entry_exact_ignores_entries_in_other_groups() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    let group1_id = vault.add_group(NewGroup::new("Group 1")).unwrap();
    let group2_id = vault.add_group(NewGroup::new("Group 2")).unwrap();

    // Add same title to both groups
    let entry1 = NewEntry::new("password1")
        .with_title("Shared Title")
        .with_parent_group(GroupId::from_uuid(group1_id));
    vault.add_entry(entry1).unwrap();

    let entry2 = NewEntry::new("password2")
        .with_title("Shared Title")
        .with_parent_group(GroupId::from_uuid(group2_id));
    vault.add_entry(entry2).unwrap();

    // Search in group 1 - should only find the entry in group 1
    let result = vault.search_entry_exact("Shared Title", Some(group1_id));
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_password().unwrap(), "password1");

    // Search in group 2 - should only find the entry in group 2
    let result = vault.search_entry_exact("Shared Title", Some(group2_id));
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_password().unwrap(), "password2");
}
