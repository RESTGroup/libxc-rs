//! Compatible wrapper functions for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime.

use super::*;
use core::ffi::{c_char, c_int};

pub unsafe fn xc_reference() -> *const c_char {
    dyload_lib().xc_reference.unwrap()()
}

pub unsafe fn xc_reference_doi() -> *const c_char {
    dyload_lib().xc_reference_doi.unwrap()()
}

pub unsafe fn xc_reference_key() -> *const c_char {
    dyload_lib().xc_reference_key.unwrap()()
}

pub unsafe fn xc_version(major: *mut c_int, minor: *mut c_int, micro: *mut c_int) {
    dyload_lib().xc_version.unwrap()(major, minor, micro)
}

pub unsafe fn xc_version_string() -> *const c_char {
    dyload_lib().xc_version_string.unwrap()()
}

pub unsafe fn xc_func_reference_get_ref(reference: *const func_reference_type) -> *const c_char {
    dyload_lib().xc_func_reference_get_ref.unwrap()(reference)
}

pub unsafe fn xc_func_reference_get_doi(reference: *const func_reference_type) -> *const c_char {
    dyload_lib().xc_func_reference_get_doi.unwrap()(reference)
}

pub unsafe fn xc_func_reference_get_bibtex(reference: *const func_reference_type) -> *const c_char {
    dyload_lib().xc_func_reference_get_bibtex.unwrap()(reference)
}

pub unsafe fn xc_func_reference_get_key(reference: *const func_reference_type) -> *const c_char {
    dyload_lib().xc_func_reference_get_key.unwrap()(reference)
}

pub unsafe fn xc_func_info_get_default_flags() -> c_int {
    dyload_lib().xc_func_info_get_default_flags.unwrap()()
}

pub unsafe fn xc_func_info_set_default_flags(flags: c_int) {
    dyload_lib().xc_func_info_set_default_flags.unwrap()(flags)
}

pub unsafe fn xc_func_info_get_number(info: *const xc_func_info_type) -> c_int {
    dyload_lib().xc_func_info_get_number.unwrap()(info)
}

pub unsafe fn xc_func_info_get_kind(info: *const xc_func_info_type) -> c_int {
    dyload_lib().xc_func_info_get_kind.unwrap()(info)
}

pub unsafe fn xc_func_info_get_name(info: *const xc_func_info_type) -> *const c_char {
    dyload_lib().xc_func_info_get_name.unwrap()(info)
}

pub unsafe fn xc_func_info_get_family(info: *const xc_func_info_type) -> c_int {
    dyload_lib().xc_func_info_get_family.unwrap()(info)
}

pub unsafe fn xc_func_info_get_flags(info: *const xc_func_info_type) -> c_int {
    dyload_lib().xc_func_info_get_flags.unwrap()(info)
}

pub unsafe fn xc_func_info_get_references(
    info: *const xc_func_info_type,
    number: c_int,
) -> *const func_reference_type {
    dyload_lib().xc_func_info_get_references.unwrap()(info, number)
}

pub unsafe fn xc_func_info_get_n_ext_params(info: *const xc_func_info_type) -> c_int {
    dyload_lib().xc_func_info_get_n_ext_params.unwrap()(info)
}

pub unsafe fn xc_func_info_get_ext_params_name(
    p: *const xc_func_info_type,
    number: c_int,
) -> *const c_char {
    dyload_lib().xc_func_info_get_ext_params_name.unwrap()(p, number)
}

pub unsafe fn xc_func_info_get_ext_params_description(
    info: *const xc_func_info_type,
    number: c_int,
) -> *const c_char {
    dyload_lib().xc_func_info_get_ext_params_description.unwrap()(info, number)
}

pub unsafe fn xc_func_info_get_ext_params_default_value(
    info: *const xc_func_info_type,
    number: c_int,
) -> f64 {
    dyload_lib().xc_func_info_get_ext_params_default_value.unwrap()(info, number)
}

