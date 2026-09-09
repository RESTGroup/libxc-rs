use std::fmt;

use crate::enums::LibXCSpin;

/// Error types for libxc operations.
///
/// This enum is `#[non_exhaustive]`: new variants may be added in patch
/// releases, so downstream matches must include a wildcard (`_`) arm.
#[derive(Debug)]
#[non_exhaustive]
pub enum LibXCError {
    /// An error indicating xc-functional / parameter not found.
    NotFound(String),
    /// Failed to initialize a functional.
    InitError { func_id: i32, spin: LibXCSpin },
    /// Error during computation (invalid input sizes, family mismatch, etc.).
    ComputeError(String),
    /// Error related to parameter setting.
    ParamSetError { param_name: String, details: String },
    /// Tried to mutate a functional that is currently shared with clones.
    ///
    /// All setters require exclusive access to the underlying C object:
    /// drop every clone (or wait for the parallel scope holding them to
    /// finish) before setting parameters, thresholds or coefficients.
    SharedError,
    /// The loaded libxc library is too old for the requested operation.
    ///
    /// `needed` is the minimum libxc version, `found` the version of the
    /// loaded library (as reported by `xc_version`).
    UnsupportedVersion { needed: (i32, i32, i32), found: (i32, i32, i32) },
    /// Error related to CUDA operations.
    #[cfg(feature = "cuda")]
    CudaError(String),
}

impl fmt::Display for LibXCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibXCError::NotFound(msg) => write!(f, "not found: {msg}"),
            LibXCError::InitError { func_id, spin } => {
                write!(f, "failed to initialize functional {func_id} with spin {spin:?}")
            },
            LibXCError::ComputeError(msg) => write!(f, "compute error: {msg}"),
            LibXCError::ParamSetError { param_name, details } => {
                write!(f, "parameter set error for {param_name}: {details}")
            },
            LibXCError::SharedError => {
                write!(
                    f,
                    "cannot mutate a shared functional: all clones must be dropped before calling setters"
                )
            },
            LibXCError::UnsupportedVersion { needed, found } => write!(
                f,
                "this operation requires libxc >= {}.{}.{}, but the loaded library is {}.{}.{}",
                needed.0, needed.1, needed.2, found.0, found.1, found.2
            ),
            #[cfg(feature = "cuda")]
            LibXCError::CudaError(msg) => write!(f, "cuda error: {msg}"),
        }
    }
}

impl std::error::Error for LibXCError {}
