//! CPU wrapper for libxc functionals.

#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use std::ffi::CStr;
use std::fmt;
use std::ops::Range;

use enumflags2::BitFlags;
use libxc_ffi::ffi;

use crate::enums::{LibXCFamily, LibXCFlags, LibXCFunctionalKind, LibXCSpin};
use crate::error::LibXCError;

unsafe fn cstr_to_string(ptr: *const std::ffi::c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

// ---------------------------------------------------------------------------
// LibXCReference
// ---------------------------------------------------------------------------

/// A literature reference for a libxc functional.
#[derive(Debug, Clone)]
pub struct LibXCReference {
    pub ref_text: String,
    pub doi: String,
    pub bibtex: String,
    pub key: String,
}

// ---------------------------------------------------------------------------
// LibXCDerivativeFlags
// ---------------------------------------------------------------------------

/// Flags controlling which derivative levels to compute.
#[derive(Debug, Clone)]
pub struct LibXCDerivativeFlags {
    pub do_exc: bool,
    pub do_vxc: bool,
    pub do_fxc: bool,
    pub do_kxc: bool,
    pub do_lxc: bool,
}

impl Default for LibXCDerivativeFlags {
    fn default() -> Self {
        Self { do_exc: true, do_vxc: true, do_fxc: false, do_kxc: false, do_lxc: false }
    }
}

// ---------------------------------------------------------------------------
// LibXCOutputLayout
// ---------------------------------------------------------------------------

/// Describes the layout of a contiguous compute output buffer.
///
/// Each component (e.g. `"zk"`, `"vrho"`) occupies a contiguous range
/// `[offset .. offset + size)` within the buffer, stored in row-major order
/// `[n_comp, npoints]` where the last dimension is contiguous.
#[derive(Debug, Clone)]
pub struct LibXCOutputLayout {
    /// Total size of the buffer in f64 elements.
    pub total_size: usize,
    /// Number of grid points.
    pub npoints: usize,
    components: Vec<(&'static str, usize, usize)>, // (name, offset, size)
}

impl LibXCOutputLayout {
    fn new(npoints: usize) -> Self {
        Self { total_size: 0, npoints, components: Vec::new() }
    }

    fn push(&mut self, name: &'static str, n_comp: i32) {
        if n_comp == 0 {
            return;
        }
        let size = self.npoints * (n_comp as usize);
        let offset = self.total_size;
        self.components.push((name, offset, size));
        self.total_size += size;
    }

    /// Returns the byte range `[offset .. offset + size)` for a named
    /// component.
    pub fn get(&self, name: &str) -> Option<Range<usize>> {
        for &(n, offset, size) in &self.components {
            if n == name {
                return Some(offset..offset + size);
            }
        }
        None
    }

    /// Iterates over component names in order.
    pub fn component_names(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(|&(n, _, _)| n)
    }

    /// Returns the size (in f64 elements) for a named component.
    pub fn component_size(&self, name: &str) -> Option<usize> {
        for &(n, _, size) in &self.components {
            if n == name {
                return Some(size);
            }
        }
        None
    }

    /// Returns the number of components (not grid points) for a named
    /// component.
    pub fn component_dim(&self, name: &str) -> Option<usize> {
        for &(n, _, size) in &self.components {
            if n == name {
                return Some(size / self.npoints);
            }
        }
        None
    }
}

impl fmt::Display for LibXCOutputLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "LibXCOutputLayout (total_size={}, npoints={}):",
            self.total_size, self.npoints
        )?;
        for &(name, offset, size) in &self.components {
            writeln!(
                f,
                "  {name}: [{offset}..{}], size={size}, dim={}",
                offset + size,
                size / self.npoints
            )?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input for LDA computation.
pub struct LibXCLdaInput<'a> {
    pub rho: &'a [f64],
}

/// Input for GGA computation.
pub struct LibXCGgaInput<'a> {
    pub rho: &'a [f64],
    pub sigma: &'a [f64],
}

/// Input for MGGA computation.
pub struct LibXCMggaInput<'a> {
    pub rho: &'a [f64],
    pub sigma: &'a [f64],
    pub lapl: Option<&'a [f64]>,
    pub tau: Option<&'a [f64]>,
}

// ---------------------------------------------------------------------------
// Preallocated output types (Variant 2)
// ---------------------------------------------------------------------------

/// Preallocated output buffers for LDA computation.
pub struct LibXCLdaOutputMut<'a> {
    pub zk: Option<&'a mut [f64]>,
    pub vrho: Option<&'a mut [f64]>,
    pub v2rho2: Option<&'a mut [f64]>,
    pub v3rho3: Option<&'a mut [f64]>,
    pub v4rho4: Option<&'a mut [f64]>,
}

/// Preallocated output buffers for GGA computation.
pub struct LibXCGgaOutputMut<'a> {
    pub zk: Option<&'a mut [f64]>,
    pub vrho: Option<&'a mut [f64]>,
    pub vsigma: Option<&'a mut [f64]>,
    pub v2rho2: Option<&'a mut [f64]>,
    pub v2rhosigma: Option<&'a mut [f64]>,
    pub v2sigma2: Option<&'a mut [f64]>,
    pub v3rho3: Option<&'a mut [f64]>,
    pub v3rho2sigma: Option<&'a mut [f64]>,
    pub v3rhosigma2: Option<&'a mut [f64]>,
    pub v3sigma3: Option<&'a mut [f64]>,
    pub v4rho4: Option<&'a mut [f64]>,
    pub v4rho3sigma: Option<&'a mut [f64]>,
    pub v4rho2sigma2: Option<&'a mut [f64]>,
    pub v4rhosigma3: Option<&'a mut [f64]>,
    pub v4sigma4: Option<&'a mut [f64]>,
}