pub unsafe fn xc_functional_get_number(name: *const c_char) -> c_int {
    dyload_lib().xc_functional_get_number.unwrap()(name)
}

pub unsafe fn xc_functional_get_name(number: c_int) -> *mut c_char {
    dyload_lib().xc_functional_get_name.unwrap()(number)
}

pub unsafe fn xc_family_from_id(id: c_int, family: *mut c_int, number: *mut c_int) -> c_int {
    dyload_lib().xc_family_from_id.unwrap()(id, family, number)
}

pub unsafe fn xc_number_of_functionals() -> c_int {
    dyload_lib().xc_number_of_functionals.unwrap()()
}

pub unsafe fn xc_maximum_name_length() -> c_int {
    dyload_lib().xc_maximum_name_length.unwrap()()
}

pub unsafe fn xc_available_functional_numbers(list: *mut c_int) {
    dyload_lib().xc_available_functional_numbers.unwrap()(list)
}

pub unsafe fn xc_available_functional_numbers_by_name(list: *mut c_int) {
    dyload_lib().xc_available_functional_numbers_by_name.unwrap()(list)
}

pub unsafe fn xc_available_functional_names(list: *mut *mut c_char) {
    dyload_lib().xc_available_functional_names.unwrap()(list)
}

pub unsafe fn xc_func_alloc() -> *mut xc_func_type {
    dyload_lib().xc_func_alloc.unwrap()()
}

pub unsafe fn xc_func_init(p: *mut xc_func_type, functional: c_int, nspin: c_int) -> c_int {
    dyload_lib().xc_func_init.unwrap()(p, functional, nspin)
}

pub unsafe fn xc_func_init_flags(
    p: *mut xc_func_type,
    functional: c_int,
    nspin: c_int,
    flags: c_int,
) -> c_int {
    dyload_lib().xc_func_init_flags.unwrap()(p, functional, nspin, flags)
}

pub unsafe fn xc_func_end(p: *mut xc_func_type) {
    dyload_lib().xc_func_end.unwrap()(p)
}

pub unsafe fn xc_func_free(p: *mut xc_func_type) {
    dyload_lib().xc_func_free.unwrap()(p)
}

pub unsafe fn xc_func_get_info(p: *const xc_func_type) -> *const xc_func_info_type {
    dyload_lib().xc_func_get_info.unwrap()(p)
}

pub unsafe fn xc_func_set_dens_threshold(p: *mut xc_func_type, t_dens: f64) {
    dyload_lib().xc_func_set_dens_threshold.unwrap()(p, t_dens)
}

pub unsafe fn xc_func_set_zeta_threshold(p: *mut xc_func_type, t_zeta: f64) {
    dyload_lib().xc_func_set_zeta_threshold.unwrap()(p, t_zeta)
}

pub unsafe fn xc_func_set_sigma_threshold(p: *mut xc_func_type, t_sigma: f64) {
    dyload_lib().xc_func_set_sigma_threshold.unwrap()(p, t_sigma)
}

pub unsafe fn xc_func_set_tau_threshold(p: *mut xc_func_type, t_tau: f64) {
    dyload_lib().xc_func_set_tau_threshold.unwrap()(p, t_tau)
}

pub unsafe fn xc_func_set_fhc_enforcement(p: *mut xc_func_type, on: c_int) {
    dyload_lib().xc_func_set_fhc_enforcement.unwrap()(p, on)
}

pub unsafe fn xc_func_set_ext_params(p: *mut xc_func_type, ext_params: *const f64) {
    dyload_lib().xc_func_set_ext_params.unwrap()(p, ext_params)
}

pub unsafe fn xc_func_get_ext_params(p: *const xc_func_type, ext_params: *mut f64) {
    dyload_lib().xc_func_get_ext_params.unwrap()(p, ext_params)
}

pub unsafe fn xc_func_set_ext_params_name(p: *mut xc_func_type, name: *const c_char, par: f64) {
    dyload_lib().xc_func_set_ext_params_name.unwrap()(p, name, par)
}

