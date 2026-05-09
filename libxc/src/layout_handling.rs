use crate::prelude::*;

/// Flags controlling which derivative levels to compute.
///
/// This struct is somehow cumbersome; but you can also use `usize` levels
/// (0..=4) which are converted to the corresponding flags via `From<usize>`.
#[derive(Debug, Clone, Copy)]
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

impl From<usize> for LibXCDerivativeFlags {
    /// Converts a derivative level (0..=4) to the corresponding flags.
    ///
    /// - Level 0: EXC
    /// - Level 1: EXC + VXC
    /// - level 2: EXC + VXC + FXC
    /// - level 3: EXC + VXC + FXC + KXC
    /// - level 4: EXC + VXC + FXC + KXC + LXC
    fn from(level: usize) -> Self {
        match level {
            0 => Self { do_exc: true, do_vxc: false, do_fxc: false, do_kxc: false, do_lxc: false },
            1 => Self { do_exc: true, do_vxc: true, do_fxc: false, do_kxc: false, do_lxc: false },
            2 => Self { do_exc: true, do_vxc: true, do_fxc: true, do_kxc: false, do_lxc: false },
            3 => Self { do_exc: true, do_vxc: true, do_fxc: true, do_kxc: true, do_lxc: false },
            4 => Self { do_exc: true, do_vxc: true, do_fxc: true, do_kxc: true, do_lxc: true },
            _ => panic!("invalid derivative level {level}"),
        }
    }
}

// -- Output label definitions (mirroring pylibxc output_labels) --------

#[rustfmt::skip]
pub(crate) const LDA_OUTPUT_LABELS: [&'static str; 5] = [
    "zk",                                                               // 1
    "vrho",                                                             // 1
    "v2rho2",                                                           // 1
    "v3rho3",                                                           // 1
    "v4rho4",                                                           // 1
];
const LDA_EXC_END: usize = 1;
const LDA_VXC_END: usize = 2;
const LDA_FXC_END: usize = 3;
const LDA_KXC_END: usize = 4;
const LDA_LXC_END: usize = 5;

#[rustfmt::skip]
pub(crate) const GGA_OUTPUT_LABELS: [&'static str; 15] = [
    "zk",                                                               // 1
    "vrho", "vsigma",                                                   // 2
    "v2rho2", "v2rhosigma", "v2sigma2",                                 // 3
    "v3rho3", "v3rho2sigma", "v3rhosigma2", "v3sigma3",                 // 4
    "v4rho4", "v4rho3sigma", "v4rho2sigma2", "v4rhosigma3", "v4sigma4", // 5
];
const GGA_EXC_END: usize = 1;
const GGA_VXC_END: usize = 3;
const GGA_FXC_END: usize = 6;
const GGA_KXC_END: usize = 10;
const GGA_LXC_END: usize = 15;

