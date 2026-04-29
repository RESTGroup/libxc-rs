/// Error types for libxc operations.
pub enum LibXCError {
    /// An error indicating xc-functional / parameter not found.
    NotFound(String),
    /// Input tensor is not valid in some way (size not enough, stride not fit,
    /// etc.).
    TensorError(String),
}
