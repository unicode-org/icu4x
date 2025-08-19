// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use icu_provider::DynamicDataMarker;

use crate::{
    dimension::provider::units::{
        categorized_display_name::{
            UnitsNameAreaCoreV1, UnitsNameDurationCoreV1, UnitsNameLengthCoreV1,
            UnitsNameMassCoreV1, UnitsNameVolumeCoreV1,
        },
        display_name::UnitsDisplayName,
    },
    measure::measureunit::MeasureUnit,
};

pub mod area;
pub mod duration;
pub mod length;
pub mod mass;
pub mod volume;

pub trait MeasureUnitCategory {
    type DataMarker: DynamicDataMarker<DataStruct = UnitsDisplayName<'static>>;
}

/// A [`MeasureUnit`] that is related to a specific category.
///
/// This is useful for type inference and for ensuring that the correct units are used.
pub struct CategorizedMeasureUnit<T: MeasureUnitCategory> {
    _category: PhantomData<T>,
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
pub struct Area;

/// A [`MeasureUnit`] that is related to the duration category.
pub struct Duration;

/// A [`MeasureUnit`] that is related to the length category.
pub struct Length;

/// A [`MeasureUnit`] that is related to the mass category.
pub struct Mass;

/// A [`MeasureUnit`] that is related to the volume category.
pub struct Volume;

impl MeasureUnitCategory for Area {
    type DataMarker = UnitsNameAreaCoreV1;
}
impl MeasureUnitCategory for Duration {
    type DataMarker = UnitsNameDurationCoreV1;
}
impl MeasureUnitCategory for Length {
    type DataMarker = UnitsNameLengthCoreV1;
}
impl MeasureUnitCategory for Mass {
    type DataMarker = UnitsNameMassCoreV1;
}
impl MeasureUnitCategory for Volume {
    type DataMarker = UnitsNameVolumeCoreV1;
}
