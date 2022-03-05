// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::buf::{BufferFormat, BufferMarker};
use crate::prelude::*;
use yoke::trait_hack::YokeTraitHack;

/// Stats on the heap size needed when attempting to zero-copy-deserialize
/// a postcard-formatted data struct.
#[derive(Debug, Copy, Clone, yoke::Yokeable, Default)]
pub struct HeapStats {
    pub bytes_needed: u64,
}

/// The [`DataMarker`] marker type for [`HeapStats`].
pub struct HeapStatsMarker;

impl DataMarker for HeapStatsMarker {
    type Yokeable = HeapStats;
}

impl DataPayload<BufferMarker> {
    /// Given a buffer known to be in postcard-0.7 format, attempt to zero-copy
    /// deserialize it and record the amount of heap allocations that occurred.
    ///
    /// Ideally, this number should be zero.
    ///
    /// [`dhat`]'s profiler must be initialized before using this.
    pub fn attempt_zero_copy_heap_size<M>(self) -> HeapStats
    where
        M: DataMarker,
        for<'a> &'a <M::Yokeable as yoke::Yokeable<'a>>::Output: serde::Serialize,
        for<'de> YokeTraitHack<<M::Yokeable as yoke::Yokeable<'de>>::Output>:
            serde::Deserialize<'de>,
    {
        let stats_before = dhat::HeapStats::get();
        // reify, but do nothing with the type
        let _reified_data: DataPayload<M> = self
            .into_deserialized(BufferFormat::Postcard07)
            .expect("Failed to deserialize BufferMarker as postcard-0.7");
        let stats_after = dhat::HeapStats::get();

        HeapStats {
            bytes_needed: stats_after.total_bytes - stats_before.total_bytes,
        }
    }
}
