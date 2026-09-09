//! Tests for cloning and thread-sharing of `LibXCFunctional`.
//!
//! `LibXCFunctional` clones share the underlying C functional through
//! reference counting. These tests verify: shared identity of clones,
//! lock-free parallel evaluation (rayon and raw threads), and the
//! exclusivity requirement of all mutating setters.

use libxc::prelude::libxc_enum_items::*;
use libxc::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

/// Simple PCG-style PRNG for deterministic test data (same as
/// test_functional.rs).
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

/// Spin dimension of an input array.
fn input_dim(name: &str, spin: LibXCSpin) -> usize {
    match (name, spin) {
        ("rho", Unpolarized) => 1,
        ("rho", Polarized) => 2,
        ("sigma", Unpolarized) => 1,
        ("sigma", Polarized) => 3,
        ("lapl", Unpolarized) | ("tau", Unpolarized) => 1,
        ("lapl", Polarized) | ("tau", Polarized) => 2,
        _ => panic!("unknown input name: {name}"),
    }
}

// ===========================================================================
// Send / Sync
// ===========================================================================

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn functional_is_send_and_sync() {
    assert_send::<LibXCFunctional>();
    assert_sync::<LibXCFunctional>();
    // sharing &functional with other threads (rayon) requires Sync
    assert_send::<&LibXCFunctional>();
}

// ===========================================================================
// Clone semantics
// ===========================================================================

#[test]
fn clone_shares_underlying_functional() {
    let func = LibXCFunctional::from_identifier("gga_c_pbe", Unpolarized);
    let cloned = func.clone();

    // both handles reference the same C object (no FFI-level copy)
    assert_eq!(func.as_ptr(), cloned.as_ptr());

    // both compute the same numbers
    let rho: Vec<f64> = pseudo_random(11, 40);
    let sigma: Vec<f64> = pseudo_random(12, 40);
    let input = HashMap::from([
        ("rho".to_string(), rho.as_slice()),
        ("sigma".to_string(), sigma.as_slice()),
    ]);
    let (buf1, layout1) = func.compute_xc(&input, 1).unwrap();
    let (buf2, layout2) = cloned.compute_xc(&input, 1).unwrap();
    for key in ["zk", "vrho", "vsigma"] {
        assert!(allclose(
            &buf1[layout1.get(key).unwrap()],
            &buf2[layout2.get(key).unwrap()],
            0.0,
            0.0
        ));
    }

    // the C object survives as long as one handle is alive
    drop(func);
    let (buf3, layout3) = cloned.compute_xc(&input, 1).unwrap();
    assert!(allclose(
        &buf1[layout1.get("zk").unwrap()],
        &buf3[layout3.get("zk").unwrap()],
        0.0,
        0.0
    ));
}

#[test]
fn clone_sees_parameters_set_before_cloning() {
    // a fresh FFI-level copy would reset these to defaults; a shared handle
    // must see the values set on the original
    let mut func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    let params = [0.1, 0.1, 0.2, 0.3, 0.2, 0.8, 0.5];
    func.set_ext_params(&params);

    let cloned = func.clone();
    // reading current values back requires xc_func_get_ext_params_value,
    // which libxc only exports from v7.0 on
    if libxc_version().0 >= 7 {
        assert_eq!(cloned.ext_param_values(), params);
        assert_ne!(cloned.ext_param_values(), cloned.ext_param_default_values());
    } else {
        assert!(matches!(
            cloned.ext_param_values_f().unwrap_err(),
            LibXCError::UnsupportedVersion { .. }
        ));
    }
}

#[test]
fn cloned_functional_supports_introspection() {
    let func = LibXCFunctional::from_identifier("hyb_gga_xc_b3lyp", Unpolarized);
    let cloned = func.clone();
    assert_eq!(cloned.identifier(), func.identifier());
    assert_eq!(cloned.number(), func.number());
    assert_eq!(cloned.family(), func.family());
    assert_eq!(cloned.spin(), func.spin());
    assert_eq!(cloned.hyb_exx_coef(), func.hyb_exx_coef());
    assert!(!cloned.references().is_empty());

    // describe() degrades to default ext-param values on libxc < 7.0
    assert!(!cloned.describe().is_empty());
    assert!(cloned.describe().contains("External Parameters"));
}

