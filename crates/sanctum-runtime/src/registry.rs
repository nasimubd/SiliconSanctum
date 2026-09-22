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

impl ContextLadder {
    #[must_use]
    pub fn smallest(&self) -> u32 {
        self.levels[0]
    }
}

impl ContextLadder {
    #[must_use]
    pub fn largest(&self) -> u32 {
        self.levels[self.levels.len() - 1]
    }
}

impl ContextLadder {
    #[must_use]
    pub fn floor(&self, requested: u32) -> u32 {
        self.levels
            .iter()
            .copied()
            .take_while(|level| *level <= requested)
            .last()
            .unwrap_or(self.smallest())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelEntry {
    pub id: RegistryModelId,
    pub backend: ModelBackend,
    pub format: ModelFormat,
    pub quantization: Quantization,
    pub weights_bytes: u64,
    pub bytes_per_kv_token: u64,
    pub contexts: ContextLadder,
}

impl ModelEntry {
    pub fn new(
        id: RegistryModelId,
        backend: ModelBackend,
        format: ModelFormat,
        quantization: Quantization,
        weights_bytes: u64,
        bytes_per_kv_token: u64,
        contexts: ContextLadder,
    ) -> Result<Self, RegistryError> {
        if weights_bytes == 0 {
            return Err(RegistryError::ZeroValue {
                field: "weights_bytes",
            });
        }
        if bytes_per_kv_token == 0 {
            return Err(RegistryError::ZeroValue {
                field: "bytes_per_kv_token",
            });
        }
        Ok(Self {
            id,
            backend,
            format,
            quantization,
            weights_bytes,
            bytes_per_kv_token,
            contexts,
        })
    }
}

impl ModelEntry {
    #[must_use]
    pub fn kv_bytes_for(&self, tokens: u32) -> u64 {
        self.bytes_per_kv_token.saturating_mul(u64::from(tokens))
    }
}

impl ModelEntry {
    #[must_use]
    pub fn total_bytes_for(&self, tokens: u32) -> u64 {
        self.weights_bytes.saturating_add(self.kv_bytes_for(tokens))
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModelRegistry {
    entries: std::collections::BTreeMap<String, ModelEntry>,
}

impl ModelRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ModelRegistry {
    pub fn insert(&mut self, entry: ModelEntry) -> Result<(), RegistryError> {
        let key = entry.id.as_str().to_owned();
        if self.entries.contains_key(&key) {
            return Err(RegistryError::DuplicateModel(key));
        }
        self.entries.insert(key, entry);
        Ok(())
    }
}

pub struct RegistryMarker;
