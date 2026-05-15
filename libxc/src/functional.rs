//! Wrapper for libxc functionals (non-computation part).
//!
//! Corresponds to original libxc's `pylibxc.functional`.
//!
//! Note the computational part is separately implemented in `compute_cpu.rs`
//! and `compute_cuda.rs`. Layout of output data structures is defined in
//! `layout_handling.rs`, which will be shared by both CPU and CUDA
//! implementations.

use crate::prelude::*;

unsafe fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

/// A literature reference for a libxc functional.
#[derive(Debug, Clone)]
pub struct LibXCReference {
    pub ref_text: String,
    pub doi: String,
    pub bibtex: String,
    pub key: String,
}

/// Safe, owning wrapper around a libxc `xc_func_type` functional.
///
/// `LibXCFunctional` is the central type of this crate. It represents a single
/// DFT exchange-correlation functional (e.g. LDA, GGA, MGGA, or a hybrid
/// variant) together with its spin polarization state, and exposes three
/// categories of operations:
///
/// # Construction
///
/// Create a functional by name or numeric ID, specifying spin:
/// - [`from_identifier`](Self::from_identifier) /
///   [`from_identifier_f`](Self::from_identifier_f) — from a string like
///   `"gga_c_pbe"`.
/// - [`from_number`](Self::from_number) /
///   [`from_number_f`](Self::from_number_f) — from a libxc functional ID (e.g.
///   `130`).
/// - The `_f` suffix marks fallible variants that return `Result`.
///
/// # Introspection
///
/// Query static properties of the functional:
/// - **Identity**: [`identifier`](Self::identifier), [`number`](Self::number),
///   [`info_name`](Self::info_name).
/// - **Classification**: [`family`](Self::family) (LDA/GGA/MGGA/hybrid),
///   [`kind`](Self::kind) (exchange/correlation/etc.), [`spin`](Self::spin).
/// - **Capabilities**: [`has_exc`](Self::has_exc), [`has_vxc`](Self::has_vxc),
///   [`has_fxc`](Self::has_fxc), [`has_kxc`](Self::has_kxc),
///   [`has_lxc`](Self::has_lxc), [`needs_laplacian`](Self::needs_laplacian),
///   [`needs_tau`](Self::needs_tau), [`flags`](Self::flags).
/// - **Hybrid / range-separated / VV10**: [`hyb_exx_coef`](Self::hyb_exx_coef),
///   [`cam_coef`](Self::cam_coef), [`vv10_coef`](Self::vv10_coef),
///   [`is_hyb_cam`](Self::is_hyb_cam).
/// - **Composition**: [`aux_funcs`](Self::aux_funcs) lists component
///   functionals and their weights (e.g. B3LYP = 0.08·LDA-X + 0.72·B88 + …).
/// - **References**: [`references`](Self::references) returns literature
///   citations; [`describe`](Self::describe) prints a human-readable summary.
///
/// # Parameter tuning
///
/// - **External parameters**: [`ext_param_values`](Self::ext_param_values),
///   [`set_ext_params`](Self::set_ext_params),
///   [`set_ext_param_map`](Self::set_ext_param_map) — modify functional
///   parameters (e.g. range-separation ω in ωB97X-V).
/// - **Hybrid / range-separated / VV10 setters** (not in pylibxc):
///   [`set_hyb_exx_coef`](Self::set_hyb_exx_coef),
///   [`set_cam_coef`](Self::set_cam_coef),
///   [`set_vv10_coef`](Self::set_vv10_coef) — override the default mixing
///   coefficients for hybrid functionals.
/// - **Numerical thresholds**:
///   [`set_dens_threshold`](Self::set_dens_threshold),
///   [`set_zeta_threshold`](Self::set_zeta_threshold),
///   [`set_sigma_threshold`](Self::set_sigma_threshold),
///   [`set_tau_threshold`](Self::set_tau_threshold).
///
/// # Computation
///
/// Evaluate the functional and its derivatives on a grid (CPU, or CUDA with
/// the `cuda` feature):
/// - [`compute_xc`] — automatic allocation, returns `(Vec<f64>, layout)`.
/// - [`compute_xc_with_unsliced_output`] — preallocated contiguous buffer.
/// - [`compute_xc_with_output`] — named per-component output buffers.
///
/// Input is a `HashMap<String, &\[f64\]>` with keys `"rho"`, `"sigma"`,
/// `"lapl"`, `"tau"` depending on the family. Output is accessed via
/// [`LibXCOutputLayout`] which maps component names (e.g. `"zk"`, `"vrho"`,
/// `"vsigma"`) to ranges in the buffer.
///
/// # Example
///
/// ```rust
/// use libxc::prelude::*;
/// use libxc_enum_items::*;
/// use std::collections::HashMap;
///
/// // Create a GGA correlation functional (unpolarized)
/// let func = LibXCFunctional::from_identifier("gga_c_pbe", Unpolarized);
/// assert_eq!(func.family(), LibXCFamily::GGA);
///
/// // Prepare input: 3 grid points
/// let rho: Vec<f64>   = vec![0.1, 0.2, 0.3];
/// let sigma: Vec<f64> = vec![0.01, 0.02, 0.03];
/// let mut input = HashMap::new();
/// input.insert("rho".into(), rho.as_slice());
/// input.insert("sigma".into(), sigma.as_slice());
///
/// // Compute energy (zk) and first derivative (vrho, vsigma)
/// let (buf, layout) = func.compute_xc(&input, 1).unwrap();
/// let zk = &buf[layout.get("zk").unwrap()];
/// let vrho = &buf[layout.get("vrho").unwrap()];
/// ```
///
/// [`compute_xc`]: Self::compute_xc
/// [`compute_xc_with_unsliced_output`]: Self::compute_xc_with_unsliced_output
/// [`compute_xc_with_output`]: Self::compute_xc_with_output
/// [`LibXCOutputLayout`]: crate::layout_handling::LibXCOutputLayout
pub struct LibXCFunctional {
    pub(crate) ptr: *mut ffi::xc_func_type,
}

