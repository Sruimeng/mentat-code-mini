//! 配置管理模块
//!
//! 提供配置文件加载、验证和错误处理功能。
//! 设计目标：
//! - 优雅处理配置文件不存在的情况
//! - 避免在错误信息中泄露敏感信息（如 API 密钥）
//! - 支持配置验证

use serde::Deserialize;
use std::fmt;
use std::fs;
use std::path::PathBuf;

/// 配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 配置文件未找到
    NotFound(PathBuf),
    /// 无法读取配置文件
    ReadError(String),
    /// 配置文件格式错误
    ParseError(String),
    /// 配置验证失败
    ValidationError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotFound(path) => {
                write!(
                    f,
                    "配置文件未找到: {}\n\n请创建配置文件或运行初始化命令。\n\n配置文件示例:\n{{\n  \"env\": {{\n    \"ANTHROPIC_AUTH_TOKEN\": \"your-api-key\",\n    \"ANTHROPIC_BASE_URL\": \"https://api.anthropic.com\"\n  }}\n}}",
                    path.display()
                )
            }
            ConfigError::ReadError(msg) => {
                write!(f, "无法读取配置文件: {}", msg)
            }
            ConfigError::ParseError(msg) => {
                write!(f, "配置文件格式错误: {}", msg)
            }
            ConfigError::ValidationError(msg) => {
                write!(f, "配置验证失败: {}", msg)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// 环境变量配置
#[derive(Deserialize, Clone)]
pub struct Env {
    #[serde(rename = "ANTHROPIC_AUTH_TOKEN")]
    pub api_key: String,
    #[serde(rename = "ANTHROPIC_BASE_URL")]
    pub base_url: String,
    #[serde(rename = "HTTPS_PROXY")]
    pub https_proxy: Option<String>,
}

/// 应用配置
#[derive(Deserialize, Clone)]
pub struct Settings {
    pub env: Env,
    /// 模型名称（可选，默认使用 claude-sonnet-4-20250514）
    #[serde(default)]
    pub model: Option<String>,
}

impl Settings {
    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 验证 API Key
        if self.env.api_key.is_empty() {
            return Err(ConfigError::ValidationError(
                "API key (ANTHROPIC_AUTH_TOKEN) 不能为空".to_string(),
            ));
        }

        // 验证 API Key 格式（基本检查，不暴露具体内容）
        if self.env.api_key.len() < 10 {
            return Err(ConfigError::ValidationError("API key 格式无效".to_string()));
        }

        // 验证 Base URL
        if self.env.base_url.is_empty() {
            return Err(ConfigError::ValidationError(
                "Base URL (ANTHROPIC_BASE_URL) 不能为空".to_string(),
            ));
        }

        if !self.env.base_url.starts_with("http://") && !self.env.base_url.starts_with("https://") {
            return Err(ConfigError::ValidationError(
                "Base URL 必须以 http:// 或 https:// 开头".to_string(),
            ));
        }

        // 验证代理 URL（如果存在）
        if let Some(proxy) = &self.env.https_proxy {
            if !proxy.is_empty()
                && !proxy.starts_with("http://")
                && !proxy.starts_with("https://")
                && !proxy.starts_with("socks5://")
            {
                return Err(ConfigError::ValidationError(
                    "代理 URL 格式无效，必须以 http://, https:// 或 socks5:// 开头".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 获取模型名称，如果未配置则返回默认值
    pub fn get_model(&self) -> String {
        self.model
            .clone()
            .unwrap_or_else(|| "claude-opus-4-5-20251101".to_string())
    }
}

/// 默认配置文件路径
const DEFAULT_CONFIG_PATH: &str = ".mentat/settings.json";

/// 加载配置文件
///
/// 按以下顺序搜索配置文件：
/// 1. 当前目录下的 .mentat/settings.json
/// 2. 用户配置目录下的 mentat/settings.json（如 ~/.config/mentat/settings.json）
///
/// # 返回
/// - `Ok(Settings)` - 成功加载并验证的配置
/// - `Err(ConfigError)` - 加载或验证失败
pub fn load_settings() -> Result<Settings, ConfigError> {
    load_settings_from_path(None)
}

/// 从指定路径加载配置文件
///
/// # 参数
/// - `custom_path` - 自定义配置文件路径，如果为 None 则使用默认搜索路径
pub fn load_settings_from_path(custom_path: Option<&str>) -> Result<Settings, ConfigError> {
    // 如果指定了自定义路径，直接使用
    if let Some(path) = custom_path {
        return load_and_validate(PathBuf::from(path));
    }

    // 搜索配置文件
    let search_paths = get_config_search_paths();

    for path in &search_paths {
        if path.exists() {
            return load_and_validate(path.clone());
        }
    }

    // 没有找到配置文件
    Err(ConfigError::NotFound(PathBuf::from(DEFAULT_CONFIG_PATH)))
}

/// 获取配置文件搜索路径列表
fn get_config_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // 1. 当前目录下的配置文件
    paths.push(PathBuf::from(DEFAULT_CONFIG_PATH));

    // 2. 用户配置目录（跨平台）
    if let Some(config_dir) = dirs_config_dir() {
        paths.push(config_dir.join("mentat/settings.json"));
    }

    // 3. 用户主目录下的 .mentat
    if let Some(home_dir) = dirs_home_dir() {
        paths.push(home_dir.join(".mentat/settings.json"));
    }

    paths
}

/// 获取用户配置目录（简化实现，不依赖 dirs crate）
fn dirs_config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        dirs_home_dir().map(|h| h.join("Library/Application Support"))
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs_home_dir().map(|h| h.join(".config")))
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA").ok().map(PathBuf::from)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

/// 获取用户主目录
fn dirs_home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
}

/// 加载并验证配置文件
fn load_and_validate(path: PathBuf) -> Result<Settings, ConfigError> {
    // 读取文件
    let content = fs::read_to_string(&path).map_err(|e| {
        // 提供有用的错误信息，但不暴露敏感内容
        let hint = match e.kind() {
            std::io::ErrorKind::NotFound => "文件不存在",
            std::io::ErrorKind::PermissionDenied => "权限不足",
            _ => "读取失败",
        };
        ConfigError::ReadError(format!("{} ({})", hint, path.display()))
    })?;

    // 解析 JSON
    let settings: Settings = serde_json::from_str(&content).map_err(|e| {
        // 提供详细的解析错误信息以帮助调试
        let error_type = match e.classify() {
            serde_json::error::Category::Io => "IO 错误",
            serde_json::error::Category::Syntax => "语法错误",
            serde_json::error::Category::Data => "数据类型错误",
            serde_json::error::Category::Eof => "文件意外结束",
        };
        ConfigError::ParseError(format!(
            "{}: 第 {} 行，第 {} 列\n   提示: 请检查 JSON 格式是否正确，特别是引号、逗号和括号",
            error_type,
            e.line(),
            e.column()
        ))
    })?;

    // 验证配置
    settings.validate()?;

    Ok(settings)
}

/// 创建默认配置文件模板
pub fn create_default_config() -> Result<PathBuf, ConfigError> {
    let config_path = PathBuf::from(DEFAULT_CONFIG_PATH);

    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| ConfigError::ReadError(format!("无法创建配置目录: {}", e)))?;
    }

    // 创建模板配置
    let template = r#"{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "your-api-key-here",
    "ANTHROPIC_BASE_URL": "https://api.anthropic.com",
    "HTTPS_PROXY": null
  },
  "model": "claude-opus-4-5-20251101"
}
"#;

    fs::write(&config_path, template)
        .map_err(|e| ConfigError::ReadError(format!("无法写入配置文件: {}", e)))?;

    Ok(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_api_key() {
        let settings = Settings {
            env: Env {
                api_key: "".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                https_proxy: None,
            },
            model: None,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_validate_short_api_key() {
        let settings = Settings {
            env: Env {
                api_key: "short".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                https_proxy: None,
            },
            model: None,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_validate_empty_base_url() {
        let settings = Settings {
            env: Env {
                api_key: "valid-api-key-12345".to_string(),
                base_url: "".to_string(),
                https_proxy: None,
            },
            model: None,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_base_url() {
        let settings = Settings {
            env: Env {
                api_key: "valid-api-key-12345".to_string(),
                base_url: "not-a-url".to_string(),
                https_proxy: None,
            },
            model: None,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_proxy() {
        let settings = Settings {
            env: Env {
                api_key: "valid-api-key-12345".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                https_proxy: Some("invalid-proxy".to_string()),
            },
            model: None,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_validate_valid_settings() {
        let settings = Settings {
            env: Env {
                api_key: "valid-api-key-12345".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                https_proxy: None,
            },
            model: None,
        };
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn test_validate_valid_settings_with_proxy() {
        let settings = Settings {
            env: Env {
                api_key: "valid-api-key-12345".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                https_proxy: Some("http://proxy.example.com:8080".to_string()),
            },
            model: None,
        };
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn test_get_model_default() {
        let settings = Settings {
            env: Env {
                api_key: "test".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                https_proxy: None,
            },
            model: None,
        };
        assert_eq!(settings.get_model(), "claude-opus-4-5-20251101");
    }

    #[test]
    fn test_get_model_custom() {
        let settings = Settings {
            env: Env {
                api_key: "test".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                https_proxy: None,
            },
            model: Some("claude-opus-4-5-20251101".to_string()),
        };
        assert_eq!(settings.get_model(), "claude-opus-4-5-20251101");
    }

    #[test]
    fn test_config_not_found_error_message() {
        let error = ConfigError::NotFound(PathBuf::from(".mentat/settings.json"));
        let msg = error.to_string();
        assert!(msg.contains("配置文件未找到"));
        assert!(msg.contains("ANTHROPIC_AUTH_TOKEN"));
    }
}
