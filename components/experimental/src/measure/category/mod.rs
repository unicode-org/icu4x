// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use icu_provider::{DataMarker, DynamicDataMarker};

use crate::{
    dimension::provider::units::{
        categorized_display_names::{
            UnitsNamesAreaLongHeavyV1, UnitsNamesAreaLongLightV1, UnitsNamesAreaLongTinyV1,
            UnitsNamesAreaNarrowHeavyV1, UnitsNamesAreaNarrowLightV1, UnitsNamesAreaNarrowTinyV1,
            UnitsNamesAreaShortHeavyV1, UnitsNamesAreaShortLightV1, UnitsNamesAreaShortTinyV1,
            UnitsNamesDurationLongHeavyV1, UnitsNamesDurationLongLightV1,
            UnitsNamesDurationLongTinyV1, UnitsNamesDurationNarrowHeavyV1,
            UnitsNamesDurationNarrowLightV1, UnitsNamesDurationNarrowTinyV1,
            UnitsNamesDurationShortHeavyV1, UnitsNamesDurationShortLightV1,
            UnitsNamesDurationShortTinyV1, UnitsNamesLengthLongHeavyV1,
            UnitsNamesLengthLongLightV1, UnitsNamesLengthLongTinyV1, UnitsNamesLengthNarrowHeavyV1,
            UnitsNamesLengthNarrowLightV1, UnitsNamesLengthNarrowTinyV1,
            UnitsNamesLengthShortHeavyV1, UnitsNamesLengthShortLightV1,
            UnitsNamesLengthShortTinyV1, UnitsNamesMassLongHeavyV1, UnitsNamesMassLongLightV1,
            UnitsNamesMassLongTinyV1, UnitsNamesMassNarrowHeavyV1, UnitsNamesMassNarrowLightV1,
            UnitsNamesMassNarrowTinyV1, UnitsNamesMassShortHeavyV1, UnitsNamesMassShortLightV1,
            UnitsNamesMassShortTinyV1, UnitsNamesVolumeLongHeavyV1, UnitsNamesVolumeLongLightV1,
            UnitsNamesVolumeLongTinyV1, UnitsNamesVolumeNarrowHeavyV1,
            UnitsNamesVolumeNarrowLightV1, UnitsNamesVolumeNarrowTinyV1,
            UnitsNamesVolumeShortHeavyV1, UnitsNamesVolumeShortLightV1,
            UnitsNamesVolumeShortTinyV1,
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
/// It has one data marker for each width (long, short, narrow)
/// and each tier (tiny, light, heavy) of the unit names.
pub trait MeasureUnitCategory {
    type DataMarkerLongTiny: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>> + DataMarker;
    type DataMarkerLongLight: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerLongHeavy: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortTiny: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortLight: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortHeavy: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerNarrowTiny: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerNarrowLight: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerNarrowHeavy: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
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
    type DataMarkerShortTiny = UnitsNamesAreaShortTinyV1;
    type DataMarkerShortLight = UnitsNamesAreaShortLightV1;
    type DataMarkerShortHeavy = UnitsNamesAreaShortHeavyV1;
    type DataMarkerNarrowTiny = UnitsNamesAreaNarrowTinyV1;
    type DataMarkerNarrowLight = UnitsNamesAreaNarrowLightV1;
    type DataMarkerNarrowHeavy = UnitsNamesAreaNarrowHeavyV1;
}
impl MeasureUnitCategory for Duration {
    type DataMarkerLongTiny = UnitsNamesDurationLongTinyV1;
    type DataMarkerLongLight = UnitsNamesDurationLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesDurationLongHeavyV1;
    type DataMarkerShortTiny = UnitsNamesDurationShortTinyV1;
    type DataMarkerShortLight = UnitsNamesDurationShortLightV1;
    type DataMarkerShortHeavy = UnitsNamesDurationShortHeavyV1;
    type DataMarkerNarrowTiny = UnitsNamesDurationNarrowTinyV1;
    type DataMarkerNarrowLight = UnitsNamesDurationNarrowLightV1;
    type DataMarkerNarrowHeavy = UnitsNamesDurationNarrowHeavyV1;
}
impl MeasureUnitCategory for Length {
    type DataMarkerLongTiny = UnitsNamesLengthLongTinyV1;
    type DataMarkerLongLight = UnitsNamesLengthLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesLengthLongHeavyV1;
    type DataMarkerShortTiny = UnitsNamesLengthShortTinyV1;
    type DataMarkerShortLight = UnitsNamesLengthShortLightV1;
    type DataMarkerShortHeavy = UnitsNamesLengthShortHeavyV1;
    type DataMarkerNarrowTiny = UnitsNamesLengthNarrowTinyV1;
    type DataMarkerNarrowLight = UnitsNamesLengthNarrowLightV1;
    type DataMarkerNarrowHeavy = UnitsNamesLengthNarrowHeavyV1;
}
impl MeasureUnitCategory for Mass {
    type DataMarkerLongTiny = UnitsNamesMassLongTinyV1;
    type DataMarkerLongLight = UnitsNamesMassLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesMassLongHeavyV1;
    type DataMarkerShortTiny = UnitsNamesMassShortTinyV1;
    type DataMarkerShortLight = UnitsNamesMassShortLightV1;
    type DataMarkerShortHeavy = UnitsNamesMassShortHeavyV1;
    type DataMarkerNarrowTiny = UnitsNamesMassNarrowTinyV1;
    type DataMarkerNarrowLight = UnitsNamesMassNarrowLightV1;
    type DataMarkerNarrowHeavy = UnitsNamesMassNarrowHeavyV1;
}
impl MeasureUnitCategory for Volume {
    type DataMarkerLongTiny = UnitsNamesVolumeLongTinyV1;
    type DataMarkerLongLight = UnitsNamesVolumeLongLightV1;
    type DataMarkerLongHeavy = UnitsNamesVolumeLongHeavyV1;
    type DataMarkerShortTiny = UnitsNamesVolumeShortTinyV1;
    type DataMarkerShortLight = UnitsNamesVolumeShortLightV1;
    type DataMarkerShortHeavy = UnitsNamesVolumeShortHeavyV1;
    type DataMarkerNarrowTiny = UnitsNamesVolumeNarrowTinyV1;
    type DataMarkerNarrowLight = UnitsNamesVolumeNarrowLightV1;
    type DataMarkerNarrowHeavy = UnitsNamesVolumeNarrowHeavyV1;
}
