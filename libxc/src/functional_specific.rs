//! Wrapper for libxc functionals (experimental, not implemented by pylibxc).

use crate::prelude::*;

impl LibXCFunctional {
    /// Set the hybrid EXX coefficient for hybrid functionals.
    ///
    /// # Panics
    ///
    /// This function does not allow pure GGA/MGGA/LDA functionals (like
    /// `gga_c_pbe`) **or hybrid CAM-type functionals** (like
    /// `hyb_gga_xc_wb97x_v`), and will panic if called on those functionals.
    ///
    /// Please use [`LibXCFunctional::set_cam_coef`] for hybrid CAM-type
    /// functionals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let mut xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_x3lyp", Polarized);
    /// xc_func.set_hyb_exx_coef(0.50);
    /// println!("xc_func: {:?}", xc_func.hyb_exx_coef());
    /// // should error for wB97X-V because it's a CAM-type hybrid functional
    /// let mut xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x_v", Polarized);
    /// assert!(xc_func.set_hyb_exx_coef_f(0.5).is_err());
    /// ```
    pub fn set_hyb_exx_coef(&mut self, hyb_exx_coef: f64) {
        self.set_hyb_exx_coef_f(hyb_exx_coef).unwrap();
    }

    /// Set the hybrid EXX coefficient for hybrid functionals (fallible
    /// version).
    pub fn set_hyb_exx_coef_f(&mut self, hyb_exx_coef: f64) -> Result<(), LibXCError> {
        // make sure the current functional is hybrid
        let details = if self.is_hyb_cam() {
            Some("Current functional is hybrid cam-type (range-separated), please call `set_cam_coef`.".to_string())
        } else if !matches!(
            self.family(),
            LibXCFamily::HybGGA | LibXCFamily::HybMGGA | LibXCFamily::HybLDA
        ) {
            Some("Current functional is not hybrid.".to_string())
        } else {
            None
        };
        // raise error if not hybrid
        if let Some(details) = details {
            return Err(LibXCError::ParamSetError {
                param_name: "hyb_exx_coef".to_string(),
                details,
            });
        } else {
            unsafe { (*self.ptr).cam_alpha = hyb_exx_coef };
            Ok(())
        }
    }

    /// Set the CAM coefficients for hybrid CAM-type (range-separated)
    /// functionals.
    ///
    /// # Panics
    ///
    /// This function only allows hybrid CAM-type functionals (like
    /// `hyb_gga_xc_wb97x_v`), and will panic if called on other functionals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let mut xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x_v", Polarized);
    /// xc_func.set_cam_coef(0.19, 0.46, 0.33);
    /// println!("xc_func: {:?}", xc_func.cam_coef());
    /// // should error for non-CAM-type hybrid functionals like B3LYP
    /// let mut xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_b3lyp", Polarized);
    /// assert!(xc_func.set_cam_coef_f(0.19, 0.46, 0.33).is_err());
    /// ```
    pub fn set_cam_coef(&mut self, cam_alpha: f64, cam_beta: f64, cam_omega: f64) {
        self.set_cam_coef_f(cam_alpha, cam_beta, cam_omega).unwrap();
    }

    pub fn set_cam_coef_f(
        &mut self,
        cam_alpha: f64,
        cam_beta: f64,
        cam_omega: f64,
    ) -> Result<(), LibXCError> {
        // make sure the current functional is hybrid CAM
        if !self.is_hyb_cam() {
            return Err(LibXCError::ParamSetError {
                param_name: "cam_alpha/beta/omega".to_string(),
                details: "Current functional is not hybrid cam-type (range-separated). Please call `set_hyb_exx_coef` if it is hybrid functional.".to_string(),
            });
        }
        // set the CAM coefficients, which is usually the canonical way to get these
        // parameters
        unsafe {
            (*self.ptr).cam_alpha = cam_alpha;
            (*self.ptr).cam_beta = cam_beta;
            (*self.ptr).cam_omega = cam_omega;
        }
        // also try to set the ext_params
        if self.ext_param_names().iter().any(|name| name == "_alpha") {
            // pattern 1: alpha, beta, omega
            let param_map = [("_alpha", cam_alpha), ("_beta", cam_beta), ("_omega", cam_omega)];
            self.set_ext_param_map_f(param_map.into_iter())?;
        } else if self.ext_param_names().iter().any(|name| name == "_csr") {
            // pattern 2: csr = alpha + beta, clr = alpha, omega
            let param_map =
                [("_csr", cam_alpha + cam_beta), ("_clr", cam_alpha), ("_omega", cam_omega)];
            self.set_ext_param_map_f(param_map.into_iter())?;
        } else if self.ext_param_names().iter().any(|name| name == "_omega") {
            // pattern 3: only omega, with alpha and beta fixed by the functional definition
            let param_map = [("_omega", cam_omega)];
            self.set_ext_param_map_f(param_map.into_iter())?;
        } else {
            // no known pattern, raise here
            return Err(LibXCError::ParamSetError {
                param_name: "ext_params of cam_coef".to_string(),
                details: "ext_params do not match known patterns, bug of this wrapper.".to_string(),
            });
        }
        Ok(())
    }

    /// Set the VV10 coefficients for VV10-type functionals.
    ///
    /// # Panics
    ///
    /// This function only allows VV10-type functionals, and will panic if
    /// called on other functionals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let mut xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x_v", Polarized);
    /// xc_func.set_vv10_coef(8.2, 0.004);
    /// println!("xc_func: {:?}", xc_func.vv10_coef());
    /// // should error for non-VV10-type functionals like B3LYP
    /// let mut xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_b3lyp", Polarized);
    /// assert!(xc_func.set_vv10_coef_f(8.2, 0.004).is_err());
    /// ```
    #[allow(non_snake_case)]
    pub fn set_vv10_coef(&mut self, nlc_b: f64, nlc_C: f64) {
        self.set_vv10_coef_f(nlc_b, nlc_C).unwrap();
    }

    #[allow(non_snake_case)]
    pub fn set_vv10_coef_f(&mut self, nlc_b: f64, nlc_C: f64) -> Result<(), LibXCError> {
        // make sure the current functional is VV10
        if self.vv10_coef().is_none() {
            return Err(LibXCError::ParamSetError {
                param_name: "nlc_b/nlc_C".to_string(),
                details: "Current functional is not VV10.".to_string(),
            });
        }
        // set the VV10 coefficients, which is the canonical way to get these parameters
        unsafe {
            (*self.ptr).nlc_b = nlc_b;
            (*self.ptr).nlc_C = nlc_C;
        }
        // also set the ext_params for specific functionals
        if [LibXCFuncId::GGA_XC_VV10 as i32, LibXCFuncId::HYB_GGA_XC_LC_VV10 as i32]
            .contains(&self.number())
        {
            let param_map = [("_b", nlc_b), ("_C", nlc_C)];
            self.set_ext_param_map_f(param_map.into_iter())?;
        }
        Ok(())
    }
}
