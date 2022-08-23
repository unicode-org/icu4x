// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::buf::{BufferFormat, BufferMarker};
use crate::prelude::*;
use yoke::trait_hack::YokeTraitHack;

/// Stats on the heap size needed when attempting to zero-copy-deserialize
/// a postcard-formatted data struct.
#[derive(Debug, Copy, Clone, yoke::Yokeable, Default)]
#[non_exhaustive]
pub struct HeapStats {
    /// Total bytes allocated during deserialization
    pub total_bytes_allocated: u64,
    /// Total bytes allocated during deserialization that have not yet been freed
    pub net_bytes_allocated: usize,
}

/// The [`DataMarker`] marker type for [`HeapStats`].
#[allow(clippy::exhaustive_structs)] // marker type
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
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not in postcard-0.7 format.
    #[allow(clippy::expect_used)] // The function documents when panics may occur.
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
            .into_deserialized(BufferFormat::Postcard1)
            .expect("Failed to deserialize BufferMarker as postcard-0.7");
        let stats_after = dhat::HeapStats::get();

        HeapStats {
            total_bytes_allocated: stats_after.total_bytes - stats_before.total_bytes,
            net_bytes_allocated: stats_after.curr_bytes - stats_before.curr_bytes,
        }
    }
}
