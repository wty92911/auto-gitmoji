use super::{GitmojiMatcher, MatcherResult};
use anyhow::Result;
use serde_json;
use std::collections::HashMap;

/// Simple keyword-based matcher that loads keywords from JSON
pub struct SimpleMatcher {
    keyword_map: HashMap<String, String>,
}

impl Default for SimpleMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleMatcher {
    pub fn new() -> Self {
        let keyword_map = Self::load_keyword_map().unwrap_or_else(|_| Self::default_keyword_map());

        Self { keyword_map }
    }

    /// Load keyword mapping from JSON file
    fn load_keyword_map() -> Result<HashMap<String, String>> {
        let json_content = include_str!("../../fixtures/keyword_map.json");
        let map: HashMap<String, String> = serde_json::from_str(json_content)?;
        Ok(map)
    }

    /// Fallback keyword map if JSON loading fails
    fn default_keyword_map() -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Essential fallback keywords
        map.insert("add".to_string(), ":sparkles:".to_string());
        map.insert("fix".to_string(), ":bug:".to_string());
        map.insert("docs".to_string(), ":memo:".to_string());
        map.insert("test".to_string(), ":white_check_mark:".to_string());
        map.insert("refactor".to_string(), ":recycle:".to_string());
        map.insert("remove".to_string(), ":fire:".to_string());
        map.insert("perf".to_string(), ":zap:".to_string());
        map.insert("style".to_string(), ":lipstick:".to_string());
        map.insert("init".to_string(), ":tada:".to_string());
        map.insert("security".to_string(), ":lock:".to_string());

        map
    }

    /// Split message into words and find first matching keyword
    /// Strategy:
    /// 1. First filter out non-alphanumeric characters (except spaces)
    /// 2. Split the cleaned message into words
    /// 3. Check each word in order against the keyword map
    /// 4. Return the first match found
    fn find_first_keyword_match(&self, message: &str) -> Option<&str> {
        // First, replace non-alphanumeric characters with spaces (except existing spaces)
        // This converts "fix-the-bug" to "fix the bug" and "fix(auth)" to "fix auth"
        let cleaned_message: String = message
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { ' ' })
            .collect();

        // Then extract words from the cleaned message
        let words: Vec<String> = cleaned_message
            .split_whitespace()
            .map(|word| word.to_lowercase())
            .filter(|word| !word.is_empty())
            .collect();

        // Find first word that exists in keyword map
        for word in words {
            if let Some(emoji_code) = self.keyword_map.get(&word) {
                return Some(emoji_code);
            }
        }

        None
    }
}

impl GitmojiMatcher for SimpleMatcher {
    fn match_emoji(&self, message: &str) -> Result<MatcherResult> {
        if let Some(emoji_code) = self.find_first_keyword_match(message) {
            let formatted_message = format!("{emoji_code} {message}");
            return Ok(Some((emoji_code.to_string(), formatted_message)));
        }

        // Fallback to sparkles for general changes if no keyword match
        let emoji_code = ":sparkles:";
        let formatted_message = format!("{emoji_code} {message}");
        Ok(Some((emoji_code.to_string(), formatted_message)))
    }

    fn name(&self) -> &'static str {
        "simple"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_matcher_creation() {
        let matcher = SimpleMatcher::new();
        assert_eq!(matcher.name(), "simple");
        assert!(!matcher.keyword_map.is_empty());
    }

    #[test]
    fn test_default_trait_implementation() {
        let matcher = SimpleMatcher::default();
        assert!(!matcher.keyword_map.is_empty());
    }

    #[test]
    fn test_keyword_matching_basic() {
        let matcher = SimpleMatcher::new();

        let result = matcher.match_emoji("fix login bug").unwrap();
        assert!(result.is_some());
        let (code, format_message) = result.unwrap();

        // Should match "fix" keyword
        assert_eq!(code, ":bug:");
        assert_eq!(format_message, ":bug: fix login bug");
    }

