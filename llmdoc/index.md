---
id: index
type: index
related_ids: []
---

# mentat-code-mini 文档索引

> LLM 优化的代码库文档系统

## 快速导航

### 🏛️ 必读文档 (The Constitution)

| 文档 | 描述 | 优先级 |
|------|------|--------|
| [**constitution.md**](./reference/constitution.md) | **项目宪法** - 核心规则与约定 | ⭐⭐⭐ |
| [doc-standard.md](./guides/doc-standard.md) | 文档编写标准 | ⭐⭐ |

---

## 📚 文档目录

### Reference (参考文档)
规范性文档，定义"是什么"和"为什么"。

| 文档 | ID | 描述 |
|------|-----|------|
| [constitution.md](./reference/constitution.md) | `constitution` | 项目宪法：规则与禁止模式 |
| [tech-stack.md](./reference/tech-stack.md) | `tech-stack` | 技术栈：语言、工具、依赖 |
| [data-models.md](./reference/data-models.md) | `data-models` | 数据模型定义 |
| [shared-utilities.md](./reference/shared-utilities.md) | `shared-utilities` | 共享工具函数清单 |

### Architecture (架构文档)
系统设计与数据流。

| 文档 | ID | 描述 |
|------|-----|------|
| [system-overview.md](./architecture/system-overview.md) | `system-overview` | 系统架构概览 |

### Guides (操作指南)
程序性文档，描述"如何做"。

| 文档 | ID | 描述 |
|------|-----|------|
| [doc-standard.md](./guides/doc-standard.md) | `doc-standard` | LLMDoc 文档编写标准 |

### Agent (策略文档)
任务相关的策略与记录。

| 文档 | ID | 描述 |
|------|-----|------|
| *(空)* | - | 待创建 |

---

## 📂 目录结构

```
llmdoc/
├── index.md              # 本文件 - 文档入口
├── architecture/         # 架构设计
│   └── system-overview.md
├── guides/               # 操作指南
│   └── doc-standard.md
├── reference/            # 规范文档
│   ├── constitution.md   # ⭐ 项目宪法
│   ├── tech-stack.md
│   ├── data-models.md
│   └── shared-utilities.md
└── agent/                # 策略文档
    └── (空)
```

---

## 🔍 按场景查找

### "我需要了解项目规则"
→ [constitution.md](./reference/constitution.md)

### "我需要了解技术栈"
→ [tech-stack.md](./reference/tech-stack.md)

### "我需要添加新功能"
1. 先读 [constitution.md](./reference/constitution.md)
2. 检查 [shared-utilities.md](./reference/shared-utilities.md)
3. 参考 [system-overview.md](./architecture/system-overview.md)

### "我需要写文档"
→ [doc-standard.md](./guides/doc-standard.md)

---

## 📊 项目状态

| 指标 | 值 |
|------|-----|
| 项目名称 | mentat-code-mini |
| 语言 | Rust (Edition 2024) |
| 当前版本 | 0.1.0 |
| 开发阶段 | 初始化 |
| 核心依赖 | 无 |

---

*最后更新: 2024-12-26*