/// Creation functions implementation.
impl LibXCFunctional {
    /// Create a new functional from a name string and spin configuration.
    ///
    /// The functional is initialized on the host (CPU). To create a functional
    /// on the GPU, use [`from_identifier_with_device`](Self::from_identifier_with_device)
    /// with [`LibXCDeviceFlag::OnDevice`] (requires the `cuda` feature).
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::*; // will import LibXCFunctional, LibXCSpin
    /// use libxc_enum_items::*; // will also import Unpolarized = LibXCSpin::Unpolarized
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `pylibxc.functional.LibXCFunctional.__init__` (without `func_flags`)
    pub fn from_identifier(name: &str, spin: LibXCSpin) -> Self {
        Self::from_identifier_f(name, spin).unwrap()
    }

    /// Create a new functional from a name string and spin configuration
    /// (fallible).
    pub fn from_identifier_f(name: &str, spin: LibXCSpin) -> Result<Self, LibXCError> {
        let func_id = crate::util::libxc_functional_get_number(name)
            .ok_or_else(|| LibXCError::NotFound(format!("functional '{name}'")))?;
        Self::from_number_f(func_id, spin)
    }

    /// Create a new functional from a functional ID and spin configuration.
    pub fn from_number(func_id: i32, spin: LibXCSpin) -> Self {
        Self::from_number_f(func_id, spin).unwrap()
    }

    /// Create a new functional from a functional ID and spin configuration
    /// (fallible).
    pub fn from_number_f(func_id: i32, spin: LibXCSpin) -> Result<Self, LibXCError> {
        unsafe {
            let ptr = ffi::xc_func_alloc();
            if ptr.is_null() {
                return Err(LibXCError::InitError { func_id, spin });
            }
            let rc = Self::init_func(ptr, func_id, spin);
            if rc != 0 {
                ffi::xc_func_free(ptr);
                return Err(LibXCError::InitError { func_id, spin });
            }
            Ok(Self { ptr })
        }
    }

    unsafe fn init_func(ptr: *mut ffi::xc_func_type, func_id: i32, spin: LibXCSpin) -> c_int {
        #[cfg(not(feature = "api-v7_1"))]
        {
            ffi::xc_func_init(ptr, func_id as c_int, spin as c_int)
        }
        #[cfg(feature = "api-v7_1")]
        {
            ffi::xc_func_init_flags(
                ptr,
                func_id as c_int,
                spin as c_int,
                LibXCDeviceFlag::OnHost as c_int,
            )
        }
    }
}

/// Creation functions for CUDA (device) functionals.
#[cfg(feature = "cuda")]
impl LibXCFunctional {
    /// Create a new functional on the specified device (GPU or CPU).
    ///
    /// For libxc >= 7.1, this uses `xc_func_init_flags` internally for
    /// per-functional CPU/GPU selection. For libxc 7.0 with CUDA support,
    /// this uses `xc_func_init` (the library must be compiled with
    /// `--enable-cuda`).
    pub fn from_identifier_with_device(
        name: &str,
        spin: LibXCSpin,
        device: LibXCDeviceFlag,
    ) -> Self {
        Self::from_identifier_with_device_f(name, spin, device).unwrap()
    }

    /// Create a new functional on the specified device (fallible).
    pub fn from_identifier_with_device_f(
        name: &str,
        spin: LibXCSpin,
        device: LibXCDeviceFlag,
    ) -> Result<Self, LibXCError> {
        let func_id = crate::util::libxc_functional_get_number(name)
            .ok_or_else(|| LibXCError::NotFound(format!("functional '{name}'")))?;
        Self::from_number_with_device_f(func_id, spin, device)
    }

    /// Create a new functional from ID on the specified device.
    pub fn from_number_with_device(func_id: i32, spin: LibXCSpin, device: LibXCDeviceFlag) -> Self {
        Self::from_number_with_device_f(func_id, spin, device).unwrap()
    }

    /// Create a new functional from ID on the specified device (fallible).
    pub fn from_number_with_device_f(
        func_id: i32,
        spin: LibXCSpin,
        device: LibXCDeviceFlag,
    ) -> Result<Self, LibXCError> {
        unsafe {
            let ptr = ffi::xc_func_alloc();
            if ptr.is_null() {
                return Err(LibXCError::InitError { func_id, spin });
            }
            let rc = {
                #[cfg(feature = "api-v7_1")]
                {
                    ffi::xc_func_init_flags(ptr, func_id as c_int, spin as c_int, device as c_int)
                }
                #[cfg(not(feature = "api-v7_1"))]
                {
                    // v7.0: only xc_func_init exists; device is a compile-time
                    // property of the library. xc_func_init works for both CPU
                    // and GPU libraries.
                    ffi::xc_func_init(ptr, func_id as c_int, spin as c_int)
                }
            };
            if rc != 0 {
                ffi::xc_func_free(ptr);
                return Err(LibXCError::InitError { func_id, spin });
            }
            (*(*ptr).info).flags |= device as i32;
            Ok(Self { ptr })
        }
    }

