//! Tests for utility functions (translated from pylibxc test_util.py).

use libxc::prelude::*;

// Ignored tests:
// - test_xc_string
// - test_xc_family_from_id: `xc_family_from_id` is not safe wrapped, and the
//   intention of this function is somehow strange? -- by ajz34

#[test]
fn test_xc_version() {
    let (major, minor, micro) = libxc_version();
    let version_str = libxc_version_string();
    let from_str: Vec<i32> = version_str.split('.').map(|s| s.parse().unwrap()).collect();
    assert_eq!(major, from_str[0]);
    assert_eq!(minor, from_str[1]);
    assert_eq!(micro, from_str[2]);
}

#[test]
fn test_xc_functional_get_number() {
    assert_eq!(libxc_functional_get_number("XC_GGA_X_GAM"), Some(32));
    assert_eq!(libxc_functional_get_number("gga_x_gam"), Some(32));
    assert_eq!(libxc_functional_get_number("GGA_X_GAM"), Some(32));

    assert_eq!(libxc_functional_get_number("nothing"), None);
}

#[test]
fn test_xc_functional_get_name() {
    assert_eq!(libxc_functional_get_name(32).as_deref(), Some("gga_x_gam"));
    assert_eq!(libxc_functional_get_name(50000), None);
}

#[test]
fn test_xc_number_of_functionals() {
    assert!(libxc_number_of_functionals() > 400);
}

#[test]
fn test_xc_available_functional_numbers() {
    let func_nums = libxc_available_functional_numbers();

    assert_eq!(func_nums.len(), libxc_number_of_functionals() as usize);

    // Spot check some functionals
    assert!(func_nums.contains(&1));
    assert!(func_nums.contains(&76));
    assert!(func_nums.contains(&540));

    // Make sure the range on all values is reasonable
    assert!(func_nums.iter().all(|&x| x > 0));
    assert!(func_nums.iter().all(|&x| x < 2000 || x > 100000));
}

#[test]
fn test_xc_available_functional_names() {
    let func_names = libxc_available_functional_names();

    assert_eq!(func_names.len(), libxc_number_of_functionals() as usize);

    // Spot check some functionals
    assert!(func_names.iter().any(|n| n == "lda_c_vwn"));
    assert!(func_names.iter().any(|n| n == "gga_c_pbe"));
    assert!(func_names.iter().any(|n| n == "gga_x_lbm"));

    // Make sure the range on all values is reasonable
    assert!(func_names.iter().all(|x| x.contains('x') || x.contains('c') || x.contains('k')));
}
