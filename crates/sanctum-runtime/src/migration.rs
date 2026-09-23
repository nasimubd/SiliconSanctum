//! Thunderbolt storage migration and rollback.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum MigrationError{InvalidInput(&'static str),ToolFailure(String),Io(String)}
impl std::fmt::Display for MigrationError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{self:?}")}}
impl std::error::Error for MigrationError{}
// Migration extensions.