    /// Returns the device flag this functional was initialized with.
    pub fn device_flag(&self) -> Option<LibXCDeviceFlag> {
        let flags = self.flags();
        if flags.contains(LibXCFlags::OnDevice) {
            Some(LibXCDeviceFlag::OnDevice)
        } else if flags.contains(LibXCFlags::OnHost) {
            Some(LibXCDeviceFlag::OnHost)
        } else {
            None
        }
    }

    /// Returns true if this functional was initialized for GPU execution.
    pub fn is_on_device(&self) -> bool {
        self.flags().contains(LibXCFlags::OnDevice)
    }
}

/// Information of functional (non-settable).
impl LibXCFunctional {
    /// Returns a raw pointer to the underlying `xc_func_type`.
    ///
    /// Intended for advanced use; the caller must not free the pointer.
    pub fn as_ptr(&self) -> *const ffi::xc_func_type {
        self.ptr
    }

    /// Returns a raw pointer to the underlying `xc_func_type`.
    ///
    /// Intended for advanced use; the caller must not free the pointer.
    pub fn info(&self) -> *const ffi::xc_func_info_type {
        unsafe { ffi::xc_func_get_info(self.ptr) }
    }

    /// Functional number (ID).
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// assert_eq!(xc_func.number(), 136);
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_number()`
    pub fn number(&self) -> i32 {
        unsafe { ffi::xc_func_info_get_number(self.info()) as i32 }
    }

    /// Functional kind (exchange, correlation, etc.).
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// assert_eq!(xc_func.kind(), LibXCFunctionalKind::Correlation);
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_kind()`
    pub fn kind(&self) -> LibXCFunctionalKind {
        let k = unsafe { ffi::xc_func_info_get_kind(self.info()) } as u32;
        match k {
            ffi::XC_EXCHANGE => LibXCFunctionalKind::Exchange,
            ffi::XC_CORRELATION => LibXCFunctionalKind::Correlation,
            ffi::XC_EXCHANGE_CORRELATION => LibXCFunctionalKind::ExchangeCorrelation,
            ffi::XC_KINETIC => LibXCFunctionalKind::Kinetic,
            _ => panic!("Unknown functional kind code: {k}"),
        }
    }

    /// Functional standard name identifier.
    ///
    /// This is also what you would pass to `from_identifier` to create the same
    /// functional, and is the canonical name for this functional in libxc.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// assert_eq!(xc_func.identifier(), "gga_c_xpbe");
    /// ```
    pub fn identifier(&self) -> String {
        crate::util::libxc_functional_get_name(self.number()).unwrap_or_default()
    }

    /// Functional name for display purposes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// println!("{:?}", xc_func.info_name());
    /// // output: "Extended PBE by Xu & Goddard III"
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_name()`
    pub fn info_name(&self) -> String {
        unsafe { cstr_to_string(ffi::xc_func_info_get_name(self.info())) }
    }

    /// Functional family (LDA, GGA, MGGA, etc.).
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// assert_eq!(xc_func.family(), LibXCFamily::GGA);
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_family()`
    pub fn family(&self) -> LibXCFamily {
        let f = unsafe { ffi::xc_func_info_get_family(self.info()) } as u32;
        match f {
            ffi::XC_FAMILY_LDA => LibXCFamily::LDA,
            ffi::XC_FAMILY_GGA => LibXCFamily::GGA,
            ffi::XC_FAMILY_MGGA => LibXCFamily::MGGA,
            ffi::XC_FAMILY_LCA => LibXCFamily::LCA,
            ffi::XC_FAMILY_OEP => LibXCFamily::OEP,
            ffi::XC_FAMILY_HYB_GGA => LibXCFamily::HybGGA,
            ffi::XC_FAMILY_HYB_MGGA => LibXCFamily::HybMGGA,
            ffi::XC_FAMILY_HYB_LDA => LibXCFamily::HybLDA,
            _ => panic!("Unknown functional family code: {f}"),
        }
    }

    /// Functional flags as bitflags.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// println!("{:?}", xc_func.flags());
    /// // Example output
    /// // v7.0 BitFlags<LibXCFlags>(0b100000000010011111, HaveEXC | HaveVXC | HaveFXC | HaveKXC | HaveLXC | Dim3 | EnforceFHC)
    /// // v7.1 BitFlags<LibXCFlags>(0b1100000000010001111, HaveEXC | HaveVXC | HaveFXC | HaveKXC | Dim3 | EnforceFHC | OnDevice)
    /// ```
    ///
    /// # Notes
    ///
    /// Returned results will differ between libxc versions (v7.0 does not have
    /// `OnDevice` flag), build configurations (LXC may not available if not
    /// configured with `--enable-lxc`), and default device may not be on CPU
    /// (`OnDevice` or `OnHost`).
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_flags()`
    pub fn flags(&self) -> BitFlags<LibXCFlags> {
        let f = unsafe { ffi::xc_func_info_get_flags(self.info()) };
        BitFlags::from_bits(f as u32).unwrap_or_else(|_| BitFlags::empty())
    }

    /// Spin channels.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// assert_eq!(xc_func.spin(), LibXCSpin::Unpolarized);
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.xc_func.contents.nspin`
    pub fn spin(&self) -> LibXCSpin {
        match unsafe { (*self.ptr).nspin as u32 } {
            ffi::XC_UNPOLARIZED => LibXCSpin::Unpolarized,
            ffi::XC_POLARIZED => LibXCSpin::Polarized,
            n => panic!("Unknown spin code: {n}"),
        }
    }