/// Preallocated output buffers for MGGA computation.
pub struct LibXCMggaOutputMut<'a> {
    pub zk: Option<&'a mut [f64]>,
    pub vrho: Option<&'a mut [f64]>,
    pub vsigma: Option<&'a mut [f64]>,
    pub vlapl: Option<&'a mut [f64]>,
    pub vtau: Option<&'a mut [f64]>,
    pub v2rho2: Option<&'a mut [f64]>,
    pub v2rhosigma: Option<&'a mut [f64]>,
    pub v2rholapl: Option<&'a mut [f64]>,
    pub v2rhotau: Option<&'a mut [f64]>,
    pub v2sigma2: Option<&'a mut [f64]>,
    pub v2sigmalapl: Option<&'a mut [f64]>,
    pub v2sigmatau: Option<&'a mut [f64]>,
    pub v2lapl2: Option<&'a mut [f64]>,
    pub v2lapltau: Option<&'a mut [f64]>,
    pub v2tau2: Option<&'a mut [f64]>,
    pub v3rho3: Option<&'a mut [f64]>,
    pub v3rho2sigma: Option<&'a mut [f64]>,
    pub v3rho2lapl: Option<&'a mut [f64]>,
    pub v3rho2tau: Option<&'a mut [f64]>,
    pub v3rhosigma2: Option<&'a mut [f64]>,
    pub v3rhosigmalapl: Option<&'a mut [f64]>,
    pub v3rhosigmatau: Option<&'a mut [f64]>,
    pub v3rholapl2: Option<&'a mut [f64]>,
    pub v3rholapltau: Option<&'a mut [f64]>,
    pub v3rhotau2: Option<&'a mut [f64]>,
    pub v3sigma3: Option<&'a mut [f64]>,
    pub v3sigma2lapl: Option<&'a mut [f64]>,
    pub v3sigma2tau: Option<&'a mut [f64]>,
    pub v3sigmalapl2: Option<&'a mut [f64]>,
    pub v3sigmalapltau: Option<&'a mut [f64]>,
    pub v3sigmatau2: Option<&'a mut [f64]>,
    pub v3lapl3: Option<&'a mut [f64]>,
    pub v3lapl2tau: Option<&'a mut [f64]>,
    pub v3lapltau2: Option<&'a mut [f64]>,
    pub v3tau3: Option<&'a mut [f64]>,
    pub v4rho4: Option<&'a mut [f64]>,
    pub v4rho3sigma: Option<&'a mut [f64]>,
    pub v4rho3lapl: Option<&'a mut [f64]>,
    pub v4rho3tau: Option<&'a mut [f64]>,
    pub v4rho2sigma2: Option<&'a mut [f64]>,
    pub v4rho2sigmalapl: Option<&'a mut [f64]>,
    pub v4rho2sigmatau: Option<&'a mut [f64]>,
    pub v4rho2lapl2: Option<&'a mut [f64]>,
    pub v4rho2lapltau: Option<&'a mut [f64]>,
    pub v4rho2tau2: Option<&'a mut [f64]>,
    pub v4rhosigma3: Option<&'a mut [f64]>,
    pub v4rhosigma2lapl: Option<&'a mut [f64]>,
    pub v4rhosigma2tau: Option<&'a mut [f64]>,
    pub v4rhosigmalapl2: Option<&'a mut [f64]>,
    pub v4rhosigmalapltau: Option<&'a mut [f64]>,
    pub v4rhosigmatau2: Option<&'a mut [f64]>,
    pub v4rholapl3: Option<&'a mut [f64]>,
    pub v4rholapl2tau: Option<&'a mut [f64]>,
    pub v4rholapltau2: Option<&'a mut [f64]>,
    pub v4rhotau3: Option<&'a mut [f64]>,
    pub v4sigma4: Option<&'a mut [f64]>,
    pub v4sigma3lapl: Option<&'a mut [f64]>,
    pub v4sigma3tau: Option<&'a mut [f64]>,
    pub v4sigma2lapl2: Option<&'a mut [f64]>,
    pub v4sigma2lapltau: Option<&'a mut [f64]>,
    pub v4sigma2tau2: Option<&'a mut [f64]>,
    pub v4sigmalapl3: Option<&'a mut [f64]>,
    pub v4sigmalapl2tau: Option<&'a mut [f64]>,
    pub v4sigmalapltau2: Option<&'a mut [f64]>,
    pub v4sigmatau3: Option<&'a mut [f64]>,
    pub v4lapl4: Option<&'a mut [f64]>,
    pub v4lapl3tau: Option<&'a mut [f64]>,
    pub v4lapl2tau2: Option<&'a mut [f64]>,
    pub v4lapltau3: Option<&'a mut [f64]>,
    pub v4tau4: Option<&'a mut [f64]>,
}

// ---------------------------------------------------------------------------
// LibXCFunctional
// ---------------------------------------------------------------------------

/// A safe wrapper around a libxc `xc_func_type` pointer.
///
/// Owns the underlying C resource and frees it on `Drop`.
pub struct LibXCFunctional {
    ptr: *mut ffi::xc_func_type,
}

impl LibXCFunctional {
    /// Create a new functional from a functional ID and spin configuration.
    ///
    /// For api-v7.1+, uses `xc_func_init_flags` with `OnHost`.
    /// For earlier versions, uses `xc_func_init`.
    pub fn new(func_id: i32, spin: LibXCSpin) -> Result<Self, LibXCError> {
        unsafe {
            let ptr = ffi::xc_func_alloc();
            if ptr.is_null() {
                return Err(LibXCError::InitError { func_id, spin });
            }
            let rc = Self::init_func(ptr, func_id, spin);
            if rc != 0 {
                ffi::xc_func_free(ptr);
                return Err(LibXCError::InitError { func_id, spin });
            }
            Ok(Self { ptr })
        }
    }

    /// Create a new functional from a name string and spin configuration.
    pub fn from_name(name: &str, spin: LibXCSpin) -> Result<Self, LibXCError> {
        let func_id = crate::util::libxc_functional_get_number(name)
            .ok_or_else(|| LibXCError::NotFound(format!("functional '{name}'")))?;
        Self::new(func_id, spin)
    }

    #[cfg(feature = "api-v7_1")]
    unsafe fn init_func(
        ptr: *mut ffi::xc_func_type,
        func_id: i32,
        spin: LibXCSpin,
    ) -> std::ffi::c_int {
        let flags = ffi::XC_FLAGS_ON_HOST as std::ffi::c_int;
        ffi::xc_func_init_flags(ptr, func_id as std::ffi::c_int, spin as std::ffi::c_int, flags)
    }

    #[cfg(not(feature = "api-v7_1"))]
    unsafe fn init_func(
        ptr: *mut ffi::xc_func_type,
        func_id: i32,
        spin: LibXCSpin,
    ) -> std::ffi::c_int {
        ffi::xc_func_init(ptr, func_id as std::ffi::c_int, spin as std::ffi::c_int)
    }

    /// Returns a raw pointer to the underlying `xc_func_type`.
    /// Intended for advanced use; the caller must not free the pointer.
    pub fn as_ptr(&self) -> *const ffi::xc_func_type {
        self.ptr
    }

    // -- Info getters -------------------------------------------------------

    fn info(&self) -> *const ffi::xc_func_info_type {
        unsafe { ffi::xc_func_get_info(self.ptr) }
    }

    /// Functional number (ID).
    pub fn number(&self) -> i32 {
        unsafe { ffi::xc_func_info_get_number(self.info()) as i32 }
    }

    /// Functional kind (exchange, correlation, etc.).
    pub fn kind(&self) -> LibXCFunctionalKind {
        let k = unsafe { ffi::xc_func_info_get_kind(self.info()) } as u32;
        match k {
            ffi::XC_EXCHANGE => LibXCFunctionalKind::Exchange,
            ffi::XC_CORRELATION => LibXCFunctionalKind::Correlation,
            ffi::XC_EXCHANGE_CORRELATION => LibXCFunctionalKind::ExchangeCorrelation,
            ffi::XC_KINETIC => LibXCFunctionalKind::Kinetic,
            _ => LibXCFunctionalKind::Exchange, // fallback
        }
    }

    /// Functional name.
    pub fn name(&self) -> String {
        unsafe { cstr_to_string(ffi::xc_func_info_get_name(self.info())) }
    }

