# Auto-Gitmoji 🚀

A Rust CLI tool that automatically prepends appropriate gitmoji to your commit messages based on intelligent keyword matching.

## ✨ Features

- **🎯 Simple KeywordMatching**: Enhanced first-word keyword matching with 200+ keyword mappings
- **🎯 LLM KeywordMatching**: AI-powered matching with LLM(now only support SiliconFlow)
- **🎨 Comprehensive Emoji Support**: All 69 official gitmojis from the gitmoji standard
- **⚡ Git Integration**: Seamless integration with your Git workflow
- **👀 Dry Run Mode**: Preview commits before executing
- **📋 Emoji Display**: View all available gitmojis with Unicode rendering
- **🛡️ Robust Error Handling**: Validates staged changes and Git repository status
- **🧪 Comprehensive Testing**: 65 tests covering unit, integration, and edge cases
- **📊 Enhanced CLI Experience**: ANSI colors, progress indicators, and user-friendly output

### 🔮 Future Features
- [ ] Support for more LLM APIs
- [ ] Custom keyword mapping configuration
- [ ] Integration with commit message templates

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
# or with your own API_KEY
cargo install auto-gitmoji --features llm
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

# Get detailed help with examples
amoji --help-message
```

### Complete Examples

```bash
# Feature additions
amoji "add user profile page"
# Result: ✨ :sparkles: add user profile page

# Bug fixes
amoji "fix memory leak in data processor"
# Result: 🐛 :bug: fix memory leak in data processor

# Hotfixes
amoji "hotfix critical authentication vulnerability"
# Result: 🚑️ :ambulance: hotfix critical authentication vulnerability

# Documentation
amoji "docs update installation guide"
# Result: 📝 :memo: docs update installation guide

# Refactoring
amoji "refactor authentication module"
# Result: ♻️ :recycle: refactor authentication module

# Performance improvements
amoji "optimize database query performance"
# Result: ⚡ :zap: optimize database query performance

# Testing
amoji "test user registration flow"
# Result: 🧪 :test_tube: test user registration flow

# Dependencies
amoji "update package dependencies"
# Result: 📦 :package: update package dependencies

# Security
amoji "security fix for JWT validation"
# Result: 🔒 :lock: security fix for JWT validation
```

## 🎯 How It Works

### Enhanced Keyword Matching Strategy

1. **🔍 Analysis**: Commit message analyzed word by word
2. **🧹 Cleaning**: Non-alphanumeric characters replaced with spaces for better word extraction
3. **📝 Extraction**: Words split by whitespace and normalized to lowercase
4. **🎯 Matching**: First word matched against comprehensive keyword database
5. **✨ Formatting**: Complete commit message formatted with appropriate gitmoji

### Keyword Categories

| Category | Keywords | Emoji |
|----------|----------|-------|
| **Features** | `add`, `new`, `create`, `implement`, `introduce`, `feat` | ✨ `:sparkles:` |
| **Bug Fixes** | `fix`, `repair`, `resolve`, `correct`, `patch` | 🐛 `:bug:` |
| **Hotfixes** | `hotfix`, `urgent`, `critical` | 🚑️ `:ambulance:` |
| **Documentation** | `docs`, `documentation`, `readme`, `comment` | 📝 `:memo:` |
| **Refactoring** | `refactor`, `restructure`, `reorganize`, `cleanup` | ♻️ `:recycle:` |
| **Performance** | `optimize`, `performance`, `speed`, `cache`, `perf` | ⚡ `:zap:` |
| **Testing** | `test`, `testing`, `spec`, `coverage` | 🧪 `:test_tube:` |
| **Security** | `security`, `vulnerability`, `auth`, `permission` | 🔒 `:lock:` |
| **Styling** | `style`, `format`, `lint`, `prettier` | 💄 `:lipstick:` |
| **Dependencies** | `deps`, `dependency`, `package`, `upgrade` | 📦 `:package:` |
| **Configuration** | `config`, `configuration`, `settings`, `env` | ⚙️ `:gear:` |

### Architecture

The tool uses a clean strategy pattern with pluggable matchers:

```rust
pub trait GitmojiMatcher {
    fn match_emoji(&self, message: &str) -> Result<MatcherResult>;
    fn name(&self) -> &'static str;
}

pub type MatcherResult = Option<(String, String)>; // (emoji_code, formatted_message)
```

**Current Matchers:**
- **SimpleMatcher**: Keyword-based matching with 200+ keywords
- **LLMMatcher**: AI-powered matching (optional feature)

## 🛠️ CLI Options

```bash
amoji [OPTIONS] [MESSAGE]

ARGUMENTS:
  [MESSAGE]  The commit message

OPTIONS:
  -d, --dry-run        Show what would be committed without actually committing
  -s, --show-emoji     Show available emoji codes
  -m, --help-message   Show help message with usage examples
  -h, --help           Print help
  -V, --version        Print version
```

## 🧪 Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test matcher::tests
```

### Running

```bash
# Development
cargo run -- "your commit message"

# With features
cargo run --features llm -- "your commit message"

# Dry run
cargo run -- "your commit message" --dry-run
```

### Project Structure

```
src/
├── main.rs              # CLI application entry point
├── lib.rs               # Library exports and integration tests
├── commit.rs            # Git commit operations
├── emoji.rs             # Emoji lookup and mapping
└── matcher/
    ├── mod.rs           # Matcher trait and factory
    ├── simple.rs        # Keyword-based matcher
    └── llm.rs           # LLM-based matcher (feature gated)
tests/
├── integration_tests.rs # Full workflow integration tests
fixtures/
├── gitmojis.json        # Official gitmoji data (69 emojis)
└── keyword_map.json     # Keyword to emoji mappings (200+ keywords)
```

## 🔧 Features & Configuration

### Optional Features

```toml
[features]
default = []
llm = ["reqwest", "tokio", "dotenvy"]
```

To enable LLM support:
```bash
cargo build --features llm
```

### Environment Variables

```bash
# For LLM feature
export API_KEY="your-api-key"
```

## 📊 Quality Metrics

- **Test Coverage**: 65 tests (56 unit + 9 integration)
- **Performance**: < 100ms for typical operations
- **Reliability**: Comprehensive error handling and edge case coverage
- **Compatibility**: Works with all major terminals and Git workflows

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

### Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust 2024 edition standards
- Add tests for new functionality

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Gitmoji](https://gitmoji.dev/) for the comprehensive emoji standard
- The Rust community for excellent tooling and libraries
- Contributors and users who provide feedback and improvements
