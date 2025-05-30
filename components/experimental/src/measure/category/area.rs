// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::measure::measureunit::MeasureUnit;
use crate::measure::provider::si_prefix::{Base, SiPrefix};
use crate::measure::provider::single_unit::SingleUnit;
use alloc::vec;

use crate::measure::category::category;

impl category::Area {
    #[cfg(feature = "compiled_data")]
    /// Returns a [`MeasureUnit`] representing the area of one square meter.
    pub fn square_meter() -> MeasureUnit {
        MeasureUnit {
            single_units: vec![SingleUnit {
                power: 2,
                si_prefix: SiPrefix {
                    power: 0,
                    base: Base::Decimal,
                },
                unit_id: crate::provider::Baked::UNIT_IDS_V1_UND_METER,
            }],
            constant_denominator: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_category() {
        let _square_meter = category::Area::square_meter();
    }
}
