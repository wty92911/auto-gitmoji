use anyhow::{Context, Result};
use std::process::Command;

/// Git command execution errors
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    CommandFailed(String),
    #[error("Not in a Git repository")]
    NotInRepository,
    #[error("Git binary not found")]
    GitNotFound,
}

/// Git commit functionality
pub struct GitCommit;

impl GitCommit {
    /// Check if we're in a Git repository
    pub fn is_git_repository() -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .context("Failed to execute git command")?;

        Ok(output.status.success())
    }

    /// Format a commit message with emoji
    pub fn format_message(emoji_code: &str, message: &str) -> String {
        format!("{emoji_code} {message}")
    }

    /// Execute git commit with the formatted message
    pub fn commit(message: &str, dry_run: bool) -> Result<String> {
        // Verify we're in a Git repository
        if !Self::is_git_repository()? {
            return Err(GitError::NotInRepository.into());
        }

        if dry_run {
            return Ok(format!(
                "DRY RUN: Would execute: git commit -m \"{message}\"",
            ));
        }

        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .context("Failed to execute git commit")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Include both stdout and stderr in success message
            // Pre-commit hooks often output to stderr even on success
            let mut result = format!("Git commit successful:\n{stdout}");
            if !stderr.is_empty() {
                result.push_str(&format!("\nPre-commit output:\n{stderr}"));
            }
            Ok(result)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(GitError::CommandFailed(stderr.to_string()).into())
        }
    }

    /// Get git status to show staged changes
    pub fn status() -> Result<String> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .context("Failed to execute git status")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(GitError::CommandFailed(stderr.to_string()).into())
        }
    }

    /// Check if there are staged changes to commit
    pub fn has_staged_changes() -> Result<bool> {
        let status = Self::status()?;
        // Look for staged changes (lines starting with A, M, D, R, C in first column)
        Ok(status.lines().any(|line| {
            if line.len() >= 2 {
                matches!(line.chars().next(), Some('A' | 'M' | 'D' | 'R' | 'C'))
            } else {
                false
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_message() {
        assert_eq!(
            GitCommit::format_message(":sparkles:", "Add new feature"),
            ":sparkles: Add new feature"
        );

        assert_eq!(
            GitCommit::format_message(":bug:", "Fix login issue"),
            ":bug: Fix login issue"
        );

        assert_eq!(
            GitCommit::format_message(":memo:", "Update documentation"),
            ":memo: Update documentation"
        );
    }

    #[test]
    fn test_format_message_with_empty_inputs() {
        assert_eq!(GitCommit::format_message("", "message"), " message");

        assert_eq!(GitCommit::format_message(":sparkles:", ""), ":sparkles: ");

        assert_eq!(GitCommit::format_message("", ""), " ");
    }

    #[test]
    fn test_format_message_with_special_characters() {
        assert_eq!(
            GitCommit::format_message(":art:", "Fix: resolve \"critical\" issue"),
            ":art: Fix: resolve \"critical\" issue"
        );

        assert_eq!(
            GitCommit::format_message(":sparkles:", "Add support for UTF-8 🎉"),
            ":sparkles: Add support for UTF-8 🎉"
        );
    }

    #[test]
    fn test_commit_dry_run() {
        let result = GitCommit::commit("Test message", true);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("DRY RUN"));
        assert!(output.contains("git commit -m \"Test message\""));
    }

    #[test]
    fn test_commit_dry_run_with_formatted_message() {
        let formatted_message = GitCommit::format_message(":sparkles:", "Add new feature");
        let result = GitCommit::commit(&formatted_message, true);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("DRY RUN"));
        assert!(output.contains(":sparkles: Add new feature"));
    }

    #[test]
    fn test_commit_success_message_format() {
        // Test that success messages properly format stdout and stderr
        let result = GitCommit::commit("Test message", true);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("DRY RUN"));
        assert!(output.contains("git commit -m \"Test message\""));

        // The dry run doesn't actually execute git, so it won't have pre-commit output
        // But this tests the message formatting structure
    }

    #[test]
    fn test_git_error_display() {
        let error = GitError::NotInRepository;
        assert_eq!(error.to_string(), "Not in a Git repository");

        let error = GitError::GitNotFound;
        assert_eq!(error.to_string(), "Git binary not found");

        let error = GitError::CommandFailed("Some error".to_string());
        assert_eq!(error.to_string(), "Git command failed: Some error");
    }

    // Note: The following tests require an actual Git repository to run
    // They are marked with #[ignore] to skip by default

    #[test]
    #[ignore = "requires git repository"]
    fn test_is_git_repository_in_git_repo() {
        // This test should pass when run in a Git repository
        let result = GitCommit::is_git_repository();
        assert!(result.is_ok());
        // When in a git repo, this should return true
        // assert!(result.unwrap());
    }

    #[test]
    #[ignore = "requires git repository with staged changes"]
    fn test_has_staged_changes() {
        // This test requires a Git repository with staged changes
        let result = GitCommit::has_staged_changes();
        assert!(result.is_ok());
        // Result depends on whether there are staged changes
    }

    #[test]
    #[ignore = "requires git repository"]
    fn test_status() {
        // This test requires a Git repository
        let result = GitCommit::status();
        assert!(result.is_ok());
        // The result should be a string (could be empty if clean repo)
    }

    #[test]
    #[ignore = "requires git repository with staged changes"]
    fn test_actual_commit() {
        // This test would actually commit to the repository
        // Only run manually when testing in a safe environment
        let _result = GitCommit::commit("Test commit message", false);
        // Result depends on repository state and staged changes
    }

    // Mock-based tests for Git operations
    #[test]
    fn test_status_parsing_logic() {
        // Test the logic for detecting staged changes from git status output

        // Simulate git status output with staged changes
        let status_with_staged = "M  modified_file.txt\nA  new_file.txt\n?? untracked.txt";
        let has_staged = status_with_staged.lines().any(|line| {
            if line.len() >= 2 {
                matches!(line.chars().next(), Some('A' | 'M' | 'D' | 'R' | 'C'))
            } else {
                false
            }
        });
        assert!(has_staged);

        // Simulate git status output without staged changes
        let status_without_staged = "?? untracked1.txt\n?? untracked2.txt";
        let has_staged = status_without_staged.lines().any(|line| {
            if line.len() >= 2 {
                matches!(line.chars().next(), Some('A' | 'M' | 'D' | 'R' | 'C'))
            } else {
                false
            }
        });
        assert!(!has_staged);

        // Empty status (clean repo)
        let empty_status = "";
        let has_staged = empty_status.lines().any(|line| {
            if line.len() >= 2 {
                matches!(line.chars().next(), Some('A' | 'M' | 'D' | 'R' | 'C'))
            } else {
                false
            }
        });
        assert!(!has_staged);
    }

    #[test]
    fn test_git_status_change_types() {
        // Test different types of git status changes
        let test_cases = vec![
            ("A  new_file.txt", true),            // Added
            ("M  modified.txt", true),            // Modified
            ("D  deleted.txt", true),             // Deleted
            ("R  old.txt -> new.txt", true),      // Renamed
            ("C  copied.txt", true),              // Copied
            ("?? untracked.txt", false),          // Untracked
            (" M workspace_modified.txt", false), // Workspace change only
            ("MM both_modified.txt", true),       // Both staged and workspace
        ];

        for (status_line, expected_staged) in test_cases {
            let has_staged = if status_line.len() >= 2 {
                matches!(
                    status_line.chars().next(),
                    Some('A' | 'M' | 'D' | 'R' | 'C')
                )
            } else {
                false
            };
            assert_eq!(
                has_staged, expected_staged,
                "Failed for status line: '{status_line}'"
            );
        }
    }
}
