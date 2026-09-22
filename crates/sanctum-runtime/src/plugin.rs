//! Domain-neutral external plugin contracts.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginError {
    EmptyField { field: &'static str },
    InvalidEndpoint,
    DuplicateCapability(String),
    UnsupportedProtocol,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginId(String);

impl PluginId {
    pub fn new(value: impl Into<String>) -> Result<Self, PluginError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(PluginError::EmptyField { field: "plugin_id" });
        }
        Ok(Self(value))
    }
}

impl PluginId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginProtocol {
    Http,
    Grpc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginEndpoint {
    pub protocol: PluginProtocol,
    pub authority: String,
}

impl PluginEndpoint {
    pub fn new(
        protocol: PluginProtocol,
        authority: impl Into<String>,
    ) -> Result<Self, PluginError> {
        let authority = authority.into();
        if authority.trim().is_empty() || authority.chars().any(char::is_whitespace) {
            return Err(PluginError::InvalidEndpoint);
        }
        Ok(Self {
            protocol,
            authority,
        })
    }
}

impl PluginEndpoint {
    #[must_use]
    pub fn uri(&self) -> String {
        let scheme = match self.protocol {
            PluginProtocol::Http => "http",
            PluginProtocol::Grpc => "grpc",
        };
        format!("{scheme}://{}", self.authority)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Capability(String);

impl Capability {
    pub fn new(value: impl Into<String>) -> Result<Self, PluginError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(PluginError::EmptyField {
                field: "capability",
            });
        }
        Ok(Self(value))
    }
}

impl Capability {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub id: PluginId,
    pub endpoint: PluginEndpoint,
    capabilities: std::collections::BTreeSet<String>,
}

impl PluginManifest {
    #[must_use]
    pub fn new(id: PluginId, endpoint: PluginEndpoint) -> Self {
        Self {
            id,
            endpoint,
            capabilities: std::collections::BTreeSet::new(),
        }
    }
}

impl PluginManifest {
    pub fn add_capability(&mut self, capability: Capability) -> Result<(), PluginError> {
        let value = capability.0;
        if !self.capabilities.insert(value.clone()) {
            return Err(PluginError::DuplicateCapability(value));
        }
        Ok(())
    }
}

impl PluginManifest {
    #[must_use]
    pub fn supports(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }
}

impl PluginManifest {
    pub fn capabilities(&self) -> impl Iterator<Item = &str> {
        self.capabilities.iter().map(String::as_str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginRequest {
    pub request_id: String,
    pub capability: String,
    pub payload: Vec<u8>,
}

impl PluginRequest {
    pub fn new(
        request_id: impl Into<String>,
        capability: impl Into<String>,
        payload: Vec<u8>,
    ) -> Result<Self, PluginError> {
        let request_id = request_id.into();
        let capability = capability.into();
        if request_id.trim().is_empty() {
            return Err(PluginError::EmptyField {
                field: "request_id",
            });
        }
        if capability.trim().is_empty() {
            return Err(PluginError::EmptyField {
                field: "capability",
            });
        }
        Ok(Self {
            request_id,
            capability,
            payload,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginStatus {
    Ok,
    InvalidRequest,
    Unavailable,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginResponse {
    pub request_id: String,
    pub status: PluginStatus,
    pub payload: Vec<u8>,
}

impl PluginResponse {
    #[must_use]
    pub fn success(request_id: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            request_id: request_id.into(),
            status: PluginStatus::Ok,
            payload,
        }
    }
}

impl PluginResponse {
    #[must_use]
    pub fn failure(request_id: impl Into<String>, status: PluginStatus) -> Self {
        Self {
            request_id: request_id.into(),
            status,
            payload: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PluginCatalog {
    entries: std::collections::BTreeMap<String, PluginManifest>,
}

impl PluginCatalog {
    pub fn register(&mut self, manifest: PluginManifest) -> Option<PluginManifest> {
        self.entries
            .insert(manifest.id.as_str().to_owned(), manifest)
    }
}

impl PluginCatalog {
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&PluginManifest> {
        self.entries.get(id)
    }
}

pub struct PluginMarker;
