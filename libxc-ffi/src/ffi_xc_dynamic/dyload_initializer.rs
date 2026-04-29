//! Library initializer implementation for dynamic loading.
//!
//! This file is generated automatically.

use super::*;
use libloading::{Library, Symbol};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}

impl DyLoadLib {
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {
        let mut result = DyLoadLib {
            __libraries: vec![],      // dummy, set later
            __libraries_path: vec![], // dummy, set later
            __error: None,
            xc_reference: get_symbol(&libs, b"xc_reference\0").map(|sym| *sym),
            xc_reference_doi: get_symbol(&libs, b"xc_reference_doi\0").map(|sym| *sym),
            xc_reference_key: get_symbol(&libs, b"xc_reference_key\0").map(|sym| *sym),
            xc_version: get_symbol(&libs, b"xc_version\0").map(|sym| *sym),
            xc_version_string: get_symbol(&libs, b"xc_version_string\0").map(|sym| *sym),
            xc_func_reference_get_ref: get_symbol(&libs, b"xc_func_reference_get_ref\0")
                .map(|sym| *sym),
            xc_func_reference_get_doi: get_symbol(&libs, b"xc_func_reference_get_doi\0")
                .map(|sym| *sym),
            xc_func_reference_get_bibtex: get_symbol(&libs, b"xc_func_reference_get_bibtex\0")
                .map(|sym| *sym),
            xc_func_reference_get_key: get_symbol(&libs, b"xc_func_reference_get_key\0")
                .map(|sym| *sym),
            xc_func_info_get_default_flags: get_symbol(&libs, b"xc_func_info_get_default_flags\0")
                .map(|sym| *sym),
            xc_func_info_set_default_flags: get_symbol(&libs, b"xc_func_info_set_default_flags\0")
                .map(|sym| *sym),
            xc_func_info_get_number: get_symbol(&libs, b"xc_func_info_get_number\0")
                .map(|sym| *sym),
            xc_func_info_get_kind: get_symbol(&libs, b"xc_func_info_get_kind\0").map(|sym| *sym),
            xc_func_info_get_name: get_symbol(&libs, b"xc_func_info_get_name\0").map(|sym| *sym),
            xc_func_info_get_family: get_symbol(&libs, b"xc_func_info_get_family\0")
                .map(|sym| *sym),
            xc_func_info_get_flags: get_symbol(&libs, b"xc_func_info_get_flags\0").map(|sym| *sym),
            xc_func_info_get_references: get_symbol(&libs, b"xc_func_info_get_references\0")
                .map(|sym| *sym),
            xc_func_info_get_n_ext_params: get_symbol(&libs, b"xc_func_info_get_n_ext_params\0")
                .map(|sym| *sym),
            xc_func_info_get_ext_params_name: get_symbol(
                &libs,
                b"xc_func_info_get_ext_params_name\0",
            )
            .map(|sym| *sym),
            xc_func_info_get_ext_params_description: get_symbol(
                &libs,
                b"xc_func_info_get_ext_params_description\0",
            )
            .map(|sym| *sym),
            xc_func_info_get_ext_params_default_value: get_symbol(
                &libs,
                b"xc_func_info_get_ext_params_default_value\0",
            )
            .map(|sym| *sym),
            xc_functional_get_number: get_symbol(&libs, b"xc_functional_get_number\0")
                .map(|sym| *sym),
            xc_functional_get_name: get_symbol(&libs, b"xc_functional_get_name\0").map(|sym| *sym),
            xc_family_from_id: get_symbol(&libs, b"xc_family_from_id\0").map(|sym| *sym),
            xc_number_of_functionals: get_symbol(&libs, b"xc_number_of_functionals\0")
                .map(|sym| *sym),
            xc_maximum_name_length: get_symbol(&libs, b"xc_maximum_name_length\0").map(|sym| *sym),
            xc_available_functional_numbers: get_symbol(
                &libs,
                b"xc_available_functional_numbers\0",
            )
            .map(|sym| *sym),
            xc_available_functional_numbers_by_name: get_symbol(
                &libs,
                b"xc_available_functional_numbers_by_name\0",
            )
            .map(|sym| *sym),
            xc_available_functional_names: get_symbol(&libs, b"xc_available_functional_names\0")
                .map(|sym| *sym),
            xc_func_alloc: get_symbol(&libs, b"xc_func_alloc\0").map(|sym| *sym),
            xc_func_init: get_symbol(&libs, b"xc_func_init\0").map(|sym| *sym),
            xc_func_init_flags: get_symbol(&libs, b"xc_func_init_flags\0").map(|sym| *sym),
            xc_func_end: get_symbol(&libs, b"xc_func_end\0").map(|sym| *sym),
            xc_func_free: get_symbol(&libs, b"xc_func_free\0").map(|sym| *sym),
            xc_func_get_info: get_symbol(&libs, b"xc_func_get_info\0").map(|sym| *sym),
            xc_func_set_dens_threshold: get_symbol(&libs, b"xc_func_set_dens_threshold\0")
                .map(|sym| *sym),
            xc_func_set_zeta_threshold: get_symbol(&libs, b"xc_func_set_zeta_threshold\0")
                .map(|sym| *sym),
            xc_func_set_sigma_threshold: get_symbol(&libs, b"xc_func_set_sigma_threshold\0")
                .map(|sym| *sym),
            xc_func_set_tau_threshold: get_symbol(&libs, b"xc_func_set_tau_threshold\0")
                .map(|sym| *sym),
            xc_func_set_fhc_enforcement: get_symbol(&libs, b"xc_func_set_fhc_enforcement\0")
                .map(|sym| *sym),
            xc_func_set_ext_params: get_symbol(&libs, b"xc_func_set_ext_params\0").map(|sym| *sym),
            xc_func_get_ext_params: get_symbol(&libs, b"xc_func_get_ext_params\0").map(|sym| *sym),
            xc_func_set_ext_params_name: get_symbol(&libs, b"xc_func_set_ext_params_name\0")
                .map(|sym| *sym),
            xc_func_get_ext_params_name: get_symbol(&libs, b"xc_func_get_ext_params_name\0")
                .map(|sym| *sym),
            xc_func_get_ext_params_value: get_symbol(&libs, b"xc_func_get_ext_params_value\0")
                .map(|sym| *sym),
            xc_lda_new: get_symbol(&libs, b"xc_lda_new\0").map(|sym| *sym),
            xc_gga_new: get_symbol(&libs, b"xc_gga_new\0").map(|sym| *sym),
            xc_mgga_new: get_symbol(&libs, b"xc_mgga_new\0").map(|sym| *sym),
            xc_lda: get_symbol(&libs, b"xc_lda\0").map(|sym| *sym),
            xc_gga: get_symbol(&libs, b"xc_gga\0").map(|sym| *sym),
            xc_mgga: get_symbol(&libs, b"xc_mgga\0").map(|sym| *sym),
            xc_lda_exc: get_symbol(&libs, b"xc_lda_exc\0").map(|sym| *sym),
            xc_gga_exc: get_symbol(&libs, b"xc_gga_exc\0").map(|sym| *sym),
            xc_mgga_exc: get_symbol(&libs, b"xc_mgga_exc\0").map(|sym| *sym),
            xc_lda_exc_vxc: get_symbol(&libs, b"xc_lda_exc_vxc\0").map(|sym| *sym),
            xc_gga_exc_vxc: get_symbol(&libs, b"xc_gga_exc_vxc\0").map(|sym| *sym),
            xc_mgga_exc_vxc: get_symbol(&libs, b"xc_mgga_exc_vxc\0").map(|sym| *sym),
            xc_lda_vxc: get_symbol(&libs, b"xc_lda_vxc\0").map(|sym| *sym),
            xc_gga_vxc: get_symbol(&libs, b"xc_gga_vxc\0").map(|sym| *sym),
            xc_mgga_vxc: get_symbol(&libs, b"xc_mgga_vxc\0").map(|sym| *sym),
            xc_lda_exc_vxc_fxc: get_symbol(&libs, b"xc_lda_exc_vxc_fxc\0").map(|sym| *sym),
            xc_gga_exc_vxc_fxc: get_symbol(&libs, b"xc_gga_exc_vxc_fxc\0").map(|sym| *sym),
            xc_mgga_exc_vxc_fxc: get_symbol(&libs, b"xc_mgga_exc_vxc_fxc\0").map(|sym| *sym),
            xc_lda_vxc_fxc: get_symbol(&libs, b"xc_lda_vxc_fxc\0").map(|sym| *sym),
            xc_gga_vxc_fxc: get_symbol(&libs, b"xc_gga_vxc_fxc\0").map(|sym| *sym),
            xc_mgga_vxc_fxc: get_symbol(&libs, b"xc_mgga_vxc_fxc\0").map(|sym| *sym),
            xc_lda_fxc: get_symbol(&libs, b"xc_lda_fxc\0").map(|sym| *sym),
            xc_gga_fxc: get_symbol(&libs, b"xc_gga_fxc\0").map(|sym| *sym),
            xc_mgga_fxc: get_symbol(&libs, b"xc_mgga_fxc\0").map(|sym| *sym),
            xc_lda_exc_vxc_fxc_kxc: get_symbol(&libs, b"xc_lda_exc_vxc_fxc_kxc\0").map(|sym| *sym),
            xc_gga_exc_vxc_fxc_kxc: get_symbol(&libs, b"xc_gga_exc_vxc_fxc_kxc\0").map(|sym| *sym),
            xc_mgga_exc_vxc_fxc_kxc: get_symbol(&libs, b"xc_mgga_exc_vxc_fxc_kxc\0")
                .map(|sym| *sym),
            xc_lda_vxc_fxc_kxc: get_symbol(&libs, b"xc_lda_vxc_fxc_kxc\0").map(|sym| *sym),
            xc_gga_vxc_fxc_kxc: get_symbol(&libs, b"xc_gga_vxc_fxc_kxc\0").map(|sym| *sym),
            xc_mgga_vxc_fxc_kxc: get_symbol(&libs, b"xc_mgga_vxc_fxc_kxc\0").map(|sym| *sym),
            xc_lda_kxc: get_symbol(&libs, b"xc_lda_kxc\0").map(|sym| *sym),
            xc_gga_kxc: get_symbol(&libs, b"xc_gga_kxc\0").map(|sym| *sym),
            xc_mgga_kxc: get_symbol(&libs, b"xc_mgga_kxc\0").map(|sym| *sym),
            xc_lda_lxc: get_symbol(&libs, b"xc_lda_lxc\0").map(|sym| *sym),
            xc_gga_lxc: get_symbol(&libs, b"xc_gga_lxc\0").map(|sym| *sym),
            xc_mgga_lxc: get_symbol(&libs, b"xc_mgga_lxc\0").map(|sym| *sym),
            xc_gga_ak13_get_asymptotic: get_symbol(&libs, b"xc_gga_ak13_get_asymptotic\0")
                .map(|sym| *sym),
            xc_gga_ak13_pars_get_asymptotic: get_symbol(
                &libs,
                b"xc_gga_ak13_pars_get_asymptotic\0",
            )
            .map(|sym| *sym),
            xc_hyb_exx_coef: get_symbol(&libs, b"xc_hyb_exx_coef\0").map(|sym| *sym),
            xc_hyb_cam_coef: get_symbol(&libs, b"xc_hyb_cam_coef\0").map(|sym| *sym),
            xc_nlc_coef: get_symbol(&libs, b"xc_nlc_coef\0").map(|sym| *sym),
            xc_num_aux_funcs: get_symbol(&libs, b"xc_num_aux_funcs\0").map(|sym| *sym),
            xc_aux_func_ids: get_symbol(&libs, b"xc_aux_func_ids\0").map(|sym| *sym),
            xc_aux_func_weights: get_symbol(&libs, b"xc_aux_func_weights\0").map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
