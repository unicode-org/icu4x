// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::measure::measureunit::MeasureUnit;

pub trait MeasureUnitCategory {
    fn category() -> Categories;
}

/// A [`MeasureUnit`] that is related to a specific category.
///
/// This is useful for type inference and for ensuring that the correct units are used.
pub struct CategorizedMeasureUnit<T: MeasureUnitCategory> {
    pub category: T,
    pub unit: MeasureUnit,
}

/// The categories of [`MeasureUnit`]s.
pub enum Categories {
    Length,
    Area,
    Volume,
    Mass,
    // TODO: add more categories in the next PRs.
}

/// A [`MeasureUnit`] that is related to the length category.
pub struct LengthMeasureUnit;

/// A [`MeasureUnit`] that is related to the area category.
pub struct AreaMeasureUnit;

/// A [`MeasureUnit`] that is related to the volume category.
pub struct VolumeMeasureUnit;

/// A [`MeasureUnit`] that is related to the mass category.
pub struct MassMeasureUnit;

impl MeasureUnitCategory for LengthMeasureUnit {
    fn category() -> Categories {
        Categories::Length
    }
}

impl LengthMeasureUnit {
    pub fn meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl MeasureUnitCategory for AreaMeasureUnit {
    fn category() -> Categories {
        Categories::Area
    }
}

impl AreaMeasureUnit {
    pub fn square_meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl MeasureUnitCategory for VolumeMeasureUnit {
    fn category() -> Categories {
        Categories::Volume
    }
}

impl VolumeMeasureUnit {
    pub fn cubic_meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl MeasureUnitCategory for MassMeasureUnit {
    fn category() -> Categories {
        Categories::Mass
    }
}

impl MassMeasureUnit {
    pub fn kilogram() -> &'static MeasureUnit {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_category() {
        let meter = LengthMeasureUnit::meter();
    }

    #[test]
    fn test_area_category() {
        let square_meter = AreaMeasureUnit::square_meter();
    }

    #[test]
    fn test_volume_category() {
        let cubic_meter = VolumeMeasureUnit::cubic_meter();
    }

    #[test]
    fn test_mass_category() {
        let kilogram = MassMeasureUnit::kilogram();
    }
}
