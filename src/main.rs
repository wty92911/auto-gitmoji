use anyhow::Result;
#[cfg(feature = "llm")]
use auto_gitmoji::matcher::llm::{LLMConfig, LLMModel, LLMProvider};
use auto_gitmoji::{commit::GitCommit, emoji::EmojiLookup, matcher::MatcherFactory};
use clap::Parser;

// ANSI color codes for terminal formatting
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";
const BRIGHT_CYAN: &str = "\x1b[96m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const BRIGHT_YELLOW: &str = "\x1b[93m";

#[derive(Parser)]
#[command(
    name = "amoji",
    version = "0.1.0",
    author = "wty92911",
    about = "Automatically prepend gitmoji to commit messages"
)]
struct Args {
    /// The commit message
    message: Option<String>,

    /// Show what would be committed without actually committing
    #[arg(long)]
    #[arg(short)]
    #[arg(default_value_t = false)]
    dry_run: bool,

    /// Show available emoji codes
    #[arg(long)]
    #[arg(short)]
    #[arg(default_value_t = false)]
    show_emoji: bool,

    /// Show help message with usage examples
    #[arg(long)]
    #[arg(short = 'm')]
    #[arg(default_value_t = false)]
    help_message: bool,
}

fn print_help_message() {
    println!(
        r#"
{BOLD}{CYAN}🚀 Auto-Gitmoji (amoji) - Smart Commit Message Enhancement{RESET}

{BOLD}{YELLOW}USAGE EXAMPLES:{RESET}
  {GREEN}amoji{RESET} {DIM}"{RESET}add new feature for users{DIM}"{RESET}           {DIM}# ✨ :sparkles: add new feature for users{RESET}
  {GREEN}amoji{RESET} {DIM}"{RESET}fix login validation bug{DIM}"{RESET} {BLUE}--dry-run{RESET}  {DIM}# Preview: 🐛 :bug: fix login validation bug{RESET}
  {GREEN}amoji{RESET} {BLUE}--show-emoji{RESET}                          {DIM}# List all available gitmojis{RESET}
  {GREEN}amoji{RESET} {BLUE}--help-message{RESET}                        {DIM}# Show this help with examples{RESET}

{BOLD}{YELLOW}SUPPORTED COMMIT TYPES{RESET} {DIM}(partial list):{RESET}
  {MAGENTA}•{RESET} {BOLD}Features:{RESET}     {CYAN}add, create, implement, introduce{RESET} → ✨ {DIM}:sparkles:{RESET}
  {MAGENTA}•{RESET} {BOLD}Bug Fixes:{RESET}    {CYAN}fix, repair, resolve, correct{RESET} → 🐛 {DIM}:bug:{RESET}
  {MAGENTA}•{RESET} {BOLD}Hotfixes:{RESET}     {CYAN}hotfix, urgent{RESET} → 🚑️ {DIM}:ambulance:{RESET}
  {MAGENTA}•{RESET} {BOLD}Docs:{RESET}         {CYAN}docs, documentation, readme{RESET} → 📝 {DIM}:memo:{RESET}
  {MAGENTA}•{RESET} {BOLD}Refactoring:{RESET}  {CYAN}refactor, restructure, cleanup{RESET} → ♻️ {DIM}:recycle:{RESET}
  {MAGENTA}•{RESET} {BOLD}Performance:{RESET}  {CYAN}optimize, performance, speed{RESET} → ⚡ {DIM}:zap:{RESET}
  {MAGENTA}•{RESET} {BOLD}Tests:{RESET}        {CYAN}test, testing, spec{RESET} → 🧪 {DIM}:test_tube:{RESET}
  {MAGENTA}•{RESET} {BOLD}Styling:{RESET}      {CYAN}style, format, lint{RESET} → 💄 {DIM}:lipstick:{RESET}
  {MAGENTA}•{RESET} {BOLD}Dependencies:{RESET} {CYAN}deps, dependency, package{RESET} → 📦 {DIM}:package:{RESET}
  {MAGENTA}•{RESET} {BOLD}Security:{RESET}     {CYAN}security, vulnerability, auth{RESET} → 🔒 {DIM}:lock:{RESET}

{BOLD}{YELLOW}HOW IT WORKS:{RESET}
  {BRIGHT_CYAN}1.{RESET} Analyzes the {BOLD}first word{RESET} of your commit message
  {BRIGHT_CYAN}2.{RESET} Matches it against {BOLD}200+{RESET} keywords
  {BRIGHT_CYAN}3.{RESET} Prepends the appropriate gitmoji
  {BRIGHT_CYAN}4.{RESET} Executes: {GREEN}git commit -m{RESET} {DIM}":emoji: your message"{RESET}

{BOLD}{BRIGHT_YELLOW}💡 TIP:{RESET} Use {BLUE}--dry-run{RESET} to preview before committing!
"#,
    );
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle help message flag
    if args.help_message {
        print_help_message();
        return Ok(());
    }

    // Handle show-emoji flag
    if args.show_emoji {
        println!("{BOLD}{CYAN}Available gitmoji codes:{RESET}");
        let mut codes = EmojiLookup::all_codes();
        codes.sort();
        for code in codes {
            if let Some(unicode) = EmojiLookup::code_to_unicode(code) {
                println!("  {unicode} {DIM}{code}{RESET}");
            }
        }
        return Ok(());
    }

    // Message is required when not showing emoji or help
    let message = args
        .message
        .ok_or_else(|| anyhow::anyhow!("Message argument is required"))?;

    // Create matcher and find appropriate emoji
    let matcher = if cfg!(feature = "llm") {
        #[cfg(feature = "llm")]
        {
            // just SiliconFlow is supported now
            MatcherFactory::llm_with_fallback(LLMConfig::from_env(
                LLMProvider::SiliconFlow,
                LLMModel::Qwen2_7bInstruct,
            )?)
        }
        #[cfg(not(feature = "llm"))]
        {
            MatcherFactory::simple()
        }
    } else {
        MatcherFactory::simple()
    };
    let match_result = matcher.match_emoji(&message)?;

    if let Some((emoji_code, formatted_message)) = match_result {
        // Get the emoji unicode for display
        let emoji_unicode = EmojiLookup::code_to_unicode(&emoji_code).unwrap_or("❓");

        // Display the emoji and message to user with enhanced formatting
        println!("{BOLD}{GREEN}🎯 Matched emoji:{RESET} {emoji_unicode} {DIM}{emoji_code}{RESET}",);
        println!("{BOLD}{BLUE}📝 Full message:{RESET} {formatted_message}");

        // Check for staged changes before committing
        if !args.dry_run {
            match GitCommit::has_staged_changes() {
                Ok(true) => {
                    println!("{BRIGHT_GREEN}✅ Staged changes detected{RESET}");
                }
                Ok(false) => {
                    println!(
                        "{YELLOW}⚠️  No staged changes found. Please stage your changes first with {BOLD}'git add'{RESET}",
                    );
                    return Ok(());
                }
                Err(e) => {
                    println!("{YELLOW}⚠️  Could not check Git status: {e}{RESET}");
                    println!("   {DIM}Proceeding anyway...{RESET}");
                }
            }
        }

        // Execute the commit
        match GitCommit::commit(&formatted_message, args.dry_run) {
            Ok(result) => {
                println!("{BRIGHT_GREEN}✅ {RESET} {result}");
            }
            Err(e) => {
                eprintln!("{RED}❌ Commit failed: {RESET} {e}");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{RED}❌ Could not find appropriate emoji for message{RESET}",);
        std::process::exit(1);
    }

    Ok(())
}
