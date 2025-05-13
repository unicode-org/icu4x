// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::measure::measureunit::MeasureUnit;

pub trait MeasureUnitCategory {}

/// A [`MeasureUnit`] that is related to a specific category.
///
/// This is useful for type inference and for ensuring that the correct units are used.
pub struct CategorizedMeasureUnit<T: MeasureUnitCategory> {
    _category: PhantomData<T>,
    pub unit: MeasureUnit,
}

/// The categories of [`MeasureUnit`]s.
pub mod category {
    /// A [`super::MeasureUnit`] that is related to the length category.
    pub struct Length;

    /// A [`super::MeasureUnit`] that is related to the area category.
    pub struct Area;

    /// A [`super::MeasureUnit`] that is related to the volume category.
    pub struct Volume;

    /// A [`super::MeasureUnit`] that is related to the mass category.
    pub struct Mass;
}

impl category::Length {
    pub fn meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl category::Area {
    pub fn square_meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl category::Volume {
    pub fn cubic_meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl category::Mass {
    pub fn kilogram() -> &'static MeasureUnit {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_category() {
        let _meter = category::Length::meter();
    }

    #[test]
    fn test_area_category() {
        let _square_meter = category::Area::square_meter();
    }

    #[test]
    fn test_volume_category() {
        let _cubic_meter = category::Volume::cubic_meter();
    }

    #[test]
    fn test_mass_category() {
        let _kilogram = category::Mass::kilogram();
    }
}
