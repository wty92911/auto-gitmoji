pub mod commit;
pub mod emoji;
pub mod matcher;

// Re-export main types for convenience
pub use commit::{GitCommit, GitError};
pub use emoji::{EMOJI_MAP, EmojiLookup};
pub use matcher::{GitmojiMatcher, MatcherFactory, MatcherResult};

// Re-export LLM types only when the feature is enabled
#[cfg(feature = "llm")]
pub use matcher::llm::{LLMConfig, LLMMatcher, LLMModel, LLMProvider, LLMWithFallbackMatcher};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::MatcherFactory;

    #[test]
    fn test_library_integration() {
        // Test that all main components work together
        let matcher = MatcherFactory::simple();

        // Test emoji lookup
        let emoji = EmojiLookup::code_to_unicode(":sparkles:");
        assert_eq!(emoji, Some("✨"));

        // Test matcher
        let result = matcher.match_emoji("fix critical bug").unwrap();
        assert!(result.is_some());
        let (code, format_message) = result.unwrap();
        assert_eq!(code, ":bug:");
        assert_eq!(format_message, ":bug: fix critical bug");

        // Test that emoji lookup works for the matched code
        let emoji_unicode = EmojiLookup::code_to_unicode(&code);
        assert_eq!(emoji_unicode, Some("🐛"));
    }

    #[test]
    fn test_end_to_end_workflow() {
        // Simulate the complete workflow: message -> emoji match -> format
        let matcher = MatcherFactory::simple();

        let test_cases = vec![
            ("add user authentication", ":sparkles:", "✨"),
            ("fix memory leak", ":bug:", "🐛"),
            ("update documentation", ":memo:", "📝"),
            ("add unit tests", ":white_check_mark:", "✅"),
        ];

        for (message, expected_code, expected_emoji) in test_cases {
            // Match emoji
            let match_result = matcher.match_emoji(message).unwrap();
            assert!(match_result.is_some());

            let (code, format_message) = match_result.unwrap();

            // Verify emoji lookup
            let emoji_from_lookup = EmojiLookup::code_to_unicode(&code);
            assert!(emoji_from_lookup.is_some());

            // Verify format message structure
            assert!(format_message.contains(message));
            assert!(format_message.starts_with(&code));

            // For exact keyword matches, should match expected code
            if code == expected_code {
                assert_eq!(emoji_from_lookup.unwrap(), expected_emoji);
            }
        }
    }

    #[test]
    fn test_public_api_exports() {
        // Test that all expected types are properly exported

        // Test GitCommit export
        let _commit = GitCommit::format_message(":sparkles:", "test");

        // Test EmojiLookup export
        let _emoji = EmojiLookup::code_to_unicode(":sparkles:");

        // Test MatcherResult export
        let _result: MatcherResult = Some((
            ":sparkles:".to_string(),
            ":sparkles: test message".to_string(),
        ));

        // Test EMOJI_MAP export
        assert!(!EMOJI_MAP.is_empty());
    }

    #[test]
    fn test_error_handling_integration() {
        // Test error handling across components

        // Test invalid emoji code
        let invalid_emoji = EmojiLookup::code_to_unicode(":nonexistent:");
        assert!(invalid_emoji.is_none());

        // Test dry run commit (should always work)
        let commit_result = GitCommit::commit("test message", true);
        assert!(commit_result.is_ok());

        // Test matcher with edge cases
        let matcher = MatcherFactory::simple();
        let edge_cases = vec!["", "   ", "\n\t", "🎉🐛✨"];

        for case in edge_cases {
            let result = matcher.match_emoji(case);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some()); // Should always return something
        }
    }

    #[test]
    fn test_data_consistency() {
        // Test consistency between emoji map and matcher results
        let matcher = MatcherFactory::simple();

        let test_messages = vec![
            "fix bug",
            "add feature",
            "update docs",
            "run tests",
            "refactor code",
        ];

        for message in test_messages {
            let result = matcher.match_emoji(message).unwrap();
            assert!(result.is_some());

            let (code, format_message) = result.unwrap();

            // The emoji code should be valid in lookup
            let lookup_emoji = EmojiLookup::code_to_unicode(&code);
            assert!(lookup_emoji.is_some());

            // Format message should contain the original message
            assert!(format_message.contains(message));
            assert!(format_message.starts_with(&code));
        }
    }

    #[test]
    fn test_performance_basic() {
        // Basic performance test - should complete quickly
        let matcher = MatcherFactory::simple();

        let start = std::time::Instant::now();

        // Run many operations
        for i in 0..1000 {
            let message = format!("fix bug number {i}");
            let _result = matcher.match_emoji(&message).unwrap();
        }

        let duration = start.elapsed();

        // Should complete in reasonable time (less than 1 second)
        assert!(
            duration.as_secs() < 1,
            "Performance test took too long: {duration:?}"
        );
    }

    // Keep the original add function test for now
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
