//! 路径验证模块 - 防止路径穿越攻击
//!
//! 提供安全的路径验证功能，确保所有文件操作都在工作目录内进行。

use std::path::{Path, PathBuf};

/// 路径验证错误类型
#[derive(Debug)]
pub enum PathValidationError {
    /// 绝对路径不允许
    AbsolutePathNotAllowed,
    /// 路径穿越攻击
    PathTraversalDetected,
    /// 路径不存在（用于读取操作）
    PathNotFound(String),
    /// 无法获取工作目录
    WorkspaceDirError(String),
    /// 路径规范化失败
    CanonicalizationFailed(String),
}

impl std::fmt::Display for PathValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathValidationError::AbsolutePathNotAllowed => {
                write!(f, "Absolute paths are not allowed")
            }
            PathValidationError::PathTraversalDetected => {
                write!(f, "Path traversal not allowed")
            }
            PathValidationError::PathNotFound(path) => {
                write!(f, "Path not found: {}", path)
            }
            PathValidationError::WorkspaceDirError(msg) => {
                write!(f, "Failed to get workspace directory: {}", msg)
            }
            PathValidationError::CanonicalizationFailed(msg) => {
                write!(f, "Invalid path: {}", msg)
            }
        }
    }
}

impl std::error::Error for PathValidationError {}

/// 路径验证器
///
/// 用于验证文件路径是否安全，防止路径穿越攻击。
pub struct PathValidator {
    workspace_root: PathBuf,
}

impl PathValidator {
    /// 创建新的路径验证器
    ///
    /// 使用当前工作目录作为工作空间根目录。
    pub fn new() -> Result<Self, PathValidationError> {
        let workspace_root = std::env::current_dir()
            .map_err(|e| PathValidationError::WorkspaceDirError(e.to_string()))?;
        Ok(Self { workspace_root })
    }

    /// 使用指定的工作空间根目录创建验证器
    #[allow(dead_code)]
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            workspace_root: root,
        }
    }

    /// 验证路径是否安全（用于读取操作）
    ///
    /// 检查：
    /// 1. 路径不是绝对路径
    /// 2. 路径不包含路径穿越序列
    /// 3. 规范化后的路径在工作目录内
    /// 4. 路径存在
    pub fn validate_for_read(&self, path: &str) -> Result<PathBuf, PathValidationError> {
        let validated = self.validate_path(path)?;

        // 对于读取操作，路径必须存在
        if !validated.exists() {
            return Err(PathValidationError::PathNotFound(path.to_string()));
        }

        Ok(validated)
    }

    /// 验证路径是否安全（用于写入操作）
    ///
    /// 检查：
    /// 1. 路径不是绝对路径
    /// 2. 路径不包含路径穿越序列
    /// 3. 规范化后的路径在工作目录内
    ///
    /// 注意：写入操作不要求路径存在
    pub fn validate_for_write(&self, path: &str) -> Result<PathBuf, PathValidationError> {
        self.validate_path(path)
    }

    /// 内部路径验证逻辑
    ///
    /// 验证步骤：
    /// 1. 拒绝绝对路径
    /// 2. 检查路径组件中是否包含 ".."
    /// 3. 构建完整路径并规范化
    /// 4. 确保路径在工作目录内
    fn validate_path(&self, path: &str) -> Result<PathBuf, PathValidationError> {
        let requested = Path::new(path);

        // 步骤 1: 拒绝绝对路径
        if requested.is_absolute() {
            return Err(PathValidationError::AbsolutePathNotAllowed);
        }

        // 步骤 2: 检查路径组件中是否包含 ".."
        if self.contains_parent_dir(requested) {
            return Err(PathValidationError::PathTraversalDetected);
        }

        // 步骤 3: 构建完整路径
        let full_path = self.workspace_root.join(requested);

        // 步骤 4: 规范化路径并验证在工作目录内
        let canonical_path = self.canonicalize_path(&full_path, requested)?;
        let canonical_workspace = self.get_canonical_workspace()?;

        // 步骤 5: 验证路径在工作目录内
        if !self.is_within_workspace(&canonical_path, &canonical_workspace) {
            return Err(PathValidationError::PathTraversalDetected);
        }

        Ok(full_path)
    }

    /// 检查路径是否包含父目录组件 (..)
    fn contains_parent_dir(&self, path: &Path) -> bool {
        path.components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    }

    /// 规范化路径，处理存在和不存在的路径
    fn canonicalize_path(
        &self,
        full_path: &Path,
        requested: &Path,
    ) -> Result<PathBuf, PathValidationError> {
        if full_path.exists() {
            // 路径存在，直接规范化
            full_path
                .canonicalize()
                .map_err(|e| PathValidationError::CanonicalizationFailed(e.to_string()))
        } else {
            // 路径不存在，规范化最近的存在的父目录
            self.canonicalize_nonexistent_path(full_path, requested)
        }
    }

    /// 规范化不存在的路径
    fn canonicalize_nonexistent_path(
        &self,
        full_path: &Path,
        requested: &Path,
    ) -> Result<PathBuf, PathValidationError> {
        let parent = full_path.parent();
        let file_name = full_path.file_name();

        match (parent, file_name) {
            (Some(p), Some(f)) if p.exists() => {
                // 父目录存在，规范化它并附加文件名
                let canonical_parent = p
                    .canonicalize()
                    .map_err(|e| PathValidationError::CanonicalizationFailed(e.to_string()))?;
                Ok(canonical_parent.join(f))
            }
            (Some(_), Some(f)) => {
                // 父目录不存在，使用工作目录 + 相对路径
                let parent_relative = requested.parent().unwrap_or(Path::new(""));
                Ok(self.workspace_root.join(parent_relative).join(f))
            }
            _ => Ok(full_path.to_path_buf()),
        }
    }

    /// 获取规范化的工作目录
    fn get_canonical_workspace(&self) -> Result<PathBuf, PathValidationError> {
        self.workspace_root
            .canonicalize()
            .map_err(|e| PathValidationError::WorkspaceDirError(e.to_string()))
    }

    /// 检查路径是否在工作目录内
    fn is_within_workspace(&self, path: &Path, canonical_workspace: &Path) -> bool {
        if path.exists() {
            // 路径存在，直接检查
            path.starts_with(canonical_workspace)
        } else {
            // 路径不存在，找到最近的存在的父目录并检查
            let nearest_existing = self.find_nearest_existing_ancestor(path);
            match nearest_existing.canonicalize() {
                Ok(canonical) => canonical.starts_with(canonical_workspace),
                Err(_) => false,
            }
        }
    }

    /// 找到最近的存在的祖先目录
    fn find_nearest_existing_ancestor(&self, path: &Path) -> PathBuf {
        let mut current = path.to_path_buf();
        while !current.exists() {
            match current.parent() {
                Some(p) if !p.as_os_str().is_empty() => current = p.to_path_buf(),
                _ => return self.workspace_root.clone(),
            }
        }
        current
    }
}

