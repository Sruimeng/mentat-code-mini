# Mentat Code Mini

<div align="center">

**你的 AI 编程助手 - Rust 驱动的命令行工具**

[![GitHub - Sruimeng/mentat-code-mini](https://img.shields.io/badge/GitHub-Sruimeng%2Fmentat--code--mini-blue?logo=github)](https://github.com/Sruimeng/mentat-code-mini)
[![npm](https://img.shields.io/npm/v/@sruim/mentat-code-mini)](https://www.npmjs.com/package/@sruim/mentat-code-mini)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md) | [简体中文](README.zh-CN.md)

</div>

---

## 概述

Mentat Code Mini 是一个用 Rust 构建的轻量级 AI 编程助手命令行工具。它提供交互式 REPL 界面与 Claude AI 对话，并内置文件操作工具。

### 核心特性

- 🤖 **交互式 REPL** - 在终端中与 Claude AI 对话
- 🔧 **内置工具** - 带路径验证的文件读写操作
- ⚡ **快速轻量** - Rust 驱动，性能优异
- 🔒 **安全可靠** - 路径验证防止未授权文件访问
- 📦 **安装简单** - 支持 npm 或 cargo 安装

---

## 安装

### 通过 npm（推荐）

```bash
npm install -g @sruim/mentat-code-mini
```

### 通过 Cargo

```bash
cargo install mentat-code-mini
```

### 从源码构建

```bash
git clone https://github.com/Sruimeng/mentat-code-mini.git
cd mentat-code-mini
cargo build --release
```

---

## 配置

### 初始化配置

```bash
mentat --init
```

这会在 `~/.mentat/config.toml` 创建配置文件。编辑它并添加你的 Anthropic API 密钥：

```toml
[env]
api_key = "your-anthropic-api-key"
base_url = "https://api.anthropic.com"
# https_proxy = "http://127.0.0.1:7890"  # 可选代理

[model]
name = "claude-sonnet-4-20250514"
```

---

## 使用方法

### 交互模式

```bash
mentat
```

这会启动 REPL 界面：

```
╔══════════════════════════════════════════════════════════╗
║                  🧠 Mentat Code v0.1.0                   ║
║                                                          ║
║  输入问题与 AI 对话，输入 /help 查看帮助                 ║
║  已加载 2 个工具                                         ║
╚══════════════════════════════════════════════════════════╝

❯ 
```

### 单命令模式

```bash
mentat -e "读取 src/main.rs 的内容"
```

### 可用命令

| 命令 | 描述 |
|------|------|
| `/help`, `/h`, `/?` | 显示帮助 |
| `/exit`, `/quit`, `/q` | 退出程序 |
| `/clear`, `/c` | 清除对话历史 |
| `/tools`, `/t` | 列出已注册的工具 |

### CLI 选项

```bash
mentat --help

选项:
  -c, --config <FILE>    配置文件路径
  -d, --debug            启用调试模式
      --log-level <LEVEL> 设置日志级别 (error, warn, info, debug, trace) [默认: info]
  -e, --execute <PROMPT> 执行单条命令后退出
      --init             初始化配置文件
  -h, --help             显示帮助
  -V, --version          显示版本
```

---

## 内置工具

| 工具 | 描述 |
|------|------|
| `read_file` | 读取文件内容（带路径验证） |
| `write_file` | 写入文件内容（带路径验证） |

---

## 开发

### 前置要求

- Rust 1.70+（Edition 2021）
- Cargo

### 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 格式化代码
cargo fmt

# Lint 检查
cargo clippy
```

### 项目结构

```
mentat-code-mini/
├── Cargo.toml           # 项目配置
├── src/
│   ├── main.rs          # 入口点 & REPL
│   ├── config.rs        # 配置管理
│   └── tools/           # 工具实现
│       ├── mod.rs       # 工具注册表
│       ├── read_file.rs
│       ├── write_file.rs
│       └── path_validator.rs
├── npm-package/         # npm 分发包
└── llmdoc/              # LLM 文档系统
```

---

## 许可证

MIT License - 详见 [LICENSE](LICENSE)

---

<div align="center">

Made with ❤️ by **Sruimeng**

</div>
