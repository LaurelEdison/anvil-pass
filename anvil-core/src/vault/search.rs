use keepass::db::GroupId;
use regex::Regex;

use crate::vault::Vault;

impl Vault {
    pub fn score(search_mode: SearchMode, p_query: &str, p_candidate: &str) -> Option<i32> {
        match search_mode {
            SearchMode::Exact => ExactSearch::score(p_query, p_candidate),
            SearchMode::Regex => RegexSearch::score(p_query, p_candidate),
            SearchMode::Fuzzy => FuzzySearch::score(p_query, p_candidate),
        }
    }
}
pub fn max_score(current: Option<i32>, new: Option<i32>) -> Option<i32> {
    match (current, new) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (None, Some(b)) => Some(b),
        (Some(a), None) => Some(a),
        (None, None) => None,
    }
}
pub enum SearchMode {
    Regex,
    Fuzzy,
    Exact,
}

pub struct SearchFilter {
    pub title: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub group: Option<GroupId>,
}
impl SearchFilter {
    pub fn new() -> Self {
        SearchFilter {
            title: None,
            username: None,
            url: None,
            group: None,
        }
    }
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn with_group(mut self, group: impl Into<GroupId>) -> Self {
        self.group = Some(group.into());
        self
    }
}

pub struct SearchResult<Id> {
    pub id: Id,
    pub score: i32,
}

pub struct FuzzySearch;
pub struct RegexSearch;
pub struct ExactSearch;

impl ExactSearch {
    fn score(p_query: &str, p_candidate: &str) -> Option<i32> {
        let query = p_query;
        let candidate = p_candidate;
        if candidate == query {
            return Some(i32::MAX);
        }
        return None;
    }
}

impl RegexSearch {
    fn score(p_query: &str, p_candidate: &str) -> Option<i32> {
        let query_regex = match Regex::new(p_query) {
            Ok(regex) => regex,
            Err(_) => return None,
        };

        if query_regex.is_match(p_candidate) {
            Some(i32::MAX)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct FuzzyScoringConfig {
    sequential_bonus: i32,
    separator_bonus: i32,
    camel_bonus: i32,
    first_letter_bonus: i32,
    leading_letter_penalty: i32,
    max_leading_letter_penalty: i32,
    unmatched_letter_penalty: i32,
    recursion_limit: usize,
    max_matches: usize,
}

impl Default for FuzzyScoringConfig {
    fn default() -> Self {
        Self {
            sequential_bonus: 15,
            separator_bonus: 30,
            camel_bonus: 30,
            first_letter_bonus: 15,
            leading_letter_penalty: -5,
            max_leading_letter_penalty: -15,
            unmatched_letter_penalty: -1,
            max_matches: 256,
            recursion_limit: 10,
        }
    }
}

impl FuzzySearch {
    //TODO
    fn score(p_query: &str, p_candidate: &str) -> Option<i32> {
        let config = FuzzyScoringConfig::default();
        let pattern_chars: Vec<char> = p_query.chars().collect();
        let text_chars: Vec<char> = p_candidate.chars().collect();
        let mut matches = Vec::with_capacity(config.max_matches);

        let (matched, score) = FuzzySearch::fuzzy_match_recursive(
            config,
            &pattern_chars,
            &text_chars,
            0,
            0,
            &[],
            &mut matches,
            0,
            0,
        );

        if matched {
            return Some(score);
        }
        None
    }

    fn fuzzy_match_recursive(
        config: FuzzyScoringConfig,
        pattern: &[char],
        text: &[char],
        pattern_idx: usize,
        text_idx: usize,
        src_matches: &[usize],
        matches: &mut Vec<usize>,
        next_match: usize,
        recursion_count: usize,
    ) -> (bool, i32) {
        // Return if recursion limit is reached
        if recursion_count >= config.recursion_limit {
            return (false, 0);
        }

        // Return if we reached ends of strings
        if pattern_idx == pattern.len() || text_idx == text.len() {
            return (false, 0);
        }

        let mut best_recursive_match = false;
        let mut best_recursive_matches = Vec::new();
        let mut best_recursive_score = 0;

        let mut first_match = true;
        let mut pattern_cur = pattern_idx;
        let mut text_cur = text_idx;
        let mut next_match = next_match;
        let mut matches_clone = Vec::new();

        while pattern_cur < pattern.len() && text_cur < text.len() {
            let pattern_char = pattern[pattern_cur].to_ascii_lowercase();
            let text_char = text[text_cur].to_ascii_lowercase();

            if pattern_char == text_char {
                if next_match >= config.max_matches {
                    return (false, 0);
                }

                if first_match && !src_matches.is_empty() {
                    matches_clone = src_matches.to_vec();
                    first_match = false;
                }

                // Recursive call
                let mut recursive_matches = Vec::new();
                let (matched, recursive_score) = FuzzySearch::fuzzy_match_recursive(
                    config,
                    pattern,
                    text,
                    pattern_cur,
                    text_cur + 1,
                    &matches_clone,
                    &mut recursive_matches,
                    next_match,
                    recursion_count + 1,
                );

                if matched {
                    if !best_recursive_match || recursive_score > best_recursive_score {
                        best_recursive_matches = recursive_matches;
                        best_recursive_score = recursive_score;
                    }
                    best_recursive_match = true;
                }

                // Store the match
                if next_match < matches_clone.len() {
                    matches_clone[next_match] = text_cur;
                } else {
                    matches_clone.push(text_cur);
                }
                next_match += 1;
                pattern_cur += 1;
            }
            text_cur += 1;
        }

        let matched = pattern_cur == pattern.len();

        if matched {
            let score = FuzzySearch::calculate_score(config, text, &matches_clone, next_match);

            // Return best result
            if best_recursive_match && best_recursive_score > score {
                matches.clear();
                matches.extend(&best_recursive_matches);
                return (true, best_recursive_score);
            }

            matches.clear();
            matches.extend(&matches_clone);
            return (true, score);
        }

        (false, 0)
    }

    fn calculate_score(
        config: FuzzyScoringConfig,
        text: &[char],
        matches: &[usize],
        match_count: usize,
    ) -> i32 {
        let mut score = 100;

        // Apply leading letter penalty
        let first_match_pos = matches.first().copied().unwrap_or(0) as i32;
        let penalty = (config.leading_letter_penalty * first_match_pos)
            .max(config.max_leading_letter_penalty);
        score += penalty;

        // Apply unmatched penalty
        let unmatched = (text.len() - match_count) as i32;
        score += config.unmatched_letter_penalty * unmatched;

        // Apply ordering bonuses
        for i in 0..match_count {
            let curr_idx = matches[i];

            if i > 0 {
                let prev_idx = matches[i - 1];
                if curr_idx == prev_idx + 1 {
                    score += config.sequential_bonus;
                }
            }

            // Check for bonuses based on neighbor character value
            if curr_idx > 0 {
                let neighbor = text[curr_idx - 1];
                let curr = text[curr_idx];

                // Camel case bonus
                if neighbor.is_ascii_lowercase() && curr.is_ascii_uppercase() {
                    score += config.camel_bonus;
                }

                // Separator bonus
                if neighbor == '_' || neighbor == ' ' {
                    score += config.separator_bonus;
                }
            } else {
                // First letter bonus
                score += config.first_letter_bonus;
            }
        }
        score
    }
}
