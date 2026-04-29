//! Tensors for storing LIBXC data.
//!
//! We do not use any external tensor libraries, but instead implement our own
//! simple tensor structures to store the data needed for LIBXC computations.

/// A CPU-based tensor for storing LIBXC data.
///
/// - Generic `D`: Vec<usize> (dynamic dimension) or [usize; N] (static
///   dimension) for shape information. Static dimension may iter faster than
///   dynamic dimension. However, it is better to communicate to API caller with
///   dynamic dimension.
pub struct LIBXCTensorCpu<D> {
    pub data: Vec<f64>,
    pub shape: D,
}
