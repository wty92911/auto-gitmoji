# Project Brief: auto-gitmoji

## 🎯 Project Overview
`auto-gitmoji` (alias: `amoji`) is a command-line tool written in **Rust** that automatically prepends appropriate gitmoji to Git commit messages based on intelligent content analysis.

## 🚀 Core Functionality
- **Input**: Single string argument (commit message)
- **Process**: Analyzes message content and selects appropriate gitmoji
- **Output**: Executes `git commit -m ":emoji: message"` and displays rendered emoji in terminal

## 📋 Key Requirements
1. CLI tool accepting single string argument
2. Smart gitmoji matching (keyword, semantic, or LLM-based)
3. Automatic Git commit execution
4. Terminal display with rendered emoji
5. Modular matcher architecture for extensibility

## 🎯 Success Criteria
- Tool correctly matches appropriate gitmoji for various commit types
- Seamless Git workflow integration
- Clear terminal feedback with emoji rendering
- Extensible architecture for different matching strategies

## 🔧 Technical Stack
- **Language**: Rust
- **CLI Framework**: clap or structopt
- **Matching Strategies**: Simple (keyword), Semantic (ML), LLM (API)
- **Git Integration**: Process execution via Command

## 📦 Project Structure
```
amoji/
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── matcher/
│   │   ├── mod.rs
│   │   ├── simple.rs
│   │   ├── semantic.rs
│   │   └── llm.rs
│   ├── commit.rs
│   ├── emoji.rs
│   └── utils.rs
├── Cargo.toml
└── README.md
```

## 🧠 Current Status
- **Phase**: Initial analysis and setup
- **Next Steps**: Architecture planning and implementation strategy 