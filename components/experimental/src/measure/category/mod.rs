// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use icu_provider::{DataMarker, DynamicDataMarker};

use crate::{
    dimension::provider::units::{
        categorized_display_names::{
            UnitsNamesAreaHeavyV1, UnitsNamesAreaLightV1, UnitsNamesAreaTinyV1,
            UnitsNamesDurationHeavyV1, UnitsNamesDurationLightV1, UnitsNamesDurationTinyV1,
            UnitsNamesLengthHeavyV1, UnitsNamesLengthLightV1, UnitsNamesLengthTinyV1,
            UnitsNamesMassHeavyV1, UnitsNamesMassLightV1, UnitsNamesMassTinyV1,
            UnitsNamesVolumeHeavyV1, UnitsNamesVolumeLightV1, UnitsNamesVolumeTinyV1,
        },
        display_names::UnitsDisplayNames,
    },
    measure::measureunit::MeasureUnit,
};

pub mod area;
pub mod duration;
pub mod length;
pub mod mass;
pub mod volume;

pub trait MeasureUnitCategory {
    type DataMarkerTiny: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>> + DataMarker;
    type DataMarkerLight: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>> + DataMarker;
    type DataMarkerHeavy: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>> + DataMarker;
}

/// A [`MeasureUnit`] that is related to a specific category.
///
/// This is useful for type inference and for ensuring that the correct units are used.
#[derive(Debug)]
pub struct CategorizedMeasureUnit<T: MeasureUnitCategory> {
    pub(crate) _category: PhantomData<T>,
    pub unit: MeasureUnit,
}

impl<T: MeasureUnitCategory> CategorizedMeasureUnit<T> {
    // TODO: remove this once we are using the short units name in the datagen to locate the units.
    /// Returns the CLDR ID of the unit.
    pub fn cldr_id(&self) -> &str {
        match self.unit.id {
            Some(id) => id,
            None => unimplemented!(),
        }
    }
}

/// A [`MeasureUnit`] that is related to the area category.
#[derive(Debug)]
#[non_exhaustive]
pub struct Area;

/// A [`MeasureUnit`] that is related to the duration category.
#[derive(Debug)]
#[non_exhaustive]
pub struct Duration;

/// A [`MeasureUnit`] that is related to the length category.
#[derive(Debug)]
#[non_exhaustive]
pub struct Length;

/// A [`MeasureUnit`] that is related to the mass category.
#[derive(Debug)]
#[non_exhaustive]
pub struct Mass;

/// A [`MeasureUnit`] that is related to the volume category.
#[derive(Debug)]
#[non_exhaustive]
pub struct Volume;

impl MeasureUnitCategory for Area {
    type DataMarkerTiny = UnitsNamesAreaTinyV1;
    type DataMarkerLight = UnitsNamesAreaLightV1;
    type DataMarkerHeavy = UnitsNamesAreaHeavyV1;
}
impl MeasureUnitCategory for Duration {
    type DataMarkerTiny = UnitsNamesDurationTinyV1;
    type DataMarkerLight = UnitsNamesDurationLightV1;
    type DataMarkerHeavy = UnitsNamesDurationHeavyV1;
}
impl MeasureUnitCategory for Length {
    type DataMarkerTiny = UnitsNamesLengthTinyV1;
    type DataMarkerLight = UnitsNamesLengthLightV1;
    type DataMarkerHeavy = UnitsNamesLengthHeavyV1;
}
impl MeasureUnitCategory for Mass {
    type DataMarkerTiny = UnitsNamesMassTinyV1;
    type DataMarkerLight = UnitsNamesMassLightV1;
    type DataMarkerHeavy = UnitsNamesMassHeavyV1;
}
impl MeasureUnitCategory for Volume {
    type DataMarkerTiny = UnitsNamesVolumeTinyV1;
    type DataMarkerLight = UnitsNamesVolumeLightV1;
    type DataMarkerHeavy = UnitsNamesVolumeHeavyV1;
}
