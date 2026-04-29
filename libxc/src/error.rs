use std::fmt;

use crate::enums::LibXCSpin;

/// Error types for libxc operations.
#[derive(Debug)]
pub enum LibXCError {
    /// An error indicating xc-functional / parameter not found.
    NotFound(String),
    /// Failed to initialize a functional.
    InitError { func_id: i32, spin: LibXCSpin },
    /// Error during computation (invalid input sizes, family mismatch, etc.).
    ComputeError(String),
}

impl fmt::Display for LibXCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibXCError::NotFound(msg) => write!(f, "not found: {msg}"),
            LibXCError::InitError { func_id, spin } => {
                write!(f, "failed to initialize functional {func_id} with spin {spin:?}")
            },
            LibXCError::ComputeError(msg) => write!(f, "compute error: {msg}"),
        }
    }
}

impl std::error::Error for LibXCError {}
