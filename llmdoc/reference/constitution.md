---
id: constitution
type: reference
related_ids: [tech-stack, doc-standard]
---

# 项目宪法 (Constitution)

> 本文档定义项目的核心规则与约定，所有开发必须遵守。

## 1. 技术约定

### 1.1 语言与版本
- **语言**: Rust
- **Edition**: 2024
- **最低支持版本**: stable

### 1.2 错误处理标准
```rust
// REQUIRED: 使用 Result 类型
fn operation() -> Result<T, Error>

// FORBIDDEN: 生产代码中使用 unwrap/expect
value.unwrap()  // ❌ 禁止
value.expect("msg")  // ❌ 禁止

// ALLOWED: 测试代码中可使用
#[cfg(test)]
fn test_case() {
    value.unwrap();  // ✅ 允许
}
```

## 2. 禁止模式 (Forbidden Patterns)

### 2.1 内存安全
- ❌ 不要使用 `unsafe` 块，除非有明确的安全注释
- ❌ 不要忽略借用检查器警告
- ❌ 不要使用裸指针操作

### 2.2 并发安全
- ❌ 不要使用全局可变状态
- ❌ 不要在没有同步原语的情况下共享数据

### 2.3 代码风格
- ❌ 不要提交未格式化的代码 (`cargo fmt` 必须通过)
- ❌ 不要忽略 clippy 警告
- ❌ 不要使用魔术数字（使用常量）

## 3. 必需模式 (Required Patterns)

### 3.1 模块组织
```
src/
├── main.rs          # 入口点
├── lib.rs           # 库导出（如需要）
├── type/            # 类型定义
│   └── mod.rs
├── utils/           # 工具函数
│   └── mod.rs
└── [domain]/        # 领域模块
    └── mod.rs
```

### 3.2 文档注释
```rust
/// 简短描述（必需）
///
/// # Arguments
/// * `param` - 参数说明
///
/// # Returns
/// 返回值说明
///
/// # Errors
/// 可能的错误情况
pub fn public_function(param: Type) -> Result<T, E>
```

## 4. 测试约定

### 4.1 测试覆盖
- 所有公共 API 必须有单元测试
- 关键路径必须有集成测试

### 4.2 测试命名
```rust
#[test]
fn test_[function_name]_[scenario]_[expected_result]() {
    // given_when_then 格式
}
```

## 5. Git 约定

### 5.1 提交信息格式
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### 5.2 分支策略
- `main/master`: 稳定版本
- `dev`: 开发分支
- `feature/*`: 功能分支
- `fix/*`: 修复分支
