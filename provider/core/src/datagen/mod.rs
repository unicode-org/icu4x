// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains various utilities required to generate ICU4X data files, typically
//! via the `icu4x-datagen` tool. End users should not need to consume anything in this module
//! as a library unless defining new types that integrate with `icu4x-datagen`.
//!
//! This module can be enabled with the `datagen` feature on `icu4x-provider-core`.

mod data_conversion;
mod heap_measure;

pub use data_conversion::{DataConverter, ReturnedPayloadError};
pub use heap_measure::{HeapStats, HeapStatsMarker};

use crate::any::AnyMarker;
use crate::buf::BufferMarker;
use crate::iter::IterableDynProvider;
use crate::serde::SerializeMarker;
use crate::DataMarker;

/// A blanket-implemented trait that allows for all the datagen-relevant traits
/// to be used in trait objects. The type parameter is the marker type needed for
/// [`IterableDynProvider`].
pub trait OmnibusDatagenProvider<M: DataMarker>:
    DataConverter<AnyMarker, SerializeMarker>
    + DataConverter<BufferMarker, HeapStatsMarker>
    + IterableDynProvider<M>
{
}

impl<M, P> OmnibusDatagenProvider<M> for P
where
    P: DataConverter<AnyMarker, SerializeMarker>
        + DataConverter<BufferMarker, HeapStatsMarker>
        + IterableDynProvider<M>,
    M: DataMarker,
{
}
