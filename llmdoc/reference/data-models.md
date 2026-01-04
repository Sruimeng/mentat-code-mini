---
id: data-models
type: reference
related_ids: [constitution, tech-stack]
---

# 数据模型文档

## 当前状态

> ⚠️ 项目处于初始阶段，尚未定义核心数据模型。

## 类型定义目录

```
src/type/
└── (空 - 待实现)
```

## 预期模型结构

根据项目名称 `mentat-code`，建议的核心类型：

```rust
// src/type/mod.rs

/// 核心领域类型导出
pub mod code;
pub mod analysis;

// src/type/code.rs

/// 代码单元表示
pub struct CodeUnit {
    pub id: String,
    pub content: String,
    pub language: Language,
    pub metadata: Metadata,
}

/// 支持的编程语言
pub enum Language {
    Rust,
    Python,
    TypeScript,
    // ...
}

/// 代码元数据
pub struct Metadata {
    pub file_path: PathBuf,
    pub line_range: (usize, usize),
    pub created_at: DateTime,
}
```

## 命名约定

### 类型命名
- 结构体: PascalCase (`CodeUnit`, `Metadata`)
- 枚举: PascalCase (`Language`, `ErrorKind`)
- 枚举变体: PascalCase (`Rust`, `Python`)
- 常量: SCREAMING_SNAKE_CASE (`MAX_SIZE`, `DEFAULT_TIMEOUT`)

### 字段命名
- 使用 snake_case
- 布尔字段使用 `is_`, `has_`, `can_` 前缀

## 序列化约定

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // JSON 使用 camelCase
pub struct ApiResponse<T> {
    pub data: T,
    pub error_code: Option<String>,
}
```
