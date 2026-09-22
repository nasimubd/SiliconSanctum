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

pub struct PluginMarker;