    /// Reference to the dimension struct from libxc.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// let dim = xc_func.dim();
    /// // output: xc_dimensions { rho: 1, sigma: 1, lapl: 0, tau: 0, zk: 1, vrho: 1, ... }
    pub fn dim(&self) -> &ffi::xc_dimensions {
        unsafe { &(*self.ptr).dim }
    }

    /// Whether this functional has a specific flag.
    pub(crate) fn has_flag(&self, flag: LibXCFlags) -> bool {
        self.flags().contains(flag)
    }

    /// Whether this functional can compute the energy density (exc).
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._has_exc`
    pub fn has_exc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveEXC)
    }

    /// Whether this functional can compute the first derivative (vxc).
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._has_vxc`
    pub fn has_vxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveVXC)
    }

    /// Whether this functional can compute the second derivative (fxc).
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._has_fxc`
    pub fn has_fxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveFXC)
    }

    /// Whether this functional can compute the third derivative (kxc).
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._has_kxc`
    pub fn has_kxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveKXC)
    }

    /// Whether this functional can compute the fourth derivative (lxc).
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._has_lxc`
    pub fn has_lxc(&self) -> bool {
        self.has_flag(LibXCFlags::HaveLXC)
    }

    /// Whether this functional requires the laplacian.
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._needs_laplacian`
    pub fn needs_laplacian(&self) -> bool {
        self.has_flag(LibXCFlags::NeedsLaplacian)
    }

    /// Whether this functional requires the kinetic energy density.
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._needs_tau`
    pub fn needs_tau(&self) -> bool {
        self.has_flag(LibXCFlags::NeedsTau)
    }

    /// Whether this is a CAM range-separated hybrid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_cam_b3lyp", Unpolarized);
    /// assert!(xc_func.is_hyb_cam());
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_cam_b3lyp", Unpolarized);
    /// assert!(xc_func.is_hyb_cam());
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional._is_hyb_cam`
    pub fn is_hyb_cam(&self) -> bool {
        self.has_flag(LibXCFlags::HybCAM)
            || self.has_flag(LibXCFlags::HybCAMY)
            || self.has_flag(LibXCFlags::HybLC)
            || self.has_flag(LibXCFlags::HybLCY)
    }

    /// Returns the literature references for this functional.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// println!("{:#?}", xc_func.references());
    /// // Output:
    /// // [
    /// //     LibXCReference {
    /// //         ref_text: "W. Ai, W.-H. Fang, and N. Q. Su,  J. Phys. Chem. Lett. 12, 1207–1213 (2021)",
    /// //         doi: "10.1021/acs.jpclett.0c03621",
    /// //         bibtex: "@article{Ai2021_1207,\n  author = {Ai, Wenna and Fang, Wei-Hai and Su, Neil Qiang},\n  title = {The Role of Range-Separated Correlation in Long-Range Corrected Hybrid Functionals},\n  journal = {J. Phys. Chem. Lett.},\n  volume = {12},\n  pages = {1207--1213},\n  year = {2021},\n  doi = {10.1021/acs.jpclett.0c03621},\n  url = {https://doi.org/10.1021/acs.jpclett.0c03621}\n}\n",
    /// //         key: "Ai2021_1207",
    /// //     },
    /// // ]
    /// ```
    pub fn references(&self) -> Vec<LibXCReference> {
        let mut refs = Vec::new();
        for i in 0..(ffi::XC_MAX_REFERENCES as i32) {
            let ref_ptr = unsafe { ffi::xc_func_info_get_references(self.info(), i) };
            if ref_ptr.is_null() {
                break;
            }
            refs.push(LibXCReference {
                ref_text: unsafe { cstr_to_string(ffi::xc_func_reference_get_ref(ref_ptr)) },
                doi: unsafe { cstr_to_string(ffi::xc_func_reference_get_doi(ref_ptr)) },
                bibtex: unsafe { cstr_to_string(ffi::xc_func_reference_get_bibtex(ref_ptr)) },
                key: unsafe { cstr_to_string(ffi::xc_func_reference_get_key(ref_ptr)) },
            });
        }
        refs
    }

    /// Returns a multi-line description of this functional.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// println!("{:}", xc_func.describe());
    /// // Output:
    /// // ID Number      : 624
    /// // Identifier     : gga_c_lypr
    /// // Description    : Short-range LYP by Ai, Fang, and Su
    /// // Attributes
    /// //     Kind       : Correlation
    /// //     Family     : GGA
    /// //     Spin       : Unpolarized
    /// // Flags
    /// //     Derivative : HaveEXC | HaveVXC | HaveFXC | HaveKXC
    /// //     Dimension  : Dim3
    /// //     CAM        : <empty>
    /// //     VV10       : <empty>
    /// //     MGGA       : EnforceFHC
    /// //     Device     : OnDevice
    /// // References
    /// //     - W. Ai, W.-H. Fang, and N. Q. Su,  J. Phys. Chem. Lett. 12, 1207–1213 (2021)
    /// //       DOI: 10.1021/acs.jpclett.0c03621
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.describe()`
    pub fn describe(&self) -> String {
        use crate::enums::libxc_enum_items::*;

        let flag_all_deriv = HaveEXC | HaveVXC | HaveFXC | HaveKXC | HaveLXC;
        let flag_all_dim = Dim1 | Dim2 | Dim3;
        let flag_all_cam = HybCAM | HybCAMY | HybLC | HybLCY;
        let flag_vv10 = VV10;
        let flag_all_mgga = NeedsLaplacian | NeedsTau | EnforceFHC;
        let flag_all_device = OnHost | OnDevice;

        let references = self.references();

        let mut lst = vec![
            format!("ID Number       : {}", self.number()),
            format!("Identifier      : {}", self.identifier()),
            format!("Description     : {}", self.info_name()),
            "Attributes".to_string(),
            format!("    Kind        : {:?}", self.kind()),
            format!("    Family      : {:?}", self.family()),
            format!("    Spin        : {:?}", self.spin()),
            "Flags".to_string(),
            format!("    Derivative  : {}", self.flags() & flag_all_deriv),
            format!("    Dimension   : {}", self.flags() & flag_all_dim),
            format!("    CAM         : {}", self.flags() & flag_all_cam),
            format!("    VV10        : {}", self.flags() & flag_vv10),
            format!("    MGGA        : {}", self.flags() & flag_all_mgga),
            format!("    Device      : {}", self.flags() & flag_all_device),
        ];
        // generate a duplicate functional object for obtaining default values
        let default_func = Self::from_number_f(self.number(), self.spin()).unwrap();
        // hyb_exx_coef
        if let Some(hyb_exx_coef) = self.hyb_exx_coef() {
            let default_hyb_exx_coef = default_func.hyb_exx_coef().unwrap();
            lst.push("Hybrid Functional (non-CAM-type)".to_string());
            if hyb_exx_coef == default_hyb_exx_coef {
                lst.push(format!("    hyb_exx_coef: {hyb_exx_coef}"));
            } else {
                lst.push(format!(
                    "    hyb_exx_coef: {hyb_exx_coef} (default: {default_hyb_exx_coef})"
                ));
            }
        }
        // cam_coef
        if let Some((cam_alpha, cam_beta, cam_omega)) = self.cam_coef() {
            let (def_alpha, def_beta, def_omega) = default_func.cam_coef().unwrap();
            lst.push("Hybrid Functional (CAM-type)".to_string());
            if (cam_alpha, cam_beta, cam_omega) == (def_alpha, def_beta, def_omega) {
                lst.push(format!("    cam_alpha   : {cam_alpha}"));
                lst.push(format!("    cam_beta    : {cam_beta}"));
                lst.push(format!("    cam_omega   : {cam_omega}"));
            } else {
                lst.push(format!("    cam_alpha   : {cam_alpha} (default: {def_alpha})"));
                lst.push(format!("    cam_beta    : {cam_beta} (default: {def_beta})"));
                lst.push(format!("    cam_omega   : {cam_omega} (default: {def_omega})"));
            }
        }
        // vv10
        if let Some((nlc_b, nlc_c)) = self.vv10_coef() {
            let (def_nlc_b, def_nlc_c) = default_func.vv10_coef().unwrap();
            lst.push("VV10 Functional".to_string());
            if (nlc_b, nlc_c) == (def_nlc_b, def_nlc_c) {
                lst.push(format!("    nlc_b       : {nlc_b}"));
                lst.push(format!("    nlc_C       : {nlc_c}"));
            } else {
                lst.push(format!("    nlc_b       : {nlc_b} (default: {def_nlc_b})"));
                lst.push(format!("    nlc_C       : {nlc_c} (default: {def_nlc_c})"));
            }
        }
        // ext_params
        let trim_float =
            |s: f64| format!("{:20.15}", s).trim_end_matches('0').trim_end_matches('.').to_string();
        let ext_param_number = self.n_ext_params();
        if ext_param_number > 0 {
            let ext_param_names = self.ext_param_names();
            let ext_param_values = self.ext_param_values();
            let ext_param_descriptions = self.ext_param_descriptions();
            let ext_param_default = self.ext_param_default_values();
            lst.push("External Parameters".to_string());
            for i in 0..ext_param_number as usize {
                if ext_param_values[i] == ext_param_default[i] {
                    lst.push(format!(
                        "    - {:>9} = {:<20} {}",
                        ext_param_names[i],
                        trim_float(ext_param_values[i]),
                        ext_param_descriptions[i],
                    ));
                } else {
                    lst.push(format!(
                        "    - {:>9} = {:<20} {:<50} (default: {:<20})",
                        ext_param_names[i],
                        trim_float(ext_param_values[i]),
                        ext_param_descriptions[i],
                        trim_float(ext_param_default[i])
                    ))
                };
            }
        }
        // references
        if !references.is_empty() {
            lst.push("References".to_string());
            for r in references {
                lst.extend([format!("    - {}", r.ref_text), format!("      DOI: {}", r.doi)]);
            }
        }
        lst.join("\n")
    }
}

/// External parameters getter and setter.
impl LibXCFunctional {
    /// Number of external parameters for this functional.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// assert_eq!(xc_func.n_ext_params(), 7);
    /// ```
    pub fn n_ext_params(&self) -> i32 {
        unsafe { ffi::xc_func_info_get_n_ext_params(self.info()) as i32 }
    }

    /// Names of the external parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// println!("{:?}", xc_func.ext_param_names());
    /// // Output: ["_a", "_b", "_c", "_d", "_m1", "_m2", "_omega"]
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_ext_param_names()`
    pub fn ext_param_names(&self) -> Vec<String> {
        let n = self.n_ext_params();
        (0..n)
            .map(|i| unsafe {
                cstr_to_string(ffi::xc_func_info_get_ext_params_name(self.info(), i as c_int))
            })
            .collect()
    }

    /// Descriptions of the external parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// println!("{:?}", xc_func.ext_param_descriptions());
    /// // Output: ["Parameter a", "Parameter b", "Parameter c", "Parameter d", "Parameter m1", "Parameter m2", "Range-separation parameter"]
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_ext_param_descriptions()`
    pub fn ext_param_descriptions(&self) -> Vec<String> {
        let n = self.n_ext_params();
        (0..n)
            .map(|i| unsafe {
                cstr_to_string(ffi::xc_func_info_get_ext_params_description(
                    self.info(),
                    i as c_int,
                ))
            })
            .collect()
    }

    /// Default values of the external parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// println!("{:7.4?}", xc_func.ext_param_default_values());
    /// // Output: [ 0.0492,  0.1320,  0.2533,  0.3490,  0.1528,  0.8734,  0.3300]
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_ext_param_default_values()`
    pub fn ext_param_default_values(&self) -> Vec<f64> {
        let n = self.n_ext_params();
        (0..n)
            .map(|i| unsafe {
                ffi::xc_func_info_get_ext_params_default_value(self.info(), i as c_int)
            })
            .collect()
    }

    /// Current values of the external parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let mut xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// xc_func.set_ext_params(&[0.1, 0.1, 0.2, 0.3, 0.2, 0.8, 0.5]);
    /// assert_eq!(xc_func.ext_param_values(), &[0.1, 0.1, 0.2, 0.3, 0.2, 0.8, 0.5]);
    pub fn ext_param_values(&self) -> Vec<f64> {
        let n = self.n_ext_params();
        (0..n).map(|i| unsafe { ffi::xc_func_get_ext_params_value(self.ptr, i as c_int) }).collect()
    }

    /// Returns a map of external parameter names to their (default value,
    /// description).
    ///
    /// # Note
    ///
    /// This function returns **default** values. These values will not be
    /// changed, even user sets custom parameters. See
    /// [`LibXCFunctional::ext_param_values`] for current values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// for (key, val) in xc_func.ext_param_default_map() {
    ///     println!("{key:>10}: {val:?}");
    /// }
    /// // Output:
    /// //     _a: (0.04918, "Parameter a")
    /// //     _b: (0.132, "Parameter b")
    /// //     _c: (0.2533, "Parameter c")
    /// //     _d: (0.349, "Parameter d")
    /// //    _m1: (0.15283842794759825, "Parameter m1")
    /// //    _m2: (0.8733624454148472, "Parameter m2")
    /// // _omega: (0.33, "Range-separation parameter")
    /// ```
    pub fn ext_param_default_map(&self) -> IndexMap<String, (f64, String)> {
        let names = self.ext_param_names();
        let descriptions = self.ext_param_descriptions();
        let default_values = self.ext_param_default_values();
        let mut map = IndexMap::new();
        for i in 0..names.len() {
            map.insert(names[i].clone(), (default_values[i], descriptions[i].clone()));
        }
        map
    }

    /// Returns a map of external parameter names to their current values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let mut xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// xc_func.set_ext_params(&[0.1, 0.1, 0.2, 0.3, 0.2, 0.8, 0.5]);
    /// for (key, val) in xc_func.ext_param_map() {
    ///     println!("{key:>10}: {val}");
    /// }
    /// // Output:
    /// //     _a: 0.1
    /// //     _b: 0.1
    /// //     _c: 0.2
    /// //     _d: 0.3
    /// //    _m1: 0.2
    /// //    _m2: 0.8
    /// // _omega: 0.5
    /// ```
    pub fn ext_param_map(&self) -> IndexMap<String, f64> {
        let names = self.ext_param_names();
        let values = self.ext_param_values();
        let mut map = IndexMap::new();
        for i in 0..names.len() {
            map.insert(names[i].clone(), values[i]);
        }
        map
    }

    /// Set all external parameters at once.
    ///
    /// # Panics
    ///
    /// Panics if the length of `params` does not match the number of external
    /// parameters expected by this functional.
    pub fn set_ext_params(&mut self, params: &[f64]) {
        self.set_ext_params_f(params).unwrap()
    }

    /// Set all external parameters at once (fallible).
    pub fn set_ext_params_f(&mut self, params: &[f64]) -> Result<(), LibXCError> {
        let n = self.n_ext_params() as usize;
        if params.len() != n {
            return Err(LibXCError::ParamSetError {
                param_name: "ext_params".to_string(),
                details: format!("expected {n} parameters, got {}", params.len()),
            });
        }
        unsafe {
            ffi::xc_func_set_ext_params(self.ptr, params.as_ptr());
        }
        Ok(())
    }

    /// Set external parameters using a map of parameter names to values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// use std::collections::HashMap; // other map types like BTreeMap, IndexMap also work
    /// let mut xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// let update_ext_param_map = HashMap::from([("_a", 0.1), ("_d", 0.2), ("_omega", 0.58)]);
    /// xc_func.set_ext_param_map(update_ext_param_map.iter());
    /// for (key, val) in xc_func.ext_param_map() {
    ///     println!("{key:>10}: {val}");
    /// }
    /// // Output:
    /// //     _a: 0.1
    /// //     _b: 0.132
    /// //     _c: 0.2533
    /// //     _d: 0.2
    /// //    _m1: 0.15283842794759825
    /// //    _m2: 0.8733624454148472
    /// // _omega: 0.58
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if any parameter name in `param_map` does not match the external
    /// parameters of this functional.
    pub fn set_ext_param_map(
        &mut self,
        param_map: impl Iterator<Item = (impl AsRef<str>, impl Borrow<f64>)>,
    ) {
        self.set_ext_param_map_f(param_map).unwrap()
    }

    /// Set external parameters using a map of parameter names to values
    /// (fallible).
    pub fn set_ext_param_map_f(
        &mut self,
        param_map: impl Iterator<Item = (impl AsRef<str>, impl Borrow<f64>)>,
    ) -> Result<(), LibXCError> {
        let mut map = self.ext_param_map();
        for (key, val) in param_map.into_iter() {
            let (key, val) = (key.as_ref(), *val.borrow());
            if !map.contains_key(key) {
                return Err(LibXCError::ParamSetError {
                    param_name: key.to_string(),
                    details: "external parameter not found".to_string(),
                });
            }
            map.insert(key.to_string(), val);
        }
        let params: Vec<f64> = map.values().cloned().collect();
        self.set_ext_params_f(&params)?;
        Ok(())
    }
}