#[rustfmt::skip]
pub(crate) const MGGA_OUTPUT_LABELS: [&'static str; 70] = [
    "zk",                                                               // 1
    "vrho", "vsigma", "vlapl", "vtau",                                  // 4
    "v2rho2", "v2rhosigma", "v2rholapl", "v2rhotau", "v2sigma2",        // 10
    "v2sigmalapl", "v2sigmatau", "v2lapl2", "v2lapltau", "v2tau2",
    "v3rho3", "v3rho2sigma", "v3rho2lapl", "v3rho2tau", "v3rhosigma2",  // 20
    "v3rhosigmalapl", "v3rhosigmatau", "v3rholapl2", "v3rholapltau",
    "v3rhotau2", "v3sigma3", "v3sigma2lapl", "v3sigma2tau",
    "v3sigmalapl2", "v3sigmalapltau", "v3sigmatau2", "v3lapl3",
    "v3lapl2tau", "v3lapltau2", "v3tau3",
    "v4rho4", "v4rho3sigma", "v4rho3lapl", "v4rho3tau", "v4rho2sigma2",  // 35
    "v4rho2sigmalapl", "v4rho2sigmatau", "v4rho2lapl2", "v4rho2lapltau",
    "v4rho2tau2", "v4rhosigma3", "v4rhosigma2lapl", "v4rhosigma2tau",
    "v4rhosigmalapl2", "v4rhosigmalapltau", "v4rhosigmatau2",
    "v4rholapl3", "v4rholapl2tau", "v4rholapltau2", "v4rhotau3",
    "v4sigma4", "v4sigma3lapl", "v4sigma3tau", "v4sigma2lapl2",
    "v4sigma2lapltau", "v4sigma2tau2", "v4sigmalapl3", "v4sigmalapl2tau",
    "v4sigmalapltau2", "v4sigmatau3", "v4lapl4", "v4lapl3tau",
    "v4lapl2tau2", "v4lapltau3", "v4tau4",
];
const MGGA_EXC_END: usize = 1;
const MGGA_VXC_END: usize = 5;
const MGGA_FXC_END: usize = 15;
const MGGA_KXC_END: usize = 35;
const MGGA_LXC_END: usize = 70;

/// Maps an output label name to its dimension value from `xc_dimensions`.
pub(crate) fn get_dim(dim: &ffi::xc_dimensions, label: &str) -> i32 {
    match label {
        "zk" => dim.zk,
        "vrho" => dim.vrho,
        "vsigma" => dim.vsigma,
        "vlapl" => dim.vlapl,
        "vtau" => dim.vtau,
        "v2rho2" => dim.v2rho2,
        "v2rhosigma" => dim.v2rhosigma,
        "v2rholapl" => dim.v2rholapl,
        "v2rhotau" => dim.v2rhotau,
        "v2sigma2" => dim.v2sigma2,
        "v2sigmalapl" => dim.v2sigmalapl,
        "v2sigmatau" => dim.v2sigmatau,
        "v2lapl2" => dim.v2lapl2,
        "v2lapltau" => dim.v2lapltau,
        "v2tau2" => dim.v2tau2,
        "v3rho3" => dim.v3rho3,
        "v3rho2sigma" => dim.v3rho2sigma,
        "v3rho2lapl" => dim.v3rho2lapl,
        "v3rho2tau" => dim.v3rho2tau,
        "v3rhosigma2" => dim.v3rhosigma2,
        "v3rhosigmalapl" => dim.v3rhosigmalapl,
        "v3rhosigmatau" => dim.v3rhosigmatau,
        "v3rholapl2" => dim.v3rholapl2,
        "v3rholapltau" => dim.v3rholapltau,
        "v3rhotau2" => dim.v3rhotau2,
        "v3sigma3" => dim.v3sigma3,
        "v3sigma2lapl" => dim.v3sigma2lapl,
        "v3sigma2tau" => dim.v3sigma2tau,
        "v3sigmalapl2" => dim.v3sigmalapl2,
        "v3sigmalapltau" => dim.v3sigmalapltau,
        "v3sigmatau2" => dim.v3sigmatau2,
        "v3lapl3" => dim.v3lapl3,
        "v3lapl2tau" => dim.v3lapl2tau,
        "v3lapltau2" => dim.v3lapltau2,
        "v3tau3" => dim.v3tau3,
        "v4rho4" => dim.v4rho4,
        "v4rho3sigma" => dim.v4rho3sigma,
        "v4rho3lapl" => dim.v4rho3lapl,
        "v4rho3tau" => dim.v4rho3tau,
        "v4rho2sigma2" => dim.v4rho2sigma2,
        "v4rho2sigmalapl" => dim.v4rho2sigmalapl,
        "v4rho2sigmatau" => dim.v4rho2sigmatau,
        "v4rho2lapl2" => dim.v4rho2lapl2,
        "v4rho2lapltau" => dim.v4rho2lapltau,
        "v4rho2tau2" => dim.v4rho2tau2,
        "v4rhosigma3" => dim.v4rhosigma3,
        "v4rhosigma2lapl" => dim.v4rhosigma2lapl,
        "v4rhosigma2tau" => dim.v4rhosigma2tau,
        "v4rhosigmalapl2" => dim.v4rhosigmalapl2,
        "v4rhosigmalapltau" => dim.v4rhosigmalapltau,
        "v4rhosigmatau2" => dim.v4rhosigmatau2,
        "v4rholapl3" => dim.v4rholapl3,
        "v4rholapl2tau" => dim.v4rholapl2tau,
        "v4rholapltau2" => dim.v4rholapltau2,
        "v4rhotau3" => dim.v4rhotau3,
        "v4sigma4" => dim.v4sigma4,
        "v4sigma3lapl" => dim.v4sigma3lapl,
        "v4sigma3tau" => dim.v4sigma3tau,
        "v4sigma2lapl2" => dim.v4sigma2lapl2,
        "v4sigma2lapltau" => dim.v4sigma2lapltau,
        "v4sigma2tau2" => dim.v4sigma2tau2,
        "v4sigmalapl3" => dim.v4sigmalapl3,
        "v4sigmalapl2tau" => dim.v4sigmalapl2tau,
        "v4sigmalapltau2" => dim.v4sigmalapltau2,
        "v4sigmatau3" => dim.v4sigmatau3,
        "v4lapl4" => dim.v4lapl4,
        "v4lapl3tau" => dim.v4lapl3tau,
        "v4lapl2tau2" => dim.v4lapl2tau2,
        "v4lapltau3" => dim.v4lapltau3,
        "v4tau4" => dim.v4tau4,
        _ => 0,
    }
}