// 注意：不实现 Default trait，因为 PathValidator::new() 可能失败
// 使用者应该显式调用 PathValidator::new() 并处理错误

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_validator() -> PathValidator {
        PathValidator::new().expect("Failed to create validator")
    }

    #[test]
    fn test_valid_relative_path() {
        let validator = create_test_validator();
        // Cargo.toml 应该存在于项目根目录
        let result = validator.validate_for_read("Cargo.toml");
        assert!(result.is_ok());
    }

    #[test]
    fn test_absolute_path_rejected() {
        let validator = create_test_validator();
        let result = validator.validate_for_read("/etc/passwd");
        assert!(matches!(
            result,
            Err(PathValidationError::AbsolutePathNotAllowed)
        ));
    }

    #[test]
    fn test_simple_traversal_rejected() {
        let validator = create_test_validator();
        let result = validator.validate_for_read("../etc/passwd");
        assert!(matches!(
            result,
            Err(PathValidationError::PathTraversalDetected)
        ));
    }

    #[test]
    fn test_nested_traversal_rejected() {
        let validator = create_test_validator();
        let result = validator.validate_for_read("foo/bar/../../../etc/passwd");
        assert!(matches!(
            result,
            Err(PathValidationError::PathTraversalDetected)
        ));
    }

    #[test]
    fn test_hidden_traversal_rejected() {
        let validator = create_test_validator();
        let result = validator.validate_for_read("src/../../../etc/passwd");
        assert!(matches!(
            result,
            Err(PathValidationError::PathTraversalDetected)
        ));
    }

    #[test]
    fn test_write_to_new_file() {
        let validator = create_test_validator();
        // 写入操作不要求文件存在
        let result = validator.validate_for_write("target/test_new_file.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_traversal_rejected() {
        let validator = create_test_validator();
        let result = validator.validate_for_write("../malicious.txt");
        assert!(matches!(
            result,
            Err(PathValidationError::PathTraversalDetected)
        ));
    }

    #[test]
    fn test_read_nonexistent_file() {
        let validator = create_test_validator();
        let result = validator.validate_for_read("nonexistent_file_12345.txt");
        assert!(matches!(result, Err(PathValidationError::PathNotFound(_))));
    }

    #[test]
    fn test_valid_nested_path() {
        let validator = create_test_validator();
        // src/main.rs 应该存在
        let result = validator.validate_for_read("src/main.rs");
        assert!(result.is_ok());
    }
}
