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

impl MeasureUnitCategory for Categories::Length {
    fn category() -> Categories {
        Categories::Length
    }
}

impl Categories::Length {
    pub fn meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl MeasureUnitCategory for Categories::Area {
    fn category() -> Categories {
        Categories::Area
    }
}

impl Categories::Area {
    pub fn square_meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl MeasureUnitCategory for Categories::Volume {
    fn category() -> Categories {
        Categories::Volume
    }
}

impl Categories::Volume {
    pub fn cubic_meter() -> &'static MeasureUnit {
        todo!()
    }
}

impl MeasureUnitCategory for Categories::Mass {
    fn category() -> Categories {
        Categories::Mass
    }
}

impl Categories::Mass {
    pub fn kilogram() -> &'static MeasureUnit {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_category() {
        let meter = Categories::Length::meter();
    }

    #[test]
    fn test_area_category() {
        let square_meter = Categories::Area::square_meter();
    }

    #[test]
    fn test_volume_category() {
        let cubic_meter = Categories::Volume::cubic_meter();
    }

    #[test]
    fn test_mass_category() {
        let kilogram = Categories::Mass::kilogram();
    }
}
