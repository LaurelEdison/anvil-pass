use anvil_core::{
    GroupId,
    vault::{
        Vault,
        entries::NewEntry,
        search::{SearchFilter, SearchMode, max_score},
    },
};
use uuid::Uuid;

use crate::common::create_vault_with_entries;

mod common;

#[test]
fn test_search_mode_exact() {
    // Test exact match
    let result = Vault::score(SearchMode::Exact, "test", "test");
    assert_eq!(result, Some(i32::MAX));

    // Test case sensitivity
    let result = Vault::score(SearchMode::Exact, "test", "TEST");
    assert_eq!(result, None);

    // Test no match
    let result = Vault::score(SearchMode::Exact, "test", "different");
    assert_eq!(result, None);

    // Test empty strings
    let result = Vault::score(SearchMode::Exact, "", "");
    assert_eq!(result, Some(i32::MAX));

    let result = Vault::score(SearchMode::Exact, "", "nonempty");
    assert_eq!(result, None);
}

#[test]
fn test_search_mode_regex() {
    // Test simple regex
    let result = Vault::score(SearchMode::Regex, r"^test$", "test");
    assert_eq!(result, Some(i32::MAX));

    // Test regex with wildcard
    let result = Vault::score(SearchMode::Regex, r"t..t", "test");
    assert_eq!(result, Some(i32::MAX));

    // Test regex with character class
    let result = Vault::score(SearchMode::Regex, r"[0-9]+", "test123");
    assert_eq!(result, Some(i32::MAX));

    // Test invalid regex
    let result = Vault::score(SearchMode::Regex, r"[", "test");
    assert_eq!(result, None);

    // Test no match
    let result = Vault::score(SearchMode::Regex, r"^abc$", "test");
    assert_eq!(result, None);
}

#[test]
fn test_search_mode_fuzzy() {
    // Test exact match via fuzzy
    let result = Vault::score(SearchMode::Fuzzy, "test", "test");
    assert!(result.is_some());
    assert!(result.unwrap() > 0);

    // Test partial match
    let result = Vault::score(SearchMode::Fuzzy, "tst", "test");
    assert!(result.is_some());
    assert!(result.unwrap() > 0);

    // Test no match
    let result = Vault::score(SearchMode::Fuzzy, "xyz", "test");
    assert_eq!(result, None);

    // Test empty query
    let result = Vault::score(SearchMode::Fuzzy, "", "test");
    assert!(result.is_none());
}

#[test]
fn test_search_filter_new() {
    let filter = SearchFilter::new();

    assert!(filter.title.is_none());
    assert!(filter.username.is_none());
    assert!(filter.url.is_none());
    assert!(filter.group.is_none());
}

#[test]
fn test_search_filter_with_title() {
    let filter = SearchFilter::new().with_title("Test Title");

    assert_eq!(filter.title, Some("Test Title".to_string()));
    assert!(filter.username.is_none());
    assert!(filter.url.is_none());
    assert!(filter.group.is_none());
}

#[test]
fn test_search_filter_with_username() {
    let filter = SearchFilter::new().with_username("test_user");

    assert_eq!(filter.username, Some("test_user".to_string()));
    assert!(filter.title.is_none());
    assert!(filter.url.is_none());
    assert!(filter.group.is_none());
}

#[test]
fn test_search_filter_with_url() {
    let filter = SearchFilter::new().with_url("https://example.com");

    assert_eq!(filter.url, Some("https://example.com".to_string()));
    assert!(filter.title.is_none());
    assert!(filter.username.is_none());
    assert!(filter.group.is_none());
}

#[test]
fn test_search_filter_with_group() {
    let group_id = GroupId::from_uuid(Uuid::new_v4());
    let filter = SearchFilter::new().with_group(group_id);

    assert_eq!(filter.group, Some(group_id));
    assert!(filter.title.is_none());
    assert!(filter.username.is_none());
    assert!(filter.url.is_none());
}

#[test]
fn test_search_filter_chaining() {
    let group_id = GroupId::from_uuid(Uuid::new_v4());
    let filter = SearchFilter::new()
        .with_title("Title")
        .with_username("User")
        .with_url("https://example.com")
        .with_group(group_id);

    assert_eq!(filter.title, Some("Title".to_string()));
    assert_eq!(filter.username, Some("User".to_string()));
    assert_eq!(filter.url, Some("https://example.com".to_string()));
    assert_eq!(filter.group, Some(group_id));
}

#[test]
fn test_search_filter_overwrite() {
    let filter = SearchFilter::new().with_title("First").with_title("Second");

    assert_eq!(filter.title, Some("Second".to_string()));

    let filter = SearchFilter::new()
        .with_username("user1")
        .with_username("user2");

    assert_eq!(filter.username, Some("user2".to_string()));
}

#[test]
fn test_exact_search_exact_match() {
    let result = Vault::score(SearchMode::Exact, "test", "test");
    assert_eq!(result, Some(i32::MAX));
}