// ===========================================================================
// Setter exclusivity
// ===========================================================================

#[test]
fn setters_are_refused_while_shared() {
    let mut func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    let guard = func.clone();

    let params = [0.1, 0.1, 0.2, 0.3, 0.2, 0.8, 0.5];
    assert!(matches!(func.set_ext_params_f(&params), Err(LibXCError::SharedError)));
    assert!(matches!(func.set_ext_param_by_name_f("_a", 0.1), Err(LibXCError::SharedError)));
    assert!(matches!(func.set_dens_threshold_f(1e-12), Err(LibXCError::SharedError)));
    assert!(matches!(func.set_zeta_threshold_f(1e-12), Err(LibXCError::SharedError)));
    assert!(matches!(func.set_sigma_threshold_f(1e-12), Err(LibXCError::SharedError)));
    assert!(matches!(func.set_tau_threshold_f(1e-12), Err(LibXCError::SharedError)));
    #[cfg(feature = "api-v7_0")]
    assert!(matches!(func.set_fhc_enforcement_f(true), Err(LibXCError::SharedError)));

    let mut hyb = LibXCFunctional::from_identifier("hyb_gga_xc_b3lyp", Unpolarized);
    let _g = hyb.clone();
    assert!(matches!(hyb.set_hyb_exx_coef_f(0.5), Err(LibXCError::SharedError)));

    let mut cam = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x", Unpolarized);
    let _g = cam.clone();
    assert!(matches!(cam.set_cam_coef_f(0.19, 0.46, 0.33), Err(LibXCError::SharedError)));

    let mut vv10 = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x_v", Unpolarized);
    let _g = vv10.clone();
    assert!(matches!(vv10.set_vv10_coef_f(6.0, 0.01), Err(LibXCError::SharedError)));

    drop(guard);
}

#[test]
#[should_panic(expected = "SharedError")]
fn non_f_setter_panics_while_shared() {
    let mut func = LibXCFunctional::from_identifier("gga_c_pbe", Unpolarized);
    let _guard = func.clone();
    func.set_dens_threshold(1e-12);
}

#[test]
fn setters_work_again_after_clones_dropped() {
    let mut func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    {
        let c1 = func.clone();
        let c2 = func.clone();
        assert!(func.set_ext_params_f(&[0.1; 7]).is_err());
        drop(c1);
        // one clone still alive
        assert!(func.set_ext_params_f(&[0.1; 7]).is_err());
        drop(c2);
    }

    // unique again: mutation is allowed
    let params = [0.1, 0.1, 0.2, 0.3, 0.2, 0.8, 0.5];
    func.set_ext_params(&params);
    if libxc_version().0 >= 7 {
        assert_eq!(func.ext_param_values(), params);
    }
    // by-name and by-map setters work on all versions
    func.set_ext_param_by_name("_a", 0.3);
    func.set_ext_param_map([("_b", 0.4)].into_iter());
    func.set_dens_threshold(1e-13);
    assert_eq!(func.dens_threshold(), 1e-13);

    // functional is still usable for computation afterwards
    let rho: Vec<f64> = pseudo_random(21, 8);
    let sigma: Vec<f64> = pseudo_random(22, 8);
    let input = HashMap::from([
        ("rho".to_string(), rho.as_slice()),
        ("sigma".to_string(), sigma.as_slice()),
    ]);
    let (_, layout) = func.compute_xc(&input, 0).unwrap();
    assert_eq!(layout.get("zk").unwrap().len(), 8);
}

// ===========================================================================
// Parallel evaluation
// ===========================================================================

