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

pub struct RegistryMarker;
