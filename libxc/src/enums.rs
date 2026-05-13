//! Wrappers for libxc enums and bitflags.

use libxc_ffi::ffi;

/// Spin polarization of the functional.
///
/// The numeric values have actual meaning:
/// - Unpolarized: 1 (input density is only `rho` with grids size);
/// - Polarized: 2 (input density is `rho_alpha` and `rho_beta`, double of grids
///   size).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCSpin {
    Unpolarized = ffi::XC_UNPOLARIZED,
    Polarized = ffi::XC_POLARIZED,
}

/// Whether the functional is relativistic or non-relativistic.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCRelavistic {
    NonRelativistic = ffi::XC_NON_RELATIVISTIC,
    Relativistic = ffi::XC_RELATIVISTIC,
}

/// The kind of the functional: exchange, correlation, exchange-correlation, or
/// kinetic.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCFunctionalKind {
    Exchange = ffi::XC_EXCHANGE,
    Correlation = ffi::XC_CORRELATION,
    ExchangeCorrelation = ffi::XC_EXCHANGE_CORRELATION,
    Kinetic = ffi::XC_KINETIC,
}

/// The family of the functional: LDA, GGA, MGGA and its hybrid variants.
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

/// Whether the functional can be evaluated on device (CUDA) or host (CPU).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u32)]
pub enum LibXCDeviceFlag {
    OnDevice = ffi::XC_FLAGS_ON_DEVICE,
    OnHost = ffi::XC_FLAGS_ON_HOST,
}

/// Bitflags for functional properties.
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

/// The functional identifier type, which is alias of FFI enum `XcFuncId`.
///
/// Note that different versions of libxc may have different sets of
/// functionals. Please check your libxc version, and the cargo feature you
/// enabled to make sure the functional you want is available.
pub use libxc_ffi::xc_funcs::XcFuncId as LibXCFuncId;

lazy_static::lazy_static! {
    /// Full list of functional identifiers (lazily generated).
    pub static ref LIBXC_FUNC_ENUM: Vec<LibXCFuncId> = {
        use strum::IntoEnumIterator;
        LibXCFuncId::iter().collect()
    };

    /// Full list of functional identifiers strings (lazily generated).
    pub static ref LIBXC_FUNC_STR: Vec<String> = {
        use strum::IntoEnumIterator;
        LibXCFuncId::iter().map(|func_id| format!("{:?}", func_id)).collect()
    };

    /// Mapping from functional identifier string to number id (lazily generated).
    pub static ref LIBXC_FUNC_MAP: indexmap::IndexMap<String, u32> = {
        use strum::IntoEnumIterator;
        let mut map = indexmap::IndexMap::new();
        for func_id in LibXCFuncId::iter() {
            let name = format!("{:?}", func_id);
            let id = func_id as u32;
            map.insert(name, id);
        }
        map
    };
}

/// Re-export all enum items for easy access.
pub mod libxc_enum_items {
    use super::*;

    pub use LibXCFamily::*;
    pub use LibXCFlags::*;
    pub use LibXCFuncId::*;
    pub use LibXCFunctionalKind::*;
    pub use LibXCRelavistic::*;
    pub use LibXCSpin::*;
}
