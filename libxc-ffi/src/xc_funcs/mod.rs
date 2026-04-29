//! Libxc functional identifier enums.
//!
//! This module provides versioned enums representing the functional IDs
//! defined in `xc_funcs_v*.h`. Each API version has its own submodule
//! containing the `XcFuncId` enum for that version.
//!
//! The enums derive `serde::Serialize` and `serde::Deserialize` for
//! easy serialization. They also implement `FromStr`, `Display`,
//! `From<XcFuncId> for u32`, and `TryFrom<u32> for XcFuncId`.
//!
//! Since API features are cumulative, `XcFuncId` is re-exported from
//! the highest enabled version's submodule.

#[cfg(feature = "api-v6_2")]
pub mod v6_2;

#[cfg(feature = "api-v7_0")]
pub mod v7_0;

#[cfg(feature = "api-v7_1")]
pub mod v7_1;

#[cfg(all(feature = "api-v6_2", not(feature = "api-v7_0")))]
pub use v6_2::XcFuncId;

#[cfg(all(feature = "api-v7_0", not(feature = "api-v7_1")))]
pub use v7_0::XcFuncId;

#[cfg(feature = "api-v7_1")]
pub use v7_1::XcFuncId;
