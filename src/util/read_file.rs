use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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

/// 执行文件读取
pub fn execute(input: &ReadFileInput) -> ReadFileOutput {
    let path = Path::new(&input.file_path);

    // 安全检查：禁止读取敏感路径
    if input.file_path.contains("..") {
        return ReadFileOutput {
            success: false,
            content: None,
            error: Some("Path traversal not allowed".to_string()),
        };
    }

    match fs::read_to_string(path) {
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

/// 返回 read_file 工具的 JSON Schema 定义（用于 Anthropic API）
pub fn tool_definition() -> serde_json::Value {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_existing_file() {
        let input = ReadFileInput {
            file_path: "Cargo.toml".to_string(),
        };
        let result = execute(&input);
        assert!(result.success);
        assert!(result.content.is_some());
        assert!(result.content.unwrap().contains("[package]"));
    }

    #[test]
    fn test_read_nonexistent_file() {
        let input = ReadFileInput {
            file_path: "nonexistent.txt".to_string(),
        };
        let result = execute(&input);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_path_traversal_blocked() {
        let input = ReadFileInput {
            file_path: "../etc/passwd".to_string(),
        };
        let result = execute(&input);
        assert!(!result.success);
        assert!(result.error.unwrap().contains("traversal"));
    }
}
