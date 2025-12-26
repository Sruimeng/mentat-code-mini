---
id: doc-standard
type: guide
related_ids: []
---

# LLMDoc 文档标准

## 核心原则

### 1. Frontmatter 必需
所有文档必须包含 YAML frontmatter：
```yaml
---
id: unique-identifier
type: reference | guide | architecture | agent
related_ids: [other-doc-id]
---
```

### 2. Type-First 原则
先定义接口/类型，再描述逻辑：
```rust
// 先定义类型
struct Config {
    name: String,
    version: String,
}

// 后描述行为
impl Config { ... }
```

### 3. Pseudocode > Prose
用伪代码替代长段落描述：

```
// BAD: 长段落描述
"该函数首先检查输入是否为空，然后遍历所有元素..."

// GOOD: 伪代码
fn process(input):
  IF input.is_empty() THEN return Error
  FOR item IN input DO
    validate(item)
  RETURN Ok
```

### 4. 负向约束优先
明确列出 "禁止事项"：

```markdown
## 禁止事项 (DO NOT)
- ❌ 不要在此模块中直接操作 IO
- ❌ 不要忽略错误返回值
- ❌ 不要使用 unwrap() 在生产代码中
```

## 文档类型

| Type | 目录 | 用途 |
|------|------|------|
| `reference` | `/llmdoc/reference/` | 宪法级规范（技术栈、约定、规则） |
| `guide` | `/llmdoc/guides/` | 操作指南（如何做某事） |
| `architecture` | `/llmdoc/architecture/` | 系统架构图、数据流 |
| `agent` | `/llmdoc/agent/` | 策略文档、任务记录 |

## 文件命名

- 使用 kebab-case: `tech-stack.md`, `data-models.md`
- 避免缩写: `configuration.md` 而非 `config.md`
- 保持简洁: 最多3个单词
