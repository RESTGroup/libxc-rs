use libxc_ffi::ffi;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCSpin {
    Polarized = ffi::XC_POLARIZED,
    Unpolarized = ffi::XC_UNPOLARIZED,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCRelavistic {
    NonRelativistic = ffi::XC_NON_RELATIVISTIC,
    Relativistic = ffi::XC_RELATIVISTIC,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCFunctionalKind {
    Exchange = ffi::XC_EXCHANGE,
    Correlation = ffi::XC_CORRELATION,
    ExchangeCorrelation = ffi::XC_EXCHANGE_CORRELATION,
    Kinetic = ffi::XC_KINETIC,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCFamily {
    LDA = ffi::XC_FAMILY_LDA,
    GGA = ffi::XC_FAMILY_GGA,
    MGGA = ffi::XC_FAMILY_MGGA,
    LCA = ffi::XC_FAMILY_LCA,
    OEP = ffi::XC_FAMILY_OEP,
    HybGGA = ffi::XC_FAMILY_HYB_GGA,
    HybMGGA = ffi::XC_FAMILY_HYB_MGGA,
    HybLDA = ffi::XC_FAMILY_HYB_LDA,
}

#[enumflags2::bitflags]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCFlags {
    HaveEXC = ffi::XC_FLAGS_HAVE_EXC,
    HaveVXC = ffi::XC_FLAGS_HAVE_VXC,
    HaveFXC = ffi::XC_FLAGS_HAVE_FXC,
    HaveKXC = ffi::XC_FLAGS_HAVE_KXC,
    HaveLXC = ffi::XC_FLAGS_HAVE_LXC,
    Dim1 = ffi::XC_FLAGS_1D,
    Dim2 = ffi::XC_FLAGS_2D,
    Dim3 = ffi::XC_FLAGS_3D,
    HybCAM = ffi::XC_FLAGS_HYB_CAM,
    HybCAMY = ffi::XC_FLAGS_HYB_CAMY,
    VV10 = ffi::XC_FLAGS_VV10,
    HybLC = ffi::XC_FLAGS_HYB_LC,
    HybLCY = ffi::XC_FLAGS_HYB_LCY,
    Stable = ffi::XC_FLAGS_STABLE,
    Development = ffi::XC_FLAGS_DEVELOPMENT,
    NeedsLaplacian = ffi::XC_FLAGS_NEEDS_LAPLACIAN,
    NeedsTau = ffi::XC_FLAGS_NEEDS_TAU,
    EnforceFHC = ffi::XC_FLAGS_ENFORCE_FHC,
    OnDevice = ffi::XC_FLAGS_ON_DEVICE,
    OnHost = ffi::XC_FLAGS_ON_HOST,
}