/// Setters for thresholds.
impl LibXCFunctional {
    /// Density threshold for numerical stability (usually smaller than 1e-10).
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_lypr", Unpolarized);
    /// println!("{:?}", xc_func.dens_threshold()); // 1e-14
    /// let xc_func = LibXCFunctional::from_identifier("gga_c_xpbe", Unpolarized);
    /// println!("{:?}", xc_func.dens_threshold()); // 1e-12
    /// ```
    pub fn dens_threshold(&self) -> f64 {
        unsafe { (*self.ptr).dens_threshold }
    }

    /// Set the density threshold.
    pub fn set_dens_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_dens_threshold(self.ptr, threshold) }
    }

    /// Zeta (spin polarization) threshold for numerical stability.
    pub fn zeta_threshold(&self) -> f64 {
        unsafe { (*self.ptr).zeta_threshold }
    }

    /// Set the zeta (spin polarization) threshold.
    pub fn set_zeta_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_zeta_threshold(self.ptr, threshold) }
    }

    /// Sigma (reduced gradient) threshold for numerical stability.
    pub fn sigma_threshold(&self) -> f64 {
        unsafe { (*self.ptr).sigma_threshold }
    }

    /// Set the sigma (reduced gradient) threshold.
    pub fn set_sigma_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_sigma_threshold(self.ptr, threshold) }
    }

    /// Tau (kinetic energy density) threshold for numerical stability.
    pub fn tau_threshold(&self) -> f64 {
        unsafe { (*self.ptr).tau_threshold }
    }

    /// Set the tau (kinetic energy density) threshold.
    pub fn set_tau_threshold(&mut self, threshold: f64) {
        unsafe { ffi::xc_func_set_tau_threshold(self.ptr, threshold) }
    }

    /// Enable or disable Fermi hole curvature enforcement (api-v7_0+).
    #[cfg(feature = "api-v7_0")]
    pub fn set_fhc_enforcement(&mut self, on: bool) {
        unsafe { ffi::xc_func_set_fhc_enforcement(self.ptr, on as c_int) }
    }
}

