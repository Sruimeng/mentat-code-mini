---
id: system-overview
type: architecture
related_ids: [constitution, tech-stack]
---

# 系统架构概览

## 项目定位

**mentat-code** - 一个 Rust 项目（具体用途待定义）

## 当前架构

```
┌─────────────────────────────────────────────────┐
│                   mentat-code                    │
├─────────────────────────────────────────────────┤
│                                                  │
│   ┌─────────────┐                               │
│   │  main.rs    │ ← 入口点                       │
│   │             │   (Hello World)               │
│   └─────────────┘                               │
│                                                  │
│   ┌─────────────┐                               │
│   │  type/      │ ← 类型定义（空）               │
│   │             │                               │
│   └─────────────┘                               │
│                                                  │
└─────────────────────────────────────────────────┘
```

## 目录职责

| 目录 | 职责 | 当前状态 |
|------|------|---------|
| `src/main.rs` | 程序入口点 | ✅ 已实现 (Hello World) |
| `src/type/` | 核心类型定义 | 🔲 空目录 |
| `src/utils/` | 工具函数 | 🔲 未创建 |
| `src/lib.rs` | 库导出 | 🔲 未创建 |

## 建议架构演进

### Phase 1: 基础设施
```
src/
├── main.rs           # 保持入口点简洁
├── lib.rs            # 库导出
├── type/
│   └── mod.rs
├── utils/
│   └── mod.rs
└── error.rs          # 统一错误类型
```

### Phase 2: 功能模块
```
src/
├── main.rs
├── lib.rs
├── type/
├── utils/
├── error.rs
├── core/             # 核心业务逻辑
│   └── mod.rs
├── parser/           # 解析器（如需要）
│   └── mod.rs
└── output/           # 输出格式化
    └── mod.rs
```

## 数据流

```
[输入] → [解析] → [处理] → [输出]
   │        │        │        │
   ▼        ▼        ▼        ▼
 CLI    Parser    Core    Formatter
```

## 入口点

### 当前入口
```rust
// src/main.rs
fn main() {
    println!("Hello, world!");
}
```

### 建议入口
```rust
// src/main.rs
use mentat_code::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()
}
```

## 外部接口

> 当前项目无外部接口，纯命令行程序。

### 计划接口
| 类型 | 描述 | 状态 |
|------|------|------|
| CLI | 命令行接口 | 🔲 待设计 |
| Library | 作为库被引用 | 🔲 待设计 |