pub unsafe fn xc_func_get_ext_params_name(p: *const xc_func_type, name: *const c_char) -> f64 {
    dyload_lib().xc_func_get_ext_params_name.unwrap()(p, name)
}

pub unsafe fn xc_func_get_ext_params_value(p: *const xc_func_type, number: c_int) -> f64 {
    dyload_lib().xc_func_get_ext_params_value.unwrap()(p, number)
}

pub unsafe fn xc_lda_new(
    p: *const xc_func_type,
    order: c_int,
    np: usize,
    rho: *const f64,
    out: *mut xc_lda_out_params,
) {
    dyload_lib().xc_lda_new.unwrap()(p, order, np, rho, out)
}

pub unsafe fn xc_gga_new(
    p: *const xc_func_type,
    order: c_int,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    out: *mut xc_gga_out_params,
) {
    dyload_lib().xc_gga_new.unwrap()(p, order, np, rho, sigma, out)
}

pub unsafe fn xc_mgga_new(
    func: *const xc_func_type,
    order: c_int,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    out: *mut xc_mgga_out_params,
) {
    dyload_lib().xc_mgga_new.unwrap()(func, order, np, rho, sigma, lapl, tau, out)
}

pub unsafe fn xc_lda(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    v2rho2: *mut f64,
    v3rho3: *mut f64,
    v4rho4: *mut f64,
) {
    dyload_lib().xc_lda.unwrap()(p, np, rho, zk, vrho, v2rho2, v3rho3, v4rho4)
}

pub unsafe fn xc_gga(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rhosigma2: *mut f64,
    v3sigma3: *mut f64,
    v4rho4: *mut f64,
    v4rho3sigma: *mut f64,
    v4rho2sigma2: *mut f64,
    v4rhosigma3: *mut f64,
    v4sigma4: *mut f64,
) {
    dyload_lib().xc_gga.unwrap()(
        p,
        np,
        rho,
        sigma,
        zk,
        vrho,
        vsigma,
        v2rho2,
        v2rhosigma,
        v2sigma2,
        v3rho3,
        v3rho2sigma,
        v3rhosigma2,
        v3sigma3,
        v4rho4,
        v4rho3sigma,
        v4rho2sigma2,
        v4rhosigma3,
        v4sigma4,
    )
}

