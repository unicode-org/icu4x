// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod data_conversion;
mod heap_measure;

pub use data_conversion::{DataConverter, ReturnedPayloadError};
pub use heap_measure::{HeapStats, HeapStatsMarker};

use crate::any::AnyMarker;
use crate::buf::BufferMarker;
use crate::iter::IterableDynProvider;
use crate::serde::SerializeMarker;
use crate::DataMarker;

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
