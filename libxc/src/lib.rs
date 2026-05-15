//! Safe Rust wrapper for [libxc](https://www.tddft.org/programs/libxc/), the library
//! of exchange-correlation functionals for density-functional theory (DFT).
//!
//! # Overview
//!
//! This crate provides [`LibXCFunctional`], the central type that represents a
//! single DFT exchange-correlation functional. It wraps the C `xc_func_type`
//! pointer, manages its lifetime, and exposes three categories of operations:
//!
//! - **Construction** — create a functional by name or ID, with spin
//!   polarization: [`from_identifier`](LibXCFunctional::from_identifier),
//!   [`from_number`](LibXCFunctional::from_number) (plus `_f` fallible
//!   variants).
//!
//! - **Introspection** — query identity, family (LDA/GGA/MGGA/hybrid), kind,
//!   capability flags, hybrid/CAM/VV10 coefficients, auxiliary functionals, and
//!   literature references.
//!
//! - **Computation** — evaluate the functional and its derivatives (up to 4th
//!   order) on a grid via [`compute_xc`](LibXCFunctional::compute_xc) and
//!   variants. With the `cuda` feature, GPU computation is available via
//!   [`cuda_compute_xc`](LibXCFunctional::cuda_compute_xc) using functionals
//!   created with
//!   [`from_identifier_with_device`](LibXCFunctional::from_identifier_with_device).
#![doc = include_str!("../readme.md")]

pub mod enums;
pub mod error;
pub mod functional;
pub mod functional_specific;
pub mod util;

pub mod compute_cpu;
pub mod layout_handling;

#[cfg(feature = "cuda")]
pub mod compute_cuda;

pub mod prelude {
    pub use crate::enums::*;
    pub use crate::error::*;
    pub use crate::functional::*;
    pub use crate::layout_handling::*;
    pub use crate::util::*;

    pub(crate) use core::borrow::Borrow;
    pub(crate) use core::ffi::{c_char, c_int, CStr};
    pub(crate) use core::ops::Range;
    pub(crate) use enumflags2::BitFlags;
    pub(crate) use indexmap::IndexMap;
    pub(crate) use libxc_ffi::ffi;
    pub(crate) use std::collections::HashMap;
}

// documentation exception
#[allow(unused)]
use prelude::*;
