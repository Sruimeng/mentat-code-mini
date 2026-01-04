//! read_file 工具 - 读取文件内容

use super::path_validator::PathValidator;
use super::Tool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

/// read_file 工具的输入参数
#[derive(Debug, Deserialize)]
pub struct ReadFileInput {
    pub file_path: String,
}

/// read_file 工具的输出结果
#[derive(Debug, Serialize)]
pub struct ReadFileOutput {
    pub success: bool,
    pub content: Option<String>,
    pub error: Option<String>,
}

/// ReadFile 工具实现
pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn definition(&self) -> Value {
        serde_json::json!({
            "name": "read_file",
            "description": "Read the contents of a file at the specified path. Use this to examine source code, configuration files, or any text file.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The path to the file to read (relative or absolute)"
                    }
                },
                "required": ["file_path"]
            }
        })
    }

    fn execute(&self, input: &Value) -> String {
        let tool_input: ReadFileInput = match serde_json::from_value(input.clone()) {
            Ok(input) => input,
            Err(e) => {
                return serde_json::to_string(&ReadFileOutput {
                    success: false,
                    content: None,
                    error: Some(format!("Invalid input: {}", e)),
                })
                .unwrap()
            }
        };

        let result = execute_read_file(&tool_input);
        serde_json::to_string(&result).unwrap()
    }
}

/// 执行文件读取
fn execute_read_file(input: &ReadFileInput) -> ReadFileOutput {
    // 创建路径验证器
    let validator = match PathValidator::new() {
        Ok(v) => v,
        Err(e) => {
            return ReadFileOutput {
                success: false,
                content: None,
                error: Some(format!("Failed to initialize path validator: {}", e)),
            };
        }
    };

    // 安全检查：验证路径
    let validated_path = match validator.validate_for_read(&input.file_path) {
        Ok(p) => p,
        Err(e) => {
            return ReadFileOutput {
                success: false,
                content: None,
                error: Some(e.to_string()),
            };
        }
    };

    // 读取文件
    match fs::read_to_string(&validated_path) {
        Ok(content) => ReadFileOutput {
            success: true,
            content: Some(content),
            error: None,
        },
        Err(e) => ReadFileOutput {
            success: false,
            content: None,
            error: Some(format!("Failed to read file: {}", e)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_existing_file() {
        let tool = ReadFileTool;
        let input = serde_json::json!({"file_path": "Cargo.toml"});
        let result = tool.execute(&input);
        assert!(result.contains("success"));
        assert!(result.contains("[package]"));
    }

    #[test]
    fn test_read_nonexistent_file() {
        let tool = ReadFileTool;
        let input = serde_json::json!({"file_path": "nonexistent.txt"});
        let result = tool.execute(&input);
        assert!(result.contains("\"success\":false"));
    }

    #[test]
    fn test_path_traversal_blocked() {
        let tool = ReadFileTool;
        let input = serde_json::json!({"file_path": "../etc/passwd"});
        let result = tool.execute(&input);
        assert!(result.contains("traversal") || result.contains("not allowed"));
    }

    #[test]
    fn test_absolute_path_blocked() {
        let tool = ReadFileTool;
        let input = serde_json::json!({"file_path": "/etc/passwd"});
        let result = tool.execute(&input);
        assert!(result.contains("Absolute") || result.contains("not allowed"));
    }

    #[test]
    fn test_nested_traversal_blocked() {
        let tool = ReadFileTool;
        let input = serde_json::json!({"file_path": "src/../../../etc/passwd"});
        let result = tool.execute(&input);
        assert!(result.contains("traversal") || result.contains("not allowed"));
    }
}