/// Mirrors pylibxc's `_check_arrays`: iterates output labels in
/// `labels[start..end]`, pushing those whose derivative level is
/// required and whose lapl/tau needs are satisfied.
pub(crate) fn check_arrays(
    layout: &mut LibXCOutputLayout,
    labels: &[&'static str],
    start: usize,
    end: usize,
    dim: &ffi::xc_dimensions,
    required: bool,
    needs_lapl: bool,
    needs_tau: bool,
) {
    for &label in &labels[start..end] {
        let label_required = required
            && !(!needs_lapl && label.contains("lapl"))
            && !(!needs_tau && label.contains("tau"));
        if label_required {
            layout.push(label, get_dim(dim, label));
        }
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

impl LibXCFunctional {
    // -- Output layout computation ------------------------------------------

    /// Compute the output layout for LDA at a given number of grid points.
    #[rustfmt::skip]
    pub fn lda_output_layout(
        &self,
        npoints: usize,
        flags: LibXCDerivativeFlags,
    ) -> LibXCOutputLayout {
        let dim = self.dim();
        let mut layout = LibXCOutputLayout::new(npoints);
        check_arrays(&mut layout, &LDA_OUTPUT_LABELS, 0, LDA_EXC_END, dim, flags.do_exc, false, false);
        check_arrays(&mut layout, &LDA_OUTPUT_LABELS, LDA_EXC_END, LDA_VXC_END, dim, flags.do_vxc, false, false);
        check_arrays(&mut layout, &LDA_OUTPUT_LABELS, LDA_VXC_END, LDA_FXC_END, dim, flags.do_fxc, false, false);
        check_arrays(&mut layout, &LDA_OUTPUT_LABELS, LDA_FXC_END, LDA_KXC_END, dim, flags.do_kxc, false, false);
        check_arrays(&mut layout, &LDA_OUTPUT_LABELS, LDA_KXC_END, LDA_LXC_END, dim, flags.do_lxc, false, false);
        layout
    }

    /// Compute the output layout for GGA at a given number of grid points.
    #[rustfmt::skip]
    pub fn gga_output_layout(
        &self,
        npoints: usize,
        flags: LibXCDerivativeFlags,
    ) -> LibXCOutputLayout {
        let dim = self.dim();
        let mut layout = LibXCOutputLayout::new(npoints);
        check_arrays(&mut layout, &GGA_OUTPUT_LABELS, 0, GGA_EXC_END, dim, flags.do_exc, false, false);
        check_arrays(&mut layout, &GGA_OUTPUT_LABELS, GGA_EXC_END, GGA_VXC_END, dim, flags.do_vxc, false, false);
        check_arrays(&mut layout, &GGA_OUTPUT_LABELS, GGA_VXC_END, GGA_FXC_END, dim, flags.do_fxc, false, false);
        check_arrays(&mut layout, &GGA_OUTPUT_LABELS, GGA_FXC_END, GGA_KXC_END, dim, flags.do_kxc, false, false);
        check_arrays(&mut layout, &GGA_OUTPUT_LABELS, GGA_KXC_END, GGA_LXC_END, dim, flags.do_lxc, false, false);
        layout
    }

    /// Compute the output layout for MGGA at a given number of grid points.
    #[rustfmt::skip]
    pub fn mgga_output_layout(
        &self,
        npoints: usize,
        flags: LibXCDerivativeFlags,
    ) -> LibXCOutputLayout {
        let dim = self.dim();
        let needs_lapl = self.needs_laplacian();
        let needs_tau = self.needs_tau();
        let mut layout = LibXCOutputLayout::new(npoints);
        check_arrays(&mut layout, &MGGA_OUTPUT_LABELS, 0, MGGA_EXC_END, dim, flags.do_exc, needs_lapl, needs_tau);
        check_arrays(&mut layout, &MGGA_OUTPUT_LABELS, MGGA_EXC_END, MGGA_VXC_END, dim, flags.do_vxc, needs_lapl, needs_tau);
        check_arrays(&mut layout, &MGGA_OUTPUT_LABELS, MGGA_VXC_END, MGGA_FXC_END, dim, flags.do_fxc, needs_lapl, needs_tau);
        check_arrays(&mut layout, &MGGA_OUTPUT_LABELS, MGGA_FXC_END, MGGA_KXC_END, dim, flags.do_kxc, needs_lapl, needs_tau);
        check_arrays(&mut layout, &MGGA_OUTPUT_LABELS, MGGA_KXC_END, MGGA_LXC_END, dim, flags.do_lxc, needs_lapl, needs_tau);
        layout
    }

    /// Compute the output layout for this functional at a given number of grid
    /// points.
    pub fn output_layout(
        &self,
        npoints: usize,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> LibXCOutputLayout {
        use crate::prelude::libxc_enum_items::*;
        let flags = flags.into();
        match self.family() {
            LDA | HybLDA => self.lda_output_layout(npoints, flags),
            GGA | HybGGA => self.gga_output_layout(npoints, flags),
            MGGA | HybMGGA => self.mgga_output_layout(npoints, flags),
            OEP | LCA => unimplemented!("output layout for OEP/LCA is not recognized."),
        }
    }

    // -- Validate derivative flags ------------------------------------------

    pub(crate) fn validate_flags(
        &self,
        flags: impl Into<LibXCDerivativeFlags>,
    ) -> Result<(), LibXCError> {
        let flags = flags.into();
        for (flag, has_cap, name) in [
            (flags.do_exc, self.has_exc(), "EXC"),
            (flags.do_vxc, self.has_vxc(), "VXC"),
            (flags.do_fxc, self.has_fxc(), "FXC"),
            (flags.do_kxc, self.has_kxc(), "KXC"),
            (flags.do_lxc, self.has_lxc(), "LXC"),
        ] {
            if flag && !has_cap {
                return Err(LibXCError::ComputeError(format!(
                    "functional '{}' does not have {name} capabilities",
                    self.identifier(),
                )));
            }
        }
        Ok(())
    }
}
