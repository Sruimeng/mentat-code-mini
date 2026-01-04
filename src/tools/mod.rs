//! 工具管理系统
//!
//! 提供统一的 Tool trait 和 ToolRegistry 用于管理所有可用工具。

mod path_validator;
mod read_file;
mod write_file;

// PathValidator 和 PathValidationError 在内部使用，不需要公开导出

use serde_json::Value;
use std::collections::HashMap;

/// 工具 trait - 所有工具必须实现此接口
pub trait Tool: Send + Sync {
    /// 工具名称
    fn name(&self) -> &'static str;

    /// 工具的 JSON Schema 定义（用于 Anthropic API）
    fn definition(&self) -> Value;

    /// 执行工具
    fn execute(&self, input: &Value) -> String;
}

/// 工具注册表 - 管理所有可用工具
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    /// 创建新的工具注册表
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// 创建并注册所有内置工具
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(read_file::ReadFileTool));
        registry.register(Box::new(write_file::WriteFileTool));
        registry
    }

    /// 注册一个工具
    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// 获取所有工具的定义（用于 API 请求）
    pub fn definitions(&self) -> Vec<Value> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// 执行指定工具
    pub fn execute(&self, name: &str, input: &Value) -> String {
        match self.tools.get(name) {
            Some(tool) => tool.execute(input),
            None => format!(r#"{{"error": "Unknown tool: {}"}}"#, name),
        }
    }

    /// 获取已注册的工具数量
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// 获取所有工具名称
    pub fn tool_names(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_builtins() {
        let registry = ToolRegistry::with_builtins();
        assert_eq!(registry.len(), 2);
        assert!(registry.tool_names().contains(&"read_file"));
        assert!(registry.tool_names().contains(&"write_file"));
    }

    #[test]
    fn test_execute_unknown_tool() {
        let registry = ToolRegistry::new();
        let result = registry.execute("unknown", &Value::Null);
        assert!(result.contains("Unknown tool"));
    }
}
