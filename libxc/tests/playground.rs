#[test]
fn playground() {
    use libxc::prelude::{libxc_enum_items::*, *};
    let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    println!("{:?}", xc_func.dens_threshold());
    let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    println!("{:?}", xc_func.dens_threshold());
}
