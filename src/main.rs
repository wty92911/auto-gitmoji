use anyhow::Result;
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("amoji")
        .version("0.1.0")
        .author("Auto-Gitmoji Team")
        .about("Automatically prepend gitmoji to commit messages")
        .arg(
            Arg::new("message")
                .help("The commit message")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show what would be committed without actually committing")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let message = matches.get_one::<String>("message").unwrap();
    let dry_run = matches.get_flag("dry-run");

    // For now, just prepend a test emoji
    let emoji_code = ":sparkles:";
    let emoji_unicode = "✨";
    let full_message = format!("{} {}", emoji_code, message);

    println!("{} {}", emoji_unicode, message);
    
    if dry_run {
        println!("DRY RUN: Would execute: git commit -m \"{}\"", full_message);
    } else {
        println!("Would execute: git commit -m \"{}\"", full_message);
        println!("(Git execution not yet implemented)");
    }

    Ok(())
} 