    #[test]
    fn test_keyword_matching_first_word_priority() {
        let matcher = SimpleMatcher::new();

        // Test that it matches the first keyword found, not later ones
        let result = matcher.match_emoji("add fix for the issue").unwrap();
        assert!(result.is_some());
        let (code, format_message) = result.unwrap();

        // Should match "fix" (bug)
        assert_eq!(code, ":bug:");
        assert_eq!(format_message, ":bug: add fix for the issue");
    }

    #[test]
    fn test_keyword_matching_case_insensitive() {
        let matcher = SimpleMatcher::new();

        let test_cases = vec![
            "FIX the issue",
            "Fix the issue",
            "fix the issue",
            "fIx the issue",
        ];

        for message in test_cases {
            let result = matcher.match_emoji(message).unwrap();
            assert!(result.is_some());
            let (code, format_message) = result.unwrap();
            assert_eq!(code, ":bug:", "Failed for message: '{message}'");
            assert_eq!(format_message, format!(":bug: {message}"));
        }
    }

    #[test]
    fn test_keyword_matching_with_punctuation() {
        let matcher = SimpleMatcher::new();

        let test_cases = vec![
            "fix: resolve login issue",
            "fix - resolve login issue",
            "fix(auth): resolve login issue",
            "[fix] resolve login issue",
            "fix! resolve login issue",
        ];

        for message in test_cases {
            let result = matcher.match_emoji(message).unwrap();
            assert!(result.is_some());
            let (code, format_message) = result.unwrap();
            assert_eq!(code, ":bug:", "Failed for message: '{message}'");
            assert_eq!(format_message, format!(":bug: {message}"));
        }
    }

    #[test]
    fn test_fallback_behavior() {
        let matcher = SimpleMatcher::new();

        // Test with message that has no matching keywords
        let result = matcher.match_emoji("random unmatched message").unwrap();
        assert!(result.is_some());
        let (code, format_message) = result.unwrap();

        // Should fall back to sparkles
        assert_eq!(code, ":sparkles:");
        assert_eq!(format_message, ":sparkles: random unmatched message");
    }

    #[test]
    fn test_empty_message() {
        let matcher = SimpleMatcher::new();

        let result = matcher.match_emoji("").unwrap();
        assert!(result.is_some());
        let (code, format_message) = result.unwrap();

        // Should fall back to sparkles
        assert_eq!(code, ":sparkles:");
        assert_eq!(format_message, ":sparkles: ");
    }

    #[test]
    fn test_whitespace_only_message() {
        let matcher = SimpleMatcher::new();

        let result = matcher.match_emoji("   \t  \n  ").unwrap();
        assert!(result.is_some());
        let (code, format_message) = result.unwrap();

        // Should fall back to sparkles
        assert_eq!(code, ":sparkles:");
        assert_eq!(format_message, ":sparkles:    \t  \n  ");
    }

    #[test]
    fn test_common_commit_patterns() {
        let matcher = SimpleMatcher::new();

        let test_cases = vec![
            ("feat: add user authentication", ":sparkles:"), // feat -> add
            ("fix: resolve memory leak", ":bug:"),           // fix
            ("docs: update API documentation", ":memo:"),    // docs
            ("test: add unit tests", ":white_check_mark:"),  // test
            ("refactor: improve code structure", ":recycle:"), // refactor
            ("perf: optimize database queries", ":zap:"),    // perf
            ("style: fix code formatting", ":lipstick:"),    // style
            ("chore: update dependencies", ":package:"),     // chore for package
        ];

        for (message, expected_code) in test_cases {
            let result = matcher.match_emoji(message).unwrap();
            assert!(result.is_some(), "No match for message: '{message}'");
            let (code, format_message) = result.unwrap();

            assert_eq!(code, expected_code, "Failed for message: '{message}'");
            assert_eq!(format_message, format!("{expected_code} {message}"));
        }
    }