#[test]
fn test_exact_search_case_sensitive() {
    let result = Vault::score(SearchMode::Exact, "test", "TEST");
    assert_eq!(result, None);

    let result = Vault::score(SearchMode::Exact, "Test", "test");
    assert_eq!(result, None);
}

#[test]
fn test_exact_search_no_match() {
    let result = Vault::score(SearchMode::Exact, "test", "different");
    assert_eq!(result, None);

    let result = Vault::score(SearchMode::Exact, "test", "test1");
    assert_eq!(result, None);
}

#[test]
fn test_exact_search_empty() {
    let result = Vault::score(SearchMode::Exact, "", "");
    assert_eq!(result, Some(i32::MAX));

    let result = Vault::score(SearchMode::Exact, "", "nonempty");
    assert_eq!(result, None);

    let result = Vault::score(SearchMode::Exact, "nonempty", "");
    assert_eq!(result, None);
}

#[test]
fn test_exact_search_with_whitespace() {
    let result = Vault::score(SearchMode::Exact, "test query", "test query");
    assert_eq!(result, Some(i32::MAX));

    let result = Vault::score(SearchMode::Exact, "test query", "test  query");
    assert_eq!(result, None);
}

#[test]
fn test_regex_search_valid_pattern() {
    let result = Vault::score(SearchMode::Regex, r"^test$", "test");
    assert_eq!(result, Some(i32::MAX));

    let result = Vault::score(SearchMode::Regex, r"t.*t", "test");
    assert_eq!(result, Some(i32::MAX));
}

#[test]
fn test_regex_search_invalid_pattern() {
    let result = Vault::score(SearchMode::Regex, r"[", "test");
    assert_eq!(result, None);

    let result = Vault::score(SearchMode::Regex, r"(unclosed", "test");
    assert_eq!(result, None);
}

#[test]
fn test_regex_search_no_match() {
    let result = Vault::score(SearchMode::Regex, r"^abc$", "test");
    assert_eq!(result, None);

    let result = Vault::score(SearchMode::Regex, r"\d+", "abcdef");
    assert_eq!(result, None);
}

#[test]
fn test_regex_search_complex_patterns() {
    // Email pattern
    let result = Vault::score(
        SearchMode::Regex,
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        "test@example.com",
    );
    assert_eq!(result, Some(i32::MAX));

    // URL pattern
    let result = Vault::score(
        SearchMode::Regex,
        r"^https?://[^\s]+$",
        "https://example.com",
    );
    assert_eq!(result, Some(i32::MAX));

    // Date pattern
    let result = Vault::score(SearchMode::Regex, r"\d{4}-\d{2}-\d{2}", "2024-01-01");
    assert_eq!(result, Some(i32::MAX));
}

#[test]
fn test_max_score_both_some() {
    let result = max_score(Some(10), Some(20));
    assert_eq!(result, Some(20));

    let result = max_score(Some(30), Some(15));
    assert_eq!(result, Some(30));
}

#[test]
fn test_max_score_one_none() {
    let result = max_score(None, Some(10));
    assert_eq!(result, Some(10));

    let result = max_score(Some(10), None);
    assert_eq!(result, Some(10));
}

#[test]
fn test_max_score_both_none() {
    let result = max_score(None, None);
    assert_eq!(result, None);
}

#[test]
fn test_search_in_vault_context() {
    let (mut vault, _temp_dir, _path, _password, _ids) = create_vault_with_entries();

    // Add entries with different titles
    let entries = vec![
        ("password1", "Test Entry 1"),
        ("password2", "Test Entry 2"),
        ("password3", "Different Entry"),
    ];

    for (pass, title) in entries {
        let entry = NewEntry::new(pass).with_title(title);
        vault.add_entry(entry).unwrap();
    }

    // Test exact search
    let result = vault.search_entry_exact("Test Entry 1", None);
    assert!(result.is_some());
    assert_eq!(result.unwrap().get_title().unwrap(), "Test Entry 1");

    // Test fuzzy search would work similarly
    // (assuming there's a search method that uses fuzzy)
}

#[test]
fn test_score_integration() {
    // Test all search modes
    let test_cases = vec![
        (SearchMode::Exact, "test", "test", Some(i32::MAX)),
        (SearchMode::Exact, "test", "TEST", None),
        (SearchMode::Regex, r"^test$", "test", Some(i32::MAX)),
        (SearchMode::Regex, r"^test$", "TEST", None),
        (SearchMode::Fuzzy, "test", "test", Some(gt(0))),
        (SearchMode::Fuzzy, "test", "different", None),
    ];

    for (mode, query, candidate, expected) in test_cases {
        let result = Vault::score(mode, query, candidate);
        match expected {
            Some(expected_val) => {
                assert!(result.is_some());
                if expected_val != gt(0) {
                    assert_eq!(result.unwrap(), expected_val);
                } else {
                    assert!(result.unwrap() > 0);
                }
            }
            None => assert!(result.is_none()),
        }
    }
}

// Helper for "greater than 0" expected value
fn gt(_: i32) -> i32 {
    // This is just a marker for the test
    0
}
