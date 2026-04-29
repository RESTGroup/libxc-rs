//! Tensors for storing LIBXC data.
//!
//! We do not use any external tensor libraries, but instead implement our own
//! simple tensor structures to store the data needed for LIBXC computations.
//!
//! What's most important is that, the imported tensor should strictly follows
//! the row-major layout (at least, the last dimension should be contiguous), so
//! that we can directly pass the raw pointer to LIBXC without worrying about
//! the layout.

use duplicate::duplicate_item;

pub struct LibxcTensorLayout {
    pub shape: Vec<usize>,
    pub stride: Vec<isize>,
    pub offset: usize,
}

impl LibxcTensorLayout {
    pub fn compute_offset(&self, indices: &[usize]) -> usize {
        assert!(indices.len() <= self.shape.len(), "Index length exceeds tensor dimensions");
        indices.iter().zip(&self.stride).map(|(i, s)| i * (*s as usize)).sum::<usize>()
            + self.offset
    }

    pub fn check_last_stride(&self) -> bool {
        if self.shape.is_empty() || self.stride.is_empty() {
            // we are not going to handle 0-dimensional tensors here
            return false;
        }
        let last_dim = self.shape.len() - 1;
        self.stride[last_dim] == 1
    }
}

/// A trait for tensor operations needed by LIBXC.
pub trait LibxcTensorViewAPI<'a> {
    type Lifetime: 'a;

    /// Get a raw pointer to the tensor data.
    ///
    /// The lifetime is None for CPU tensors, but for GPU it is `SyncOnDrop`.
    fn data_ptr(&'a self, offset: usize) -> (*const f64, Self::Lifetime);

    /// Get the tensor layout (shape, stride, offset).
    fn layout(&self) -> &LibxcTensorLayout;

    /// Get the shape of the tensor.
    fn shape(&self) -> &[usize] {
        &self.layout().shape
    }

    /// Get the stride of the tensor.
    fn stride(&self) -> &[isize] {
        &self.layout().stride
    }

    /// Get a raw pointer to the tensor data for a given set of indices.
    fn indexed_data_ptr(&'a self, indices: &[usize]) -> (*const f64, Self::Lifetime) {
        let offset = self.layout().compute_offset(indices);
        self.data_ptr(offset)
    }
}

/// A trait for mutable tensor operations needed by LIBXC.
pub trait LibxcTensorMutAPI<'a>: LibxcTensorViewAPI<'a> {
    /// Get a mutable raw pointer to the tensor data.
    fn data_mut_ptr(&'a mut self, offset: usize) -> (*mut f64, Self::Lifetime);

    /// Get a mutable raw pointer to the tensor data for a given set of indices.
    fn indexed_data_mut_ptr(&'a mut self, indices: &[usize]) -> (*mut f64, Self::Lifetime) {
        let offset = self.layout().compute_offset(indices);
        self.data_mut_ptr(offset)
    }
}

pub struct LibxcTensor<R, S> {
    pub data: R,
    pub layout: LibxcTensorLayout,
    pub stream: Option<S>,
}

mod cpu_tensor {
    use super::*;

    #[duplicate_item(TYPE; [Vec<f64>]; [&'a [f64]]; [&'a mut [f64]];)]
    impl<'a, S> LibxcTensorViewAPI<'a> for LibxcTensor<TYPE, S> {
        type Lifetime = ();

        fn data_ptr(&'a self, offset: usize) -> (*const f64, Self::Lifetime) {
            (unsafe { self.data.as_ptr().add(offset) }, ())
        }

        fn layout(&self) -> &LibxcTensorLayout {
            &self.layout
        }
    }

    #[duplicate_item(TYPE; [Vec<f64>]; [&'a mut [f64]];)]
    impl<'a, S> LibxcTensorMutAPI<'a> for LibxcTensor<TYPE, S> {
        fn data_mut_ptr(&'a mut self, offset: usize) -> (*mut f64, Self::Lifetime) {
            (unsafe { self.data.as_mut_ptr().add(offset) }, ())
        }
    }

    pub type LibxcTensorCpu = LibxcTensor<Vec<f64>, ()>;
    pub type LibxcTensorCpuView<'a> = LibxcTensor<&'a [f64], ()>;
    pub type LibxcTensorCpuMut<'a> = LibxcTensor<&'a mut [f64], ()>;
}

#[cfg(feature = "cuda")]
mod cuda_tensor {
    use super::*;
    use cudarc::driver::*;
    use std::sync::Arc;

    #[duplicate_item(TYPE; [CudaSlice<f64>]; [CudaView<'a, f64>]; [CudaViewMut<'a, f64>];)]
    impl<'a, S> LibxcTensorViewAPI<'a> for LibxcTensor<TYPE, S> {
        type Lifetime = SyncOnDrop<'a>;

        fn data_ptr(&'a self, offset: usize) -> (*const f64, Self::Lifetime) {
            let (cu_device_ptr, sync_on_drop) = self.data.device_ptr(self.data.stream());
            (unsafe { (cu_device_ptr as usize as *const f64).add(offset) }, sync_on_drop)
        }

        fn layout(&self) -> &LibxcTensorLayout {
            &self.layout
        }
    }

    #[duplicate_item(TYPE; [CudaSlice<f64>]; [CudaViewMut<'a, f64>];)]
    impl<'a> LibxcTensorMutAPI<'a> for LibxcTensor<TYPE, Arc<CudaStream>> {
        fn data_mut_ptr(&'a mut self, offset: usize) -> (*mut f64, Self::Lifetime) {
            self.stream = Some(self.data.stream().clone());
            let (cu_device_ptr, sync_on_drop) =
                self.data.device_ptr_mut(self.stream.as_ref().unwrap());
            (unsafe { (cu_device_ptr as usize as *mut f64).add(offset) }, sync_on_drop)
        }
    }

    pub type LibxcTensorCuda = LibxcTensor<CudaSlice<f64>, Arc<CudaStream>>;
    pub type LibxcTensorCudaView<'a> = LibxcTensor<CudaView<'a, f64>, Arc<CudaStream>>;
    pub type LibxcTensorCudaMut<'a> = LibxcTensor<CudaViewMut<'a, f64>, Arc<CudaStream>>;
}

pub use cpu_tensor::*;
#[cfg(feature = "cuda")]
pub use cuda_tensor::*;