pub unsafe fn xc_mgga(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl_rho: *const f64,
    tau: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    vlapl: *mut f64,
    vtau: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2rholapl: *mut f64,
    v2rhotau: *mut f64,
    v2sigma2: *mut f64,
    v2sigmalapl: *mut f64,
    v2sigmatau: *mut f64,
    v2lapl2: *mut f64,
    v2lapltau: *mut f64,
    v2tau2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rho2lapl: *mut f64,
    v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64,
    v3rhosigmalapl: *mut f64,
    v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64,
    v3rholapltau: *mut f64,
    v3rhotau2: *mut f64,
    v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64,
    v3sigma2tau: *mut f64,
    v3sigmalapl2: *mut f64,
    v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64,
    v3lapl3: *mut f64,
    v3lapl2tau: *mut f64,
    v3lapltau2: *mut f64,
    v3tau3: *mut f64,
    v4rho4: *mut f64,
    v4rho3sigma: *mut f64,
    v4rho3lapl: *mut f64,
    v4rho3tau: *mut f64,
    v4rho2sigma2: *mut f64,
    v4rho2sigmalapl: *mut f64,
    v4rho2sigmatau: *mut f64,
    v4rho2lapl2: *mut f64,
    v4rho2lapltau: *mut f64,
    v4rho2tau2: *mut f64,
    v4rhosigma3: *mut f64,
    v4rhosigma2lapl: *mut f64,
    v4rhosigma2tau: *mut f64,
    v4rhosigmalapl2: *mut f64,
    v4rhosigmalapltau: *mut f64,
    v4rhosigmatau2: *mut f64,
    v4rholapl3: *mut f64,
    v4rholapl2tau: *mut f64,
    v4rholapltau2: *mut f64,
    v4rhotau3: *mut f64,
    v4sigma4: *mut f64,
    v4sigma3lapl: *mut f64,
    v4sigma3tau: *mut f64,
    v4sigma2lapl2: *mut f64,
    v4sigma2lapltau: *mut f64,
    v4sigma2tau2: *mut f64,
    v4sigmalapl3: *mut f64,
    v4sigmalapl2tau: *mut f64,
    v4sigmalapltau2: *mut f64,
    v4sigmatau3: *mut f64,
    v4lapl4: *mut f64,
    v4lapl3tau: *mut f64,
    v4lapl2tau2: *mut f64,
    v4lapltau3: *mut f64,
    v4tau4: *mut f64,
) {
    dyload_lib().xc_mgga.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl_rho,
        tau,
        zk,
        vrho,
        vsigma,
        vlapl,
        vtau,
        v2rho2,
        v2rhosigma,
        v2rholapl,
        v2rhotau,
        v2sigma2,
        v2sigmalapl,
        v2sigmatau,
        v2lapl2,
        v2lapltau,
        v2tau2,
        v3rho3,
        v3rho2sigma,
        v3rho2lapl,
        v3rho2tau,
        v3rhosigma2,
        v3rhosigmalapl,
        v3rhosigmatau,
        v3rholapl2,
        v3rholapltau,
        v3rhotau2,
        v3sigma3,
        v3sigma2lapl,
        v3sigma2tau,
        v3sigmalapl2,
        v3sigmalapltau,
        v3sigmatau2,
        v3lapl3,
        v3lapl2tau,
        v3lapltau2,
        v3tau3,
        v4rho4,
        v4rho3sigma,
        v4rho3lapl,
        v4rho3tau,
        v4rho2sigma2,
        v4rho2sigmalapl,
        v4rho2sigmatau,
        v4rho2lapl2,
        v4rho2lapltau,
        v4rho2tau2,
        v4rhosigma3,
        v4rhosigma2lapl,
        v4rhosigma2tau,
        v4rhosigmalapl2,
        v4rhosigmalapltau,
        v4rhosigmatau2,
        v4rholapl3,
        v4rholapl2tau,
        v4rholapltau2,
        v4rhotau3,
        v4sigma4,
        v4sigma3lapl,
        v4sigma3tau,
        v4sigma2lapl2,
        v4sigma2lapltau,
        v4sigma2tau2,
        v4sigmalapl3,
        v4sigmalapl2tau,
        v4sigmalapltau2,
        v4sigmatau3,
        v4lapl4,
        v4lapl3tau,
        v4lapl2tau2,
        v4lapltau3,
        v4tau4,
    )
}

pub unsafe fn xc_lda_exc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64) {
    dyload_lib().xc_lda_exc.unwrap()(p, np, rho, zk)
}

pub unsafe fn xc_gga_exc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    zk: *mut f64,
) {
    dyload_lib().xc_gga_exc.unwrap()(p, np, rho, sigma, zk)
}

pub unsafe fn xc_mgga_exc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    zk: *mut f64,
) {
    dyload_lib().xc_mgga_exc.unwrap()(p, np, rho, sigma, lapl, tau, zk)
}

pub unsafe fn xc_lda_exc_vxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
) {
    dyload_lib().xc_lda_exc_vxc.unwrap()(p, np, rho, zk, vrho)
}

pub unsafe fn xc_gga_exc_vxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
) {
    dyload_lib().xc_gga_exc_vxc.unwrap()(p, np, rho, sigma, zk, vrho, vsigma)
}

pub unsafe fn xc_mgga_exc_vxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    vlapl: *mut f64,
    vtau: *mut f64,
) {
    dyload_lib().xc_mgga_exc_vxc.unwrap()(
        p, np, rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau,
    )
}

pub unsafe fn xc_lda_vxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64) {
    dyload_lib().xc_lda_vxc.unwrap()(p, np, rho, vrho)
}