    /// Functional family (LDA, GGA, MGGA, etc.).
    pub fn family(&self) -> LibXCFamily {
        let f = unsafe { ffi::xc_func_info_get_family(self.info()) } as u32;
        match f {
            ffi::XC_FAMILY_LDA => LibXCFamily::LDA,
            ffi::XC_FAMILY_GGA => LibXCFamily::GGA,
            ffi::XC_FAMILY_MGGA => LibXCFamily::MGGA,
            ffi::XC_FAMILY_LCA => LibXCFamily::LCA,
            ffi::XC_FAMILY_OEP => LibXCFamily::OEP,
            ffi::XC_FAMILY_HYB_GGA => LibXCFamily::HybGGA,
            ffi::XC_FAMILY_HYB_MGGA => LibXCFamily::HybMGGA,
            ffi::XC_FAMILY_HYB_LDA => LibXCFamily::HybLDA,
            _ => LibXCFamily::LDA, // fallback
        }
    }

    /// Functional flags as bitflags.
    pub fn flags(&self) -> BitFlags<LibXCFlags> {
        let f = unsafe { ffi::xc_func_info_get_flags(self.info()) };
        BitFlags::from_bits(f as u32).unwrap_or_else(|_| BitFlags::empty())
    }

    /// Number of spin channels (1 = unpolarized, 2 = polarized).
    pub fn nspin(&self) -> i32 {
        unsafe { (*self.ptr).nspin }
    }

    /// Reference to the dimension struct from libxc.
    pub fn dim(&self) -> &ffi::xc_dimensions {
        unsafe { &(*self.ptr).dim }
    }

    // -- Convenience flag getters -------------------------------------------

    fn has_flag(&self, flag: LibXCFlags) -> bool {
        self.flags().contains(flag)
    }

