//! Example density data for Li atom, translated from
//! pylibxc/example_densities.py.

use cached::proc_macro::cached;
use libxc::prelude::*;
use std::collections::HashMap;

pub fn test_input(data: &[[f64; 9]], spin: LibXCSpin) -> HashMap<String, Vec<f64>> {
    const RHO_A: usize = 0;
    const RHO_B: usize = 1;
    const SIGMA_AA: usize = 2;
    const SIGMA_AB: usize = 3;
    const SIGMA_BB: usize = 4;
    const LAPL_A: usize = 5;
    const LAPL_B: usize = 6;
    const TAU_A: usize = 7;
    const TAU_B: usize = 8;

    let mut result = HashMap::new();
    match spin {
        LibXCSpin::Unpolarized => {
            let rho = data.iter().map(|r| r[RHO_A] + r[RHO_B]).collect();
            let sigma =
                data.iter().map(|r| r[SIGMA_AA] + 2.0 * r[SIGMA_AB] + r[SIGMA_BB]).collect();
            let lapl = data.iter().map(|r| r[LAPL_A] + r[LAPL_B]).collect();
            let tau = data.iter().map(|r| r[TAU_A] + r[TAU_B]).collect();
            result.insert("rho".to_string(), rho);
            result.insert("sigma".to_string(), sigma);
            result.insert("lapl".to_string(), lapl);
            result.insert("tau".to_string(), tau);
            result
        },
        LibXCSpin::Polarized => {
            // [data[RHO_A][0], data[RHO_B][0], data[RHO_A][1], data[RHO_B][1], ...]
            let rho = data.iter().flat_map(|r| [r[RHO_A], r[RHO_B]]).collect();
            let sigma = data.iter().flat_map(|r| [r[SIGMA_AA], r[SIGMA_AB], r[SIGMA_BB]]).collect();
            let lapl = data.iter().flat_map(|r| [r[LAPL_A], r[LAPL_B]]).collect();
            let tau = data.iter().flat_map(|r| [r[TAU_A], r[TAU_B]]).collect();
            result.insert("rho".to_string(), rho);
            result.insert("sigma".to_string(), sigma);
            result.insert("lapl".to_string(), lapl);
            result.insert("tau".to_string(), tau);
            result
        },
    }
}

#[cached]
pub fn test_data(species: String, spin: LibXCSpin) -> HashMap<String, Vec<f64>> {
    // parse data from example_densities.toml
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/regression/example_densities.toml");
    let content = std::fs::read_to_string(path).expect("Failed to read example_densities.toml");
    let data: HashMap<String, Vec<f64>> = toml::from_str(&content).unwrap();
    let data_folded: Vec<[f64; 9]> =
        data[&species].chunks(9).map(|chunk| chunk.try_into().unwrap()).collect();
    test_input(&data_folded, spin)
}
