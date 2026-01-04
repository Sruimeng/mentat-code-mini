# Mentat Code Mini

<div align="center">

**Your AI Coding Agent - A Rust-powered CLI tool**

[![GitHub - Sruimeng/mentat-code-mini](https://img.shields.io/badge/GitHub-Sruimeng%2Fmentat--code--mini-blue?logo=github)](https://github.com/Sruimeng/mentat-code-mini)
[![npm](https://img.shields.io/npm/v/@sruim/mentat-code-mini)](https://www.npmjs.com/package/@sruim/mentat-code-mini)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md) | [简体中文](README.zh-CN.md)

</div>

---

## Overview

Mentat Code Mini is a lightweight AI coding assistant CLI tool built with Rust. It provides an interactive REPL interface to chat with Claude AI, with built-in tools for file operations.

### Key Features

- 🤖 **Interactive REPL** - Chat with Claude AI in your terminal
- 🔧 **Built-in Tools** - File read/write operations with path validation
- ⚡ **Fast & Lightweight** - Rust-powered for optimal performance
- 🔒 **Secure** - Path validation prevents unauthorized file access
- 📦 **Easy Installation** - Available via npm or cargo

---

## Installation

### Via npm (Recommended)

```bash
npm install -g @sruim/mentat-code-mini
```

### Via Cargo

```bash
cargo install mentat-code-mini
```

### From Source

```bash
git clone https://github.com/Sruimeng/mentat-code-mini.git
cd mentat-code-mini
cargo build --release
```

---

## Configuration

### Initialize Config

```bash
mentat --init
```

This creates a configuration file at `~/.mentat/config.toml`. Edit it to add your Anthropic API key:

```toml
[env]
api_key = "your-anthropic-api-key"
base_url = "https://api.anthropic.com"
# https_proxy = "http://127.0.0.1:7890"  # Optional proxy

[model]
name = "claude-sonnet-4-20250514"
```

---

## Usage

### Interactive Mode

```bash
mentat
```

This starts the REPL interface:

```
╔══════════════════════════════════════════════════════════╗
║                  🧠 Mentat Code v0.1.0                   ║
║                                                          ║
║  输入问题与 AI 对话，输入 /help 查看帮助                 ║
║  已加载 2 个工具                                         ║
╚══════════════════════════════════════════════════════════╝

❯ 
```

### Single Command Mode

```bash
mentat -e "Read the contents of src/main.rs"
```

### Available Commands

| Command | Description |
|---------|-------------|
| `/help`, `/h`, `/?` | Show help |
| `/exit`, `/quit`, `/q` | Exit program |
| `/clear`, `/c` | Clear conversation history |
| `/tools`, `/t` | List registered tools |

### CLI Options

```bash
mentat --help

Options:
  -c, --config <FILE>    Config file path
  -d, --debug            Enable debug mode
      --log-level <LEVEL> Set log level (error, warn, info, debug, trace) [default: info]
  -e, --execute <PROMPT> Execute single command and exit
      --init             Initialize config file
  -h, --help             Print help
  -V, --version          Print version
```

---

## Built-in Tools

| Tool | Description |
|------|-------------|
| `read_file` | Read file contents with path validation |
| `write_file` | Write content to file with path validation |

---

## Development

### Prerequisites

- Rust 1.70+ (Edition 2021)
- Cargo

### Build

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint check
cargo clippy
```

### Project Structure

```
mentat-code-mini/
├── Cargo.toml           # Project configuration
├── src/
│   ├── main.rs          # Entry point & REPL
│   ├── config.rs        # Configuration management
│   └── tools/           # Tool implementations
│       ├── mod.rs       # Tool registry
│       ├── read_file.rs
│       ├── write_file.rs
│       └── path_validator.rs
├── npm-package/         # npm distribution
└── llmdoc/              # LLM documentation system
```

---

## License

MIT License - see [LICENSE](LICENSE) for details.

---

<div align="center">

Made with ❤️ by **Sruimeng**

</div>
