// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use icu_provider::{DataMarker, DynamicDataMarker};

use crate::{
    dimension::provider::units::{
        categorized_display_names::{
            UnitsNamesAreaLongHeavyV1, UnitsNamesAreaLongLightV1, UnitsNamesAreaLongTinyV1,
            UnitsNamesAreaShortNarrowHeavyV1, UnitsNamesAreaShortNarrowLightV1,
            UnitsNamesAreaShortNarrowTinyV1, UnitsNamesDurationLongHeavyV1,
            UnitsNamesDurationLongLightV1, UnitsNamesDurationLongTinyV1,
            UnitsNamesDurationShortNarrowHeavyV1, UnitsNamesDurationShortNarrowLightV1,
            UnitsNamesDurationShortNarrowTinyV1, UnitsNamesLengthLongHeavyV1,
            UnitsNamesLengthLongLightV1, UnitsNamesLengthLongTinyV1,
            UnitsNamesLengthShortNarrowHeavyV1, UnitsNamesLengthShortNarrowLightV1,
            UnitsNamesLengthShortNarrowTinyV1, UnitsNamesMassLongHeavyV1,
            UnitsNamesMassLongLightV1, UnitsNamesMassLongTinyV1, UnitsNamesMassShortNarrowHeavyV1,
            UnitsNamesMassShortNarrowLightV1, UnitsNamesMassShortNarrowTinyV1,
            UnitsNamesVolumeLongHeavyV1, UnitsNamesVolumeLongLightV1, UnitsNamesVolumeLongTinyV1,
            UnitsNamesVolumeShortNarrowHeavyV1, UnitsNamesVolumeShortNarrowLightV1,
            UnitsNamesVolumeShortNarrowTinyV1,
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

/// A category of measure units, such as length or mass.
///
/// For each tier (tiny, light, heavy) of the unit names, it has one data marker for
/// the long names, and one for the short and the narrow names together.
pub trait MeasureUnitCategory {
    type DataMarkerLongTiny: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>> + DataMarker;
    type DataMarkerLongLight: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerLongHeavy: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortNarrowTiny: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortNarrowLight: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortNarrowHeavy: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
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
    type DataMarkerLongTiny = UnitsNamesAreaLongTinyV1;
    type DataMarkerLongLight = UnitsNamesAreaLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesAreaLongHeavyV1;
    type DataMarkerShortNarrowTiny = UnitsNamesAreaShortNarrowTinyV1;
    type DataMarkerShortNarrowLight = UnitsNamesAreaShortNarrowLightV1;
    type DataMarkerShortNarrowHeavy = UnitsNamesAreaShortNarrowHeavyV1;
}
impl MeasureUnitCategory for Duration {
    type DataMarkerLongTiny = UnitsNamesDurationLongTinyV1;
    type DataMarkerLongLight = UnitsNamesDurationLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesDurationLongHeavyV1;
    type DataMarkerShortNarrowTiny = UnitsNamesDurationShortNarrowTinyV1;
    type DataMarkerShortNarrowLight = UnitsNamesDurationShortNarrowLightV1;
    type DataMarkerShortNarrowHeavy = UnitsNamesDurationShortNarrowHeavyV1;
}
impl MeasureUnitCategory for Length {
    type DataMarkerLongTiny = UnitsNamesLengthLongTinyV1;
    type DataMarkerLongLight = UnitsNamesLengthLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesLengthLongHeavyV1;
    type DataMarkerShortNarrowTiny = UnitsNamesLengthShortNarrowTinyV1;
    type DataMarkerShortNarrowLight = UnitsNamesLengthShortNarrowLightV1;
    type DataMarkerShortNarrowHeavy = UnitsNamesLengthShortNarrowHeavyV1;
}
impl MeasureUnitCategory for Mass {
    type DataMarkerLongTiny = UnitsNamesMassLongTinyV1;
    type DataMarkerLongLight = UnitsNamesMassLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesMassLongHeavyV1;
    type DataMarkerShortNarrowTiny = UnitsNamesMassShortNarrowTinyV1;
    type DataMarkerShortNarrowLight = UnitsNamesMassShortNarrowLightV1;
    type DataMarkerShortNarrowHeavy = UnitsNamesMassShortNarrowHeavyV1;
}
impl MeasureUnitCategory for Volume {
    type DataMarkerLongTiny = UnitsNamesVolumeLongTinyV1;
    type DataMarkerLongLight = UnitsNamesVolumeLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesVolumeLongHeavyV1;
    type DataMarkerShortNarrowTiny = UnitsNamesVolumeShortNarrowTinyV1;
    type DataMarkerShortNarrowLight = UnitsNamesVolumeShortNarrowLightV1;
    type DataMarkerShortNarrowHeavy = UnitsNamesVolumeShortNarrowHeavyV1;
}