/// Getters for hybrid/cam/VV10 coefficients and auxiliary functionals.
impl LibXCFunctional {
    /// Fraction of Hartree-Fock exchange for global hybrids.
    ///
    /// Please note that this is only applicable for global hybrids (HybGGA,
    /// HybMGGA, HybLDA), and will return `None` for other functionals,
    /// including pure functionals and **range-separated hybrids**. For
    /// range-separated hybrids, use `cam_coef()` to get the CAM
    /// coefficients instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_x3lyp", Unpolarized);
    /// assert_eq!(xc_func.hyb_exx_coef(), Some(0.218));
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x", Unpolarized);
    /// assert_eq!(xc_func.hyb_exx_coef(), None);
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_hyb_exx_coef`
    pub fn hyb_exx_coef(&self) -> Option<f64> {
        if matches!(self.family(), LibXCFamily::HybGGA | LibXCFamily::HybMGGA | LibXCFamily::HybLDA)
            && !self.is_hyb_cam()
        {
            Some(unsafe { ffi::xc_hyb_exx_coef(self.ptr) })
        } else {
            None
        }
    }

    /// Range-separated hybrid coefficients (omega, alpha, beta).
    ///
    /// Only applicable for CAM hybrids (HybGGA, HybMGGA, HybLDA with CAM
    /// flags), and will return `None` for other functionals, including pure
    /// functionals and global hybrids. For global hybrids, use `hyb_exx_coef()`
    /// to get the fraction of Hartree-Fock exchange instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_x3lyp", Unpolarized);
    /// assert_eq!(xc_func.cam_coef(), None);
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x", Unpolarized);
    /// assert_eq!(xc_func.cam_coef(), Some((0.3, 1.0, -0.842294)));
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_cam_coef()`
    pub fn cam_coef(&self) -> Option<(f64, f64, f64)> {
        if matches!(self.family(), LibXCFamily::HybGGA | LibXCFamily::HybMGGA | LibXCFamily::HybLDA)
            && self.is_hyb_cam()
        {
            let mut omega: f64 = 0.0;
            let mut alpha: f64 = 0.0;
            let mut beta: f64 = 0.0;
            unsafe { ffi::xc_hyb_cam_coef(self.ptr, &mut omega, &mut alpha, &mut beta) };
            Some((omega, alpha, beta))
        } else {
            None
        }
    }

