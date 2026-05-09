//! Tests for LibXCFunctional (translated from pylibxc test_functional.py).

use libxc::prelude::libxc_enum_items::*;
use libxc::prelude::*;
use std::collections::HashMap;

// Ignored tests:

const COMPUTE_TEST_DIM: usize = 5;

/// Simple PCG-style PRNG for deterministic test data.
fn pseudo_random(seed: u64, n: usize) -> Vec<f64> {
    let mut state = seed;
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        v.push(((state >> 33) as f64 + 0.5) / (1u64 << 31) as f64);
    }
    v
}

/// Compare two slices with relative and absolute tolerance (mirrors
/// np.allclose).
fn allclose(a: &[f64], b: &[f64], rtol: f64, atol: f64) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(&x, &y)| (x - y).abs() <= atol + rtol * y.abs())
}

/// Extract a named component from a (buffer, layout) pair.
fn get_component<'a>(buf: &'a [f64], layout: &LibXCOutputLayout, name: &str) -> Option<&'a [f64]> {
    layout.get(name).map(|range| &buf[range])
}

/// Compare named components between two (buffer, layout) pairs.
fn dict_array_comp(
    buf_a: &[f64],
    layout_a: &LibXCOutputLayout,
    buf_b: &[f64],
    layout_b: &LibXCOutputLayout,
    keys: &[&str],
) -> bool {
    for &key in keys {
        let a = get_component(buf_a, layout_a, key);
        let b = get_component(buf_b, layout_b, key);
        match (a, b) {
            (Some(a), Some(b)) => {
                if !allclose(a, b, 1e-7, 1e-14) {
                    return false;
                }
            },
            (None, None) => {},
            _ => return false,
        }
    }
    true
}

// -- Spin dimension tuples: (rho_dim, sigma_dim, lapl_dim, tau_dim) --------
fn size_tuple(spin: LibXCSpin) -> (usize, usize, usize, usize) {
    match spin {
        Unpolarized => (1, 1, 1, 1),
        Polarized => (2, 3, 2, 2),
    }
}

// ===========================================================================
// test_libxc_functional_build
// ===========================================================================

#[test]
fn test_libxc_functional_build() {
    // Numeric ID
    LibXCFunctional::from_number(1, Unpolarized);
    LibXCFunctional::from_number(1, Polarized);

    // String name (XC_ prefix and lowercase both work)
    LibXCFunctional::from_identifier("XC_LDA_C_VWN", Polarized);
    LibXCFunctional::from_identifier("lda_c_vwn", Unpolarized);

    // Invalid functional name
    assert!(LibXCFunctional::from_identifier_f("something", Unpolarized).is_err());

    // Invalid functional number
    assert!(LibXCFunctional::from_number_f(5000, Unpolarized).is_err());
}

// ===========================================================================
// test_libxc_functional_info
// ===========================================================================

#[test]
fn test_libxc_functional_info() {
    // LDA_X (functional id 1)
    let func = LibXCFunctional::from_number(1, Unpolarized);
    assert_eq!(func.number(), 1);
    assert_eq!(func.kind(), LibXCFunctionalKind::Exchange);
    assert_eq!(func.info_name(), "Slater exchange");
    assert_eq!(func.family(), LibXCFamily::LDA);
    // Flags: need HAVE_EXC + 3D
    assert!(func.has_exc());
    assert!(func.flags().contains(Dim3));
    // References
    assert_eq!(func.references().len(), 2);

    // HYB_MGGA_XC_WB97M_V
    let func = LibXCFunctional::from_identifier("hyb_mgga_xc_wb97m_v", Unpolarized);
    assert_eq!(func.number(), 531);
    assert_eq!(func.kind(), LibXCFunctionalKind::ExchangeCorrelation);
    assert_eq!(func.info_name(), "wB97M-V exchange-correlation functional");
    assert_eq!(func.family(), LibXCFamily::HybMGGA);
    // Flags: HAVE_EXC + 3D + HYB_CAM + VV10 + NEEDS_TAU
    assert!(func.has_exc());
    assert!(func.flags().contains(Dim3));
    assert!(func.is_hyb_cam());
    assert!(func.flags().contains(VV10));
    assert!(func.needs_tau());
    assert_eq!(func.references().len(), 1);
}