pub unsafe fn xc_gga_vxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    vrho: *mut f64,
    vsigma: *mut f64,
) {
    dyload_lib().xc_gga_vxc.unwrap()(p, np, rho, sigma, vrho, vsigma)
}

pub unsafe fn xc_mgga_vxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    vlapl: *mut f64,
    vtau: *mut f64,
) {
    dyload_lib().xc_mgga_vxc.unwrap()(p, np, rho, sigma, lapl, tau, vrho, vsigma, vlapl, vtau)
}

pub unsafe fn xc_lda_exc_vxc_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    v2rho2: *mut f64,
) {
    dyload_lib().xc_lda_exc_vxc_fxc.unwrap()(p, np, rho, zk, vrho, v2rho2)
}

pub unsafe fn xc_gga_exc_vxc_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
) {
    dyload_lib().xc_gga_exc_vxc_fxc.unwrap()(
        p, np, rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
    )
}

pub unsafe fn xc_mgga_exc_vxc_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    vlapl: *mut f64,
    vtau: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2rholapl: *mut f64,
    v2rhotau: *mut f64,
    v2sigma2: *mut f64,
    v2sigmalapl: *mut f64,
    v2sigmatau: *mut f64,
    v2lapl2: *mut f64,
    v2lapltau: *mut f64,
    v2tau2: *mut f64,
) {
    dyload_lib().xc_mgga_exc_vxc_fxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl,
        tau,
        zk,
        vrho,
        vsigma,
        vlapl,
        vtau,
        v2rho2,
        v2rhosigma,
        v2rholapl,
        v2rhotau,
        v2sigma2,
        v2sigmalapl,
        v2sigmatau,
        v2lapl2,
        v2lapltau,
        v2tau2,
    )
}

pub unsafe fn xc_lda_vxc_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    vrho: *mut f64,
    v2rho2: *mut f64,
) {
    dyload_lib().xc_lda_vxc_fxc.unwrap()(p, np, rho, vrho, v2rho2)
}

pub unsafe fn xc_gga_vxc_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
) {
    dyload_lib().xc_gga_vxc_fxc.unwrap()(
        p, np, rho, sigma, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
    )
}

pub unsafe fn xc_mgga_vxc_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    vlapl: *mut f64,
    vtau: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2rholapl: *mut f64,
    v2rhotau: *mut f64,
    v2sigma2: *mut f64,
    v2sigmalapl: *mut f64,
    v2sigmatau: *mut f64,
    v2lapl2: *mut f64,
    v2lapltau: *mut f64,
    v2tau2: *mut f64,
) {
    dyload_lib().xc_mgga_vxc_fxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl,
        tau,
        vrho,
        vsigma,
        vlapl,
        vtau,
        v2rho2,
        v2rhosigma,
        v2rholapl,
        v2rhotau,
        v2sigma2,
        v2sigmalapl,
        v2sigmatau,
        v2lapl2,
        v2lapltau,
        v2tau2,
    )
}

pub unsafe fn xc_lda_fxc(p: *const xc_func_type, np: usize, rho: *const f64, v2rho2: *mut f64) {
    dyload_lib().xc_lda_fxc.unwrap()(p, np, rho, v2rho2)
}

pub unsafe fn xc_gga_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
) {
    dyload_lib().xc_gga_fxc.unwrap()(p, np, rho, sigma, v2rho2, v2rhosigma, v2sigma2)
}

pub unsafe fn xc_mgga_fxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2rholapl: *mut f64,
    v2rhotau: *mut f64,
    v2sigma2: *mut f64,
    v2sigmalapl: *mut f64,
    v2sigmatau: *mut f64,
    v2lapl2: *mut f64,
    v2lapltau: *mut f64,
    v2tau2: *mut f64,
) {
    dyload_lib().xc_mgga_fxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl,
        tau,
        v2rho2,
        v2rhosigma,
        v2rholapl,
        v2rhotau,
        v2sigma2,
        v2sigmalapl,
        v2sigmatau,
        v2lapl2,
        v2lapltau,
        v2tau2,
    )
}

