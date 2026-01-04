//! write_file 工具 - 写入文件内容

use super::Tool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// write_file 工具的输入参数
#[derive(Debug, Deserialize)]
pub struct WriteFileInput {
    pub file_path: String,
    pub content: String,
}

/// write_file 工具的输出结果
#[derive(Debug, Serialize)]
pub struct WriteFileOutput {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

/// WriteFile 工具实现
pub struct WriteFileTool;

impl Tool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn definition(&self) -> Value {
        serde_json::json!({
            "name": "write_file",
            "description": "Write content to a file at the specified path. Creates parent directories if they don't exist. Use this to create or overwrite files.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The path to the file to write (relative or absolute)"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file"
                    }
                },
                "required": ["file_path", "content"]
            }
        })
    }

    fn execute(&self, input: &Value) -> String {
        let tool_input: WriteFileInput = match serde_json::from_value(input.clone()) {
            Ok(input) => input,
            Err(e) => {
                return serde_json::to_string(&WriteFileOutput {
                    success: false,
                    message: None,
                    error: Some(format!("Invalid input: {}", e)),
                })
                .unwrap()
            }
        };

        let result = execute_write_file(&tool_input);
        serde_json::to_string(&result).unwrap()
    }
}

/// 执行文件写入
fn execute_write_file(input: &WriteFileInput) -> WriteFileOutput {
    let path = Path::new(&input.file_path);

    // 安全检查：禁止路径穿越
    if input.file_path.contains("..") {
        return WriteFileOutput {
            success: false,
            message: None,
            error: Some("Path traversal not allowed".to_string()),
        };
    }

    // 确保父目录存在
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                return WriteFileOutput {
                    success: false,
                    message: None,
                    error: Some(format!("Failed to create directory: {}", e)),
                };
            }
        }
    }

    // 写入文件
    match fs::write(path, &input.content) {
        Ok(()) => WriteFileOutput {
            success: true,
            message: Some(format!(
                "Successfully wrote {} bytes to {}",
                input.content.len(),
                input.file_path
            )),
            error: None,
        },
        Err(e) => WriteFileOutput {
            success: false,
            message: None,
            error: Some(format!("Failed to write file: {}", e)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_new_file() {
        let tool = WriteFileTool;
        let test_path = "target/test_write_tool.txt";
        let input = serde_json::json!({
            "file_path": test_path,
            "content": "Hello, World!"
        });
        let result = tool.execute(&input);
        assert!(result.contains("\"success\":true"));

        // 验证内容
        let content = fs::read_to_string(test_path).unwrap();
        assert_eq!(content, "Hello, World!");

        // 清理
        let _ = fs::remove_file(test_path);
    }

    #[test]
    fn test_path_traversal_blocked() {
        let tool = WriteFileTool;
        let input = serde_json::json!({
            "file_path": "../etc/test",
            "content": "malicious"
        });
        let result = tool.execute(&input);
        assert!(result.contains("traversal"));
    }
}
