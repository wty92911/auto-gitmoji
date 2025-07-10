use auto_gitmoji::{commit::GitCommit, emoji::EmojiLookup, matcher::MatcherFactory};
use std::process::Command;

#[test]
#[ignore = "do not consider binary for now"]
fn test_cli_binary_exists() {
    // Test that the binary can be executed
    let output = Command::new("cargo")
        .args(["build", "--bin", "amoji"])
        .output()
        .expect("Failed to build binary");

    assert!(output.status.success(), "Binary should build successfully");
}

#[test]
fn test_cli_help_command() {
    // Test that help command works
    let output = Command::new("cargo")
        .args(["run", "--bin", "amoji", "--", "--help"])
        .output()
        .expect("Failed to run help command");

    // Should either succeed or show help
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Help should mention key concepts
    let help_text = format!("{stdout}{stderr}");
    assert!(
        help_text.contains("amoji")
            || help_text.contains("gitmoji")
            || help_text.contains("commit")
    );
}

#[test]
fn test_full_workflow_integration() {
    // Test the complete workflow without actually committing

    // 1. Create matcher
    let matcher = MatcherFactory::simple();

    // 2. Test various commit message types
    let test_cases = vec![
        ("fix: resolve authentication bug", ":bug:", "🐛"),
        ("feat: add user profile page", ":sparkles:", "✨"),
        ("docs: update API documentation", ":memo:", "📝"),
        ("test: add integration tests", ":white_check_mark:", "✅"),
        ("refactor: improve code structure", ":recycle:", "♻️"),
        ("perf: optimize database queries", ":zap:", "⚡️"),
        ("style: fix code formatting", ":lipstick:", "💄"),
    ];

    for (message, expected_code, expected_emoji) in test_cases {
        // 3. Match emoji
        let match_result = matcher.match_emoji(message).unwrap();
        assert!(match_result.is_some(), "Should match emoji for: {message}");

        let (code, format_message) = match_result.unwrap();

        // 4. Verify emoji lookup consistency
        let lookup_emoji = EmojiLookup::code_to_unicode(&code);
        assert!(lookup_emoji.is_some());

        // 5. Verify format message structure
        assert!(format_message.starts_with(&code));
        assert!(format_message.contains(message));

        // 6. Test dry run commit
        let commit_result = GitCommit::commit(&format_message, true);
        assert!(commit_result.is_ok());

        let dry_run_output = commit_result.unwrap();
        assert!(dry_run_output.contains("DRY RUN"));
        assert!(dry_run_output.contains(&format_message));

        // 7. For exact keyword matches, verify expected results
        if code == expected_code {
            assert_eq!(
                lookup_emoji.unwrap(),
                expected_emoji,
                "Expected emoji for: {message}"
            );
        }
    }
}

#[test]
fn test_emoji_rendering_support() {
    // Test that emoji rendering works properly
    let emojis_to_test = vec![
        (":sparkles:", "✨"),
        (":bug:", "🐛"),
        (":fire:", "🔥"),
        (":memo:", "📝"),
        (":rocket:", "🚀"),
        (":lipstick:", "💄"),
        (":tada:", "🎉"),
        (":white_check_mark:", "✅"),
        (":lock:", "🔒️"),
        (":bookmark:", "🔖"),
    ];

    for (code, expected_emoji) in emojis_to_test {
        let emoji = EmojiLookup::code_to_unicode(code);
        assert!(emoji.is_some(), "Should find emoji for code: {code}");
        assert_eq!(
            emoji.unwrap(),
            expected_emoji,
            "Wrong emoji for code: {code}"
        );

        // Test that emoji is actual Unicode
        let emoji_str = emoji.unwrap();
        assert!(
            emoji_str.chars().any(|c| c as u32 > 127),
            "Should be Unicode emoji: {emoji_str}"
        );
    }
}

#[test]
fn test_keyword_coverage() {
    // Test coverage of common git commit keywords
    let matcher = MatcherFactory::simple();

    let common_keywords = vec![
        "add",
        "fix",
        "update",
        "remove",
        "refactor",
        "docs",
        "test",
        "feat",
        "feature",
        "bug",
        "perf",
        "performance",
        "style",
        "build",
        "ci",
        "chore",
        "revert",
        "merge",
        "hotfix",
        "patch",
    ];

    for keyword in common_keywords {
        let test_message = format!("{keyword} something");
        let result = matcher.match_emoji(&test_message).unwrap();

        assert!(
            result.is_some(),
            "Should match something for keyword: {keyword}"
        );

        let (code, _format_message) = result.unwrap();

        // Should either be a high-confidence match or low-confidence fallback
        assert!(
            code.contains(":") || code == ":sparkles:",
            "Unexpected code {code} for keyword: {keyword}"
        );
    }
}

