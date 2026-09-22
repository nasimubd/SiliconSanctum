//! Asynchronous model-process supervision.

use serde::{Deserialize, Serialize};

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