pub unsafe fn xc_lda_exc_vxc_fxc_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    v2rho2: *mut f64,
    v3rho3: *mut f64,
) {
    dyload_lib().xc_lda_exc_vxc_fxc_kxc.unwrap()(p, np, rho, zk, vrho, v2rho2, v3rho3)
}

pub unsafe fn xc_gga_exc_vxc_fxc_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rhosigma2: *mut f64,
    v3sigma3: *mut f64,
) {
    dyload_lib().xc_gga_exc_vxc_fxc_kxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        zk,
        vrho,
        vsigma,
        v2rho2,
        v2rhosigma,
        v2sigma2,
        v3rho3,
        v3rho2sigma,
        v3rhosigma2,
        v3sigma3,
    )
}

pub unsafe fn xc_mgga_exc_vxc_fxc_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    vlapl: *mut f64,
    vtau: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2rholapl: *mut f64,
    v2rhotau: *mut f64,
    v2sigma2: *mut f64,
    v2sigmalapl: *mut f64,
    v2sigmatau: *mut f64,
    v2lapl2: *mut f64,
    v2lapltau: *mut f64,
    v2tau2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rho2lapl: *mut f64,
    v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64,
    v3rhosigmalapl: *mut f64,
    v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64,
    v3rholapltau: *mut f64,
    v3rhotau2: *mut f64,
    v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64,
    v3sigma2tau: *mut f64,
    v3sigmalapl2: *mut f64,
    v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64,
    v3lapl3: *mut f64,
    v3lapl2tau: *mut f64,
    v3lapltau2: *mut f64,
    v3tau3: *mut f64,
) {
    dyload_lib().xc_mgga_exc_vxc_fxc_kxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl,
        tau,
        zk,
        vrho,
        vsigma,
        vlapl,
        vtau,
        v2rho2,
        v2rhosigma,
        v2rholapl,
        v2rhotau,
        v2sigma2,
        v2sigmalapl,
        v2sigmatau,
        v2lapl2,
        v2lapltau,
        v2tau2,
        v3rho3,
        v3rho2sigma,
        v3rho2lapl,
        v3rho2tau,
        v3rhosigma2,
        v3rhosigmalapl,
        v3rhosigmatau,
        v3rholapl2,
        v3rholapltau,
        v3rhotau2,
        v3sigma3,
        v3sigma2lapl,
        v3sigma2tau,
        v3sigmalapl2,
        v3sigmalapltau,
        v3sigmatau2,
        v3lapl3,
        v3lapl2tau,
        v3lapltau2,
        v3tau3,
    )
}

pub unsafe fn xc_lda_vxc_fxc_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    vrho: *mut f64,
    v2rho2: *mut f64,
    v3rho3: *mut f64,
) {
    dyload_lib().xc_lda_vxc_fxc_kxc.unwrap()(p, np, rho, vrho, v2rho2, v3rho3)
}

pub unsafe fn xc_gga_vxc_fxc_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rhosigma2: *mut f64,
    v3sigma3: *mut f64,
) {
    dyload_lib().xc_gga_vxc_fxc_kxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        vrho,
        vsigma,
        v2rho2,
        v2rhosigma,
        v2sigma2,
        v3rho3,
        v3rho2sigma,
        v3rhosigma2,
        v3sigma3,
    )
}