    /// Whether this functional can compute the energy density (exc).
    pub fn has_exc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveEXC)
    }
    /// Whether this functional can compute the first derivative (vxc).
    pub fn has_vxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveVXC)
    }
    /// Whether this functional can compute the second derivative (fxc).
    pub fn has_fxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveFXC)
    }
    /// Whether this functional can compute the third derivative (kxc).
    pub fn has_kxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveKXC)
    }
    /// Whether this functional can compute the fourth derivative (lxc).
    pub fn has_lxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveLXC)
    }
    /// Whether this functional requires the laplacian.
    pub fn needs_laplacian(&self) -> bool {
        self.has_flag(LibXCFlags::NeedsLaplacian)
    }
    /// Whether this functional requires the kinetic energy density.
    pub fn needs_tau(&self) -> bool {
        self.has_flag(LibXCFlags::NeedsTau)
    }
    /// Whether this is a CAM range-separated hybrid.
    pub fn is_hyb_cam(&self) -> bool {
        self.has_flag(LibXCFlags::HybCAM)
    }

    // -- References ---------------------------------------------------------

    /// Returns the literature references for this functional.
    pub fn references(&self) -> Vec<LibXCReference> {
        let mut refs = Vec::new();
        for i in 0..(ffi::XC_MAX_REFERENCES as i32) {
            let ref_ptr = unsafe { ffi::xc_func_info_get_references(self.info(), i) };
            if ref_ptr.is_null() {
                break;
            }
            refs.push(LibXCReference {
                ref_text: unsafe { cstr_to_string(ffi::xc_func_reference_get_ref(ref_ptr)) },
                doi: unsafe { cstr_to_string(ffi::xc_func_reference_get_doi(ref_ptr)) },
                bibtex: unsafe { cstr_to_string(ffi::xc_func_reference_get_bibtex(ref_ptr)) },
                key: unsafe { cstr_to_string(ffi::xc_func_reference_get_key(ref_ptr)) },
            });
        }
        refs
    }

    // -- External parameters ------------------------------------------------

    /// Number of external parameters for this functional.
    pub fn n_ext_params(&self) -> i32 {
        unsafe { ffi::xc_func_info_get_n_ext_params(self.info()) as i32 }
    }

    /// Names of the external parameters.
    pub fn ext_param_names(&self) -> Vec<String> {
        let n = self.n_ext_params();
        (0..n)
            .map(|i| unsafe {
                cstr_to_string(ffi::xc_func_info_get_ext_params_name(
                    self.info(),
                    i as std::ffi::c_int,
                ))
            })
            .collect()
    }

    /// Descriptions of the external parameters.
    pub fn ext_param_descriptions(&self) -> Vec<String> {
        let n = self.n_ext_params();
        (0..n)
            .map(|i| unsafe {
                cstr_to_string(ffi::xc_func_info_get_ext_params_description(
                    self.info(),
                    i as std::ffi::c_int,
                ))
            })
            .collect()
    }

    /// Default values of the external parameters.
    pub fn ext_param_default_values(&self) -> Vec<f64> {
        let n = self.n_ext_params();
        (0..n)
            .map(|i| unsafe {
                ffi::xc_func_info_get_ext_params_default_value(self.info(), i as std::ffi::c_int)
            })
            .collect()
    }

    // -- Setters ------------------------------------------------------------

    /// Set all external parameters at once.
    pub fn set_ext_params(&mut self, params: &[f64]) {
        unsafe {
            ffi::xc_func_set_ext_params(self.ptr, params.as_ptr());
        }
    }

    /// Set the density threshold.
    pub fn set_dens_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_dens_threshold(self.ptr, threshold) }
    }

    /// Set the zeta (spin polarization) threshold.
    pub fn set_zeta_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_zeta_threshold(self.ptr, threshold) }
    }

    /// Set the sigma (reduced gradient) threshold.
    pub fn set_sigma_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_sigma_threshold(self.ptr, threshold) }
    }

    /// Set the tau (kinetic energy density) threshold.
    pub fn set_tau_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_tau_threshold(self.ptr, threshold) }
    }

    /// Enable or disable Fermi hole curvature enforcement (api-v7_0+).
    #[cfg(feature = "api-v7_0")]
    pub fn set_fhc_enforcement(&mut self, on: bool) {
        unsafe { ffi::xc_func_set_fhc_enforcement(self.ptr, on as std::ffi::c_int) }
    }

    // -- Hybrid functional info ---------------------------------------------

    /// Fraction of Hartree-Fock exchange for global hybrids.
    pub fn hyb_exx_coef(&self) -> f64 {
        unsafe { ffi::xc_hyb_exx_coef(self.ptr) }
    }

    /// Range-separated hybrid coefficients (omega, alpha, beta).
    pub fn cam_coef(&self) -> (f64, f64, f64) {
        let mut omega: f64 = 0.0;
        let mut alpha: f64 = 0.0;
        let mut beta: f64 = 0.0;
        unsafe {
            ffi::xc_hyb_cam_coef(self.ptr, &mut omega, &mut alpha, &mut beta);
        }
        (omega, alpha, beta)
    }

    /// VV10 non-local correlation coefficients (nlc_b, nlc_C).
    pub fn vv10_coef(&self) -> (f64, f64) {
        let mut nlc_b: f64 = 0.0;
        let mut nlc_C: f64 = 0.0;
        unsafe {
            ffi::xc_nlc_coef(self.ptr, &mut nlc_b, &mut nlc_C);
        }
        (nlc_b, nlc_C)
    }

    // -- Auxiliary functionals ----------------------------------------------

    /// Number of auxiliary functionals in a mixed functional.
    pub fn num_aux_funcs(&self) -> i32 {
        unsafe { ffi::xc_num_aux_funcs(self.ptr) as i32 }
    }

    /// IDs of the auxiliary functionals.
    pub fn aux_func_ids(&self) -> Vec<i32> {
        let n = self.num_aux_funcs();
        let mut ids = vec![0 as std::ffi::c_int; n as usize];
        unsafe { ffi::xc_aux_func_ids(self.ptr, ids.as_mut_ptr()) }
        ids.into_iter().collect()
    }

    /// Weights of the auxiliary functionals.
    pub fn aux_func_weights(&self) -> Vec<f64> {
        let n = self.num_aux_funcs();
        let mut weights = vec![0.0f64; n as usize];
        unsafe { ffi::xc_aux_func_weights(self.ptr, weights.as_mut_ptr()) }
        weights
    }

    // -- Output layout computation ------------------------------------------

    /// Compute the output layout for LDA at a given number of grid points.
    pub fn lda_output_layout(
        &self,
        npoints: usize,
        flags: &LibXCDerivativeFlags,
    ) -> LibXCOutputLayout {
        let dim = self.dim();
        let mut layout = LibXCOutputLayout::new(npoints);
        if flags.do_exc {
            layout.push("zk", dim.zk);
        }
        if flags.do_vxc {
            layout.push("vrho", dim.vrho);
        }
        if flags.do_fxc {
            layout.push("v2rho2", dim.v2rho2);
        }
        if flags.do_kxc {
            layout.push("v3rho3", dim.v3rho3);
        }
        if flags.do_lxc {
            layout.push("v4rho4", dim.v4rho4);
        }
        layout
    }

    /// Compute the output layout for GGA at a given number of grid points.
    pub fn gga_output_layout(
        &self,
        npoints: usize,
        flags: &LibXCDerivativeFlags,
    ) -> LibXCOutputLayout {
        let dim = self.dim();
        let mut layout = LibXCOutputLayout::new(npoints);
        if flags.do_exc {
            layout.push("zk", dim.zk);
        }
        if flags.do_vxc {
            layout.push("vrho", dim.vrho);
            layout.push("vsigma", dim.vsigma);
        }
        if flags.do_fxc {
            layout.push("v2rho2", dim.v2rho2);
            layout.push("v2rhosigma", dim.v2rhosigma);
            layout.push("v2sigma2", dim.v2sigma2);
        }
        if flags.do_kxc {
            layout.push("v3rho3", dim.v3rho3);
            layout.push("v3rho2sigma", dim.v3rho2sigma);
            layout.push("v3rhosigma2", dim.v3rhosigma2);
            layout.push("v3sigma3", dim.v3sigma3);
        }
        if flags.do_lxc {
            layout.push("v4rho4", dim.v4rho4);
            layout.push("v4rho3sigma", dim.v4rho3sigma);
            layout.push("v4rho2sigma2", dim.v4rho2sigma2);
            layout.push("v4rhosigma3", dim.v4rhosigma3);
            layout.push("v4sigma4", dim.v4sigma4);
        }
        layout
    }

    /// Compute the output layout for MGGA at a given number of grid points.
    pub fn mgga_output_layout(
        &self,
        npoints: usize,
        flags: &LibXCDerivativeFlags,
    ) -> LibXCOutputLayout {
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();
        let mut layout = LibXCOutputLayout::new(npoints);
        if flags.do_exc {
            layout.push("zk", dim.zk);
        }
        if flags.do_vxc {
            layout.push("vrho", dim.vrho);
            layout.push("vsigma", dim.vsigma);
            if needs_lapl {
                layout.push("vlapl", dim.vlapl);
            }
            if needs_tau {
                layout.push("vtau", dim.vtau);
            }
        }
        if flags.do_fxc {
            layout.push("v2rho2", dim.v2rho2);
            layout.push("v2rhosigma", dim.v2rhosigma);
            if needs_lapl {
                layout.push("v2rholapl", dim.v2rholapl);
            }
            if needs_tau {
                layout.push("v2rhotau", dim.v2rhotau);
            }
            layout.push("v2sigma2", dim.v2sigma2);
            if needs_lapl {
                layout.push("v2sigmalapl", dim.v2sigmalapl);
            }
            if needs_tau {
                layout.push("v2sigmatau", dim.v2sigmatau);
            }
            if needs_lapl {
                layout.push("v2lapl2", dim.v2lapl2);
            }
            if needs_lapl && needs_tau {
                layout.push("v2lapltau", dim.v2lapltau);
            }
            if needs_tau {
                layout.push("v2tau2", dim.v2tau2);
            }
        }
        if flags.do_kxc {
            layout.push("v3rho3", dim.v3rho3);
            layout.push("v3rho2sigma", dim.v3rho2sigma);
            if needs_lapl {
                layout.push("v3rho2lapl", dim.v3rho2lapl);
            }
            if needs_tau {
                layout.push("v3rho2tau", dim.v3rho2tau);
            }
            layout.push("v3rhosigma2", dim.v3rhosigma2);
            if needs_lapl {
                layout.push("v3rhosigmalapl", dim.v3rhosigmalapl);
            }
            if needs_tau {
                layout.push("v3rhosigmatau", dim.v3rhosigmatau);
            }
            if needs_lapl {
                layout.push("v3rholapl2", dim.v3rholapl2);
            }
            if needs_lapl && needs_tau {
                layout.push("v3rholapltau", dim.v3rholapltau);
            }
            if needs_tau {
                layout.push("v3rhotau2", dim.v3rhotau2);
            }
            layout.push("v3sigma3", dim.v3sigma3);
            if needs_lapl {
                layout.push("v3sigma2lapl", dim.v3sigma2lapl);
            }
            if needs_tau {
                layout.push("v3sigma2tau", dim.v3sigma2tau);
            }
            if needs_lapl {
                layout.push("v3sigmalapl2", dim.v3sigmalapl2);
            }
            if needs_lapl && needs_tau {
                layout.push("v3sigmalapltau", dim.v3sigmalapltau);
            }
            if needs_tau {
                layout.push("v3sigmatau2", dim.v3sigmatau2);
            }
            if needs_lapl {
                layout.push("v3lapl3", dim.v3lapl3);
            }
            if needs_lapl && needs_tau {
                layout.push("v3lapl2tau", dim.v3lapl2tau);
            }
            if needs_lapl && needs_tau {
                layout.push("v3lapltau2", dim.v3lapltau2);
            }
            if needs_tau {
                layout.push("v3tau3", dim.v3tau3);
            }
        }
        if flags.do_lxc {
            layout.push("v4rho4", dim.v4rho4);
            layout.push("v4rho3sigma", dim.v4rho3sigma);
            if needs_lapl {
                layout.push("v4rho3lapl", dim.v4rho3lapl);
            }
            if needs_tau {
                layout.push("v4rho3tau", dim.v4rho3tau);
            }
            layout.push("v4rho2sigma2", dim.v4rho2sigma2);
            if needs_lapl {
                layout.push("v4rho2sigmalapl", dim.v4rho2sigmalapl);
            }
            if needs_tau {
                layout.push("v4rho2sigmatau", dim.v4rho2sigmatau);
            }
            if needs_lapl {
                layout.push("v4rho2lapl2", dim.v4rho2lapl2);
            }
            if needs_lapl && needs_tau {
                layout.push("v4rho2lapltau", dim.v4rho2lapltau);
            }
            if needs_tau {
                layout.push("v4rho2tau2", dim.v4rho2tau2);
            }
            layout.push("v4rhosigma3", dim.v4rhosigma3);
            if needs_lapl {
                layout.push("v4rhosigma2lapl", dim.v4rhosigma2lapl);
            }
            if needs_tau {
                layout.push("v4rhosigma2tau", dim.v4rhosigma2tau);
            }
            if needs_lapl {
                layout.push("v4rhosigmalapl2", dim.v4rhosigmalapl2);
            }
            if needs_lapl && needs_tau {
                layout.push("v4rhosigmalapltau", dim.v4rhosigmalapltau);
            }
            if needs_tau {
                layout.push("v4rhosigmatau2", dim.v4rhosigmatau2);
            }
            if needs_lapl {
                layout.push("v4rholapl3", dim.v4rholapl3);
            }
            if needs_lapl && needs_tau {
                layout.push("v4rholapl2tau", dim.v4rholapl2tau);
            }
            if needs_lapl && needs_tau {
                layout.push("v4rholapltau2", dim.v4rholapltau2);
            }
            if needs_tau {
                layout.push("v4rhotau3", dim.v4rhotau3);
            }
            layout.push("v4sigma4", dim.v4sigma4);
            if needs_lapl {
                layout.push("v4sigma3lapl", dim.v4sigma3lapl);
            }
            if needs_tau {
                layout.push("v4sigma3tau", dim.v4sigma3tau);
            }
            if needs_lapl {
                layout.push("v4sigma2lapl2", dim.v4sigma2lapl2);
            }
            if needs_lapl && needs_tau {
                layout.push("v4sigma2lapltau", dim.v4sigma2lapltau);
            }
            if needs_tau {
                layout.push("v4sigma2tau2", dim.v4sigma2tau2);
            }
            if needs_lapl {
                layout.push("v4sigmalapl3", dim.v4sigmalapl3);
            }
            if needs_lapl && needs_tau {
                layout.push("v4sigmalapl2tau", dim.v4sigmalapl2tau);
            }
            if needs_lapl && needs_tau {
                layout.push("v4sigmalapltau2", dim.v4sigmalapltau2);
            }
            if needs_tau {
                layout.push("v4sigmatau3", dim.v4sigmatau3);
            }
            if needs_lapl {
                layout.push("v4lapl4", dim.v4lapl4);
            }
            if needs_lapl && needs_tau {
                layout.push("v4lapl3tau", dim.v4lapl3tau);
            }
            if needs_lapl && needs_tau {
                layout.push("v4lapl2tau2", dim.v4lapl2tau2);
            }
            if needs_lapl && needs_tau {
                layout.push("v4lapltau3", dim.v4lapltau3);
            }
            if needs_tau {
                layout.push("v4tau4", dim.v4tau4);
            }
        }
        layout
    }

    // -- Validate derivative flags ------------------------------------------

    fn validate_flags(&self, flags: &LibXCDerivativeFlags) -> Result<(), LibXCError> {
        if flags.do_exc && !self.has_exc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have EXC capabilities",
                self.name()
            )));
        }
        if flags.do_vxc && !self.has_vxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have VXC capabilities",
                self.name()
            )));
        }
        if flags.do_fxc && !self.has_fxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have FXC capabilities",
                self.name()
            )));
        }
        if flags.do_kxc && !self.has_kxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have KXC capabilities",
                self.name()
            )));
        }
        if flags.do_lxc && !self.has_lxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have LXC capabilities",
                self.name()
            )));
        }
        Ok(())
    }

    // -- LDA compute --------------------------------------------------------

    /// Compute LDA functional with automatic allocation.
    /// Returns `(buffer, layout)` where buffer is a contiguous f64 array.
    pub fn compute_lda(
        &self,
        input: &LibXCLdaInput,
        flags: &LibXCDerivativeFlags,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        self.validate_flags(flags)?;
        let nspin = self.nspin() as usize;
        if input.rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = input.rho.len() / nspin;
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
                input.rho.as_ptr(),
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
        input: &LibXCLdaInput,
        output: &mut LibXCLdaOutputMut,
    ) -> Result<(), LibXCError> {
        let nspin = self.nspin() as usize;
        if input.rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = input.rho.len() / nspin;
        let dim = self.dim();

        let null = std::ptr::null_mut::<f64>();
        // Validates the size of a user-provided output slice and returns a mutable
        // pointer. Uses as_ptr() + cast because the Option<&mut [f64]> is
        // behind a shared reference in the closure, preventing as_mut_ptr().
        // The cast is safe: the &mut [f64] proves the data is valid for
        // writing.
        let validate_and_ptr = |slice: &Option<&mut [f64]>,
                                expected_dim: i32,
                                name: &str|
         -> Result<*mut f64, LibXCError> {
            match slice {
                Some(s) => {
                    let expected = npoints * (expected_dim as usize);
                    if s.len() != expected {
                        return Err(LibXCError::ComputeError(format!(
                            "{name}: expected size {expected}, got {}",
                            s.len()
                        )));
                    }
                    Ok(s.as_ptr() as *mut f64)
                },
                None => Ok(null),
            }
        };

        let zk_ptr = validate_and_ptr(&output.zk, dim.zk, "zk")?;
        let vrho_ptr = validate_and_ptr(&output.vrho, dim.vrho, "vrho")?;
        let v2rho2_ptr = validate_and_ptr(&output.v2rho2, dim.v2rho2, "v2rho2")?;
        let v3rho3_ptr = validate_and_ptr(&output.v3rho3, dim.v3rho3, "v3rho3")?;
        let v4rho4_ptr = validate_and_ptr(&output.v4rho4, dim.v4rho4, "v4rho4")?;

        unsafe {
            ffi::xc_lda(
                self.ptr,
                npoints,
                input.rho.as_ptr(),
                zk_ptr,
                vrho_ptr,
                v2rho2_ptr,
                v3rho3_ptr,
                v4rho4_ptr,
            );
        }
        Ok(())
    }

    // -- GGA compute --------------------------------------------------------

    /// Compute GGA functional with automatic allocation.
    pub fn compute_gga(
        &self,
        input: &LibXCGgaInput,
        flags: &LibXCDerivativeFlags,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        self.validate_flags(flags)?;
        let nspin = self.nspin() as usize;
        if input.rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = input.rho.len() / nspin;
        let dim = self.dim();
        let expected_sigma = npoints * (dim.sigma as usize);
        if input.sigma.len() != expected_sigma {
            return Err(LibXCError::ComputeError(format!(
                "sigma: expected size {expected_sigma}, got {}",
                input.sigma.len()
            )));
        }

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
                input.rho.as_ptr(),
                input.sigma.as_ptr(),
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
        input: &LibXCGgaInput,
        output: &mut LibXCGgaOutputMut,
    ) -> Result<(), LibXCError> {
        let nspin = self.nspin() as usize;
        if input.rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = input.rho.len() / nspin;
        let dim = self.dim();
        let expected_sigma = npoints * (dim.sigma as usize);
        if input.sigma.len() != expected_sigma {
            return Err(LibXCError::ComputeError(format!(
                "sigma: expected size {expected_sigma}, got {}",
                input.sigma.len()
            )));
        }

        let null = std::ptr::null_mut::<f64>();
        let validate_and_ptr = |slice: &Option<&mut [f64]>,
                                expected_dim: i32,
                                name: &str|
         -> Result<*mut f64, LibXCError> {
            match slice {
                Some(s) => {
                    let expected = npoints * (expected_dim as usize);
                    if s.len() != expected {
                        return Err(LibXCError::ComputeError(format!(
                            "{name}: expected size {expected}, got {}",
                            s.len()
                        )));
                    }
                    Ok(s.as_ptr() as *mut f64)
                },
                None => Ok(null),
            }
        };

        let zk_ptr = validate_and_ptr(&output.zk, dim.zk, "zk")?;
        let vrho_ptr = validate_and_ptr(&output.vrho, dim.vrho, "vrho")?;
        let vsigma_ptr = validate_and_ptr(&output.vsigma, dim.vsigma, "vsigma")?;
        let v2rho2_ptr = validate_and_ptr(&output.v2rho2, dim.v2rho2, "v2rho2")?;
        let v2rhosigma_ptr = validate_and_ptr(&output.v2rhosigma, dim.v2rhosigma, "v2rhosigma")?;
        let v2sigma2_ptr = validate_and_ptr(&output.v2sigma2, dim.v2sigma2, "v2sigma2")?;
        let v3rho3_ptr = validate_and_ptr(&output.v3rho3, dim.v3rho3, "v3rho3")?;
        let v3rho2sigma_ptr =
            validate_and_ptr(&output.v3rho2sigma, dim.v3rho2sigma, "v3rho2sigma")?;
        let v3rhosigma2_ptr =
            validate_and_ptr(&output.v3rhosigma2, dim.v3rhosigma2, "v3rhosigma2")?;
        let v3sigma3_ptr = validate_and_ptr(&output.v3sigma3, dim.v3sigma3, "v3sigma3")?;
        let v4rho4_ptr = validate_and_ptr(&output.v4rho4, dim.v4rho4, "v4rho4")?;
        let v4rho3sigma_ptr =
            validate_and_ptr(&output.v4rho3sigma, dim.v4rho3sigma, "v4rho3sigma")?;
        let v4rho2sigma2_ptr =
            validate_and_ptr(&output.v4rho2sigma2, dim.v4rho2sigma2, "v4rho2sigma2")?;
        let v4rhosigma3_ptr =
            validate_and_ptr(&output.v4rhosigma3, dim.v4rhosigma3, "v4rhosigma3")?;
        let v4sigma4_ptr = validate_and_ptr(&output.v4sigma4, dim.v4sigma4, "v4sigma4")?;

        unsafe {
            ffi::xc_gga(
                self.ptr,
                npoints,
                input.rho.as_ptr(),
                input.sigma.as_ptr(),
                zk_ptr,
                vrho_ptr,
                vsigma_ptr,
                v2rho2_ptr,
                v2rhosigma_ptr,
                v2sigma2_ptr,
                v3rho3_ptr,
                v3rho2sigma_ptr,
                v3rhosigma2_ptr,
                v3sigma3_ptr,
                v4rho4_ptr,
                v4rho3sigma_ptr,
                v4rho2sigma2_ptr,
                v4rhosigma3_ptr,
                v4sigma4_ptr,
            );
        }
        Ok(())
    }

    // -- MGGA compute -------------------------------------------------------

    /// Compute MGGA functional with automatic allocation.
    pub fn compute_mgga(
        &self,
        input: &LibXCMggaInput,
        flags: &LibXCDerivativeFlags,
    ) -> Result<(Vec<f64>, LibXCOutputLayout), LibXCError> {
        self.validate_flags(flags)?;
        let nspin = self.nspin() as usize;
        if input.rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = input.rho.len() / nspin;
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();

        let expected_sigma = npoints * (dim.sigma as usize);
        if input.sigma.len() != expected_sigma {
            return Err(LibXCError::ComputeError(format!(
                "sigma: expected size {expected_sigma}, got {}",
                input.sigma.len()
            )));
        }

        // Validate lapl/tau inputs
        let lapl_ptr = match (&input.lapl, needs_lapl) {
            (Some(l), true) => {
                let expected = npoints * (dim.lapl as usize);
                if l.len() != expected {
                    return Err(LibXCError::ComputeError(format!(
                        "lapl: expected size {expected}, got {}",
                        l.len()
                    )));
                }
                l.as_ptr()
            },
            (None, true) => {
                return Err(LibXCError::ComputeError("lapl required but not provided".into()));
            },
            (_, false) => std::ptr::null(),
        };

        let tau_ptr = match (&input.tau, needs_tau) {
            (Some(t), true) => {
                let expected = npoints * (dim.tau as usize);
                if t.len() != expected {
                    return Err(LibXCError::ComputeError(format!(
                        "tau: expected size {expected}, got {}",
                        t.len()
                    )));
                }
                t.as_ptr()
            },
            (None, true) => {
                return Err(LibXCError::ComputeError("tau required but not provided".into()));
            },
            (_, false) => std::ptr::null(),
        };

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
                input.rho.as_ptr(),
                input.sigma.as_ptr(),
                lapl_ptr,
                tau_ptr,
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
    #[allow(clippy::too_many_arguments)]
    pub fn compute_mgga_with_output(
        &self,
        input: &LibXCMggaInput,
        output: &mut LibXCMggaOutputMut,
    ) -> Result<(), LibXCError> {
        let nspin = self.nspin() as usize;
        if input.rho.len() % nspin != 0 {
            return Err(LibXCError::ComputeError(
                "rho input has invalid shape: size not divisible by nspin".into(),
            ));
        }
        let npoints = input.rho.len() / nspin;
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();

        let expected_sigma = npoints * (dim.sigma as usize);
        if input.sigma.len() != expected_sigma {
            return Err(LibXCError::ComputeError(format!(
                "sigma: expected size {expected_sigma}, got {}",
                input.sigma.len()
            )));
        }

        let lapl_ptr = match (&input.lapl, needs_lapl) {
            (Some(l), true) => {
                let expected = npoints * (dim.lapl as usize);
                if l.len() != expected {
                    return Err(LibXCError::ComputeError(format!(
                        "lapl: expected size {expected}, got {}",
                        l.len()
                    )));
                }
                l.as_ptr()
            },
            (None, true) => {
                return Err(LibXCError::ComputeError("lapl required but not provided".into()));
            },
            (_, false) => std::ptr::null(),
        };

        let tau_ptr = match (&input.tau, needs_tau) {
            (Some(t), true) => {
                let expected = npoints * (dim.tau as usize);
                if t.len() != expected {
                    return Err(LibXCError::ComputeError(format!(
                        "tau: expected size {expected}, got {}",
                        t.len()
                    )));
                }
                t.as_ptr()
            },
            (None, true) => {
                return Err(LibXCError::ComputeError("tau required but not provided".into()));
            },
            (_, false) => std::ptr::null(),
        };

        let null = std::ptr::null_mut::<f64>();
        let validate_and_ptr = |slice: &Option<&mut [f64]>,
                                expected_dim: i32,
                                name: &str|
         -> Result<*mut f64, LibXCError> {
            match slice {
                Some(s) => {
                    let expected = npoints * (expected_dim as usize);
                    if s.len() != expected {
                        return Err(LibXCError::ComputeError(format!(
                            "{name}: expected size {expected}, got {}",
                            s.len()
                        )));
                    }
                    Ok(s.as_ptr() as *mut f64)
                },
                None => Ok(null),
            }
        };

        // Validate all output slices
        let zk_ptr = validate_and_ptr(&output.zk, dim.zk, "zk")?;
        let vrho_ptr = validate_and_ptr(&output.vrho, dim.vrho, "vrho")?;
        let vsigma_ptr = validate_and_ptr(&output.vsigma, dim.vsigma, "vsigma")?;
        let vlapl_ptr = validate_and_ptr(&output.vlapl, dim.vlapl, "vlapl")?;
        let vtau_ptr = validate_and_ptr(&output.vtau, dim.vtau, "vtau")?;
        let v2rho2_ptr = validate_and_ptr(&output.v2rho2, dim.v2rho2, "v2rho2")?;
        let v2rhosigma_ptr = validate_and_ptr(&output.v2rhosigma, dim.v2rhosigma, "v2rhosigma")?;
        let v2rholapl_ptr = validate_and_ptr(&output.v2rholapl, dim.v2rholapl, "v2rholapl")?;
        let v2rhotau_ptr = validate_and_ptr(&output.v2rhotau, dim.v2rhotau, "v2rhotau")?;
        let v2sigma2_ptr = validate_and_ptr(&output.v2sigma2, dim.v2sigma2, "v2sigma2")?;
        let v2sigmalapl_ptr =
            validate_and_ptr(&output.v2sigmalapl, dim.v2sigmalapl, "v2sigmalapl")?;
        let v2sigmatau_ptr = validate_and_ptr(&output.v2sigmatau, dim.v2sigmatau, "v2sigmatau")?;
        let v2lapl2_ptr = validate_and_ptr(&output.v2lapl2, dim.v2lapl2, "v2lapl2")?;
        let v2lapltau_ptr = validate_and_ptr(&output.v2lapltau, dim.v2lapltau, "v2lapltau")?;
        let v2tau2_ptr = validate_and_ptr(&output.v2tau2, dim.v2tau2, "v2tau2")?;
        let v3rho3_ptr = validate_and_ptr(&output.v3rho3, dim.v3rho3, "v3rho3")?;
        let v3rho2sigma_ptr =
            validate_and_ptr(&output.v3rho2sigma, dim.v3rho2sigma, "v3rho2sigma")?;
        let v3rho2lapl_ptr = validate_and_ptr(&output.v3rho2lapl, dim.v3rho2lapl, "v3rho2lapl")?;
        let v3rho2tau_ptr = validate_and_ptr(&output.v3rho2tau, dim.v3rho2tau, "v3rho2tau")?;
        let v3rhosigma2_ptr =
            validate_and_ptr(&output.v3rhosigma2, dim.v3rhosigma2, "v3rhosigma2")?;
        let v3rhosigmalapl_ptr =
            validate_and_ptr(&output.v3rhosigmalapl, dim.v3rhosigmalapl, "v3rhosigmalapl")?;
        let v3rhosigmatau_ptr =
            validate_and_ptr(&output.v3rhosigmatau, dim.v3rhosigmatau, "v3rhosigmatau")?;
        let v3rholapl2_ptr = validate_and_ptr(&output.v3rholapl2, dim.v3rholapl2, "v3rholapl2")?;
        let v3rholapltau_ptr =
            validate_and_ptr(&output.v3rholapltau, dim.v3rholapltau, "v3rholapltau")?;
        let v3rhotau2_ptr = validate_and_ptr(&output.v3rhotau2, dim.v3rhotau2, "v3rhotau2")?;
        let v3sigma3_ptr = validate_and_ptr(&output.v3sigma3, dim.v3sigma3, "v3sigma3")?;
        let v3sigma2lapl_ptr =
            validate_and_ptr(&output.v3sigma2lapl, dim.v3sigma2lapl, "v3sigma2lapl")?;
        let v3sigma2tau_ptr =
            validate_and_ptr(&output.v3sigma2tau, dim.v3sigma2tau, "v3sigma2tau")?;
        let v3sigmalapl2_ptr =
            validate_and_ptr(&output.v3sigmalapl2, dim.v3sigmalapl2, "v3sigmalapl2")?;
        let v3sigmalapltau_ptr =
            validate_and_ptr(&output.v3sigmalapltau, dim.v3sigmalapltau, "v3sigmalapltau")?;
        let v3sigmatau2_ptr =
            validate_and_ptr(&output.v3sigmatau2, dim.v3sigmatau2, "v3sigmatau2")?;
        let v3lapl3_ptr = validate_and_ptr(&output.v3lapl3, dim.v3lapl3, "v3lapl3")?;
        let v3lapl2tau_ptr = validate_and_ptr(&output.v3lapl2tau, dim.v3lapl2tau, "v3lapl2tau")?;
        let v3lapltau2_ptr = validate_and_ptr(&output.v3lapltau2, dim.v3lapltau2, "v3lapltau2")?;
        let v3tau3_ptr = validate_and_ptr(&output.v3tau3, dim.v3tau3, "v3tau3")?;
        let v4rho4_ptr = validate_and_ptr(&output.v4rho4, dim.v4rho4, "v4rho4")?;
        let v4rho3sigma_ptr =
            validate_and_ptr(&output.v4rho3sigma, dim.v4rho3sigma, "v4rho3sigma")?;
        let v4rho3lapl_ptr = validate_and_ptr(&output.v4rho3lapl, dim.v4rho3lapl, "v4rho3lapl")?;
        let v4rho3tau_ptr = validate_and_ptr(&output.v4rho3tau, dim.v4rho3tau, "v4rho3tau")?;
        let v4rho2sigma2_ptr =
            validate_and_ptr(&output.v4rho2sigma2, dim.v4rho2sigma2, "v4rho2sigma2")?;
        let v4rho2sigmalapl_ptr =
            validate_and_ptr(&output.v4rho2sigmalapl, dim.v4rho2sigmalapl, "v4rho2sigmalapl")?;
        let v4rho2sigmatau_ptr =
            validate_and_ptr(&output.v4rho2sigmatau, dim.v4rho2sigmatau, "v4rho2sigmatau")?;
        let v4rho2lapl2_ptr =
            validate_and_ptr(&output.v4rho2lapl2, dim.v4rho2lapl2, "v4rho2lapl2")?;
        let v4rho2lapltau_ptr =
            validate_and_ptr(&output.v4rho2lapltau, dim.v4rho2lapltau, "v4rho2lapltau")?;
        let v4rho2tau2_ptr = validate_and_ptr(&output.v4rho2tau2, dim.v4rho2tau2, "v4rho2tau2")?;
        let v4rhosigma3_ptr =
            validate_and_ptr(&output.v4rhosigma3, dim.v4rhosigma3, "v4rhosigma3")?;
        let v4rhosigma2lapl_ptr =
            validate_and_ptr(&output.v4rhosigma2lapl, dim.v4rhosigma2lapl, "v4rhosigma2lapl")?;
        let v4rhosigma2tau_ptr =
            validate_and_ptr(&output.v4rhosigma2tau, dim.v4rhosigma2tau, "v4rhosigma2tau")?;
        let v4rhosigmalapl2_ptr =
            validate_and_ptr(&output.v4rhosigmalapl2, dim.v4rhosigmalapl2, "v4rhosigmalapl2")?;
        let v4rhosigmalapltau_ptr = validate_and_ptr(
            &output.v4rhosigmalapltau,
            dim.v4rhosigmalapltau,
            "v4rhosigmalapltau",
        )?;
        let v4rhosigmatau2_ptr =
            validate_and_ptr(&output.v4rhosigmatau2, dim.v4rhosigmatau2, "v4rhosigmatau2")?;
        let v4rholapl3_ptr = validate_and_ptr(&output.v4rholapl3, dim.v4rholapl3, "v4rholapl3")?;
        let v4rholapl2tau_ptr =
            validate_and_ptr(&output.v4rholapl2tau, dim.v4rholapl2tau, "v4rholapl2tau")?;
        let v4rholapltau2_ptr =
            validate_and_ptr(&output.v4rholapltau2, dim.v4rholapltau2, "v4rholapltau2")?;
        let v4rhotau3_ptr = validate_and_ptr(&output.v4rhotau3, dim.v4rhotau3, "v4rhotau3")?;
        let v4sigma4_ptr = validate_and_ptr(&output.v4sigma4, dim.v4sigma4, "v4sigma4")?;
        let v4sigma3lapl_ptr =
            validate_and_ptr(&output.v4sigma3lapl, dim.v4sigma3lapl, "v4sigma3lapl")?;
        let v4sigma3tau_ptr =
            validate_and_ptr(&output.v4sigma3tau, dim.v4sigma3tau, "v4sigma3tau")?;
        let v4sigma2lapl2_ptr =
            validate_and_ptr(&output.v4sigma2lapl2, dim.v4sigma2lapl2, "v4sigma2lapl2")?;
        let v4sigma2lapltau_ptr =
            validate_and_ptr(&output.v4sigma2lapltau, dim.v4sigma2lapltau, "v4sigma2lapltau")?;
        let v4sigma2tau2_ptr =
            validate_and_ptr(&output.v4sigma2tau2, dim.v4sigma2tau2, "v4sigma2tau2")?;
        let v4sigmalapl3_ptr =
            validate_and_ptr(&output.v4sigmalapl3, dim.v4sigmalapl3, "v4sigmalapl3")?;
        let v4sigmalapl2tau_ptr =
            validate_and_ptr(&output.v4sigmalapl2tau, dim.v4sigmalapl2tau, "v4sigmalapl2tau")?;
        let v4sigmalapltau2_ptr =
            validate_and_ptr(&output.v4sigmalapltau2, dim.v4sigmalapltau2, "v4sigmalapltau2")?;
        let v4sigmatau3_ptr =
            validate_and_ptr(&output.v4sigmatau3, dim.v4sigmatau3, "v4sigmatau3")?;
        let v4lapl4_ptr = validate_and_ptr(&output.v4lapl4, dim.v4lapl4, "v4lapl4")?;
        let v4lapl3tau_ptr = validate_and_ptr(&output.v4lapl3tau, dim.v4lapl3tau, "v4lapl3tau")?;
        let v4lapl2tau2_ptr =
            validate_and_ptr(&output.v4lapl2tau2, dim.v4lapl2tau2, "v4lapl2tau2")?;
        let v4lapltau3_ptr = validate_and_ptr(&output.v4lapltau3, dim.v4lapltau3, "v4lapltau3")?;
        let v4tau4_ptr = validate_and_ptr(&output.v4tau4, dim.v4tau4, "v4tau4")?;

        unsafe {
            ffi::xc_mgga(
                self.ptr,
                npoints,
                input.rho.as_ptr(),
                input.sigma.as_ptr(),
                lapl_ptr,
                tau_ptr,
                zk_ptr,
                vrho_ptr,
                vsigma_ptr,
                vlapl_ptr,
                vtau_ptr,
                v2rho2_ptr,
                v2rhosigma_ptr,
                v2rholapl_ptr,
                v2rhotau_ptr,
                v2sigma2_ptr,
                v2sigmalapl_ptr,
                v2sigmatau_ptr,
                v2lapl2_ptr,
                v2lapltau_ptr,
                v2tau2_ptr,
                v3rho3_ptr,
                v3rho2sigma_ptr,
                v3rho2lapl_ptr,
                v3rho2tau_ptr,
                v3rhosigma2_ptr,
                v3rhosigmalapl_ptr,
                v3rhosigmatau_ptr,
                v3rholapl2_ptr,
                v3rholapltau_ptr,
                v3rhotau2_ptr,
                v3sigma3_ptr,
                v3sigma2lapl_ptr,
                v3sigma2tau_ptr,
                v3sigmalapl2_ptr,
                v3sigmalapltau_ptr,
                v3sigmatau2_ptr,
                v3lapl3_ptr,
                v3lapl2tau_ptr,
                v3lapltau2_ptr,
                v3tau3_ptr,
                v4rho4_ptr,
                v4rho3sigma_ptr,
                v4rho3lapl_ptr,
                v4rho3tau_ptr,
                v4rho2sigma2_ptr,
                v4rho2sigmalapl_ptr,
                v4rho2sigmatau_ptr,
                v4rho2lapl2_ptr,
                v4rho2lapltau_ptr,
                v4rho2tau2_ptr,
                v4rhosigma3_ptr,
                v4rhosigma2lapl_ptr,
                v4rhosigma2tau_ptr,
                v4rhosigmalapl2_ptr,
                v4rhosigmalapltau_ptr,
                v4rhosigmatau2_ptr,
                v4rholapl3_ptr,
                v4rholapl2tau_ptr,
                v4rholapltau2_ptr,
                v4rhotau3_ptr,
                v4sigma4_ptr,
                v4sigma3lapl_ptr,
                v4sigma3tau_ptr,
                v4sigma2lapl2_ptr,
                v4sigma2lapltau_ptr,
                v4sigma2tau2_ptr,
                v4sigmalapl3_ptr,
                v4sigmalapl2tau_ptr,
                v4sigmalapltau2_ptr,
                v4sigmatau3_ptr,
                v4lapl4_ptr,
                v4lapl3tau_ptr,
                v4lapl2tau2_ptr,
                v4lapltau3_ptr,
                v4tau4_ptr,
            );
        }
        Ok(())
    }
}

impl Drop for LibXCFunctional {
    fn drop(&mut self) {
        unsafe {
            ffi::xc_func_end(self.ptr);
            ffi::xc_func_free(self.ptr);
        }
    }
}

impl fmt::Debug for LibXCFunctional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LibXCFunctional")
            .field("name", &self.name())
            .field("number", &self.number())
            .field("family", &self.family())
            .field("spin", &self.nspin())
            .finish()
    }
}

impl fmt::Display for LibXCFunctional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LibXCFunctional({} [{}])", self.name(), self.number())
    }
}
