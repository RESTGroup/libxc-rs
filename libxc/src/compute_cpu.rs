//! CPU wrapper for libxc functionals.

use crate::prelude::*;

/// Unified input map for all functional families.
///
/// Keys are `"rho"`, `"sigma"`, `"lapl"`, `"tau"`.
/// Required keys depend on the functional family:
/// - LDA: `"rho"`
/// - GGA: `"rho"`, `"sigma"`
/// - MGGA: `"rho"`, `"sigma"`; `"lapl"` and `"tau"` if the functional needs
///   them
pub type LibXCCpuInput<'a> = HashMap<&'static str, &'a [f64]>;

/// Extract a required input slice from the map.
fn require_input<'a>(
    input: &LibXCCpuInput<'a>,
    key: &'static str,
) -> Result<&'a [f64], LibXCError> {
    input
        .get(&key)
        .map(|s| *s)
        .ok_or_else(|| LibXCError::ComputeError(format!("{key}: required input not provided")))
}

/// Validate a required input's size and return a const pointer.
fn require_input_ptr(
    input: &LibXCCpuInput,
    key: &'static str,
    npoints: usize,
    expected_dim: i32,
) -> Result<*const f64, LibXCError> {
    let slice = require_input(input, key)?;
    let expected = npoints * (expected_dim as usize);
    if slice.len() != expected {
        return Err(LibXCError::ComputeError(format!(
            "{key}: expected size {expected}, got {}",
            slice.len()
        )));
    }
    Ok(slice.as_ptr())
}

/// Validate a conditionally-required input and return a const pointer.
/// Returns null if `required` is false; errors if required but absent.
fn conditional_input_ptr(
    input: &LibXCCpuInput,
    key: &'static str,
    npoints: usize,
    expected_dim: i32,
    required: bool,
) -> Result<*const f64, LibXCError> {
    match (input.get(key).copied(), required) {
        (Some(slice), true) => {
            let expected = npoints * (expected_dim as usize);
            if slice.len() != expected {
                return Err(LibXCError::ComputeError(format!(
                    "{key}: expected size {expected}, got {}",
                    slice.len()
                )));
            }
            Ok(slice.as_ptr())
        },
        (None, true) => {
            Err(LibXCError::ComputeError(format!("{key}: required input not provided")))
        },
        (_, false) => Ok(std::ptr::null()),
    }
}

// ---------------------------------------------------------------------------
// Preallocated output types
// ---------------------------------------------------------------------------

/// Unified output map for all functional families (preallocated buffers).
///
/// Keys are derivative component names (e.g. `"zk"`, `"vrho"`, `"v2rho2"`,
/// ...). Key present = user provides that buffer; absent = null pointer passed
/// to libxc.
pub type LibXCCpuOutputMut<'a> = HashMap<&'static str, &'a mut [f64]>;

/// Validate an output slice from the map and return a mutable pointer.
/// Returns null if the key is absent; validates size if present.
fn validate_output_ptr(
    output: &LibXCCpuOutputMut,
    key: &'static str,
    npoints: usize,
    expected_dim: i32,
) -> Result<*mut f64, LibXCError> {
    match output.get(key) {
        Some(s) => {
            let expected = npoints * (expected_dim as usize);
            if s.len() != expected {
                return Err(LibXCError::ComputeError(format!(
                    "{key}: expected size {expected}, got {}",
                    s.len()
                )));
            }
            Ok(s.as_ptr() as *mut f64)
        },
        None => Ok(std::ptr::null_mut::<f64>()),
    }
}

/// Validate all output entries for a given label set, returning a pointer map.
fn validate_output_ptrs(
    output: &LibXCCpuOutputMut,
    labels: &[&'static str],
    npoints: usize,
    dim: &ffi::xc_dimensions,
) -> Result<HashMap<&'static str, *mut f64>, LibXCError> {
    let mut ptrs = HashMap::new();
    for &label in labels {
        let d = crate::layout_handling::get_dim(dim, label);
        ptrs.insert(label, validate_output_ptr(output, label, npoints, d)?);
    }
    Ok(ptrs)
}

