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

pub struct LibXCTensorLayout {
    pub shape: Vec<usize>,
    pub stride: Vec<isize>,
    pub offset: usize,
}

impl LibXCTensorLayout {
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
pub trait LibXCTensorViewAPI<'a> {
    /// The lifetime of the syncronization object for GPU tensors. For CPU
    /// tensors, this is None.
    type Lifetime: 'a;

    /// Get a raw pointer to the tensor data.
    ///
    /// The lifetime is None for CPU tensors, but for GPU it is `SyncOnDrop`.
    ///
    /// # Safety
    ///
    /// Offset must not exceed the total number of elements.
    unsafe fn data_ptr(&'a mut self, offset: usize) -> (*const f64, Self::Lifetime);

    /// Get the tensor layout (shape, stride, offset).
    fn layout(&self) -> &LibXCTensorLayout;

    /// Get the shape of the tensor.
    fn shape(&self) -> &[usize] {
        &self.layout().shape
    }

    /// Get the stride of the tensor.
    fn stride(&self) -> &[isize] {
        &self.layout().stride
    }

    /// Get a raw pointer to the tensor data for a given set of indices.
    ///
    /// # Safety
    ///
    /// Offset must not exceed the total number of elements.
    unsafe fn indexed_data_ptr(&'a mut self, indices: &[usize]) -> (*const f64, Self::Lifetime) {
        let offset = self.layout().compute_offset(indices);
        self.data_ptr(offset)
    }
}

/// A trait for mutable tensor operations needed by LIBXC.
pub trait LibXCTensorMutAPI<'a>: LibXCTensorViewAPI<'a> {
    /// Get a mutable raw pointer to the tensor data.
    ///
    /// # Safety
    ///
    /// Offset must not exceed the total number of elements.
    unsafe fn data_mut_ptr(&'a mut self, offset: usize) -> (*mut f64, Self::Lifetime);

    /// Get a mutable raw pointer to the tensor data for a given set of indices.
    ///
    /// # Safety
    ///
    /// Offset must not exceed the total number of elements.
    unsafe fn indexed_data_mut_ptr(&'a mut self, indices: &[usize]) -> (*mut f64, Self::Lifetime) {
        let offset = self.layout().compute_offset(indices);
        self.data_mut_ptr(offset)
    }
}

pub struct LibXCTensorBase<R, S = ()> {
    /// The actual data of the tensor. For CPU tensors, this is a Vec or a
    /// slice.
    pub data: R,
    /// The layout of the tensor, including shape, stride and offset.
    pub layout: LibXCTensorLayout,
    /// Device stream. Only useful for GPU tensors.
    pub stream: Option<S>,
}

mod cpu_tensor {
    use super::*;

    pub type LibXCTensorCpu = LibXCTensorBase<Vec<f64>>;
    pub type LibXCTensorCpuView<'a> = LibXCTensorBase<&'a [f64]>;
    pub type LibXCTensorCpuMut<'a> = LibXCTensorBase<&'a mut [f64]>;

    #[duplicate_item(TYPE; [LibXCTensorCpu]; [LibXCTensorCpuView<'_>]; [LibXCTensorCpuMut<'_>];)]
    impl LibXCTensorViewAPI<'_> for TYPE {
        type Lifetime = ();

        unsafe fn data_ptr(&mut self, offset: usize) -> (*const f64, Self::Lifetime) {
            unsafe { (self.data.as_ptr().add(offset), ()) }
        }

        fn layout(&self) -> &LibXCTensorLayout {
            &self.layout
        }
    }

    #[duplicate_item(TYPE; [LibXCTensorCpu]; [LibXCTensorCpuMut<'_>];)]
    impl LibXCTensorMutAPI<'_> for TYPE {
        unsafe fn data_mut_ptr(&mut self, offset: usize) -> (*mut f64, Self::Lifetime) {
            unsafe { (self.data.as_mut_ptr().add(offset), ()) }
        }
    }
}

#[cfg(feature = "cuda")]
mod cuda_tensor {
    use super::*;
    use cudarc::driver::*;
    use std::sync::Arc;

    pub type LibXCTensorCuda = LibXCTensorBase<CudaSlice<f64>, Arc<CudaStream>>;
    pub type LibXCTensorCudaView<'a> = LibXCTensorBase<CudaView<'a, f64>, Arc<CudaStream>>;
    pub type LibXCTensorCudaMut<'a> = LibXCTensorBase<CudaViewMut<'a, f64>, Arc<CudaStream>>;

    #[duplicate_item(TYPE; [LibXCTensorCuda]; [LibXCTensorCudaView<'_>]; [LibXCTensorCudaMut<'_>];)]
    impl<'a> LibXCTensorViewAPI<'a> for TYPE {
        type Lifetime = SyncOnDrop<'a>;

        unsafe fn data_ptr(&'a mut self, offset: usize) -> (*const f64, Self::Lifetime) {
            let stream = self.stream.get_or_insert_with(|| self.data.stream().clone());
            let (cu_device_ptr, sync_on_drop) = self.data.device_ptr(stream);
            let data_ptr = unsafe { (cu_device_ptr as usize as *const f64).add(offset) };
            (data_ptr, sync_on_drop)
        }

        fn layout(&self) -> &LibXCTensorLayout {
            &self.layout
        }
    }

    #[duplicate_item(TYPE; [LibXCTensorCuda]; [LibXCTensorCudaMut<'a>];)]
    impl<'a> LibXCTensorMutAPI<'a> for TYPE {
        unsafe fn data_mut_ptr(&'a mut self, offset: usize) -> (*mut f64, Self::Lifetime) {
            let stream = self.stream.get_or_insert_with(|| self.data.stream().clone());
            let (cu_device_ptr, sync_on_drop) = self.data.device_ptr_mut(stream);
            let data_ptr = unsafe { (cu_device_ptr as usize as *mut f64).add(offset) };
            (data_ptr, sync_on_drop)
        }
    }
}

pub use cpu_tensor::*;
#[cfg(feature = "cuda")]
pub use cuda_tensor::*;

#[cfg(feature = "cuda")]
#[test]
fn playground() {
    use cudarc::driver::*;

    // define a cuda 1-d tensor [1, 2, 3] and get its data pointer
    let ctx = CudaContext::new(0).unwrap();
    let stream = ctx.default_stream();
    let data = stream.clone_htod(&vec![1.0, 2.0, 4.0]).unwrap();
    let data = data.as_view();
    let mut tensor = LibXCTensorCudaView {
        data,
        layout: LibXCTensorLayout { shape: vec![3], stride: vec![1], offset: 0 },
        stream: None,
    };
    let data_ptr = unsafe { tensor.data_ptr(0) };
    println!("Data pointer: {:p}", data_ptr.0);
}
