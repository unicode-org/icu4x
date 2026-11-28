// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use icu_provider::{DataMarker, DynamicDataMarker};

use crate::{
    dimension::provider::units::{
        categorized_display_names::{
            UnitsNamesAreaLongEssentialV1, UnitsNamesAreaLongImperialV1,
            UnitsNamesAreaLongMetricV1, UnitsNamesAreaLongOutlierV1,
            UnitsNamesAreaNarrowEssentialV1, UnitsNamesAreaNarrowImperialV1,
            UnitsNamesAreaNarrowMetricV1, UnitsNamesAreaNarrowOutlierV1,
            UnitsNamesAreaShortEssentialV1, UnitsNamesAreaShortImperialV1,
            UnitsNamesAreaShortMetricV1, UnitsNamesAreaShortOutlierV1,
            UnitsNamesDurationLongEssentialV1, UnitsNamesDurationLongImperialV1,
            UnitsNamesDurationLongMetricV1, UnitsNamesDurationLongOutlierV1,
            UnitsNamesDurationNarrowEssentialV1, UnitsNamesDurationNarrowImperialV1,
            UnitsNamesDurationNarrowMetricV1, UnitsNamesDurationNarrowOutlierV1,
            UnitsNamesDurationShortEssentialV1, UnitsNamesDurationShortImperialV1,
            UnitsNamesDurationShortMetricV1, UnitsNamesDurationShortOutlierV1,
            UnitsNamesLengthLongEssentialV1, UnitsNamesLengthLongImperialV1,
            UnitsNamesLengthLongMetricV1, UnitsNamesLengthLongOutlierV1,
            UnitsNamesLengthNarrowEssentialV1, UnitsNamesLengthNarrowImperialV1,
            UnitsNamesLengthNarrowMetricV1, UnitsNamesLengthNarrowOutlierV1,
            UnitsNamesLengthShortEssentialV1, UnitsNamesLengthShortImperialV1,
            UnitsNamesLengthShortMetricV1, UnitsNamesLengthShortOutlierV1,
            UnitsNamesMassLongEssentialV1, UnitsNamesMassLongImperialV1,
            UnitsNamesMassLongMetricV1, UnitsNamesMassLongOutlierV1,
            UnitsNamesMassNarrowEssentialV1, UnitsNamesMassNarrowImperialV1,
            UnitsNamesMassNarrowMetricV1, UnitsNamesMassNarrowOutlierV1,
            UnitsNamesMassShortEssentialV1, UnitsNamesMassShortImperialV1,
            UnitsNamesMassShortMetricV1, UnitsNamesMassShortOutlierV1,
            UnitsNamesVolumeLongEssentialV1, UnitsNamesVolumeLongImperialV1,
            UnitsNamesVolumeLongMetricV1, UnitsNamesVolumeLongOutlierV1,
            UnitsNamesVolumeNarrowEssentialV1, UnitsNamesVolumeNarrowImperialV1,
            UnitsNamesVolumeNarrowMetricV1, UnitsNamesVolumeNarrowOutlierV1,
            UnitsNamesVolumeShortEssentialV1, UnitsNamesVolumeShortImperialV1,
            UnitsNamesVolumeShortMetricV1, UnitsNamesVolumeShortOutlierV1,
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
/// and each tier (essential, imperial, metric, outlier) of the unit names.
/// A unit is in the same tier in every locale.
pub trait MeasureUnitCategory {
    type DataMarkerLongEssential: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerLongImperial: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerLongMetric: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerLongOutlier: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortEssential: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortImperial: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortMetric: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerShortOutlier: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerNarrowEssential: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerNarrowImperial: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerNarrowMetric: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
        + DataMarker;
    type DataMarkerNarrowOutlier: DynamicDataMarker<DataStruct = UnitsDisplayNames<'static>>
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
    type DataMarkerLongEssential = UnitsNamesAreaLongEssentialV1;
    type DataMarkerLongImperial = UnitsNamesAreaLongImperialV1;
    type DataMarkerLongMetric = UnitsNamesAreaLongMetricV1;
    type DataMarkerLongOutlier = UnitsNamesAreaLongOutlierV1;
    type DataMarkerShortEssential = UnitsNamesAreaShortEssentialV1;
    type DataMarkerShortImperial = UnitsNamesAreaShortImperialV1;
    type DataMarkerShortMetric = UnitsNamesAreaShortMetricV1;
    type DataMarkerShortOutlier = UnitsNamesAreaShortOutlierV1;
    type DataMarkerNarrowEssential = UnitsNamesAreaNarrowEssentialV1;
    type DataMarkerNarrowImperial = UnitsNamesAreaNarrowImperialV1;
    type DataMarkerNarrowMetric = UnitsNamesAreaNarrowMetricV1;
    type DataMarkerNarrowOutlier = UnitsNamesAreaNarrowOutlierV1;
}
impl MeasureUnitCategory for Duration {
    type DataMarkerLongEssential = UnitsNamesDurationLongEssentialV1;
    type DataMarkerLongImperial = UnitsNamesDurationLongImperialV1;
    type DataMarkerLongMetric = UnitsNamesDurationLongMetricV1;
    type DataMarkerLongOutlier = UnitsNamesDurationLongOutlierV1;
    type DataMarkerShortEssential = UnitsNamesDurationShortEssentialV1;
    type DataMarkerShortImperial = UnitsNamesDurationShortImperialV1;
    type DataMarkerShortMetric = UnitsNamesDurationShortMetricV1;
    type DataMarkerShortOutlier = UnitsNamesDurationShortOutlierV1;
    type DataMarkerNarrowEssential = UnitsNamesDurationNarrowEssentialV1;
    type DataMarkerNarrowImperial = UnitsNamesDurationNarrowImperialV1;
    type DataMarkerNarrowMetric = UnitsNamesDurationNarrowMetricV1;
    type DataMarkerNarrowOutlier = UnitsNamesDurationNarrowOutlierV1;
}
impl MeasureUnitCategory for Length {
    type DataMarkerLongEssential = UnitsNamesLengthLongEssentialV1;
    type DataMarkerLongImperial = UnitsNamesLengthLongImperialV1;
    type DataMarkerLongMetric = UnitsNamesLengthLongMetricV1;
    type DataMarkerLongOutlier = UnitsNamesLengthLongOutlierV1;
    type DataMarkerShortEssential = UnitsNamesLengthShortEssentialV1;
    type DataMarkerShortImperial = UnitsNamesLengthShortImperialV1;
    type DataMarkerShortMetric = UnitsNamesLengthShortMetricV1;
    type DataMarkerShortOutlier = UnitsNamesLengthShortOutlierV1;
    type DataMarkerNarrowEssential = UnitsNamesLengthNarrowEssentialV1;
    type DataMarkerNarrowImperial = UnitsNamesLengthNarrowImperialV1;
    type DataMarkerNarrowMetric = UnitsNamesLengthNarrowMetricV1;
    type DataMarkerNarrowOutlier = UnitsNamesLengthNarrowOutlierV1;
}
impl MeasureUnitCategory for Mass {
    type DataMarkerLongEssential = UnitsNamesMassLongEssentialV1;
    type DataMarkerLongImperial = UnitsNamesMassLongImperialV1;
    type DataMarkerLongMetric = UnitsNamesMassLongMetricV1;
    type DataMarkerLongOutlier = UnitsNamesMassLongOutlierV1;
    type DataMarkerShortEssential = UnitsNamesMassShortEssentialV1;
    type DataMarkerShortImperial = UnitsNamesMassShortImperialV1;
    type DataMarkerShortMetric = UnitsNamesMassShortMetricV1;
    type DataMarkerShortOutlier = UnitsNamesMassShortOutlierV1;
    type DataMarkerNarrowEssential = UnitsNamesMassNarrowEssentialV1;
    type DataMarkerNarrowImperial = UnitsNamesMassNarrowImperialV1;
    type DataMarkerNarrowMetric = UnitsNamesMassNarrowMetricV1;
    type DataMarkerNarrowOutlier = UnitsNamesMassNarrowOutlierV1;
}
impl MeasureUnitCategory for Volume {
    type DataMarkerLongEssential = UnitsNamesVolumeLongEssentialV1;
    type DataMarkerLongImperial = UnitsNamesVolumeLongImperialV1;
    type DataMarkerLongMetric = UnitsNamesVolumeLongMetricV1;
    type DataMarkerLongOutlier = UnitsNamesVolumeLongOutlierV1;
    type DataMarkerShortEssential = UnitsNamesVolumeShortEssentialV1;
    type DataMarkerShortImperial = UnitsNamesVolumeShortImperialV1;
    type DataMarkerShortMetric = UnitsNamesVolumeShortMetricV1;
    type DataMarkerShortOutlier = UnitsNamesVolumeShortOutlierV1;
    type DataMarkerNarrowEssential = UnitsNamesVolumeNarrowEssentialV1;
    type DataMarkerNarrowImperial = UnitsNamesVolumeNarrowImperialV1;
    type DataMarkerNarrowMetric = UnitsNamesVolumeNarrowMetricV1;
    type DataMarkerNarrowOutlier = UnitsNamesVolumeNarrowOutlierV1;
}
