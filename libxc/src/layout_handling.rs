use crate::prelude::*;

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

impl LibXCFunctional {
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
