---
id: tech-stack
type: reference
related_ids: [constitution]
---

# 技术栈文档

## 核心技术

| 类别 | 技术 | 版本 |
|------|------|------|
| 语言 | Rust | Edition 2024 |
| 构建工具 | Cargo | latest stable |
| 包管理 | Cargo | 内置 |

## 项目配置

### Cargo.toml
```toml
[package]
name = "mentat-code"
version = "0.1.0"
edition = "2024"

[dependencies]
# 当前无依赖
```

## 构建命令

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行
cargo run

# 测试
cargo test

# 格式化
cargo fmt

# Lint 检查
cargo clippy
```

## 项目结构

```
mentat-code/
├── Cargo.toml       # 项目配置
├── Cargo.lock       # 依赖锁定
├── src/
│   ├── main.rs      # 入口点
│   └── type/        # 类型定义目录（空）
├── target/          # 构建输出
├── llmdoc/          # LLM 文档系统
└── .claude/         # Claude Code 配置
```

## 开发工具

### 推荐 IDE 扩展
- rust-analyzer
- CodeLLDB (调试)
- crates (依赖管理)

### 代码质量工具
- `cargo fmt` - 代码格式化
- `cargo clippy` - Lint 检查
- `cargo test` - 测试运行
- `cargo doc` - 文档生成
