// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "compiled_data")]
use crate::measure::{
    category::{Area, CategorizedMeasureUnit},
    measureunit::MeasureUnit,
    provider::{
        si_prefix::{Base, SiPrefix},
        single_unit::SingleUnit,
    },
    single_unit_vec::SingleUnitVec,
};

#[cfg(feature = "compiled_data")]
impl Area {
    /// Returns a [`MeasureUnit`] representing the area of one square meter.
    pub fn square_meter() -> CategorizedMeasureUnit<Area> {
        CategorizedMeasureUnit {
            _category: core::marker::PhantomData,
            unit: MeasureUnit {
                single_units: SingleUnitVec::One(SingleUnit {
                    power: 2,
                    si_prefix: SiPrefix {
                        power: 0,
                        base: Base::Decimal,
                    },
                    unit_id: crate::provider::Baked::UNIT_IDS_V1_UND_METER,
                }),
                constant_denominator: 0,
            },
        }
    }
}

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use crate::measure::parser::MeasureUnitParser;

    #[test]
    fn test_area_category() {
        let parser = MeasureUnitParser::default();
        let square_meter = Area::square_meter();
        let square_meter_parsed = parser.try_from_str("square-meter").unwrap();
        assert_eq!(square_meter.unit, square_meter_parsed);
    }
}
