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
mod iter;
mod payload;
pub use data_conversion::{DataConverter, ReturnedPayloadError};
pub use heap_measure::{HeapStats, HeapStatsMarker};
pub use iter::{IterableDynProvider, IterableResourceProvider};
pub use payload::{ExportBox, ExportMarker};

use crate::prelude::*;

/// An object capable of exporting data payloads in some form.
pub trait DataExporter: Sync {
    /// Save a `payload` corresponding to the given key and options.
    /// Takes non-mut self as it can be called concurrently.
    fn put_payload(
        &self,
        key: ResourceKey,
        options: &ResourceOptions,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError>;

    /// Function called after all keys have been fully dumped.
    /// Takes non-mut self as it can be called concurrently.
    fn flush(&self, _key: ResourceKey) -> Result<(), DataError> {
        Ok(())
    }

    /// This function has to be called before the object is dropped (after all
    /// keys have been fully dumped). This conceptually takes ownership, so
    /// clients *may not* interact with this object after close has been called.
    fn close(&mut self) -> Result<(), DataError> {
        Ok(())
    }
}

/// A [`DynProvider`] that can be used for exporting data.
pub trait ExportableProvider: IterableDynProvider<ExportMarker> + Sync {}
impl<T> ExportableProvider for T where T: IterableDynProvider<ExportMarker> + Sync {}

#[macro_export]
macro_rules! make_exportable_provider {
    ($provider:ty, [ $($struct_m:ident),+, ]) => {
        $crate::impl_dyn_provider!(
            $provider,
            [ $($struct_m),+, ],
            $crate::datagen::ExportMarker
        );

        impl $crate::datagen::IterableDynProvider<$crate::datagen::ExportMarker> for $provider {
            fn supported_options_for_key(&self, key: $crate::ResourceKey) -> Result<Vec<$crate::ResourceOptions>, $crate::DataError> {
                #![allow(non_upper_case_globals)]
                // Reusing the struct names as identifiers
                $(
                    const $struct_m: ResourceKeyHash = <$struct_m as $crate::ResourceMarker>::KEY.get_hash();
                )+
                match key.get_hash() {
                    $(
                        $struct_m => {
                            $crate::datagen::IterableResourceProvider::<$struct_m>::supported_options(self)
                        }
                    )+,
                    _ => Err($crate::DataErrorKind::MissingResourceKey.with_key(key))
                }
            }
        }

        impl $crate::datagen::DataConverter<$crate::buf::BufferMarker, $crate::datagen::HeapStatsMarker> for $provider {
            fn convert(&self, key: $crate::ResourceKey, from: DataPayload<$crate::buf::BufferMarker>) -> Result<$crate::DataPayload<$crate::datagen::HeapStatsMarker>, $crate::datagen::ReturnedPayloadError<$crate::buf::BufferMarker>> {
                #![allow(non_upper_case_globals)]
                // Reusing the struct names as identifiers
                $(
                    const $struct_m: ResourceKeyHash = <$struct_m as $crate::ResourceMarker>::KEY.get_hash();
                )+
                match key.get_hash() {
                    $(
                        $struct_m => {
                            let heap_stats = from.attempt_zero_copy_heap_size::<$struct_m>();
                            return Ok($crate::DataPayload::from_owned(heap_stats));
                        }
                    )+,
                    _ => Err($crate::datagen::ReturnedPayloadError(from, $crate::DataErrorKind::MissingResourceKey.with_key(key)))
                }
            }
        }
    };
}
