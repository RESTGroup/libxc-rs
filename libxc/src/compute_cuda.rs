//! CUDA (device) wrapper for libxc functionals.
//!
//! This module provides compute methods that operate on GPU memory via
//! `cudarc` types. The FFI calls are the same as CPU (`xc_lda`, `xc_gga`,
//! `xc_mgga`), but the functional must be initialized with
//! `XC_FLAGS_ON_DEVICE` via [`LibXCFunctional::from_identifier_with_device`]
//! and the input/output arrays must reside in GPU memory.

use crate::prelude::*;
use cudarc::driver::{CudaSlice, CudaStream, CudaView, CudaViewMut, DevicePtr, DevicePtrMut};
use std::sync::Arc;

/// Unified input map for CUDA compute.
///
/// Keys are `"rho"`, `"sigma"`, `"lapl"`, `"tau"`. Values are
/// `CudaView<f64>` — borrowed views into GPU-allocated arrays.
pub type LibXCCudaInput<'a> = HashMap<String, CudaView<'a, f64>>;

/// Unified output map for CUDA compute (preallocated device buffers).
///
/// Keys are output component names (e.g. `"zk"`, `"vrho"`). Values are
/// `CudaViewMut<f64>` — borrowed mutable views into GPU-allocated arrays.
pub type LibXCCudaOutputMut<'a> = HashMap<String, CudaViewMut<'a, f64>>;

/// Extract a raw `*const f64` device pointer from a `CudaView`.
fn cuda_view_ptr(view: &CudaView<f64>, stream: &Arc<CudaStream>) -> *const f64 {
    let (ptr, _sync) = view.device_ptr(stream);
    ptr as *const f64
}

/// Extract a raw `*mut f64` device pointer from a `CudaViewMut`.
fn cuda_view_mut_ptr(view: &mut CudaViewMut<f64>, stream: &Arc<CudaStream>) -> *mut f64 {
    let (ptr, _sync) = view.device_ptr_mut(stream);
    ptr as *mut f64
}

/// Validate a required CUDA input and return a const device pointer.
fn require_cuda_input_ptr(
    input: &LibXCCudaInput,
    key: &str,
    npoints: usize,
    expected_dim: i32,
    stream: &Arc<CudaStream>,
) -> Result<*const f64, LibXCError> {
    let view = input.get(key).ok_or_else(|| {
        LibXCError::ComputeError(format!("{key}: required CUDA input not provided"))
    })?;
    let expected = npoints * (expected_dim as usize);
    if view.len() != expected {
        return Err(LibXCError::ComputeError(format!(
            "{key}: expected size {expected}, got {}",
            view.len()
        )));
    }
    Ok(cuda_view_ptr(view, stream))
}

/// Validate a conditionally-required CUDA input and return a const device
/// pointer.
fn conditional_cuda_input_ptr(
    input: &LibXCCudaInput,
    key: &str,
    npoints: usize,
    expected_dim: i32,
    required: bool,
    stream: &Arc<CudaStream>,
) -> Result<*const f64, LibXCError> {
    match (input.get(key), required) {
        (Some(view), true) => {
            let expected = npoints * (expected_dim as usize);
            if view.len() != expected {
                return Err(LibXCError::ComputeError(format!(
                    "{key}: expected size {expected}, got {}",
                    view.len()
                )));
            }
            Ok(cuda_view_ptr(view, stream))
        },
        (None, true) => {
            Err(LibXCError::ComputeError(format!("{key}: required CUDA input not provided")))
        },
        (_, false) => Ok(std::ptr::null()),
    }
}

/// Validate an output CudaViewMut from the map and return a mutable device
/// pointer.
fn validate_cuda_output_ptr(
    output: &mut LibXCCudaOutputMut,
    key: &str,
    npoints: usize,
    expected_dim: i32,
    stream: &Arc<CudaStream>,
) -> Result<*mut f64, LibXCError> {
    match output.get_mut(key) {
        Some(view) => {
            let expected = npoints * (expected_dim as usize);
            if view.len() != expected {
                return Err(LibXCError::ComputeError(format!(
                    "{key}: expected size {expected}, got {}",
                    view.len()
                )));
            }
            Ok(cuda_view_mut_ptr(view, stream))
        },
        None => Ok(std::ptr::null_mut()),
    }
}

