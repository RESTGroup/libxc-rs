use libxc::prelude::{libxc_enum_items::*, *};
use rayon::prelude::*;
use std::collections::HashMap;

use crate::example_densities;

lazy_static::lazy_static! {
    static ref REF: Vec<((String, String, String), HashMap<String, Vec<f64>>)> = {
        // CARGO_MANIFEST_DIR/tests/regression/reference.toml
        // example toml content:
        // [gga_c.acgga.Li]
        // zk = [-6.155181882468815e-02, -4.534982135558897e-02]
        // vrho = [-1.150256867650166e-01, -1.148983776346154e-01]
        // vsigma = [4.344359952306926e-05, 8.688719904613852e-05]
        // (category, xc_name, species) -> {zk, vrho, vsigma}
        let mut m = Vec::new();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/regression/reference.toml");
        let content = std::fs::read_to_string(path).expect("Failed to read reference.toml");
        let data: HashMap<String, HashMap<String, HashMap<String, HashMap<String, Vec<f64>>>>> = toml::from_str(&content).expect("Failed to parse reference.toml");
        for (category, xc_map) in data {
            for (xc_name, species_map) in xc_map {
                for (species, values) in species_map {
                    m.push(((category.clone(), xc_name.clone(), species.clone()), values));
                }
            }
        }
        m
    };
}

/// Compare two slices with relative and absolute tolerance (mirrors
/// np.allclose).
fn allclose(a: &[f64], b: &[f64], rtol: f64, atol: f64) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(&x, &y)| (x - y).abs() <= atol + rtol * y.abs())
}

fn test_regression_entry(
    category: &str,
    xc_name: &str,
    species: &str,
    reference: &HashMap<String, Vec<f64>>,
) {
    let spin = if species.contains("restr") { Unpolarized } else { Polarized };
    let input = example_densities::test_data(species, spin);
    let input_ref = input.iter().map(|(k, v)| (k.clone(), v.as_slice())).collect();
    let xc_identifier = category.to_owned() + "_" + xc_name;
    let xc = LibXCFunctional::from_identifier(&xc_identifier, spin);
    // we always test exc and vxc, but not higher derivatives
    let (out_buffer, out_layout) = xc.compute_xc(&input_ref, 1).unwrap();
    for (key, ref_values) in reference {
        let ref_out = &out_buffer[out_layout.get(key).unwrap()];
        let (rtol, atol) = if key == "zk" { (5e-8, 1e-10) } else { (5e-5, 1e-7) };
        assert!(allclose(ref_out, ref_values, rtol, atol));
    }
}

#[test]
fn test_regression() {
    REF.par_iter().for_each(|((category, xc_name, species), reference)| {
        test_regression_entry(category, xc_name, species, reference);
    });
}
