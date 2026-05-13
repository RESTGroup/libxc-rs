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
pub type LibXCCpuInput<'a> = HashMap<String, &'a [f64]>;

/// Extract a required input slice from the map.
fn require_input<'a>(input: &LibXCCpuInput<'a>, key: &str) -> Result<&'a [f64], LibXCError> {
    input
        .get(key)
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
pub type LibXCCpuOutputMut<'a> = HashMap<String, &'a mut [f64]>;

/// Validate an output slice from the map and return a mutable pointer.
/// Returns null if the key is absent; validates size if present.
fn validate_output_ptr(
    output: &LibXCCpuOutputMut,
    key: &str,
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

    /// Compute the functional with automatic allocation, dispatching by family.
    ///
    /// Inspects `self.family()` and delegates to
    /// [`compute_lda`](Self::compute_lda),
    /// [`compute_gga`](Self::compute_gga), or
    /// [`compute_mgga`](Self::compute_mgga) accordingly. Hybrid families
    /// (HybLDA, HybGGA, HybMGGA) are dispatched to their base family's
    /// compute.
    ///
    /// Returns `(buffer, layout)` where `buffer` is a contiguous `Vec<f64>` and
    /// `layout` describes how to index named components (e.g. `"zk"`, `"vrho"`)
    /// within it. Use [`LibXCOutputLayout::get`] to extract a `Range<usize>`
    /// for each component.
    ///
    /// The `flags` parameter controls which derivative levels to compute:
    /// - `0`: energy only (EXC)
    /// - `1`: energy + first derivative (EXC + VXC)
    /// - `2`: up to second derivative (EXC + VXC + FXC)
    /// - `3`: up to third derivative (EXC + VXC + FXC + KXC)
    /// - `4`: up to fourth derivative (EXC + VXC + FXC + KXC + LXC)
    ///
    /// You can also pass a [`LibXCDerivativeFlags`] struct for fine-grained
    /// control over individual derivative levels.
    ///
    /// # Input keys
    ///
    /// | Family | Required keys |
    /// |--------|--------------|
    /// | LDA    | `"rho"` |
    /// | GGA    | `"rho"`, `"sigma"` |
    /// | MGGA   | `"rho"`, `"sigma"`, `"tau"` (and `"lapl"` if the functional needs it) |
    ///
    /// Input arrays use row-major order `[n_comp, npoints]` with the last
    /// dimension contiguous. For unpolarized calculations, `"rho"` has shape
    /// `[npoints]`; for polarized, `[2 * npoints]` (alpha then beta).
    ///
    /// # Output keys
    ///
    /// | Level | LDA components | GGA adds | MGGA adds |
    /// |-------|---------------|----------|-----------|
    /// | EXC   | `zk` | `zk` | `zk` |
    /// | VXC   | `vrho` | `vrho`, `vsigma` | `vrho`, `vsigma`, `vlapl`, `vtau` |
    /// | FXC   | `v2rho2` | `v2rho2`, `v2rhosigma`, `v2sigma2` | (10 components) |
    /// | KXC   | `v3rho3` | (4 components) | (20 components) |
    /// | LXC   | `v4rho4` | (5 components) | (35 components) |
    ///
    /// # Errors
    ///
    /// Returns [`LibXCError::ComputeError`] if:
    /// - A required input key is missing
    /// - An input array has the wrong size
    /// - A requested derivative level is not supported by the functional
    ///
    /// # Examples
    ///
    /// LDA exchange on 5 grid points (unpolarized, derivative level 1):
    ///
    /// ```
    /// use libxc::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let func = LibXCFunctional::from_identifier("lda_x", LibXCSpin::Unpolarized);
    /// let rho = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    /// let input = HashMap::from([("rho".to_string(), rho.as_slice())]);
    ///
    /// let (buf, layout) = func.compute_xc(&input, 1).unwrap();
    ///
    /// // Extract the energy-per-particle (zk) array:
    /// let zk = &buf[layout.get("zk").unwrap()]; // length = 5
    /// assert!((zk[0] - (-0.3428)).abs() < 1e-3);
    ///
    /// // Extract the first derivative (vrho) array:
    /// let vrho = &buf[layout.get("vrho").unwrap()]; // length = 5
    /// assert!((vrho[0] - (-0.4571)).abs() < 1e-3);
    /// ```
    ///
    /// GGA exchange (PBE) requires `"sigma"` in addition to `"rho"`:
    ///
    /// ```
    /// use libxc::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let func = LibXCFunctional::from_identifier("gga_x_pbe", LibXCSpin::Unpolarized);
    /// let rho = vec![0.1, 0.2, 0.3];
    /// let sigma = vec![0.01, 0.02, 0.03];
    /// let input = HashMap::from([
    ///     ("rho".to_string(), rho.as_slice()),
    ///     ("sigma".to_string(), sigma.as_slice()),
    /// ]);
    ///
    /// let (buf, layout) = func.compute_xc(&input, 1).unwrap();
    /// let zk = &buf[layout.get("zk").unwrap()];
    /// let vrho = &buf[layout.get("vrho").unwrap()];
    /// let vsigma = &buf[layout.get("vsigma").unwrap()];
    /// ```
    ///
    /// MGGA correlation (TPSS) requires `"tau"` (and `"lapl"` if the functional
    /// needs the laplacian):
    ///
    /// ```
    /// use libxc::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let func = LibXCFunctional::from_identifier("mgga_c_tpss", LibXCSpin::Unpolarized);
    /// let rho = vec![0.1, 0.2, 0.3];
    /// let sigma = vec![0.01, 0.02, 0.03];
    /// let tau = vec![0.05, 0.1, 0.15];
    /// let input = HashMap::from([
    ///     ("rho".to_string(), rho.as_slice()),
    ///     ("sigma".to_string(), sigma.as_slice()),
    ///     ("tau".to_string(), tau.as_slice()),
    /// ]);
    ///
    /// let (buf, layout) = func.compute_xc(&input, 1).unwrap();
    /// let vtau = &buf[layout.get("vtau").unwrap()];
    /// ```
    ///
    /// Higher derivative levels include more output components:
    ///
    /// ```
    /// use libxc::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let func = LibXCFunctional::from_identifier("lda_x", LibXCSpin::Unpolarized);
    /// let rho = vec![0.1, 0.2, 0.3];
    /// let input = HashMap::from([("rho".to_string(), rho.as_slice())]);
    ///
    /// // Level 2 = EXC + VXC + FXC
    /// let (buf, layout) = func.compute_xc(&input, 2).unwrap();
    /// let v2rho2 = &buf[layout.get("v2rho2").unwrap()];
    /// ```
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
            OEP | LCA => {
                Err(LibXCError::ComputeError("compute_xc: OEP/LCA family is not supported".into()))
            },
        }
    }

    /// Compute the functional into a user-provided contiguous buffer,
    /// dispatching by family.
    ///
    /// This is the preallocated variant of [`compute_xc`](Self::compute_xc):
    /// instead of allocating a new `Vec<f64>`, the caller provides a `&mut
    /// [f64]` buffer. The buffer must be at least `layout.total_size`
    /// elements long, where `layout` can be obtained from
    /// [`output_layout`](Self::output_layout).
    ///
    /// Returns the [`LibXCOutputLayout`] that describes how to index named
    /// components within the buffer. Unused trailing elements (if the buffer is
    /// larger than `total_size`) are left unchanged.
    ///
    /// This variant is useful when you want to reuse the same buffer across
    /// multiple evaluations (e.g. in a SCF loop) to avoid repeated allocations.
    ///
    /// # Errors
    ///
    /// Returns [`LibXCError::ComputeError`] if the buffer is too small or if
    /// any input validation fails (same conditions as
    /// [`compute_xc`](Self::compute_xc)).
    ///
    /// # Examples
    ///
    /// ```
    /// use libxc::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let func = LibXCFunctional::from_identifier("lda_x", LibXCSpin::Unpolarized);
    /// let rho = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    /// let input = HashMap::from([("rho".to_string(), rho.as_slice())]);
    ///
    /// // Get the layout to know the required buffer size
    /// let layout = func.output_layout(5, 1);
    /// let mut buf = vec![0.0f64; layout.total_size];
    ///
    /// let layout = func.compute_xc_with_unsliced_output(&input, &mut buf, 1).unwrap();
    ///
    /// let zk = &buf[layout.get("zk").unwrap()];
    /// assert!((zk[0] - (-0.3428)).abs() < 1e-3);
    /// ```
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
            OEP | LCA => {
                Err(LibXCError::ComputeError("compute_xc: OEP/LCA family is not supported".into()))
            },
        }
    }

    /// Compute the functional into named user-provided output buffers,
    /// dispatching by family.
    ///
    /// This is the most explicit compute variant: the caller provides a
    /// [`LibXCCpuOutputMut`] (a `HashMap<String, &mut [f64]>`) whose keys are
    /// the output component names (e.g. `"zk"`, `"vrho"`, `"vsigma"`). Only the
    /// components present in the map are computed; absent keys are passed as
    /// null pointers to libxc.
    ///
    /// Unlike [`compute_xc`](Self::compute_xc) and
    /// [`compute_xc_with_unsliced_output`](Self::compute_xc_with_unsliced_output),
    /// this method does **not** take a `deriv_flags` parameter — the derivative
    /// level is implicitly determined by which output keys you provide.
    ///
    /// Each output buffer must have the correct size: `npoints * n_comp`, where
    /// `n_comp` depends on the component and spin polarization. For example,
    /// for an unpolarized GGA functional on 5 grid points, `"vrho"` needs 5
    /// elements and `"vsigma"` needs 5 elements; for a polarized GGA,
    /// `"vrho"` needs 10 and `"vsigma"` needs 15.
    ///
    /// # Errors
    ///
    /// Returns [`LibXCError::ComputeError`] if any output buffer has the wrong
    /// size or if any input validation fails.
    ///
    /// # Examples
    ///
    /// LDA exchange, requesting only `"zk"` and `"vrho"`:
    ///
    /// ```
    /// use libxc::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let func = LibXCFunctional::from_identifier("lda_x", LibXCSpin::Unpolarized);
    /// let rho = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    /// let input = HashMap::from([("rho".to_string(), rho.as_slice())]);
    ///
    /// let mut zk = vec![0.0f64; 5];
    /// let mut vrho = vec![0.0f64; 5];
    /// let mut output = HashMap::new();
    /// output.insert("zk".to_string(), zk.as_mut_slice());
    /// output.insert("vrho".to_string(), vrho.as_mut_slice());
    ///
    /// func.compute_xc_with_output(&input, &mut output).unwrap();
    ///
    /// assert!((zk[0] - (-0.3428)).abs() < 1e-3);
    /// assert!((vrho[0] - (-0.4571)).abs() < 1e-3);
    /// ```
    ///
    /// GGA exchange (PBE), requesting `"zk"`, `"vrho"`, and `"vsigma"`:
    ///
    /// ```
    /// use libxc::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let func = LibXCFunctional::from_identifier("gga_x_pbe", LibXCSpin::Unpolarized);
    /// let rho = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    /// let sigma = vec![0.01, 0.02, 0.03, 0.04, 0.05];
    /// let input = HashMap::from([
    ///     ("rho".to_string(), rho.as_slice()),
    ///     ("sigma".to_string(), sigma.as_slice()),
    /// ]);
    ///
    /// let mut zk = vec![0.0f64; 5];
    /// let mut vrho = vec![0.0f64; 5];
    /// let mut vsigma = vec![0.0f64; 5];
    /// let mut output = HashMap::new();
    /// output.insert("zk".to_string(), zk.as_mut_slice());
    /// output.insert("vrho".to_string(), vrho.as_mut_slice());
    /// output.insert("vsigma".to_string(), vsigma.as_mut_slice());
    ///
    /// func.compute_xc_with_output(&input, &mut output).unwrap();
    ///
    /// assert!((zk[0] - (-0.3516)).abs() < 1e-3);
    /// assert!((vsigma[0] - (-0.0855)).abs() < 1e-3);
    /// ```
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
            OEP | LCA => {
                Err(LibXCError::ComputeError("compute_xc: OEP/LCA family is not supported".into()))
            },
        }
    }
}