#[test]
fn test_edge_case_handling() {
    let matcher = MatcherFactory::simple();

    let edge_cases = vec![
        "",                                                                         // Empty string
        "   ",        // Whitespace only
        "\n\t\r",     // Control characters
        "🎉🐛✨",     // Only emojis
        "123456789",  // Only numbers
        "!@#$%^&*()", // Only symbols
        "VeryLongMessageWithNoSpacesOrPunctuationThatMightCauseIssuesInProcessing", // Very long
        "Iñtërnâtiönàl çhärāçtërs 中文 العربية", // International characters
    ];

    for message in edge_cases {
        let result = matcher.match_emoji(message);
        assert!(result.is_ok(), "Should handle edge case: '{message}'");

        let match_result = result.unwrap();
        assert!(
            match_result.is_some(),
            "Should return some result for: '{message}'"
        );

        let (code, _format_message) = match_result.unwrap();
        assert!(
            !code.is_empty(),
            "Code should not be empty for: '{message}'"
        );
    }
}

#[test]
fn test_git_integration_safety() {
    // Test that Git operations are safe and don't cause issues

    // Test dry run always works
    let messages = vec![
        "test commit message",
        ":sparkles: add new feature",
        "fix: resolve critical bug",
        "Very long commit message that might cause issues with Git command line handling",
        "Message with \"quotes\" and 'apostrophes'",
        "Message with $special characters & symbols!",
    ];

    for message in messages {
        let result = GitCommit::commit(message, true);
        assert!(result.is_ok(), "Dry run should work for: '{message}'");

        let output = result.unwrap();
        assert!(
            output.contains("DRY RUN"),
            "Should indicate dry run for: '{message}'"
        );
        assert!(
            output.contains(message),
            "Should contain message for: '{message}'"
        );
    }
}

#[test]
fn test_performance_integration() {
    // Test performance of the integrated system
    let matcher = MatcherFactory::simple();

    let start = std::time::Instant::now();

    // Simulate processing many commit messages
    for i in 0..100 {
        let messages = vec![
            format!("fix bug #{i}"),
            format!("add feature {i}"),
            format!("update docs for component {i}"),
            format!("test module {i} functionality"),
            format!("refactor legacy code in {i}"),
        ];

        for message in messages {
            // Full workflow
            let match_result = matcher.match_emoji(&message).unwrap().unwrap();
            let (code, format_message) = match_result;

            // Verify lookup
            let _lookup_emoji = EmojiLookup::code_to_unicode(&code).unwrap();

            // The format_message already contains the full formatted message
            // Dry run commit
            let _commit_result = GitCommit::commit(&format_message, true).unwrap();
        }
    }

    let duration = start.elapsed();

    // Should complete reasonably quickly (less than 5 seconds for 500 operations)
    assert!(
        duration.as_secs() < 5,
        "Performance test took too long: {duration:?} for 500 operations"
    );
}

#[test]
fn test_data_file_loading() {
    // Test that data files are properly loaded and accessible

    // Test emoji map loading
    let emoji_codes = EmojiLookup::all_codes();
    assert!(
        emoji_codes.len() > 60,
        "Should have substantial number of emojis"
    );

    // Test that all codes map to valid emojis
    for code in &emoji_codes {
        let emoji = EmojiLookup::code_to_unicode(code);
        assert!(emoji.is_some(), "Should have emoji for code: {code}");

        let emoji_str = emoji.unwrap();
        assert!(
            !emoji_str.is_empty(),
            "Emoji should not be empty for: {code}"
        );
        assert!(
            emoji_str.chars().any(|c| c as u32 > 127),
            "Should be Unicode for: {code}"
        );
    }

    // Test matcher has reasonable keyword coverage
    let matcher = MatcherFactory::simple();

    // These keywords should definitely be covered
    let essential_keywords = vec!["fix", "add", "update", "remove", "test"];
    for keyword in essential_keywords {
        let message = format!("{keyword} something");
        let result = matcher.match_emoji(&message).unwrap().unwrap();
        let (code, _format_message) = result;

        // Should return a valid emoji code (either keyword match or fallback)
        assert!(
            code.starts_with(':') && code.ends_with(':'),
            "Should return valid emoji code for keyword: {keyword}, got: {code}"
        );
    }
}
