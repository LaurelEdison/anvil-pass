use std::vec;

use keepass::db::{Entry, EntryId, EntryRef, Group, GroupId};
use regex::Regex;

use crate::vault::{DatabaseProcessingError, Vault};

pub struct SearchQuery {
    pub title: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub group: Option<GroupId>,
}
impl SearchQuery {
    pub fn new() -> Self {
        SearchQuery {
            title: None,
            username: None,
            url: None,
            group: None,
        }
    }
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into().to_lowercase());
        self
    }
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into().to_lowercase());
        self
    }
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into().to_lowercase());
        self
    }
    pub fn with_group(mut self, group: impl Into<GroupId>) -> Self {
        self.group = Some(group.into());
        self
    }
}

pub trait SearchEngine {
    fn search(&self, vault: &Vault, query: &SearchQuery) -> Vec<SearchResult>;
}

pub struct SearchResult {
    pub entry_id: EntryId,
    pub score: i32,
}

pub struct SimpleSearch;
pub struct FuzzySearch;
pub struct RegexSearch;

impl SearchEngine for SimpleSearch {
    fn search(&self, vault: &Vault, query: &SearchQuery) -> Vec<SearchResult> {
        let mut results = Vec::new();
        for entry in vault.database.iter_all_entries() {
            if matches_query(&entry, query) {
                results.push(SearchResult {
                    entry_id: entry.id(),
                    score: 1,
                });
            }
        }
        results
    }
}
fn matches_query(entry: &EntryRef, query: &SearchQuery) -> bool {
    let entry_title = entry
        .get_title()
        .unwrap_or_default()
        .to_string()
        .to_lowercase();
    let entry_username = entry
        .get_username()
        .unwrap_or_default()
        .to_string()
        .to_lowercase();
    let entry_url = entry
        .get_url()
        .unwrap_or_default()
        .to_string()
        .to_lowercase();
    if let Some(ref title) = query.title {
        if !entry_title.contains(title) {
            return false;
        }
    }
    if let Some(ref username) = query.username {
        if !entry_username.contains(username) {
            return false;
        }
    }
    if let Some(ref url) = query.url {
        if !entry_url.contains(url) {
            return false;
        }
    }
    true
}

struct QueryRegex {
    title: Option<Regex>,
    username: Option<Regex>,
    url: Option<Regex>,
}

impl QueryRegex {
    fn new(query: &SearchQuery) -> Result<QueryRegex, DatabaseProcessingError> {
        let title_regex = query.title.as_ref().map(|p| Regex::new(p));
        let username_regex = query.username.as_ref().map(|p| Regex::new(p));
        let url_regex = query.url.as_ref().map(|p| Regex::new(p));

        if title_regex.as_ref().map_or(false, |r| r.is_err())
            || username_regex.as_ref().map_or(false, |r| r.is_err())
            || url_regex.as_ref().map_or(false, |r| r.is_err())
        {
            return Err(DatabaseProcessingError::InvalidRegex);
        }

        let title_regex = title_regex.and_then(Result::ok);
        let username_regex = username_regex.and_then(Result::ok);
        let url_regex = url_regex.and_then(Result::ok);

        Ok(QueryRegex {
            title: title_regex,
            username: username_regex,
            url: url_regex,
        })
    }
}

impl SearchEngine for RegexSearch {
    fn search(&self, vault: &Vault, query: &SearchQuery) -> Vec<SearchResult> {
        let query_regex = match QueryRegex::new(query) {
            Ok(regex) => regex,
            Err(_) => return Vec::new(),
        };

        let mut results = Vec::new();
        for entry in vault.database.iter_all_entries() {
            if matches_regex(&entry, &query_regex) {
                results.push(SearchResult {
                    entry_id: entry.id(),
                    score: 1,
                });
            }
        }
        results
    }
}

fn matches_regex(entry: &EntryRef, query_regex: &QueryRegex) -> bool {
    let entry_title = entry
        .get_title()
        .unwrap_or_default()
        .to_string()
        .to_lowercase();
    let entry_username = entry
        .get_username()
        .unwrap_or_default()
        .to_string()
        .to_lowercase();
    let entry_url = entry
        .get_url()
        .unwrap_or_default()
        .to_string()
        .to_lowercase();

    if let Some(ref regex) = query_regex.title {
        if !regex.is_match(entry_title.as_str()) {
            return false;
        }
    }
    if let Some(ref regex) = query_regex.username {
        if !regex.is_match(&entry_username.as_str()) {
            return false;
        }
    }
    if let Some(ref regex) = query_regex.url {
        if regex.is_match(&entry_url.as_str()) {
            return false;
        }
    }
    true
}

pub struct FuzzyScoringConfig {
    sequential_bonus: i32,
    separator_bonus: i32,
    camel_bonus: i32,
    first_letter_bonus: i32,
    leading_letter_penalty: i32,
    max_leading_letter_penalty: i32,
    unmatched_letter_penalty: i32,
}

impl FuzzyScoringConfig {
    fn default() -> Self {
        Self {
            sequential_bonus: 15,
            separator_bonus: 30,
            camel_bonus: 30,
            first_letter_bonus: 15,
            leading_letter_penalty: -5,
            max_leading_letter_penalty: -15,
            unmatched_letter_penalty: -1,
        }
    }
}

// Do later
impl SearchEngine for FuzzySearch {
    // TODO
    fn search(&self, vault: &Vault, query: &SearchQuery) -> Vec<SearchResult> {
        let mut results = Vec::new();
        results
    }
}
