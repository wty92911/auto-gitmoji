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

pub mod commit;
pub mod emoji;
pub mod matcher;

// Re-export main types for convenience
pub use commit::{GitCommit, GitError};
pub use emoji::{EMOJI_MAP, EmojiLookup};
pub use matcher::{GitmojiMatcher, MatcherResult};
