pub mod simple;

use anyhow::Result;

/// Confidence score for emoji matches
pub type MatcherResult = Option<(String, String, f32)>; // (emoji_code, emoji_unicode, confidence)

/// Core trait for gitmoji matching strategies
pub trait GitmojiMatcher {
    /// Match a commit message to an appropriate gitmoji
    /// Returns (emoji_code, emoji_unicode, confidence_score) or None
    fn match_emoji(&self, message: &str) -> Result<MatcherResult>;

    /// Get the name of this matcher
    fn name(&self) -> &'static str;
}

/// Factory for creating matcher instances
pub struct MatcherFactory;

impl MatcherFactory {
    /// Create a simple keyword-based matcher
    pub fn simple() -> Box<dyn GitmojiMatcher> {
        Box::new(simple::SimpleMatcher::new())
    }
}
