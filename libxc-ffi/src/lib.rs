//! FFI bindings for libxc.
//!
//! This crate provides both static and dynamic FFI bindings for the
//! [libxc](https://www.tddft.org/programs/libxc/) library of exchange-correlation
//! functionals for density-functional theory.
//!
//! # API Version Features
//!
//! - `api-v6_2`: Base API (libxc 6.2.2)
//! - `api-v7_0`: Extends api-v6_2 (libxc 7.0.0, default)
//! - `api-v7_1`: Extends api-v7_0 (libxc 7.1.0)
//!
//! Features are cumulative: enabling `api-v7_1` also enables all functions from
//! earlier versions.
//!
//! # Loading Modes
//!
//! - **Static linking** (default without `dynamic_loading`): Links against
//!   `libxc` at compile time.
//! - **Dynamic loading** (default with `dynamic_loading` feature): Loads
//!   `libxc.so` at runtime via `libloading`. Set `LIBXC_DYLOAD` environment
//!   variable to specify the library path.
//!
//! # Functional IDs
//!
//! The [`xc_funcs`] module provides versioned enums for functional identifiers
//! (e.g., `XcFuncId::LDA_X = 1`).

#![allow(non_snake_case)]

// Static FFI bindings
#[cfg(not(feature = "dynamic_loading"))]
pub mod ffi_static;
#[cfg(not(feature = "dynamic_loading"))]
pub use ffi_static as ffi;

// Dynamic loading FFI bindings
#[cfg(feature = "dynamic_loading")]
pub mod ffi_dynamic;
#[cfg(feature = "dynamic_loading")]
pub use ffi_dynamic as ffi;

// Functional ID enums
pub mod xc_funcs;
