---
id: shared-utilities
type: reference
related_ids: [constitution, tech-stack]
---

# 共享工具函数文档

> 本文档记录可复用的工具函数，防止重复实现。

## 当前状态

> ⚠️ 项目处于初始阶段，尚未实现工具函数。

## 预期目录结构

```
src/
├── utils/
│   ├── mod.rs       # 模块导出
│   ├── string.rs    # 字符串处理
│   ├── path.rs      # 路径处理
│   ├── io.rs        # IO 操作
│   └── error.rs     # 错误处理
└── common/
    ├── mod.rs       # 模块导出
    ├── constants.rs # 常量定义
    └── types.rs     # 通用类型
```

## 工具函数清单

### 字符串工具 (计划)
| 函数 | 描述 | 状态 |
|------|------|------|
| `trim_indent` | 去除多行字符串缩进 | 🔲 待实现 |
| `slugify` | 转换为 URL 友好格式 | 🔲 待实现 |

### 路径工具 (计划)
| 函数 | 描述 | 状态 |
|------|------|------|
| `normalize_path` | 规范化路径 | 🔲 待实现 |
| `relative_to` | 计算相对路径 | 🔲 待实现 |

### IO 工具 (计划)
| 函数 | 描述 | 状态 |
|------|------|------|
| `read_file_safe` | 安全读取文件 | 🔲 待实现 |
| `write_atomic` | 原子写入 | 🔲 待实现 |

## 禁止重复实现

> 在实现新功能前，先检查本清单！

### 检查流程
```
1. 搜索本文档中的函数列表
2. 搜索 src/utils/ 目录
3. 检查依赖库是否已提供
4. 如确需新函数，先更新本文档
```

## 依赖优先原则

优先使用成熟的 crate 而非自行实现：

| 需求 | 推荐 crate |
|------|-----------|
| 日志 | `tracing`, `log` |
| 错误处理 | `thiserror`, `anyhow` |
| 序列化 | `serde`, `serde_json` |
| 异步运行时 | `tokio` |
| HTTP 客户端 | `reqwest` |
| 命令行解析 | `clap` |
