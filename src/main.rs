use anyhow::Result;
use clap::{Arg, Command};
use auto_gitmoji::{
    matcher::MatcherFactory,
    commit::GitCommit,
    emoji::EmojiLookup,
};

fn main() -> Result<()> {
    let matches = Command::new("amoji")
        .version("0.1.0")
        .author("Auto-Gitmoji Team")
        .about("Automatically prepend gitmoji to commit messages")
        .arg(
            Arg::new("message")
                .help("The commit message")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show what would be committed without actually committing")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-emoji")
                .long("show-emoji")
                .help("Show available emoji codes")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Handle show-emoji flag
    if matches.get_flag("show-emoji") {
        println!("Available gitmoji codes:");
        let mut codes = EmojiLookup::all_codes();
        codes.sort();
        for code in codes {
            if let Some(unicode) = EmojiLookup::code_to_unicode(code) {
                println!("  {} {}", unicode, code);
            }
        }
        return Ok(());
    }

    // Message is required when not showing emoji
    let message = matches.get_one::<String>("message")
        .ok_or_else(|| anyhow::anyhow!("Message argument is required"))?;
    let dry_run = matches.get_flag("dry-run");

    // Create matcher and find appropriate emoji
    let matcher = MatcherFactory::default();
    let match_result = matcher.match_emoji(message)?;

    if let Some((emoji_code, emoji_unicode, confidence)) = match_result {
        // Format the commit message
        let full_message = GitCommit::format_message(&emoji_code, message);
        
        // Display the emoji and message to user
        println!("🎯 Matched emoji: {} {} (confidence: {:.0}%)", 
                 emoji_unicode, emoji_code, confidence * 100.0);
        println!("📝 Full message: {}", full_message);
        
        // Check for staged changes before committing
        if !dry_run {
            match GitCommit::has_staged_changes() {
                Ok(true) => {
                    println!("✅ Staged changes detected");
                }
                Ok(false) => {
                    println!("⚠️  No staged changes found. Please stage your changes first with 'git add'");
                    return Ok(());
                }
                Err(e) => {
                    println!("⚠️  Could not check Git status: {}", e);
                    println!("   Proceeding anyway...");
                }
            }
        }
        
        // Execute the commit
        match GitCommit::commit(&full_message, dry_run) {
            Ok(result) => {
                println!("✅ {}", result);
            }
            Err(e) => {
                eprintln!("❌ Commit failed: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("❌ Could not find appropriate emoji for message");
        std::process::exit(1);
    }

    Ok(())
} 