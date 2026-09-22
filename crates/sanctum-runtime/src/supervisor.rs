//! Asynchronous model-process supervision.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SupervisorError {
    #[error("{field} path must be absolute: {path}")]
    RelativePath { field: &'static str, path: std::path::PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutablePath(std::path::PathBuf);

impl ExecutablePath {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Result<Self, SupervisorError> {
        let path = path.into();
        if !path.is_absolute() {
            return Err(SupervisorError::RelativePath { field: "executable", path });
        }
        Ok(Self(path))
    }

    #[must_use]
    pub fn as_path(&self) -> &std::path::Path { &self.0 }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelPath(std::path::PathBuf);

impl ModelPath {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Result<Self, SupervisorError> {
        let path = path.into();
        if !path.is_absolute() {
            return Err(SupervisorError::RelativePath { field: "model", path });
        }
        Ok(Self(path))
    }

    #[must_use]
    pub fn as_path(&self) -> &std::path::Path { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendKind {
    LlamaServer,
    MlxLm,
}

impl std::fmt::Display for BackendKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::LlamaServer => "llama-server",
            Self::MlxLm => "mlx-lm",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{BackendKind, ExecutablePath, SupervisorError};

    #[test]
    fn formats_llama_server_backend() {
        assert_eq!(BackendKind::LlamaServer.to_string(), "llama-server");
    }

    #[test]
    fn formats_mlx_backend() {
        assert_eq!(BackendKind::MlxLm.to_string(), "mlx-lm");
    }

    #[test]
    fn rejects_relative_executable_path() {
        assert!(matches!(ExecutablePath::new("bin/server"), Err(SupervisorError::RelativePath { field: "executable", .. })));
    }

    #[test]
    fn accepts_absolute_executable_path() {
        let path = ExecutablePath::new("/usr/bin/true").unwrap();
        assert_eq!(path.as_path(), std::path::Path::new("/usr/bin/true"));
    }
}