    /// VV10 non-local correlation coefficients (nlc_b, nlc_C).
    ///
    /// Only applicable for functionals with the `VV10` flag, and will return
    /// `None` for other functionals.
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x", Unpolarized);
    /// assert_eq!(xc_func.vv10_coef(), None);
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x_v", Unpolarized);
    /// assert_eq!(xc_func.vv10_coef(), Some((6.0, 0.01)));
    /// let xc_func = LibXCFunctional::from_identifier("mgga_c_scanl_rvv10", Unpolarized);
    /// assert_eq!(xc_func.vv10_coef(), Some((15.7, 0.0093)));
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_vv10_coef()`
    pub fn vv10_coef(&self) -> Option<(f64, f64)> {
        if self.has_flag(LibXCFlags::VV10) {
            let mut nlc_b: f64 = 0.0;
            #[allow(non_snake_case)]
            let mut nlc_C: f64 = 0.0;
            unsafe {
                ffi::xc_nlc_coef(self.ptr, &mut nlc_b, &mut nlc_C);
            }
            Some((nlc_b, nlc_C))
        } else {
            None
        }
    }

    /// Weights of the auxiliary functionals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_b3lyp", Unpolarized);
    /// println!("{:?}", xc_func.aux_funcs());
    /// // Output: [("lda_x", 0.08), ("gga_x_b88", 0.72), ("lda_c_vwn_rpa", 0.19), ("gga_c_lyp", 0.81)]
    /// // Output may have minor precision difference.
    /// # let identifiers = xc_func.aux_funcs().iter().map(|(name, _)| name.clone()).collect::<Vec<String>>();
    /// # assert_eq!(identifiers, vec![
    /// #     "lda_x".to_string(),
    /// #     "gga_x_b88".to_string(),
    /// #     "lda_c_vwn_rpa".to_string(),
    /// #     "gga_c_lyp".to_string(),
    /// # ]);
    /// # let weights: Vec<f64> = xc_func.aux_funcs().iter().map(|(_, weight)| *weight).collect();
    /// # let expected_weights = vec![0.08, 0.72, 0.19, 0.81];
    /// # for (w, ew) in weights.iter().zip(expected_weights.iter()) {
    /// #     assert!((w - ew).abs() < 1e-6, "Expected weight {ew}, got {w}");
    /// # }
    /// ```
    ///
    /// Note some functionals does not have auxiliary functionals, and will
    /// return an empty vector.
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_wb97x_v", Unpolarized);
    /// assert!(xc_func.aux_funcs().is_empty());
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_aux_funcs(return_ids=False)`
    pub fn aux_funcs(&self) -> Vec<(String, f64)> {
        self.aux_funcs_by_id()
            .into_iter()
            .map(|(id, weight)| {
                let name = crate::util::libxc_functional_get_name(id).unwrap_or_default();
                (name, weight)
            })
            .collect()
    }

    /// IDs of the auxiliary functionals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libxc::prelude::{libxc_enum_items::*, *};
    /// let xc_func = LibXCFunctional::from_identifier("hyb_gga_xc_b3lyp", Unpolarized);
    /// // Output: [(1, 0.08), (106, 0.72), (8, 0.19), (131, 0.81)]
    /// # let ids = xc_func.aux_funcs_by_id().iter().map(|(id, _)| *id).collect::<Vec<i32>>();
    /// # assert_eq!(ids, vec![1, 106, 8, 131]);
    /// ```
    ///
    /// # PyLibxc counterpart
    ///
    /// `LibXCFunctional.get_aux_funcs(return_ids=True)`
    pub fn aux_funcs_by_id(&self) -> Vec<(i32, f64)> {
        let n = unsafe { ffi::xc_num_aux_funcs(self.ptr) as i32 };
        let mut ids = vec![0 as c_int; n as usize];
        let mut weights = vec![0.0f64; n as usize];
        unsafe { ffi::xc_aux_func_ids(self.ptr, ids.as_mut_ptr()) }
        unsafe { ffi::xc_aux_func_weights(self.ptr, weights.as_mut_ptr()) }
        ids.into_iter().zip(weights).collect()
    }
}

impl Drop for LibXCFunctional {
    fn drop(&mut self) {
        unsafe {
            ffi::xc_func_end(self.ptr);
            ffi::xc_func_free(self.ptr);
        }
    }
}

impl core::fmt::Debug for LibXCFunctional {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LibXCFunctional")
            .field("name", &self.identifier())
            .field("number", &self.number())
            .field("family", &self.family())
            .field("spin", &self.spin())
            .finish()
    }
}