/// Validate all output entries for a given label set, returning a pointer map.
fn validate_cuda_output_ptrs(
    output: &mut LibXCCudaOutputMut,
    labels: &[&'static str],
    npoints: usize,
    dim: &ffi::xc_dimensions,
    stream: &Arc<CudaStream>,
) -> Result<HashMap<&'static str, *mut f64>, LibXCError> {
    let mut ptrs = HashMap::new();
    for &label in labels {
        let d = crate::layout_handling::get_dim(dim, label);
        let ptr = validate_cuda_output_ptr(output, label, npoints, d, stream)?;
        if !ptr.is_null() {
            ptrs.insert(label, ptr);
        }
    }
    Ok(ptrs)
}

/// Helper to get a pointer from the validated map, defaulting to null.
fn ptr_of(ptrs: &HashMap<&'static str, *mut f64>, key: &str) -> *mut f64 {
    ptrs.get(key).copied().unwrap_or(std::ptr::null_mut())
}

/// Guard that the functional is initialized for GPU execution.
fn guard_on_device(func: &LibXCFunctional) -> Result<(), LibXCError> {
    if !func.is_on_device() {
        Err(LibXCError::ComputeError(
            "functional was not initialized for GPU; use from_identifier_with_device with OnDevice"
                .into(),
        ))
    } else {
        Ok(())
    }
}

impl LibXCFunctional {
    // -- LDA CUDA private helpers ------------------------------------------

    /// Validate CUDA input and compute output layout for LDA.
    fn cuda_lda_prepare(
        &self,
        input: &LibXCCudaInput,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
        stream: &Arc<CudaStream>,
    ) -> Result<(usize, *const f64, LibXCOutputLayout), LibXCError> {
        let flags = deriv_flags.into();
        self.validate_flags(flags)?;
        guard_on_device(self)?;
        let view = input.get("rho").ok_or_else(|| {
            LibXCError::ComputeError("rho: required CUDA input not provided".into())
        })?;
        let nspin = self.spin() as usize;
        if view.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = view.len() / nspin;
        let rho_ptr = cuda_view_ptr(view, stream);
        let layout = self.lda_output_layout(npoints, flags);
        Ok((npoints, rho_ptr, layout))
    }

    /// Invoke `xc_lda` FFI call with device pointers.
    fn cuda_lda_call(
        &self,
        npoints: usize,
        rho_ptr: *const f64,
        output_base: *mut f64,
        layout: &LibXCOutputLayout,
    ) {
        let ptr_for = |name: &str| -> *mut f64 {
            match layout.get(name) {
                Some(range) => unsafe { output_base.add(range.start) },
                None => std::ptr::null_mut::<f64>(),
            }
        };
        unsafe {
            ffi::xc_lda(
                self.ptr,
                npoints,
                rho_ptr,
                ptr_for("zk"),
                ptr_for("vrho"),
                ptr_for("v2rho2"),
                ptr_for("v3rho3"),
                ptr_for("v4rho4"),
            );
        }
    }

    // -- LDA CUDA compute --------------------------------------------------

