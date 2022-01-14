// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains various utilities required to generate ICU4X data files, typically
//! via the `icu4x-datagen` tool. End users should not need to consume anything in this module
//! as a library unless defining new types that integrate with `icu4x-datagen`.
//!
//! This module can be enabled with the `datagen` feature on `icu4x-provider-core`.

mod crabbake;
mod data_conversion;
mod heap_measure;
mod iter;
pub use self::crabbake::{CrabbakeBox, CrabbakeMarker};
pub use data_conversion::{DataConverter, ReturnedPayloadError};
pub use heap_measure::{HeapStats, HeapStatsMarker};
pub use iter::{IterableDynProvider, IterableResourceProvider};
