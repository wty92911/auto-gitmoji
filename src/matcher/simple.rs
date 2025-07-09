use super::{GitmojiMatcher, MatcherResult};
use crate::emoji::EmojiLookup;
use anyhow::Result;
use std::collections::HashMap;
use serde_json;

/// Simple keyword-based matcher that loads keywords from JSON
pub struct SimpleMatcher {
    keyword_map: HashMap<String, String>,
}

impl SimpleMatcher {
    pub fn new() -> Self {
        let keyword_map = Self::load_keyword_map()
            .unwrap_or_else(|_| Self::default_keyword_map());
        
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
    /// 1. Split the message into words (lowercase, alphanumeric only)
    /// 2. Check each word in order against the keyword map
    /// 3. Return the first match found
    fn find_first_keyword_match(&self, message: &str) -> Option<&str> {
        // Split message into words, convert to lowercase, remove punctuation
        let words: Vec<String> = message
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                    .collect::<String>()
                    .to_lowercase()
            })
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
            if let Some(emoji_unicode) = EmojiLookup::code_to_unicode(emoji_code) {
                // High confidence for exact keyword matches
                return Ok(Some((emoji_code.to_string(), emoji_unicode.to_string(), 1.0)));
            }
        }
        
        // Fallback to sparkles for general changes if no keyword match
        Ok(Some((":sparkles:".to_string(), "✨".to_string(), 0.3)))
    }
    
    fn name(&self) -> &'static str {
        "simple"
    }
} 