    /// Compute LDA functional on GPU with automatic device allocation.
    ///
    /// Returns `(CudaSlice<f64>, LibXCOutputLayout)` where the slice is a
    /// contiguous zero-initialized GPU buffer.
    pub fn cuda_compute_lda(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(CudaSlice<f64>, LibXCOutputLayout), LibXCError> {
        let (npoints, rho_ptr, layout) = self.cuda_lda_prepare(input, flags, stream)?;
        let mut buffer = stream
            .alloc_zeros::<f64>(layout.total_size)
            .map_err(|e| LibXCError::CudaError(format!("CUDA allocation failed: {e}")))?;
        {
            let (output_base, _sync) = buffer.device_ptr_mut(stream);
            self.cuda_lda_call(npoints, rho_ptr, output_base as *mut f64, &layout);
        }
        Ok((buffer, layout))
    }

    /// Compute LDA functional on GPU with preallocated contiguous device
    /// buffer.
    pub fn cuda_compute_lda_with_unsliced_output(
        &self,
        input: &LibXCCudaInput,
        output: &mut CudaSlice<f64>,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        let stream = output.stream().clone();
        let (npoints, rho_ptr, layout) = self.cuda_lda_prepare(input, deriv_flags, &stream)?;
        if output.len() < layout.total_size {
            return Err(LibXCError::ComputeError(format!(
                "output buffer has too small size: expected {}, got {}",
                layout.total_size,
                output.len()
            )));
        }
        let (output_base, _sync) = output.device_ptr_mut(&stream);
        self.cuda_lda_call(npoints, rho_ptr, output_base as *mut f64, &layout);
        Ok(layout)
    }

    /// Compute LDA functional on GPU with named per-component output buffers.
    pub fn cuda_compute_lda_with_output(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        output: &mut LibXCCudaOutputMut,
    ) -> Result<(), LibXCError> {
        let view = input.get("rho").ok_or_else(|| {
            LibXCError::ComputeError("rho: required CUDA input not provided".into())
        })?;
        guard_on_device(self)?;
        let nspin = self.spin() as usize;
        if view.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = view.len() / nspin;
        let rho_ptr = cuda_view_ptr(view, stream);
        let dim = self.dim();
        let ptrs = validate_cuda_output_ptrs(output, &LDA_OUTPUT_LABELS, npoints, dim, stream)?;

        unsafe {
            ffi::xc_lda(
                self.ptr,
                npoints,
                rho_ptr,
                ptr_of(&ptrs, "zk"),
                ptr_of(&ptrs, "vrho"),
                ptr_of(&ptrs, "v2rho2"),
                ptr_of(&ptrs, "v3rho3"),
                ptr_of(&ptrs, "v4rho4"),
            );
        }
        Ok(())
    }

    // -- GGA CUDA private helpers ------------------------------------------

    /// Validate CUDA input and compute output layout for GGA.
    fn cuda_gga_prepare(
        &self,
        input: &LibXCCudaInput,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
        stream: &Arc<CudaStream>,
    ) -> Result<(usize, *const f64, *const f64, LibXCOutputLayout), LibXCError> {
        let flags = deriv_flags.into();
        self.validate_flags(flags)?;
        guard_on_device(self)?;
        let view = input.get("rho").ok_or_else(|| {
            LibXCError::ComputeError("rho: required CUDA input not provided".into())
        })?;
        let nspin = self.spin() as usize;
        if view.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = view.len() / nspin;
        let dim = self.dim();
        let rho_ptr = cuda_view_ptr(view, stream);
        let sigma_ptr = require_cuda_input_ptr(input, "sigma", npoints, dim.sigma, stream)?;
        let layout = self.gga_output_layout(npoints, flags);
        Ok((npoints, rho_ptr, sigma_ptr, layout))
    }

    /// Invoke `xc_gga` FFI call with device pointers.
    fn cuda_gga_call(
        &self,
        npoints: usize,
        rho_ptr: *const f64,
        sigma_ptr: *const f64,
        output_base: *mut f64,
        layout: &LibXCOutputLayout,
    ) {
        let ptr_for = |name: &str| -> *mut f64 {
            match layout.get(name) {
                Some(range) => unsafe { output_base.add(range.start) },
                None => std::ptr::null_mut::<f64>(),
            }
        };
        unsafe {
            ffi::xc_gga(
                self.ptr,
                npoints,
                rho_ptr,
                sigma_ptr as *mut f64,
                ptr_for("zk"),
                ptr_for("vrho"),
                ptr_for("vsigma"),
                ptr_for("v2rho2"),
                ptr_for("v2rhosigma"),
                ptr_for("v2sigma2"),
                ptr_for("v3rho3"),
                ptr_for("v3rho2sigma"),
                ptr_for("v3rhosigma2"),
                ptr_for("v3sigma3"),
                ptr_for("v4rho4"),
                ptr_for("v4rho3sigma"),
                ptr_for("v4rho2sigma2"),
                ptr_for("v4rhosigma3"),
                ptr_for("v4sigma4"),
            );
        }
    }

    // -- GGA CUDA compute --------------------------------------------------

    /// Compute GGA functional on GPU with automatic device allocation.
    pub fn cuda_compute_gga(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(CudaSlice<f64>, LibXCOutputLayout), LibXCError> {
        let (npoints, rho_ptr, sigma_ptr, layout) = self.cuda_gga_prepare(input, flags, stream)?;
        let mut buffer = stream
            .alloc_zeros::<f64>(layout.total_size)
            .map_err(|e| LibXCError::CudaError(format!("CUDA allocation failed: {e}")))?;
        {
            let (output_base, _sync) = buffer.device_ptr_mut(stream);
            self.cuda_gga_call(npoints, rho_ptr, sigma_ptr, output_base as *mut f64, &layout);
        }
        Ok((buffer, layout))
    }

    /// Compute GGA functional on GPU with preallocated contiguous device
    /// buffer.
    pub fn cuda_compute_gga_with_unsliced_output(
        &self,
        input: &LibXCCudaInput,
        output: &mut CudaSlice<f64>,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        let stream = output.stream().clone();
        let (npoints, rho_ptr, sigma_ptr, layout) =
            self.cuda_gga_prepare(input, deriv_flags, &stream)?;
        if output.len() < layout.total_size {
            return Err(LibXCError::ComputeError(format!(
                "output buffer has too small size: expected {}, got {}",
                layout.total_size,
                output.len()
            )));
        }
        let (output_base, _sync) = output.device_ptr_mut(&stream);
        self.cuda_gga_call(npoints, rho_ptr, sigma_ptr, output_base as *mut f64, &layout);
        Ok(layout)
    }

    /// Compute GGA functional on GPU with named per-component output buffers.
    pub fn cuda_compute_gga_with_output(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        output: &mut LibXCCudaOutputMut,
    ) -> Result<(), LibXCError> {
        let view = input.get("rho").ok_or_else(|| {
            LibXCError::ComputeError("rho: required CUDA input not provided".into())
        })?;
        guard_on_device(self)?;
        let nspin = self.spin() as usize;
        if view.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = view.len() / nspin;
        let rho_ptr = cuda_view_ptr(view, stream);
        let dim = self.dim();
        let sigma_ptr = require_cuda_input_ptr(input, "sigma", npoints, dim.sigma, stream)?;
        let ptrs = validate_cuda_output_ptrs(output, &GGA_OUTPUT_LABELS, npoints, dim, stream)?;

        unsafe {
            ffi::xc_gga(
                self.ptr,
                npoints,
                rho_ptr,
                sigma_ptr as *mut f64,
                ptr_of(&ptrs, "zk"),
                ptr_of(&ptrs, "vrho"),
                ptr_of(&ptrs, "vsigma"),
                ptr_of(&ptrs, "v2rho2"),
                ptr_of(&ptrs, "v2rhosigma"),
                ptr_of(&ptrs, "v2sigma2"),
                ptr_of(&ptrs, "v3rho3"),
                ptr_of(&ptrs, "v3rho2sigma"),
                ptr_of(&ptrs, "v3rhosigma2"),
                ptr_of(&ptrs, "v3sigma3"),
                ptr_of(&ptrs, "v4rho4"),
                ptr_of(&ptrs, "v4rho3sigma"),
                ptr_of(&ptrs, "v4rho2sigma2"),
                ptr_of(&ptrs, "v4rhosigma3"),
                ptr_of(&ptrs, "v4sigma4"),
            );
        }
        Ok(())
    }

    // -- MGGA CUDA private helpers -----------------------------------------

    /// Validate CUDA input and compute output layout for MGGA.
    fn cuda_mgga_prepare(
        &self,
        input: &LibXCCudaInput,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
        stream: &Arc<CudaStream>,
    ) -> Result<
        (usize, *const f64, *const f64, *const f64, *const f64, LibXCOutputLayout),
        LibXCError,
    > {
        let flags = deriv_flags.into();
        self.validate_flags(flags)?;
        guard_on_device(self)?;
        let view = input.get("rho").ok_or_else(|| {
            LibXCError::ComputeError("rho: required CUDA input not provided".into())
        })?;
        let nspin = self.spin() as usize;
        if view.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = view.len() / nspin;
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();
        let rho_ptr = cuda_view_ptr(view, stream);
        let sigma_ptr = require_cuda_input_ptr(input, "sigma", npoints, dim.sigma, stream)?;
        let lapl_ptr =
            conditional_cuda_input_ptr(input, "lapl", npoints, dim.lapl, needs_lapl, stream)?;
        let tau_ptr =
            conditional_cuda_input_ptr(input, "tau", npoints, dim.tau, needs_tau, stream)?;
        let layout = self.mgga_output_layout(npoints, flags);
        Ok((npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, layout))
    }

    /// Invoke `xc_mgga` FFI call with device pointers.
    #[allow(clippy::too_many_arguments)]
    fn cuda_mgga_call(
        &self,
        npoints: usize,
        rho_ptr: *const f64,
        sigma_ptr: *const f64,
        lapl_ptr: *const f64,
        tau_ptr: *const f64,
        output_base: *mut f64,
        layout: &LibXCOutputLayout,
    ) {
        let ptr_for = |name: &str| -> *mut f64 {
            match layout.get(name) {
                Some(range) => unsafe { output_base.add(range.start) },
                None => std::ptr::null_mut::<f64>(),
            }
        };

        unsafe {
            ffi::xc_mgga(
                self.ptr,
                npoints,
                rho_ptr,
                sigma_ptr as *mut f64,
                lapl_ptr as *mut f64,
                tau_ptr as *mut f64,
                ptr_for("zk"),
                ptr_for("vrho"),
                ptr_for("vsigma"),
                ptr_for("vlapl"),
                ptr_for("vtau"),
                ptr_for("v2rho2"),
                ptr_for("v2rhosigma"),
                ptr_for("v2rholapl"),
                ptr_for("v2rhotau"),
                ptr_for("v2sigma2"),
                ptr_for("v2sigmalapl"),
                ptr_for("v2sigmatau"),
                ptr_for("v2lapl2"),
                ptr_for("v2lapltau"),
                ptr_for("v2tau2"),
                ptr_for("v3rho3"),
                ptr_for("v3rho2sigma"),
                ptr_for("v3rho2lapl"),
                ptr_for("v3rho2tau"),
                ptr_for("v3rhosigma2"),
                ptr_for("v3rhosigmalapl"),
                ptr_for("v3rhosigmatau"),
                ptr_for("v3rholapl2"),
                ptr_for("v3rholapltau"),
                ptr_for("v3rhotau2"),
                ptr_for("v3sigma3"),
                ptr_for("v3sigma2lapl"),
                ptr_for("v3sigma2tau"),
                ptr_for("v3sigmalapl2"),
                ptr_for("v3sigmalapltau"),
                ptr_for("v3sigmatau2"),
                ptr_for("v3lapl3"),
                ptr_for("v3lapl2tau"),
                ptr_for("v3lapltau2"),
                ptr_for("v3tau3"),
                ptr_for("v4rho4"),
                ptr_for("v4rho3sigma"),
                ptr_for("v4rho3lapl"),
                ptr_for("v4rho3tau"),
                ptr_for("v4rho2sigma2"),
                ptr_for("v4rho2sigmalapl"),
                ptr_for("v4rho2sigmatau"),
                ptr_for("v4rho2lapl2"),
                ptr_for("v4rho2lapltau"),
                ptr_for("v4rho2tau2"),
                ptr_for("v4rhosigma3"),
                ptr_for("v4rhosigma2lapl"),
                ptr_for("v4rhosigma2tau"),
                ptr_for("v4rhosigmalapl2"),
                ptr_for("v4rhosigmalapltau"),
                ptr_for("v4rhosigmatau2"),
                ptr_for("v4rholapl3"),
                ptr_for("v4rholapl2tau"),
                ptr_for("v4rholapltau2"),
                ptr_for("v4rhotau3"),
                ptr_for("v4sigma4"),
                ptr_for("v4sigma3lapl"),
                ptr_for("v4sigma3tau"),
                ptr_for("v4sigma2lapl2"),
                ptr_for("v4sigma2lapltau"),
                ptr_for("v4sigma2tau2"),
                ptr_for("v4sigmalapl3"),
                ptr_for("v4sigmalapl2tau"),
                ptr_for("v4sigmalapltau2"),
                ptr_for("v4sigmatau3"),
                ptr_for("v4lapl4"),
                ptr_for("v4lapl3tau"),
                ptr_for("v4lapl2tau2"),
                ptr_for("v4lapltau3"),
                ptr_for("v4tau4"),
            );
        }
    }

    // -- MGGA CUDA compute -------------------------------------------------

    /// Compute MGGA functional on GPU with automatic device allocation.
    pub fn cuda_compute_mgga(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(CudaSlice<f64>, LibXCOutputLayout), LibXCError> {
        let (npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, layout) =
            self.cuda_mgga_prepare(input, flags, stream)?;
        let mut buffer = stream
            .alloc_zeros::<f64>(layout.total_size)
            .map_err(|e| LibXCError::CudaError(format!("CUDA allocation failed: {e}")))?;
        {
            let (output_base, _sync) = buffer.device_ptr_mut(stream);
            self.cuda_mgga_call(
                npoints,
                rho_ptr,
                sigma_ptr,
                lapl_ptr,
                tau_ptr,
                output_base as *mut f64,
                &layout,
            );
        }
        Ok((buffer, layout))
    }

    /// Compute MGGA functional on GPU with preallocated contiguous device
    /// buffer.
    pub fn cuda_compute_mgga_with_unsliced_output(
        &self,
        input: &LibXCCudaInput,
        output: &mut CudaSlice<f64>,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        let stream = output.stream().clone();
        let (npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, layout) =
            self.cuda_mgga_prepare(input, deriv_flags, &stream)?;
        if output.len() < layout.total_size {
            return Err(LibXCError::ComputeError(format!(
                "output buffer has too small size: expected {}, got {}",
                layout.total_size,
                output.len()
            )));
        }
        let (output_base, _sync) = output.device_ptr_mut(&stream);
        self.cuda_mgga_call(
            npoints,
            rho_ptr,
            sigma_ptr,
            lapl_ptr,
            tau_ptr,
            output_base as *mut f64,
            &layout,
        );
        Ok(layout)
    }

    /// Compute MGGA functional on GPU with named per-component output buffers.
    pub fn cuda_compute_mgga_with_output(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        output: &mut LibXCCudaOutputMut,
    ) -> Result<(), LibXCError> {
        let view = input.get("rho").ok_or_else(|| {
            LibXCError::ComputeError("rho: required CUDA input not provided".into())
        })?;
        guard_on_device(self)?;
        let nspin = self.spin() as usize;
        if view.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = view.len() / nspin;
        let rho_ptr = cuda_view_ptr(view, stream);
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();
        let sigma_ptr = require_cuda_input_ptr(input, "sigma", npoints, dim.sigma, stream)?;
        let lapl_ptr =
            conditional_cuda_input_ptr(input, "lapl", npoints, dim.lapl, needs_lapl, stream)?;
        let tau_ptr =
            conditional_cuda_input_ptr(input, "tau", npoints, dim.tau, needs_tau, stream)?;
        let ptrs = validate_cuda_output_ptrs(output, &MGGA_OUTPUT_LABELS, npoints, dim, stream)?;

        unsafe {
            ffi::xc_mgga(
                self.ptr,
                npoints,
                rho_ptr,
                sigma_ptr as *mut f64,
                lapl_ptr as *mut f64,
                tau_ptr as *mut f64,
                ptr_of(&ptrs, "zk"),
                ptr_of(&ptrs, "vrho"),
                ptr_of(&ptrs, "vsigma"),
                ptr_of(&ptrs, "vlapl"),
                ptr_of(&ptrs, "vtau"),
                ptr_of(&ptrs, "v2rho2"),
                ptr_of(&ptrs, "v2rhosigma"),
                ptr_of(&ptrs, "v2rholapl"),
                ptr_of(&ptrs, "v2rhotau"),
                ptr_of(&ptrs, "v2sigma2"),
                ptr_of(&ptrs, "v2sigmalapl"),
                ptr_of(&ptrs, "v2sigmatau"),
                ptr_of(&ptrs, "v2lapl2"),
                ptr_of(&ptrs, "v2lapltau"),
                ptr_of(&ptrs, "v2tau2"),
                ptr_of(&ptrs, "v3rho3"),
                ptr_of(&ptrs, "v3rho2sigma"),
                ptr_of(&ptrs, "v3rho2lapl"),
                ptr_of(&ptrs, "v3rho2tau"),
                ptr_of(&ptrs, "v3rhosigma2"),
                ptr_of(&ptrs, "v3rhosigmalapl"),
                ptr_of(&ptrs, "v3rhosigmatau"),
                ptr_of(&ptrs, "v3rholapl2"),
                ptr_of(&ptrs, "v3rholapltau"),
                ptr_of(&ptrs, "v3rhotau2"),
                ptr_of(&ptrs, "v3sigma3"),
                ptr_of(&ptrs, "v3sigma2lapl"),
                ptr_of(&ptrs, "v3sigma2tau"),
                ptr_of(&ptrs, "v3sigmalapl2"),
                ptr_of(&ptrs, "v3sigmalapltau"),
                ptr_of(&ptrs, "v3sigmatau2"),
                ptr_of(&ptrs, "v3lapl3"),
                ptr_of(&ptrs, "v3lapl2tau"),
                ptr_of(&ptrs, "v3lapltau2"),
                ptr_of(&ptrs, "v3tau3"),
                ptr_of(&ptrs, "v4rho4"),
                ptr_of(&ptrs, "v4rho3sigma"),
                ptr_of(&ptrs, "v4rho3lapl"),
                ptr_of(&ptrs, "v4rho3tau"),
                ptr_of(&ptrs, "v4rho2sigma2"),
                ptr_of(&ptrs, "v4rho2sigmalapl"),
                ptr_of(&ptrs, "v4rho2sigmatau"),
                ptr_of(&ptrs, "v4rho2lapl2"),
                ptr_of(&ptrs, "v4rho2lapltau"),
                ptr_of(&ptrs, "v4rho2tau2"),
                ptr_of(&ptrs, "v4rhosigma3"),
                ptr_of(&ptrs, "v4rhosigma2lapl"),
                ptr_of(&ptrs, "v4rhosigma2tau"),
                ptr_of(&ptrs, "v4rhosigmalapl2"),
                ptr_of(&ptrs, "v4rhosigmalapltau"),
                ptr_of(&ptrs, "v4rhosigmatau2"),
                ptr_of(&ptrs, "v4rholapl3"),
                ptr_of(&ptrs, "v4rholapl2tau"),
                ptr_of(&ptrs, "v4rholapltau2"),
                ptr_of(&ptrs, "v4rhotau3"),
                ptr_of(&ptrs, "v4sigma4"),
                ptr_of(&ptrs, "v4sigma3lapl"),
                ptr_of(&ptrs, "v4sigma3tau"),
                ptr_of(&ptrs, "v4sigma2lapl2"),
                ptr_of(&ptrs, "v4sigma2lapltau"),
                ptr_of(&ptrs, "v4sigma2tau2"),
                ptr_of(&ptrs, "v4sigmalapl3"),
                ptr_of(&ptrs, "v4sigmalapl2tau"),
                ptr_of(&ptrs, "v4sigmalapltau2"),
                ptr_of(&ptrs, "v4sigmatau3"),
                ptr_of(&ptrs, "v4lapl4"),
                ptr_of(&ptrs, "v4lapl3tau"),
                ptr_of(&ptrs, "v4lapl2tau2"),
                ptr_of(&ptrs, "v4lapltau3"),
                ptr_of(&ptrs, "v4tau4"),
            );
        }
        Ok(())
    }

    // -- Unified CUDA dispatch ---------------------------------------------

    /// Compute the functional on GPU with automatic device allocation,
    /// dispatching by family.
    ///
    /// The functional must be initialized for GPU execution via
    /// [`from_identifier_with_device`](Self::from_identifier_with_device)
    /// with `LibXCDeviceFlag::OnDevice`.
    ///
    /// # Input keys
    ///
    /// Same as CPU compute:
    /// - LDA: `"rho"`
    /// - GGA: `"rho"`, `"sigma"`
    /// - MGGA: `"rho"`, `"sigma"`, `"tau"` (and `"lapl"` if needed)
    ///
    /// # Output
    ///
    /// Returns `(CudaSlice<f64>, LibXCOutputLayout)` where the slice is a
    /// contiguous GPU buffer. Use `layout.get("zk")` etc. to extract ranges.
    pub fn cuda_compute_xc(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(CudaSlice<f64>, LibXCOutputLayout), LibXCError> {
        use crate::prelude::libxc_enum_items::*;
        match self.family() {
            LDA | HybLDA => self.cuda_compute_lda(stream, input, flags),
            GGA | HybGGA => self.cuda_compute_gga(stream, input, flags),
            MGGA | HybMGGA => self.cuda_compute_mgga(stream, input, flags),
            OEP | LCA => Err(LibXCError::ComputeError(
                "cuda_compute_xc: OEP/LCA family is not supported".into(),
            )),
        }
    }

    /// Compute the functional on GPU into a user-provided contiguous device
    /// buffer, dispatching by family.
    pub fn cuda_compute_xc_with_unsliced_output(
        &self,
        input: &LibXCCudaInput,
        output: &mut CudaSlice<f64>,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        use crate::prelude::libxc_enum_items::*;
        match self.family() {
            LDA | HybLDA => self.cuda_compute_lda_with_unsliced_output(input, output, deriv_flags),
            GGA | HybGGA => self.cuda_compute_gga_with_unsliced_output(input, output, deriv_flags),
            MGGA | HybMGGA => {
                self.cuda_compute_mgga_with_unsliced_output(input, output, deriv_flags)
            },
            OEP | LCA => Err(LibXCError::ComputeError(
                "cuda_compute_xc: OEP/LCA family is not supported".into(),
            )),
        }
    }

    /// Compute the functional on GPU into named per-component output buffers,
    /// dispatching by family.
    pub fn cuda_compute_xc_with_output(
        &self,
        stream: &Arc<CudaStream>,
        input: &LibXCCudaInput,
        output: &mut LibXCCudaOutputMut,
    ) -> Result<(), LibXCError> {
        use crate::prelude::libxc_enum_items::*;
        match self.family() {
            LDA | HybLDA => self.cuda_compute_lda_with_output(stream, input, output),
            GGA | HybGGA => self.cuda_compute_gga_with_output(stream, input, output),
            MGGA | HybMGGA => self.cuda_compute_mgga_with_output(stream, input, output),
            OEP | LCA => Err(LibXCError::ComputeError(
                "cuda_compute_xc: OEP/LCA family is not supported".into(),
            )),
        }
    }
}
