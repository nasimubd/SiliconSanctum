//! Page-aligned uncached model I/O.

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirectIoError {
    #[error("invalid buffer layout: size={size}, alignment={alignment}")]
    InvalidLayout { size: usize, alignment: usize },
    #[error("failed to open model file {path}: {source}")]
    Open {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to configure F_NOCACHE: {0}")]
    NoCache(#[source] std::io::Error),
    #[error("positional read failed at offset {offset}: {source}")]
    Read {
        offset: u64,
        #[source]
        source: std::io::Error,
    },
    #[error("model offset {offset} does not fit off_t")]
    OffsetOverflow { offset: u64 },
}
