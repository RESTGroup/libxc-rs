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
    // -- LDA compute --------------------------------------------------------

    /// Compute LDA functional with automatic allocation.
    /// Returns `(buffer, layout)` where buffer is a contiguous f64 array.
    pub fn compute_lda(
        &self,
        input: &LibXCCpuInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        let flags = flags.into();
        self.validate_flags(flags)?;
        let rho = require_input(input, "rho")?;
        let nspin = self.spin() as usize;
        if rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = rho.len() / nspin;
        let rho_ptr = rho.as_ptr();
        let layout = self.lda_output_layout(npoints, flags);
        let mut buffer = vec![0.0f64; layout.total_size];

        // Build pointers for each output component
        let null = std::ptr::null_mut::<f64>();
        let base = buffer.as_mut_ptr();
        let ptr_for = |name: &str| -> *mut f64 {
            if let Some(range) = layout.get(name) {
                unsafe { base.add(range.start) }
            } else {
                null
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

    // -- GGA compute --------------------------------------------------------

    /// Compute GGA functional with automatic allocation.
    pub fn compute_gga(
        &self,
        input: &LibXCCpuInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        let flags = flags.into();
        self.validate_flags(flags)?;
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

        let layout = self.gga_output_layout(npoints, flags);
        let mut buffer = vec![0.0f64; layout.total_size];

        let null = std::ptr::null_mut::<f64>();
        let base = buffer.as_mut_ptr();
        let ptr_for = |name: &str| -> *mut f64 {
            if let Some(range) = layout.get(name) {
                unsafe { base.add(range.start) }
            } else {
                null
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

    // -- MGGA compute -------------------------------------------------------

    /// Compute MGGA functional with automatic allocation.
    pub fn compute_mgga(
        &self,
        input: &LibXCCpuInput,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        let flags = flags.into();
        self.validate_flags(flags)?;
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

        let layout = self.mgga_output_layout(npoints, flags);
        let mut buffer = vec![0.0f64; layout.total_size];

        let null = std::ptr::null_mut::<f64>();
        let base = buffer.as_mut_ptr();
        let ptr_for = |name: &str| -> *mut f64 {
            if let Some(range) = layout.get(name) {
                unsafe { base.add(range.start) }
            } else {
                null
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
}