/// Helper to get a pointer from the validated map, defaulting to null.
fn ptr_of(ptrs: &HashMap<&'static str, *mut f64>, key: &str) -> *mut f64 {
    ptrs.get(key).copied().unwrap_or(std::ptr::null_mut())
}

impl LibXCFunctional {
    // -- LDA private helpers -----------------------------------------------

    /// Validate input and compute output layout for LDA.
    /// Returns `(npoints, rho_ptr, layout)`.
    fn lda_prepare(
        &self,
        input: &LibXCCpuInput,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(usize, *const f64, LibXCOutputLayout), LibXCError> {
        let flags = deriv_flags.into();
        self.validate_flags(flags)?;
        let rho = require_input(input, "rho")?;
        let nspin = self.spin() as usize;
        if rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = rho.len() / nspin;
        let layout = self.lda_output_layout(npoints, flags);
        Ok((npoints, rho.as_ptr(), layout))
    }

    /// Invoke `xc_lda` FFI call, writing into `output` according to `layout`.
    fn lda_call(
        &self,
        npoints: usize,
        rho_ptr: *const f64,
        output: &mut [f64],
        layout: &LibXCOutputLayout,
    ) {
        let mut ptr_for = |name: &str| -> *mut f64 {
            match layout.get(name) {
                Some(range) => unsafe { output.as_mut_ptr().add(range.start) },
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

    // -- LDA compute --------------------------------------------------------

    /// Compute LDA functional with preallocated output buffer slice. Validates
    /// buffer sizes and passes null for absent components.
    pub fn compute_lda_with_unsliced_output(
        &self,
        input: &LibXCCpuInput,
        output: &mut [f64],
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        let (npoints, rho_ptr, layout) = self.lda_prepare(input, deriv_flags)?;
        if output.len() < layout.total_size {
            return Err(LibXCError::ComputeError(format!(
                "output buffer has too small size: expected {}, got {}",
                layout.total_size,
                output.len()
            )));
        }
        self.lda_call(npoints, rho_ptr, output, &layout);
        Ok(layout)
    }

    /// Compute LDA functional with automatic allocation.
    /// Returns `(buffer, layout)` where buffer is a contiguous f64 array.
    pub fn compute_lda(
        &self,
        input: &LibXCCpuInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        let (npoints, rho_ptr, layout) = self.lda_prepare(input, flags)?;
        let mut buffer = vec![0.0f64; layout.total_size];
        self.lda_call(npoints, rho_ptr, &mut buffer, &layout);
        Ok((buffer, layout))
    }

    /// Compute LDA functional with user-preallocated output buffers.
    pub fn compute_lda_with_output(
        &self,
        input: &LibXCCpuInput,
        output: &LibXCCpuOutputMut,
    ) -> Result<(), LibXCError> {
        let rho = require_input(input, "rho")?;
        let nspin = self.spin() as usize;
        if rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = rho.len() / nspin;
        let rho_ptr = rho.as_ptr();
        let dim = self.dim();
        let ptrs = validate_output_ptrs(output, &LDA_OUTPUT_LABELS, npoints, &dim)?;

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

    // -- GGA private helpers -----------------------------------------------

    /// Validate input and compute output layout for GGA.
    /// Returns `(npoints, rho_ptr, sigma_ptr, layout)`.
    fn gga_prepare(
        &self,
        input: &LibXCCpuInput,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(usize, *const f64, *const f64, LibXCOutputLayout), LibXCError> {
        let flags = deriv_flags.into();
        self.validate_flags(flags)?;
        let rho = require_input(input, "rho")?;
        let nspin = self.spin() as usize;
        if rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = rho.len() / nspin;
        let dim = self.dim();
        let sigma_ptr = require_input_ptr(input, "sigma", npoints, dim.sigma)?;
        let layout = self.gga_output_layout(npoints, flags);
        Ok((npoints, rho.as_ptr(), sigma_ptr, layout))
    }

    /// Invoke `xc_gga` FFI call, writing into `output` according to `layout`.
    fn gga_call(
        &self,
        npoints: usize,
        rho_ptr: *const f64,
        sigma_ptr: *const f64,
        output: &mut [f64],
        layout: &LibXCOutputLayout,
    ) {
        let mut ptr_for = |name: &str| -> *mut f64 {
            match layout.get(name) {
                Some(range) => unsafe { output.as_mut_ptr().add(range.start) },
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

    // -- GGA compute --------------------------------------------------------

    /// Compute GGA functional with preallocated output buffer slice. Validates
    /// buffer sizes and passes null for absent components.
    pub fn compute_gga_with_unsliced_output(
        &self,
        input: &LibXCCpuInput,
        output: &mut [f64],
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        let (npoints, rho_ptr, sigma_ptr, layout) = self.gga_prepare(input, deriv_flags)?;
        if output.len() < layout.total_size {
            return Err(LibXCError::ComputeError(format!(
                "output buffer has too small size: expected {}, got {}",
                layout.total_size,
                output.len()
            )));
        }
        self.gga_call(npoints, rho_ptr, sigma_ptr, output, &layout);
        Ok(layout)
    }

    /// Compute GGA functional with automatic allocation.
    pub fn compute_gga(
        &self,
        input: &LibXCCpuInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        let (npoints, rho_ptr, sigma_ptr, layout) = self.gga_prepare(input, flags)?;
        let mut buffer = vec![0.0f64; layout.total_size];
        self.gga_call(npoints, rho_ptr, sigma_ptr, &mut buffer, &layout);
        Ok((buffer, layout))
    }

    /// Compute GGA functional with user-preallocated output buffers.
    pub fn compute_gga_with_output(
        &self,
        input: &LibXCCpuInput,
        output: &LibXCCpuOutputMut,
    ) -> Result<(), LibXCError> {
        let rho = require_input(input, "rho")?;
        let nspin = self.spin() as usize;
        if rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = rho.len() / nspin;
        let rho_ptr = rho.as_ptr();
        let dim = self.dim();
        let sigma_ptr = require_input_ptr(input, "sigma", npoints, dim.sigma)?;
        let ptrs = validate_output_ptrs(output, &GGA_OUTPUT_LABELS, npoints, &dim)?;

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

    // -- MGGA private helpers ----------------------------------------------

    /// Validate input and compute output layout for MGGA.
    /// Returns `(npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, layout)`.
    fn mgga_prepare(
        &self,
        input: &LibXCCpuInput,
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<
        (usize, *const f64, *const f64, *const f64, *const f64, LibXCOutputLayout),
        LibXCError,
    > {
        let flags = deriv_flags.into();
        self.validate_flags(flags)?;
        let rho = require_input(input, "rho")?;
        let nspin = self.spin() as usize;
        if rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = rho.len() / nspin;
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();
        let sigma_ptr = require_input_ptr(input, "sigma", npoints, dim.sigma)?;
        let lapl_ptr = conditional_input_ptr(input, "lapl", npoints, dim.lapl, needs_lapl)?;
        let tau_ptr = conditional_input_ptr(input, "tau", npoints, dim.tau, needs_tau)?;
        let layout = self.mgga_output_layout(npoints, flags);
        Ok((npoints, rho.as_ptr(), sigma_ptr, lapl_ptr, tau_ptr, layout))
    }

    /// Invoke `xc_mgga` FFI call, writing into `output` according to `layout`.
    fn mgga_call(
        &self,
        npoints: usize,
        rho_ptr: *const f64,
        sigma_ptr: *const f64,
        lapl_ptr: *const f64,
        tau_ptr: *const f64,
        output: &mut [f64],
        layout: &LibXCOutputLayout,
    ) {
        let mut ptr_for = |name: &str| -> *mut f64 {
            match layout.get(name) {
                Some(range) => unsafe { output.as_mut_ptr().add(range.start) },
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

    // -- MGGA compute -------------------------------------------------------

    /// Compute MGGA functional with preallocated output buffer slice. Validates
    /// buffer sizes and passes null for absent components.
    pub fn compute_mgga_with_unsliced_output(
        &self,
        input: &LibXCCpuInput,
        output: &mut [f64],
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        let (npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, layout) =
            self.mgga_prepare(input, deriv_flags)?;
        if output.len() < layout.total_size {
            return Err(LibXCError::ComputeError(format!(
                "output buffer has too small size: expected {}, got {}",
                layout.total_size,
                output.len()
            )));
        }
        self.mgga_call(npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, output, &layout);
        Ok(layout)
    }

    /// Compute MGGA functional with automatic allocation.
    pub fn compute_mgga(
        &self,
        input: &LibXCCpuInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        let (npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, layout) =
            self.mgga_prepare(input, flags)?;
        let mut buffer = vec![0.0f64; layout.total_size];
        self.mgga_call(npoints, rho_ptr, sigma_ptr, lapl_ptr, tau_ptr, &mut buffer, &layout);
        Ok((buffer, layout))
    }

    /// Compute MGGA functional with user-preallocated output buffers.
    pub fn compute_mgga_with_output(
        &self,
        input: &LibXCCpuInput,
        output: &LibXCCpuOutputMut,
    ) -> Result<(), LibXCError> {
        let rho = require_input(input, "rho")?;
        let nspin = self.spin() as usize;
        if rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = rho.len() / nspin;
        let rho_ptr = rho.as_ptr();
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();
        let sigma_ptr = require_input_ptr(input, "sigma", npoints, dim.sigma)?;
        let lapl_ptr = conditional_input_ptr(input, "lapl", npoints, dim.lapl, needs_lapl)?;
        let tau_ptr = conditional_input_ptr(input, "tau", npoints, dim.tau, needs_tau)?;
        let ptrs = validate_output_ptrs(output, &MGGA_OUTPUT_LABELS, npoints, &dim)?;

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

    // -- Unified dispatch ---------------------------------------------------

    /// Compute functional with automatic allocation, dispatching by family.
    ///
    /// Inspects `self.family()` and delegates to `compute_lda`, `compute_gga`,
    /// or `compute_mgga` accordingly. Hybrid families (HybLDA, HybGGA, HybMGGA)
    /// are dispatched to their base family's compute.
    pub fn compute_xc(
        &self,
        input: &LibXCCpuInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        use crate::prelude::libxc_enum_items::*;
        match self.family() {
            LDA | HybLDA => self.compute_lda(input, flags),
            GGA | HybGGA => self.compute_gga(input, flags),
            MGGA | HybMGGA => self.compute_mgga(input, flags),
            OEP | LCA => Err(LibXCError::ComputeError(
                "compute_xc: OEP/LCA family is not supported".into(),
            )),
        }
    }

    /// Compute functional with preallocated output buffer slice, dispatching by
    /// family.
    pub fn compute_xc_with_unsliced_output(
        &self,
        input: &LibXCCpuInput,
        output: &mut [f64],
        deriv_flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<LibXCOutputLayout, LibXCError> {
        use crate::prelude::libxc_enum_items::*;
        match self.family() {
            LDA | HybLDA => self.compute_lda_with_unsliced_output(input, output, deriv_flags),
            GGA | HybGGA => self.compute_gga_with_unsliced_output(input, output, deriv_flags),
            MGGA | HybMGGA => self.compute_mgga_with_unsliced_output(input, output, deriv_flags),
            OEP | LCA => Err(LibXCError::ComputeError(
                "compute_xc: OEP/LCA family is not supported".into(),
            )),
        }
    }

    /// Compute functional with user-preallocated output buffers, dispatching by
    /// family.
    pub fn compute_xc_with_output(
        &self,
        input: &LibXCCpuInput,
        output: &LibXCCpuOutputMut,
    ) -> Result<(), LibXCError> {
        use crate::prelude::libxc_enum_items::*;
        match self.family() {
            LDA | HybLDA => self.compute_lda_with_output(input, output),
            GGA | HybGGA => self.compute_gga_with_output(input, output),
            MGGA | HybMGGA => self.compute_mgga_with_output(input, output),
            OEP | LCA => Err(LibXCError::ComputeError(
                "compute_xc: OEP/LCA family is not supported".into(),
            )),
        }
    }
}