/// Evaluate a functional in parallel chunks (sharing `&func` between rayon
/// workers, no locks) and compare with the serial result over the full grid.
fn run_parallel_compute_test(identifier: &str, spin: LibXCSpin, inputs: &[&str], seed: u64) {
    const NPOINT: usize = 2048;
    const CHUNK: usize = 64;

    let func = LibXCFunctional::from_identifier(identifier, spin);
    let data: Vec<(&str, Vec<f64>)> = inputs
        .iter()
        .map(|&name| (name, pseudo_random(seed, NPOINT * input_dim(name, spin))))
        .collect();

    // serial reference over the full grid
    let input_full: HashMap<String, &[f64]> =
        data.iter().map(|(n, v)| (n.to_string(), v.as_slice())).collect();
    let (ser_buf, ser_layout) = func.compute_xc(&input_full, 1).unwrap();
    let zk_ser = &ser_buf[ser_layout.get("zk").unwrap()];

    // pattern 1: borrow &func inside parallel jobs (requires Sync)
    let n_chunks = NPOINT / CHUNK;
    let zk_par_shared: Vec<f64> = (0..n_chunks)
        .into_par_iter()
        .flat_map(|c| {
            let input: HashMap<String, &[f64]> = data
                .iter()
                .map(|(n, v)| {
                    let d = input_dim(n, spin);
                    (n.to_string(), &v[c * CHUNK * d..(c + 1) * CHUNK * d])
                })
                .collect();
            let (buf, layout) = func.compute_xc(&input, 1).unwrap();
            buf[layout.get("zk").unwrap()].to_vec()
        })
        .collect();

    // pattern 2: clone the handle into each job (requires Send); all clones
    // still reference the same C functional
    let shared = func.clone();
    let zk_par_cloned: Vec<f64> = (0..n_chunks)
        .into_par_iter()
        .flat_map(|c| {
            let func = shared.clone();
            let input: HashMap<String, &[f64]> = data
                .iter()
                .map(|(n, v)| {
                    let d = input_dim(n, spin);
                    (n.to_string(), &v[c * CHUNK * d..(c + 1) * CHUNK * d])
                })
                .collect();
            let (buf, layout) = func.compute_xc(&input, 1).unwrap();
            buf[layout.get("zk").unwrap()].to_vec()
        })
        .collect();

    assert_eq!(zk_par_shared.len(), zk_ser.len());
    assert_eq!(zk_par_cloned.len(), zk_ser.len());
    assert!(allclose(&zk_par_shared, zk_ser, 1e-7, 1e-14));
    assert!(allclose(&zk_par_cloned, zk_ser, 1e-7, 1e-14));
}

#[test]
fn parallel_compute_lda() {
    run_parallel_compute_test("lda_x", Unpolarized, &["rho"], 101);
    run_parallel_compute_test("lda_x", Polarized, &["rho"], 102);
}

#[test]
fn parallel_compute_gga() {
    run_parallel_compute_test("gga_c_pbe", Unpolarized, &["rho", "sigma"], 103);
    run_parallel_compute_test("gga_c_pbe", Polarized, &["rho", "sigma"], 104);
}

#[test]
fn parallel_compute_hybrid_gga() {
    // hybrid with auxiliary functionals: exercises the mix path concurrently
    run_parallel_compute_test("hyb_gga_xc_b3lyp", Unpolarized, &["rho", "sigma"], 105);
    run_parallel_compute_test("hyb_gga_xc_b3lyp", Polarized, &["rho", "sigma"], 106);
}

#[test]
fn parallel_compute_mgga() {
    run_parallel_compute_test("mgga_c_tpss", Unpolarized, &["rho", "sigma", "tau"], 107);
    run_parallel_compute_test("mgga_c_tpss", Polarized, &["rho", "sigma", "tau"], 108);
}

#[test]
fn clone_can_be_sent_to_os_threads() {
    let func = LibXCFunctional::from_identifier("lda_x", Unpolarized);

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let func = func.clone();
            std::thread::spawn(move || {
                let rho: Vec<f64> = pseudo_random(200, 32);
                let input = HashMap::from([("rho".to_string(), rho.as_slice())]);
                let (buf, layout) = func.compute_xc(&input, 0).unwrap();
                buf[layout.get("zk").unwrap()].to_vec()
            })
        })
        .collect();

    let results: Vec<Vec<f64>> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for r in &results[1..] {
        assert!(allclose(&results[0], r, 0.0, 0.0));
    }

    // original handle still valid after worker threads dropped their clones
    let rho: Vec<f64> = pseudo_random(200, 32);
    let input = HashMap::from([("rho".to_string(), rho.as_slice())]);
    let (buf, layout) = func.compute_xc(&input, 0).unwrap();
    assert!(allclose(&results[0], &buf[layout.get("zk").unwrap()], 0.0, 0.0));
}
