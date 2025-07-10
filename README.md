# Auto-Gitmoji

A Rust CLI tool that automatically prepends appropriate gitmoji to your commit messages based on intelligent keyword matching.

## ✨ Features

- **Intelligent Matching**: Uses first-word keyword matching with 200+ keyword mappings
- **Comprehensive Emoji Support**: All 69 official gitmojis from the gitmoji standard
- **Git Integration**: Seamlessly integrates with your Git workflow
- **Dry Run Mode**: Preview commits before executing
- **Emoji Display**: View all available gitmojis
- **Robust Error Handling**: Validates staged changes and Git repository status

### Future Features
- [ ] Use LLM to predict the emoji
- [ ] Better error info
## 🚀 Installation

### Prerequisites

- Rust 1.70+ with Cargo
- Git installed and configured

### From Source

```bash
git clone https://github.com/yourusername/auto-gitmoji.git
cd auto-gitmoji
cargo build --release
cargo install --path .
```

### From Crates.io

```bash
cargo install auto-gitmoji
```

## 📋 Usage

### Basic Usage

```bash
# Commit with automatic gitmoji selection
amoji "add new user authentication feature"
# Output: ✨ :sparkles: add new user authentication feature

# Preview without committing (dry run)
amoji "fix login validation bug" --dry-run
# Output: 🐛 :bug: fix login validation bug

# Show all available emojis
amoji --show-emoji
```

### Complete Examples

```bash
# Feature additions
amoji "add user profile page"
# Result: ✨ :sparkles: add user profile page

# Bug fixes
amoji "fix memory leak in data processor"
# Result: 🐛 :bug: fix memory leak in data processor

# Documentation
amoji "docs update installation guide"
# Result: 📝 :memo: docs update installation guide

# Refactoring
amoji "refactor authentication module"
# Result: ♻️ :recycle: refactor authentication module

# Performance improvements
amoji "optimize database queries"
# Result: ⚡ :zap: optimize database queries

# Security fixes
amoji "hotfix critical vulnerability in auth"
# Result: 🚑️ :ambulance: hotfix critical vulnerability in auth
```

## 🎯 How It Works

### First-Word Matching Strategy

1. **Split**: Commit message split into words by whitespace
2. **Filter**: Keep only alphanumeric words (+ hyphens/underscores)
3. **Match**: Find first word that exists in keyword map

### Keyword Categories Example

- **Feature**: `new`, `create`, `implement`, `introduce`
- **Fixes**: `fix`, `repair`, `resolve`, `correct`, `hotfix`
- **Documentation**: `docs`, `documentation`, `readme`, `comment`
- **Refactoring**: `refactor`, `restructure`, `reorganize`, `cleanup`
- **Performance**: `optimize`, `performance`, `speed`, `cache`
- **Testing**: `test`, `testing`, `spec`, `coverage`
- **Security**: `security`, `vulnerability`, `auth`, `permission`
- **Styling**: `style`, `format`, `lint`, `prettier`
- **Dependencies**: `deps`, `dependency`, `package`, `upgrade`
- **Configuration**: `config`, `configuration`, `settings`, `env`

### Emoji Mapping

Uses the complete official gitmoji specification with 69 different emojis covering all common development activities. The tool loads emojis from JSON first, with a hardcoded fallback for reliability.

## 🛠️ CLI Options

```bash
amoji [OPTIONS] [MESSAGE]

ARGUMENTS:
  [MESSAGE]  The commit message

OPTIONS:
      --dry-run      Show what would be committed without actually committing
      --show-emoji   Show available emoji codes
  -h, --help         Print help
  -V, --version      Print version
```

## 🔧 Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running

```bash
cargo run -- "your commit message"
```

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
