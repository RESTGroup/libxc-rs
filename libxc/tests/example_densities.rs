//! Example density data for Li atom, translated from
//! pylibxc/example_densities.py.

use itertools::izip;
use libxc::prelude::*;
use std::collections::HashMap;

const LI_DATA: [[f64; 9]; 7] = [
    [
        6.4861166230150555e+00,
        6.5036649228387189e+00,
        1.4194047833239990e+03,
        1.4232293901824482e+03,
        1.4270643024997728e+03,
        -1.0610898372216599e+04,
        -1.0639664359639688e+04,
        2.7368479581946655e+01,
        2.7428079400825251e+01,
    ],
    [
        2.1707638135485769e+00,
        2.1763114221928235e+00,
        1.5451531107452470e+02,
        1.5495115738563598e+02,
        1.5538823310246494e+02,
        -5.2123140153080456e+01,
        -5.2318439378538102e+01,
        8.9092835174206417e+00,
        8.9249768819286928e+00,
    ],
    [
        3.0095776303929744e-02,
        3.0057395981539382e-02,
        2.3266883924948207e-02,
        2.3271876430827901e-02,
        2.3276870007977606e-02,
        4.9086025614566042e-01,
        4.8057597402684793e-01,
        9.9398815379935043e-02,
        9.6801757312633019e-02,
    ],
    [
        4.7984046357356829e-03,
        1.5964624289378327e-06,
        3.6980454953559325e-06,
        1.5182461958770077e-08,
        6.2332156653825732e-11,
        -3.4357867977894050e-03,
        3.4775215518530016e-05,
        1.0119865465024043e-04,
        4.8804904138848394e-06,
    ],
    [
        2.9893451559105509e-04,
        5.0929426813304125e-11,
        5.1228755095525470e-08,
        3.5867753338704198e-14,
        2.5112765812232162e-20,
        7.5203211551453606e-05,
        2.2491041491179418e-10,
        2.1421399347676239e-05,
        6.1636189666388111e-11,
    ],
    [
        1.8563966531289203e-06,
        1.8161853710439767e-06,
        8.3595323953118973e-11,
        8.3199268725174546e-11,
        8.2805089914907147e-11,
        1.1179467690003199e-03,
        3.9256032612949076e-05,
        2.7526678760974991e-04,
        5.6991078127697100e-06,
    ],
    [
        1.5177006792460753e-11,
        5.4528995709775956e-12,
        2.6138182385288591e-21,
        8.5857220241656684e-22,
        2.8201893149897212e-22,
        1.0874855750589578e-05,
        1.9948650998117136e-11,
        2.7187178133630866e-06,
        6.4648860561879137e-12,
    ],
];

pub fn test_input(data: &[[f64; 9]], spin: LibXCSpin) -> HashMap<&str, Vec<f64>> {
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
            let rho = izip!(data[RHO_A], data[RHO_B]).map(|(a, b)| a + b).collect();
            let sigma = izip!(data[SIGMA_AA], data[SIGMA_AB], data[SIGMA_BB])
                .map(|(aa, ab, bb)| aa + 2.0 * ab + bb)
                .collect();
            let lapl = izip!(data[LAPL_A], data[LAPL_B]).map(|(a, b)| a + b).collect();
            let tau = izip!(data[TAU_A], data[TAU_B]).map(|(a, b)| a + b).collect();
            result.insert("rho", rho);
            result.insert("sigma", sigma);
            result.insert("lapl", lapl);
            result.insert("tau", tau);
            result
        },
        LibXCSpin::Polarized => {
            // [data[RHO_A][0], data[RHO_B][0], data[RHO_A][1], data[RHO_B][1], ...]
            let rho = izip!(data[RHO_A], data[RHO_B]).flat_map(|(a, b)| [a, b]).collect();
            let sigma = izip!(data[SIGMA_AA], data[SIGMA_AB], data[SIGMA_BB])
                .flat_map(|(aa, ab, bb)| [aa, ab, bb])
                .collect();
            let lapl = izip!(data[LAPL_A], data[LAPL_B]).flat_map(|(a, b)| [a, b]).collect();
            let tau = izip!(data[TAU_A], data[TAU_B]).flat_map(|(a, b)| [a, b]).collect();
            result.insert("rho", rho);
            result.insert("sigma", sigma);
            result.insert("lapl", lapl);
            result.insert("tau", tau);
            result
        },
    }
}

pub fn test_data(species: &str, spin: LibXCSpin) -> HashMap<&str, Vec<f64>> {
    match species {
        "Li" => test_input(&LI_DATA, spin),
        _ => panic!("Unknown species: {species}"),
    }
}