pub unsafe fn xc_mgga_vxc_fxc_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    vlapl: *mut f64,
    vtau: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2rholapl: *mut f64,
    v2rhotau: *mut f64,
    v2sigma2: *mut f64,
    v2sigmalapl: *mut f64,
    v2sigmatau: *mut f64,
    v2lapl2: *mut f64,
    v2lapltau: *mut f64,
    v2tau2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rho2lapl: *mut f64,
    v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64,
    v3rhosigmalapl: *mut f64,
    v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64,
    v3rholapltau: *mut f64,
    v3rhotau2: *mut f64,
    v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64,
    v3sigma2tau: *mut f64,
    v3sigmalapl2: *mut f64,
    v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64,
    v3lapl3: *mut f64,
    v3lapl2tau: *mut f64,
    v3lapltau2: *mut f64,
    v3tau3: *mut f64,
) {
    dyload_lib().xc_mgga_vxc_fxc_kxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl,
        tau,
        vrho,
        vsigma,
        vlapl,
        vtau,
        v2rho2,
        v2rhosigma,
        v2rholapl,
        v2rhotau,
        v2sigma2,
        v2sigmalapl,
        v2sigmatau,
        v2lapl2,
        v2lapltau,
        v2tau2,
        v3rho3,
        v3rho2sigma,
        v3rho2lapl,
        v3rho2tau,
        v3rhosigma2,
        v3rhosigmalapl,
        v3rhosigmatau,
        v3rholapl2,
        v3rholapltau,
        v3rhotau2,
        v3sigma3,
        v3sigma2lapl,
        v3sigma2tau,
        v3sigmalapl2,
        v3sigmalapltau,
        v3sigmatau2,
        v3lapl3,
        v3lapl2tau,
        v3lapltau2,
        v3tau3,
    )
}

pub unsafe fn xc_lda_kxc(p: *const xc_func_type, np: usize, rho: *const f64, v3rho3: *mut f64) {
    dyload_lib().xc_lda_kxc.unwrap()(p, np, rho, v3rho3)
}

pub unsafe fn xc_gga_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rhosigma2: *mut f64,
    v3sigma3: *mut f64,
) {
    dyload_lib().xc_gga_kxc.unwrap()(p, np, rho, sigma, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3)
}

pub unsafe fn xc_mgga_kxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rho2lapl: *mut f64,
    v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64,
    v3rhosigmalapl: *mut f64,
    v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64,
    v3rholapltau: *mut f64,
    v3rhotau2: *mut f64,
    v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64,
    v3sigma2tau: *mut f64,
    v3sigmalapl2: *mut f64,
    v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64,
    v3lapl3: *mut f64,
    v3lapl2tau: *mut f64,
    v3lapltau2: *mut f64,
    v3tau3: *mut f64,
) {
    dyload_lib().xc_mgga_kxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl,
        tau,
        v3rho3,
        v3rho2sigma,
        v3rho2lapl,
        v3rho2tau,
        v3rhosigma2,
        v3rhosigmalapl,
        v3rhosigmatau,
        v3rholapl2,
        v3rholapltau,
        v3rhotau2,
        v3sigma3,
        v3sigma2lapl,
        v3sigma2tau,
        v3sigmalapl2,
        v3sigmalapltau,
        v3sigmatau2,
        v3lapl3,
        v3lapl2tau,
        v3lapltau2,
        v3tau3,
    )
}

pub unsafe fn xc_lda_lxc(p: *const xc_func_type, np: usize, rho: *const f64, v4rho4: *mut f64) {
    dyload_lib().xc_lda_lxc.unwrap()(p, np, rho, v4rho4)
}

pub unsafe fn xc_gga_lxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    v4rho4: *mut f64,
    v4rho3sigma: *mut f64,
    v4rho2sigma2: *mut f64,
    v4rhosigma3: *mut f64,
    v4sigma4: *mut f64,
) {
    dyload_lib().xc_gga_lxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        v4rho4,
        v4rho3sigma,
        v4rho2sigma2,
        v4rhosigma3,
        v4sigma4,
    )
}

