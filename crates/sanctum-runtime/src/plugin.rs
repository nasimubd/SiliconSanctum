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

pub struct PluginMarker;
