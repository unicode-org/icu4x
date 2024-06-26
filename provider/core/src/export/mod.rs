// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains various utilities required to generate ICU4X data files, typically
//! via the `icu_datagen` reference crate. End users should not need to consume anything in
//! this module as a library unless defining new types that integrate with `icu_datagen`.
//!
//! This module can be enabled with the `datagen` Cargo feature on `icu_provider`.

mod payload;

pub use payload::{ExportBox, ExportMarker};

use crate::prelude::*;
use std::collections::HashSet;

/// An object capable of exporting data payloads in some form.
pub trait DataExporter: Sync {
    /// Save a `payload` corresponding to the given marker and locale.
    /// Takes non-mut self as it can be called concurrently.
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        locale: &DataLocale,
        marker_attributes: &DataMarkerAttributes,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError>;

    /// Function called for singleton markers.
    /// Takes non-mut self as it can be called concurrently.
    fn flush_singleton(
        &self,
        marker: DataMarkerInfo,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        self.put_payload(marker, Default::default(), Default::default(), payload)?;
        self.flush(marker)
    }

    /// Function called after a non-singleton marker has been fully enumerated.
    /// Does not include built-in fallback.
    ///
    /// Takes non-mut self as it can be called concurrently.
    fn flush(&self, _marker: DataMarkerInfo) -> Result<(), DataError> {
        Ok(())
    }

    /// This function has to be called before the object is dropped (after all
    /// markers have been fully dumped). This conceptually takes ownership, so
    /// clients *may not* interact with this object after close has been called.
    fn close(&mut self) -> Result<(), DataError> {
        Ok(())
    }
}

impl DataExporter for Box<dyn DataExporter> {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        locale: &DataLocale,
        marker_attributes: &DataMarkerAttributes,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        (**self).put_payload(marker, locale, marker_attributes, payload)
    }

    fn flush_singleton(
        &self,
        marker: DataMarkerInfo,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        (**self).flush_singleton(marker, payload)
    }

    fn flush(&self, marker: DataMarkerInfo) -> Result<(), DataError> {
        (**self).flush(marker)
    }

    fn close(&mut self) -> Result<(), DataError> {
        (**self).close()
    }
}

/// A [`DynamicDataProvider`] that can be used for exporting data.
///
/// Use [`make_exportable_provider`] to implement this.
pub trait ExportableProvider:
    crate::data_provider::IterableDynamicDataProvider<ExportMarker> + Sync
{
    /// Returns the set of supported markers
    fn supported_markers(&self) -> HashSet<DataMarkerInfo>;
}

impl ExportableProvider for Box<dyn ExportableProvider> {
    fn supported_markers(&self) -> HashSet<DataMarkerInfo> {
        (**self).supported_markers()
    }
}

/// This macro can be used on a data provider to allow it to be used for data generation.
///
/// Data generation 'compiles' data by using this data provider (which usually translates data from
/// different sources and doesn't have to be efficient) to generate data structs, and then writing
/// them to an efficient format like [`BlobDataProvider`] or [`BakedDataProvider`]. The requirements
/// for `make_exportable_provider` are:
/// * The data struct has to implement [`serde::Serialize`](::serde::Serialize) and [`databake::Bake`]
/// * The provider needs to implement [`IterableDataProvider`] for all specified [`DataMarker`]s.
///   This allows the generating code to know which [`DataLocale`] to collect.
///
/// [`BlobDataProvider`]: ../../icu_provider_blob/struct.BlobDataProvider.html
/// [`BakedDataProvider`]: ../../icu_datagen/index.html
#[macro_export]
#[doc(hidden)] // macro
macro_rules! __make_exportable_provider {
    ($provider:ty, [ $($(#[$cfg:meta])? $struct_m:ty),+, ]) => {
        impl $crate::export::ExportableProvider for $provider {
            fn supported_markers(&self) -> std::collections::HashSet<$crate::DataMarkerInfo> {
                std::collections::HashSet::from_iter([
                    $(
                        $(#[$cfg])?
                        <$struct_m>::INFO,
                    )+
                ])
            }
        }

        $crate::dynutil::impl_dynamic_data_provider!(
            $provider,
            [ $($(#[$cfg])? $struct_m),+, ],
            $crate::export::ExportMarker
        );

        $crate::dynutil::impl_iterable_dynamic_data_provider!(
            $provider,
            [ $($(#[$cfg])? $struct_m),+, ],
            $crate::export::ExportMarker
        );
    };
}
#[doc(inline)]
pub use __make_exportable_provider as make_exportable_provider;

/// A `DataExporter` that forks to multiple `DataExporter`s.
#[derive(Default)]
pub struct MultiExporter(Vec<Box<dyn DataExporter>>);

impl MultiExporter {
    /// Creates a `MultiExporter` for the given exporters.
    pub const fn new(exporters: Vec<Box<dyn DataExporter>>) -> Self {
        Self(exporters)
    }
}

impl core::fmt::Debug for MultiExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiExporter")
            .field("0", &format!("vec[len = {}]", self.0.len()))
            .finish()
    }
}

impl DataExporter for MultiExporter {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        locale: &DataLocale,
        marker_attributes: &DataMarkerAttributes,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        self.0
            .iter()
            .try_for_each(|e| e.put_payload(marker, locale, marker_attributes, payload))
    }

    fn flush_singleton(
        &self,
        marker: DataMarkerInfo,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        self.0
            .iter()
            .try_for_each(|e| e.flush_singleton(marker, payload))
    }

    fn flush(&self, marker: DataMarkerInfo) -> Result<(), DataError> {
        self.0.iter().try_for_each(|e| e.flush(marker))
    }

    fn close(&mut self) -> Result<(), DataError> {
        self.0.iter_mut().try_for_each(|e| e.close())
    }
}
