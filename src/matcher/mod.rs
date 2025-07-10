#[cfg(feature = "llm")]
pub mod llm;
pub mod simple;

use anyhow::Result;

/// Return type for emoji matches
pub type MatcherResult = Option<(String, String)>; // (emoji_code, format_message)

/// Core trait for gitmoji matching strategies
pub trait GitmojiMatcher {
    /// Match a commit message to an appropriate gitmoji
    /// Returns (emoji_code, formatted_message) or None
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

    /// Create an LLM matcher with the given configuration
    #[cfg(feature = "llm")]
    pub fn llm(config: llm::LLMConfig) -> Box<dyn GitmojiMatcher> {
        Box::new(llm::LLMMatcher::new(config))
    }

    /// Create an LLM matcher with fallback to simple matcher
    #[cfg(feature = "llm")]
    pub fn llm_with_fallback(config: llm::LLMConfig) -> Box<dyn GitmojiMatcher> {
        Box::new(llm::LLMWithFallbackMatcher::new(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher_factory_simple() {
        let matcher = MatcherFactory::simple();
        assert_eq!(matcher.name(), "simple");
    }

    #[test]
    fn test_matcher_result_none() {
        let result: MatcherResult = None;
        assert!(result.is_none());
    }

    #[test]
    fn test_gitmoji_matcher_trait() {
        let matcher = MatcherFactory::simple();

        // Test trait methods
        assert_eq!(matcher.name(), "simple");

        // Test match_emoji returns proper result
        let result = matcher.match_emoji("fix bug").unwrap();
        assert!(result.is_some());

        let (code, format_message) = result.unwrap();
        assert!(!code.is_empty());
        assert!(!format_message.is_empty());
    }

    #[test]
    fn test_multiple_matcher_instances() {
        let matcher1 = MatcherFactory::simple();
        let matcher2 = MatcherFactory::simple();

        // Both should have same name
        assert_eq!(matcher1.name(), matcher2.name());

        // Both should produce same results for same input
        let message = "fix authentication bug";
        let result1 = matcher1.match_emoji(message).unwrap();
        let result2 = matcher2.match_emoji(message).unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_matcher_error_handling() {
        let matcher = MatcherFactory::simple();

        // Test that matcher handles various edge cases without panicking
        let test_cases = vec![
            "",
            "   ",
            "\n\t",
            "🎉🐛✨",
            "very_long_message_with_no_punctuation_or_spaces_that_might_cause_issues",
            "Mix3d c@se w1th numb3rs & $ymb0ls!",
        ];

        for message in test_cases {
            let result = matcher.match_emoji(message);
            assert!(result.is_ok(), "Matcher should handle: '{message}'");
            assert!(
                result.unwrap().is_some(),
                "Should always return some result for: '{message}'"
            );
        }
    }

    #[test]
    fn test_format_message_structure() {
        let matcher = MatcherFactory::simple();

        let test_messages = vec![
            "fix critical bug", // Should have keyword match
            "add new feature",  // Should have keyword match
            "random text here", // Should have fallback
        ];

        for message in test_messages {
            let result = matcher.match_emoji(message).unwrap();
            assert!(result.is_some());
            let (code, format_message) = result.unwrap();

            // Code should follow :name: format
            assert!(code.starts_with(':'), "Code should start with ':': {code}");
            assert!(code.ends_with(':'), "Code should end with ':': {code}");

            // Format message should contain the original message
            assert!(
                format_message.contains(message),
                "Format message should contain original message: '{format_message}' should contain '{message}'"
            );

            // Format message should start with the emoji code
            assert!(
                format_message.starts_with(&code),
                "Format message should start with emoji code: '{format_message}' should start with '{code}'"
            );
        }
    }

    #[test]
    fn test_emoji_code_format() {
        let matcher = MatcherFactory::simple();

        let result = matcher.match_emoji("fix bug").unwrap().unwrap();
        let (code, _format_message) = result;

        // Emoji code should follow :name: format
        assert!(code.starts_with(':'), "Code should start with ':': {code}");
        assert!(code.ends_with(':'), "Code should end with ':': {code}");
        assert!(code.len() > 2, "Code should be more than just '::': {code}");
    }

    #[test]
    fn test_formatted_message_content() {
        let matcher = MatcherFactory::simple();

        let result = matcher.match_emoji("fix bug").unwrap().unwrap();
        let (_code, format_message) = result;

        // Format message should contain meaningful content
        assert!(
            !format_message.is_empty(),
            "Format message should not be empty"
        );
        assert!(
            format_message.contains("fix bug"),
            "Format message should contain original text"
        );
    }

    #[test]
    fn test_consistent_results() {
        let matcher = MatcherFactory::simple();

        // Same input should always produce same output
        let message = "fix authentication issue";
        let results: Vec<_> = (0..5)
            .map(|_| matcher.match_emoji(message).unwrap())
            .collect();

        // All results should be identical
        let first_result = &results[0];
        for result in &results[1..] {
            assert_eq!(result, first_result, "Results should be consistent");
        }
    }

    #[test]
    fn test_matcher_trait_object() {
        // Test that we can use the matcher as a trait object
        let matcher: Box<dyn GitmojiMatcher> = MatcherFactory::simple();

        let result = matcher.match_emoji("add feature").unwrap();
        assert!(result.is_some());

        let (code, format_message) = result.unwrap();
        assert!(!code.is_empty());
        assert!(!format_message.is_empty());
    }

    #[test]
    fn test_factory_pattern() {
        // Test that factory properly creates instances
        let matcher = MatcherFactory::simple();

        // Should implement the trait correctly
        assert_eq!(matcher.name(), "simple");

        // Should be functional
        let result = matcher.match_emoji("test message");
        assert!(result.is_ok());
    }
}
