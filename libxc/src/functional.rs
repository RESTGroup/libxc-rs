//! Wrapper for libxc functionals (non-computation part).

use crate::prelude::*;

unsafe fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

/// A literature reference for a libxc functional.
#[derive(Debug, Clone)]
pub struct LibXCReference {
    pub ref_text: String,
    pub doi: String,
    pub bibtex: String,
    pub key: String,
}

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

impl core::fmt::Display for LibXCOutputLayout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

/// A safe wrapper around a libxc `xc_func_type` pointer.
///
/// Owns the underlying C resource and frees it on `Drop`.
pub struct LibXCFunctional {
    pub(crate) ptr: *mut ffi::xc_func_type,
}

/// Creation functions implementation.
impl LibXCFunctional {
    /// Create a new functional from a functional ID and spin configuration.
    ///
    /// For api-v7.1+, uses `xc_func_init_flags` with `OnHost`.
    /// For earlier versions, uses `xc_func_init`.
    pub fn from_id(func_id: i32, spin: LibXCSpin) -> Result<Self, LibXCError> {
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
    pub fn from_identifier(name: &str, spin: LibXCSpin) -> Result<Self, LibXCError> {
        let func_id = crate::util::libxc_functional_get_number(name)
            .ok_or_else(|| LibXCError::NotFound(format!("functional '{name}'")))?;
        Self::from_id(func_id, spin)
    }

    #[cfg(feature = "api-v7_1")]
    unsafe fn init_func(ptr: *mut ffi::xc_func_type, func_id: i32, spin: LibXCSpin) -> c_int {
        let flags = ffi::XC_FLAGS_ON_HOST as c_int;
        ffi::xc_func_init_flags(ptr, func_id as c_int, spin as c_int, flags)
    }

    #[cfg(not(feature = "api-v7_1"))]
    unsafe fn init_func(ptr: *mut ffi::xc_func_type, func_id: i32, spin: LibXCSpin) -> c_int {
        ffi::xc_func_init(ptr, func_id as c_int, spin as c_int)
    }
}

/// Information getters and setters implementation.
impl LibXCFunctional {
    /* #region info getters */

    /// Returns a raw pointer to the underlying `xc_func_type`.
    ///
    /// Intended for advanced use; the caller must not free the pointer.
    pub fn as_ptr(&self) -> *const ffi::xc_func_type {
        self.ptr
    }

    /// Returns a raw pointer to the underlying `xc_func_type`.
    ///
    /// Intended for advanced use; the caller must not free the pointer.
    pub fn info(&self) -> *const ffi::xc_func_info_type {
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
    pub fn identifier(&self) -> String {
        unsafe { cstr_to_string(ffi::xc_functional_get_name(self.number())) }
    }

    /// Functional name.
    pub fn info_name(&self) -> String {
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
            _ => panic!("Unknown functional family code: {f}"),
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

    /* #endregion */

    /* #region convenience flag getters */

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

    /* #endregion */

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
                cstr_to_string(ffi::xc_func_info_get_ext_params_name(self.info(), i as c_int))
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
                    i as c_int,
                ))
            })
            .collect()
    }

    /// Default values of the external parameters.
    pub fn ext_param_default_values(&self) -> Vec<f64> {
        let n = self.n_ext_params();
        (0..n)
            .map(|i| unsafe {
                ffi::xc_func_info_get_ext_params_default_value(self.info(), i as c_int)
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
        unsafe { ffi::xc_func_set_fhc_enforcement(self.ptr, on as c_int) }
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
        #[allow(non_snake_case)]
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
        let mut ids = vec![0 as c_int; n as usize];
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

    pub(crate) fn validate_flags(&self, flags: &LibXCDerivativeFlags) -> Result<(), LibXCError> {
        if flags.do_exc && !self.has_exc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have EXC capabilities",
                self.identifier()
            )));
        }
        if flags.do_vxc && !self.has_vxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have VXC capabilities",
                self.identifier()
            )));
        }
        if flags.do_fxc && !self.has_fxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have FXC capabilities",
                self.identifier()
            )));
        }
        if flags.do_kxc && !self.has_kxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have KXC capabilities",
                self.identifier()
            )));
        }
        if flags.do_lxc && !self.has_lxc() {
            return Err(LibXCError::ComputeError(format!(
                "functional '{}' does not have LXC capabilities",
                self.identifier()
            )));
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

impl core::fmt::Debug for LibXCFunctional {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LibXCFunctional")
            .field("name", &self.identifier())
            .field("number", &self.number())
            .field("family", &self.family())
            .field("spin", &self.nspin())
            .finish()
    }
}