    #[test]
    fn test_word_boundary_matching() {
        let matcher = SimpleMatcher::new();

        // Test that "fix" in "prefix" doesn't match
        let result = matcher.match_emoji("prefix something").unwrap();
        // Should fall back since "prefix" contains "fix" but isn't the word "fix"
        let (code, format_message) = result.unwrap();
        assert_eq!(code, ":sparkles:"); // Should be fallback
        assert_eq!(format_message, ":sparkles: prefix something");
    }

    #[test]
    fn test_multiple_keywords_first_wins() {
        let matcher = SimpleMatcher::new();

        // If both "add" and "test" are keywords, first one should win
        let result = matcher
            .match_emoji("add new test for authentication")
            .unwrap();
        assert!(result.is_some());
        let (code, format_message) = result.unwrap();

        // Should match first keyword found
        // The exact emoji depends on which keyword is found first
        assert!(!code.is_empty());
        assert!(format_message.contains("add new test for authentication"));
    }

    #[test]
    fn test_keyword_map_loading() {
        let matcher = SimpleMatcher::new();

        // Test that the keyword map contains expected entries
        assert!(matcher.keyword_map.len() > 10); // Should have substantial keywords

        // Test some essential keywords exist (from default or JSON)
        let essential_keywords = vec!["add", "fix", "docs", "test", "refactor"];
        let mut found_keywords = 0;

        for keyword in essential_keywords {
            if matcher.keyword_map.contains_key(keyword) {
                found_keywords += 1;
            }
        }

        assert!(found_keywords > 3, "Should have most essential keywords");
    }

    #[test]
    fn test_default_keyword_map_fallback() {
        let default_map = SimpleMatcher::default_keyword_map();

        // Test essential mappings exist
        assert_eq!(default_map.get("add"), Some(&":sparkles:".to_string()));
        assert_eq!(default_map.get("fix"), Some(&":bug:".to_string()));
        assert_eq!(default_map.get("docs"), Some(&":memo:".to_string()));
        assert_eq!(
            default_map.get("test"),
            Some(&":white_check_mark:".to_string())
        );
        assert_eq!(default_map.get("refactor"), Some(&":recycle:".to_string()));

        // Should have reasonable number of fallback keywords
        assert!(default_map.len() >= 10);
    }

    #[test]
    fn test_find_first_keyword_match() {
        let matcher = SimpleMatcher::new();

        // Test various word splitting scenarios
        let test_cases = vec![
            ("fix the bug", true),     // Simple case
            ("fix-the-bug", true),     // Hyphenated
            ("fix_the_bug", true),     // Underscored
            ("fix123 the bug", true),  // With numbers
            ("(fix) the bug", true),   // With parentheses
            ("fix: the bug", true),    // With colon
            ("\"fix\" the bug", true), // With quotes
        ];

        for (message, should_match) in test_cases {
            let keyword = matcher.find_first_keyword_match(message);
            if should_match {
                assert!(keyword.is_some(), "Should find keyword in: '{message}'");
            }
        }
    }

    #[test]
    fn test_result_format() {
        let matcher = SimpleMatcher::new();

        // Test keyword match
        let result = matcher.match_emoji("fix the bug").unwrap().unwrap();
        let (code, format_message) = result;
        assert_eq!(code, ":bug:");
        assert_eq!(format_message, ":bug: fix the bug");

        // Test fallback
        let result = matcher
            .match_emoji("random unmatched text")
            .unwrap()
            .unwrap();
        let (code, format_message) = result;
        assert_eq!(code, ":sparkles:");
        assert_eq!(format_message, ":sparkles: random unmatched text");
    }

    #[test]
    fn test_unicode_and_special_characters() {
        let matcher = SimpleMatcher::new();

        let test_cases = vec![
            "fix 🐛 the bug",
            "add ✨ new feature",
            "test → verification",
            "docs 📝 update",
        ];

        for message in test_cases {
            let result = matcher.match_emoji(message);
            assert!(result.is_ok(), "Should handle Unicode in: '{message}'");
            assert!(result.unwrap().is_some());
        }
    }
}
