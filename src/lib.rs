pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod matcher;
pub mod emoji;
pub mod commit;

// Re-export main types for convenience
pub use matcher::{GitmojiMatcher, MatcherResult};
pub use emoji::{EmojiLookup, EMOJI_MAP};
pub use commit::{GitCommit, GitError};