// ===========================================================================
// test_ext_params
// ===========================================================================

#[test]
fn test_ext_params() {
    // Functional with no external parameters
    let func = LibXCFunctional::from_number(1, Unpolarized);
    assert_eq!(func.n_ext_params(), 0);
    assert!(func.ext_param_descriptions().is_empty());
    assert!(func.ext_param_default_values().is_empty());

    let mut func = func;
    func.set_dens_threshold(1e-16);
    func.set_dens_threshold(5.0);

    // Functional with external parameters: HYB_GGA_XC_HSE06
    let mut func = LibXCFunctional::from_identifier("hyb_gga_xc_hse06", Unpolarized);
    assert_eq!(func.n_ext_params(), 3);
    assert_eq!(func.ext_param_descriptions().len(), 3);
    assert_eq!(func.ext_param_default_values().len(), 3);
    assert!(func.ext_param_descriptions().iter().all(|x| x.contains("param")));

    func.set_dens_threshold(1e-16);
    func.set_dens_threshold(5.0);

    // Valid set
    func.set_ext_params(&[5.0, 3.0, 3.0]);
}

#[test]
#[should_panic(expected = "Expected 3 external parameters")]
fn test_ext_params_wrong_length() {
    let mut func = LibXCFunctional::from_identifier("hyb_gga_xc_hse06", Unpolarized);
    func.set_ext_params(&[5.0, 3.0]); // should be 3 params
}

// ===========================================================================
// test_lda_compute
// ===========================================================================

