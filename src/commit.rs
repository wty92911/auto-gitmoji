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
            Ok(format!("Git commit successful:\n{stdout}",))
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
