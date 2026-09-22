//! Model registry and context budgeting.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    EmptyField { field: &'static str },
    ZeroValue { field: &'static str },
    UnorderedContextLadder,
    DuplicateModel(String),
    UnknownModel(String),
    InvalidEndpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegistryModelId(String);

impl RegistryModelId {
    pub fn new(value: impl Into<String>) -> Result<Self, RegistryError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(RegistryError::EmptyField { field: "model_id" });
        }
        Ok(Self(value))
    }
}

impl RegistryModelId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelBackend {
    LlamaServer,
    MlxLm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFormat {
    Gguf,
    Mlx,
    Safetensors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantization {
    F16,
    Q8,
    Q6,
    Q5,
    Q4,
}

impl Quantization {
    #[must_use]
    pub const fn bits(self) -> u8 {
        match self {
            Self::F16 => 16,
            Self::Q8 => 8,
            Self::Q6 => 6,
            Self::Q5 => 5,
            Self::Q4 => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextLadder {
    levels: Vec<u32>,
}

impl ContextLadder {
    pub fn new(levels: Vec<u32>) -> Result<Self, RegistryError> {
        if levels.is_empty() {
            return Err(RegistryError::EmptyField {
                field: "context_ladder",
            });
        }
        if levels.contains(&0) {
            return Err(RegistryError::ZeroValue {
                field: "context_tokens",
            });
        }
        if !levels.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(RegistryError::UnorderedContextLadder);
        }
        Ok(Self { levels })
    }
}

impl ContextLadder {
    #[must_use]
    pub fn levels(&self) -> &[u32] {
        &self.levels
    }
}

pub struct RegistryMarker;
