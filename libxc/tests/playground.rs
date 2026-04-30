use libxc::enums::*;
use libxc::functional::*;

#[test]
fn playground() {
    let xc_func = LibXCFunctional::from_identifier("gga_c_pbe", LibXCSpin::Polarized).unwrap();
    println!("Functional: {xc_func:?}");
    // println!("reference: {:#?}", xc_func.references());
    println!("{:#?}", xc_func.ext_param_default_values());
}