fn run_lda_compute_test(spin: LibXCSpin) {
    let ndim = size_tuple(spin);
    let rho: Vec<f64> = pseudo_random(42, COMPUTE_TEST_DIM * ndim.0);
    let input = HashMap::from([("rho", rho.as_slice())]);

    let func = LibXCFunctional::from_identifier("lda_c_vwn", spin);

    let do_e = func.has_exc();
    let do_v = func.has_vxc();
    let do_f = func.has_fxc();
    let do_k = func.has_kxc();
    let do_l = func.has_lxc();

    let flags_full = LibXCDerivativeFlags {
        do_exc: do_e,
        do_vxc: do_v,
        do_fxc: do_f,
        do_kxc: do_k,
        do_lxc: do_l,
    };
    let flags_ev = LibXCDerivativeFlags {
        do_exc: do_e,
        do_vxc: do_v,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_e = LibXCDerivativeFlags {
        do_exc: do_e,
        do_vxc: false,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_v = LibXCDerivativeFlags {
        do_exc: false,
        do_vxc: do_v,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_f = LibXCDerivativeFlags {
        do_exc: false,
        do_vxc: false,
        do_fxc: do_f,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_k = LibXCDerivativeFlags {
        do_exc: false,
        do_vxc: false,
        do_fxc: false,
        do_kxc: do_k,
        do_lxc: false,
    };
    let flags_l = LibXCDerivativeFlags {
        do_exc: false,
        do_vxc: false,
        do_fxc: false,
        do_kxc: false,
        do_lxc: do_l,
    };

    let (buf_full, layout_full) = func.compute_xc(&input, flags_full).unwrap();
    let (buf_ev, layout_ev) = func.compute_xc(&input, flags_ev).unwrap();
    let (buf_e, layout_e) = func.compute_xc(&input, flags_e).unwrap();
    let (buf_v, layout_v) = func.compute_xc(&input, flags_v).unwrap();

    // Size checks
    if do_e {
        assert_eq!(get_component(&buf_full, &layout_full, "zk").unwrap().len(), COMPUTE_TEST_DIM);
    }
    if do_v {
        assert_eq!(
            get_component(&buf_full, &layout_full, "vrho").unwrap().len(),
            COMPUTE_TEST_DIM * ndim.0
        );
    }

    // Consistency: full vs exc+vxc
    if do_e {
        assert!(allclose(
            get_component(&buf_full, &layout_full, "zk").unwrap(),
            get_component(&buf_ev, &layout_ev, "zk").unwrap(),
            1e-7,
            1e-14,
        ));
    }
    if do_v {
        assert!(allclose(
            get_component(&buf_full, &layout_full, "vrho").unwrap(),
            get_component(&buf_ev, &layout_ev, "vrho").unwrap(),
            1e-7,
            1e-14,
        ));
    }

    // Consistency: full vs single-level
    if do_e {
        assert!(allclose(
            get_component(&buf_full, &layout_full, "zk").unwrap(),
            get_component(&buf_e, &layout_e, "zk").unwrap(),
            1e-7,
            1e-14,
        ));
    }
    if do_v {
        assert!(allclose(
            get_component(&buf_full, &layout_full, "vrho").unwrap(),
            get_component(&buf_v, &layout_v, "vrho").unwrap(),
            1e-7,
            1e-14,
        ));
    }

    if do_f {
        let (buf_f, layout_f) = func.compute_xc(&input, flags_f).unwrap();
        assert!(allclose(
            get_component(&buf_full, &layout_full, "v2rho2").unwrap(),
            get_component(&buf_f, &layout_f, "v2rho2").unwrap(),
            1e-7,
            1e-14,
        ));
    }
    if do_k {
        let (buf_k, layout_k) = func.compute_xc(&input, flags_k).unwrap();
        assert!(allclose(
            get_component(&buf_full, &layout_full, "v3rho3").unwrap(),
            get_component(&buf_k, &layout_k, "v3rho3").unwrap(),
            1e-7,
            1e-14,
        ));
    }
    if do_l {
        let (buf_l, layout_l) = func.compute_xc(&input, flags_l).unwrap();
        assert!(allclose(
            get_component(&buf_full, &layout_full, "v4rho4").unwrap(),
            get_component(&buf_l, &layout_l, "v4rho4").unwrap(),
            1e-7,
            1e-14,
        ));
    }
}

#[test]
fn test_lda_compute_unpolarized() {
    run_lda_compute_test(Unpolarized);
}

#[test]
fn test_lda_compute_polarized() {
    run_lda_compute_test(Polarized);
}

// ===========================================================================
// test_gga_compute
// ===========================================================================

fn run_gga_compute_test(spin: LibXCSpin) {
    let ndim = size_tuple(spin);
    let rho: Vec<f64> = pseudo_random(43, COMPUTE_TEST_DIM * ndim.0);
    let sigma: Vec<f64> = pseudo_random(44, COMPUTE_TEST_DIM * ndim.1);
    let input = HashMap::from([("rho", rho.as_slice()), ("sigma", sigma.as_slice())]);

    let func = LibXCFunctional::from_identifier("gga_c_pbe", spin);

    let do_e = func.has_exc();
    let do_v = func.has_vxc();
    let do_f = func.has_fxc();
    let do_k = func.has_kxc();
    let do_l = func.has_lxc();

    let flags_full = LibXCDerivativeFlags {
        do_exc: do_e,
        do_vxc: do_v,
        do_fxc: do_f,
        do_kxc: do_k,
        do_lxc: do_l,
    };
    let (buf_full, layout_full) = func.compute_xc(&input, flags_full).unwrap();

    // Size checks
    if do_e {
        assert_eq!(get_component(&buf_full, &layout_full, "zk").unwrap().len(), COMPUTE_TEST_DIM);
    }
    if do_v {
        assert_eq!(
            get_component(&buf_full, &layout_full, "vrho").unwrap().len(),
            COMPUTE_TEST_DIM * ndim.0
        );
        assert_eq!(
            get_component(&buf_full, &layout_full, "vsigma").unwrap().len(),
            COMPUTE_TEST_DIM * ndim.1
        );
    }

    // Consistency: exc+vxc vs individual
    if do_e && do_v {
        let flags_ev = LibXCDerivativeFlags {
            do_exc: do_e,
            do_vxc: do_v,
            do_fxc: false,
            do_kxc: false,
            do_lxc: false,
        };
        let (buf_ev, layout_ev) = func.compute_xc(&input, flags_ev).unwrap();
        assert!(dict_array_comp(&buf_full, &layout_full, &buf_ev, &layout_ev, &[
            "zk", "vrho", "vsigma"
        ]));
    }

    if do_e {
        let flags_e = LibXCDerivativeFlags {
            do_exc: do_e,
            do_vxc: false,
            do_fxc: false,
            do_kxc: false,
            do_lxc: false,
        };
        let (buf_e, layout_e) = func.compute_xc(&input, flags_e).unwrap();
        assert!(dict_array_comp(&buf_full, &layout_full, &buf_e, &layout_e, &["zk"]));
    }
    if do_v {
        let flags_v = LibXCDerivativeFlags {
            do_exc: false,
            do_vxc: do_v,
            do_fxc: false,
            do_kxc: false,
            do_lxc: false,
        };
        let (buf_v, layout_v) = func.compute_xc(&input, flags_v).unwrap();
        assert!(dict_array_comp(&buf_full, &layout_full, &buf_v, &layout_v, &["vrho", "vsigma"]));
    }
    if do_f {
        let flags_f = LibXCDerivativeFlags {
            do_exc: false,
            do_vxc: false,
            do_fxc: do_f,
            do_kxc: false,
            do_lxc: false,
        };
        let (buf_f, layout_f) = func.compute_xc(&input, flags_f).unwrap();
        assert!(dict_array_comp(&buf_full, &layout_full, &buf_f, &layout_f, &[
            "v2rho2",
            "v2rhosigma",
            "v2sigma2"
        ]));
    }
    if do_k {
        let flags_k = LibXCDerivativeFlags {
            do_exc: false,
            do_vxc: false,
            do_fxc: false,
            do_kxc: do_k,
            do_lxc: false,
        };
        let (buf_k, layout_k) = func.compute_xc(&input, flags_k).unwrap();
        assert!(dict_array_comp(&buf_full, &layout_full, &buf_k, &layout_k, &[
            "v3rho3",
            "v3rho2sigma",
            "v3rhosigma2",
            "v3sigma3"
        ]));
    }
    if do_l {
        let flags_l = LibXCDerivativeFlags {
            do_exc: false,
            do_vxc: false,
            do_fxc: false,
            do_kxc: false,
            do_lxc: do_l,
        };
        let (buf_l, layout_l) = func.compute_xc(&input, flags_l).unwrap();
        assert!(dict_array_comp(&buf_full, &layout_full, &buf_l, &layout_l, &[
            "v4rho4",
            "v4rho3sigma",
            "v4rho2sigma2",
            "v4rhosigma3",
            "v4sigma4"
        ]));
    }
}

#[test]
fn test_gga_compute_unpolarized() {
    run_gga_compute_test(Unpolarized);
}

#[test]
fn test_gga_compute_polarized() {
    run_gga_compute_test(Polarized);
}

// ===========================================================================
// test_mgga_compute
// ===========================================================================

fn run_mgga_compute_test(spin: LibXCSpin) {
    let ndim = size_tuple(spin);
    let rho: Vec<f64> = pseudo_random(45, COMPUTE_TEST_DIM * ndim.0);
    let sigma: Vec<f64> = pseudo_random(46, COMPUTE_TEST_DIM * ndim.1);
    let tau: Vec<f64> = pseudo_random(47, COMPUTE_TEST_DIM * ndim.3);
    let input = HashMap::from([
        ("rho", rho.as_slice()),
        ("sigma", sigma.as_slice()),
        ("tau", tau.as_slice()),
    ]);

    let func = LibXCFunctional::from_identifier("mgga_c_tpss", spin);

    // mgga_c_tpss has exc + vxc, may have fxc depending on version
    let flags_ev = LibXCDerivativeFlags {
        do_exc: true,
        do_vxc: true,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_e = LibXCDerivativeFlags {
        do_exc: true,
        do_vxc: false,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_v = LibXCDerivativeFlags {
        do_exc: false,
        do_vxc: true,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };

    let (buf_ev, layout_ev) = func.compute_xc(&input, flags_ev).unwrap();
    let (buf_e, layout_e) = func.compute_xc(&input, flags_e).unwrap();
    let (buf_v, layout_v) = func.compute_xc(&input, flags_v).unwrap();

    // Size checks
    assert_eq!(get_component(&buf_ev, &layout_ev, "zk").unwrap().len(), COMPUTE_TEST_DIM);
    assert_eq!(
        get_component(&buf_ev, &layout_ev, "vrho").unwrap().len(),
        COMPUTE_TEST_DIM * ndim.0
    );
    assert_eq!(
        get_component(&buf_ev, &layout_ev, "vsigma").unwrap().len(),
        COMPUTE_TEST_DIM * ndim.1
    );

    // Consistency
    assert!(dict_array_comp(&buf_ev, &layout_ev, &buf_e, &layout_e, &["zk"]));
    assert!(dict_array_comp(&buf_ev, &layout_ev, &buf_v, &layout_v, &["vrho", "vsigma", "vtau"]));
}

#[test]
fn test_mgga_compute_unpolarized() {
    run_mgga_compute_test(Unpolarized);
}

#[test]
fn test_mgga_compute_polarized() {
    run_mgga_compute_test(Polarized);
}

// ===========================================================================
// test_mgga_lapl_compute
// ===========================================================================

fn run_mgga_lapl_compute_test(spin: LibXCSpin) {
    let ndim = size_tuple(spin);
    let rho: Vec<f64> = pseudo_random(48, COMPUTE_TEST_DIM * ndim.0);
    let sigma: Vec<f64> = pseudo_random(49, COMPUTE_TEST_DIM * ndim.1);
    let tau: Vec<f64> = pseudo_random(50, COMPUTE_TEST_DIM * ndim.3);
    let lapl: Vec<f64> = pseudo_random(51, COMPUTE_TEST_DIM * ndim.3);

    let func = LibXCFunctional::from_identifier("mgga_x_br89", spin);

    let input = HashMap::from([
        ("rho", rho.as_slice()),
        ("sigma", sigma.as_slice()),
        ("tau", tau.as_slice()),
        ("lapl", lapl.as_slice()),
    ]);

    let flags_ev = LibXCDerivativeFlags {
        do_exc: true,
        do_vxc: true,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_e = LibXCDerivativeFlags {
        do_exc: true,
        do_vxc: false,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };
    let flags_v = LibXCDerivativeFlags {
        do_exc: false,
        do_vxc: true,
        do_fxc: false,
        do_kxc: false,
        do_lxc: false,
    };

    let (buf_ev, layout_ev) = func.compute_xc(&input, flags_ev).unwrap();
    let (buf_e, layout_e) = func.compute_xc(&input, flags_e).unwrap();
    let (buf_v, layout_v) = func.compute_xc(&input, flags_v).unwrap();

    // Size checks
    assert_eq!(get_component(&buf_ev, &layout_ev, "zk").unwrap().len(), COMPUTE_TEST_DIM);
    assert_eq!(
        get_component(&buf_ev, &layout_ev, "vrho").unwrap().len(),
        COMPUTE_TEST_DIM * ndim.0
    );
    assert_eq!(
        get_component(&buf_ev, &layout_ev, "vsigma").unwrap().len(),
        COMPUTE_TEST_DIM * ndim.1
    );

    // Consistency
    assert!(dict_array_comp(&buf_ev, &layout_ev, &buf_e, &layout_e, &["zk"]));
    assert!(dict_array_comp(&buf_ev, &layout_ev, &buf_v, &layout_v, &["vrho", "vsigma", "vtau"]));

    // Missing "lapl" input should fail for a functional that needs laplacian
    let input_no_lapl = HashMap::from([
        ("rho", rho.as_slice()),
        ("sigma", sigma.as_slice()),
        ("tau", tau.as_slice()),
    ]);
    assert!(func.compute_xc(&input_no_lapl, flags_ev).is_err());
}

#[test]
fn test_mgga_lapl_compute_unpolarized() {
    run_mgga_lapl_compute_test(Unpolarized);
}

#[test]
fn test_mgga_lapl_compute_polarized() {
    run_mgga_lapl_compute_test(Polarized);
}

// ===========================================================================
// test_deriv_flags
// ===========================================================================

#[test]
fn test_deriv_flags_rejects_unsupported() {
    // Requesting FXC on a functional that doesn't support it should error
    let rho: Vec<f64> = vec![0.1; 5];
    let sigma: Vec<f64> = vec![0.1; 5];
    let tau: Vec<f64> = vec![0.1; 5];
    let lapl: Vec<f64> = vec![0.1; 5];
    let input = HashMap::from([
        ("rho", rho.as_slice()),
        ("sigma", sigma.as_slice()),
        ("tau", tau.as_slice()),
        ("lapl", lapl.as_slice()),
    ]);

    // mgga_c_tpss: if it doesn't have fxc, requesting it should fail
    let func = LibXCFunctional::from_identifier("mgga_c_tpss", Unpolarized);
    if !func.has_fxc() {
        let flags_fxc = LibXCDerivativeFlags {
            do_exc: false,
            do_vxc: false,
            do_fxc: true,
            do_kxc: false,
            do_lxc: false,
        };
        assert!(func.compute_xc(&input, flags_fxc).is_err());
    }
}

// ===========================================================================
// test_hyb_getters
// ===========================================================================

#[test]
fn test_hyb_getters() {
    let func = LibXCFunctional::from_identifier("hyb_gga_xc_b3lyp", Unpolarized);
    let exx = func.hyb_exx_coef();
    assert!(exx.is_some());
    assert!((exx.unwrap() - 0.2).abs() < 1e-10);

    // B3LYP is not a CAM functional
    assert!(func.cam_coef().is_none());
    // B3LYP is not VV10
    assert!(func.vv10_coef().is_none());
}

// ===========================================================================
// test_cam_getters
// ===========================================================================

#[test]
fn test_cam_getters() {
    let func = LibXCFunctional::from_identifier("hyb_gga_xc_cam_b3lyp", Unpolarized);

    // CAM-B3LYP is not a global hybrid (it's range-separated)
    assert!(func.hyb_exx_coef().is_none());

    let cam = func.cam_coef();
    assert!(cam.is_some());
    let (omega, alpha, beta) = cam.unwrap();
    assert!((omega - 0.33).abs() < 1e-10);
    assert!((alpha - 0.65).abs() < 1e-10);
    assert!((beta - (-0.46)).abs() < 1e-10);

    // CAM-B3LYP is not VV10
    assert!(func.vv10_coef().is_none());
}

// ===========================================================================
// test_vv10_getters
// ===========================================================================

#[test]
fn test_vv10_getters() {
    let func = LibXCFunctional::from_identifier("gga_xc_vv10", Unpolarized);

    let vv10 = func.vv10_coef();
    assert!(vv10.is_some());
    let (b, c) = vv10.unwrap();
    assert!((b - 5.9).abs() < 1e-10);
    assert!((c - 0.0093).abs() < 1e-10);

    // VV10 is not a CAM functional
    assert!(func.cam_coef().is_none());
    // VV10 is not a global hybrid
    assert!(func.hyb_exx_coef().is_none());
}