pub unsafe fn xc_mgga_lxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    v4rho4: *mut f64,
    v4rho3sigma: *mut f64,
    v4rho3lapl: *mut f64,
    v4rho3tau: *mut f64,
    v4rho2sigma2: *mut f64,
    v4rho2sigmalapl: *mut f64,
    v4rho2sigmatau: *mut f64,
    v4rho2lapl2: *mut f64,
    v4rho2lapltau: *mut f64,
    v4rho2tau2: *mut f64,
    v4rhosigma3: *mut f64,
    v4rhosigma2lapl: *mut f64,
    v4rhosigma2tau: *mut f64,
    v4rhosigmalapl2: *mut f64,
    v4rhosigmalapltau: *mut f64,
    v4rhosigmatau2: *mut f64,
    v4rholapl3: *mut f64,
    v4rholapl2tau: *mut f64,
    v4rholapltau2: *mut f64,
    v4rhotau3: *mut f64,
    v4sigma4: *mut f64,
    v4sigma3lapl: *mut f64,
    v4sigma3tau: *mut f64,
    v4sigma2lapl2: *mut f64,
    v4sigma2lapltau: *mut f64,
    v4sigma2tau2: *mut f64,
    v4sigmalapl3: *mut f64,
    v4sigmalapl2tau: *mut f64,
    v4sigmalapltau2: *mut f64,
    v4sigmatau3: *mut f64,
    v4lapl4: *mut f64,
    v4lapl3tau: *mut f64,
    v4lapl2tau2: *mut f64,
    v4lapltau3: *mut f64,
    v4tau4: *mut f64,
) {
    dyload_lib().xc_mgga_lxc.unwrap()(
        p,
        np,
        rho,
        sigma,
        lapl,
        tau,
        v4rho4,
        v4rho3sigma,
        v4rho3lapl,
        v4rho3tau,
        v4rho2sigma2,
        v4rho2sigmalapl,
        v4rho2sigmatau,
        v4rho2lapl2,
        v4rho2lapltau,
        v4rho2tau2,
        v4rhosigma3,
        v4rhosigma2lapl,
        v4rhosigma2tau,
        v4rhosigmalapl2,
        v4rhosigmalapltau,
        v4rhosigmatau2,
        v4rholapl3,
        v4rholapl2tau,
        v4rholapltau2,
        v4rhotau3,
        v4sigma4,
        v4sigma3lapl,
        v4sigma3tau,
        v4sigma2lapl2,
        v4sigma2lapltau,
        v4sigma2tau2,
        v4sigmalapl3,
        v4sigmalapl2tau,
        v4sigmalapltau2,
        v4sigmatau3,
        v4lapl4,
        v4lapl3tau,
        v4lapl2tau2,
        v4lapltau3,
        v4tau4,
    )
}

pub unsafe fn xc_gga_ak13_get_asymptotic(homo: f64) -> f64 {
    dyload_lib().xc_gga_ak13_get_asymptotic.unwrap()(homo)
}

pub unsafe fn xc_gga_ak13_pars_get_asymptotic(homo: f64, ext_params: *const f64) -> f64 {
    dyload_lib().xc_gga_ak13_pars_get_asymptotic.unwrap()(homo, ext_params)
}

pub unsafe fn xc_hyb_exx_coef(p: *const xc_func_type) -> f64 {
    dyload_lib().xc_hyb_exx_coef.unwrap()(p)
}

pub unsafe fn xc_hyb_cam_coef(
    p: *const xc_func_type,
    omega: *mut f64,
    alpha: *mut f64,
    beta: *mut f64,
) {
    dyload_lib().xc_hyb_cam_coef.unwrap()(p, omega, alpha, beta)
}

pub unsafe fn xc_nlc_coef(p: *const xc_func_type, nlc_b: *mut f64, nlc_C: *mut f64) {
    dyload_lib().xc_nlc_coef.unwrap()(p, nlc_b, nlc_C)
}

pub unsafe fn xc_num_aux_funcs(p: *const xc_func_type) -> c_int {
    dyload_lib().xc_num_aux_funcs.unwrap()(p)
}

pub unsafe fn xc_aux_func_ids(p: *const xc_func_type, ids: *mut c_int) {
    dyload_lib().xc_aux_func_ids.unwrap()(p, ids)
}

pub unsafe fn xc_aux_func_weights(p: *const xc_func_type, weights: *mut f64) {
    dyload_lib().xc_aux_func_weights.unwrap()(p, weights)
}
