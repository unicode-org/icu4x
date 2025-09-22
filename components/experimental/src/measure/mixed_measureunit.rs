// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::measure::{measureunit::MeasureUnit, parser::InvalidUnitError};

use super::single_unit_vec::SingleUnitVec;

/// The [`MixedMeasureUnit`] struct represents a CLDR mixed unit,
/// which is a combination of one or more single units used together to express a measurement.
///
/// # Examples
/// - `foot-and-inch`
/// - `kilometer-and-meter`
///
/// Note: Compound units such as `meter-per-second` or units with a constant denominator are not supported in mixed units.
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct MixedMeasureUnit {
    /// Contains the single mixed units.
    pub(crate) mixed_units: SingleUnitVec,
}

impl MixedMeasureUnit {
    /// Returns a slice of the mixed units contained within this mixed unit.
    pub fn try_from_str(mixed_units_str: &str) -> Result<MixedMeasureUnit, InvalidUnitError> {
        let mixed_units_strs = mixed_units_str.split("-and-");
        let mut mixed_units = SingleUnitVec::Zero;
        for unit in mixed_units_strs {
            let unit = MeasureUnit::try_from_str(unit)?;
            let internal_single_units = unit.single_units();
            if internal_single_units.len() > 1 {
                return Err(InvalidUnitError);
            }
            if unit.constant_denominator != 0 {
                return Err(InvalidUnitError);
            }
            let single_unit = match internal_single_units.first() {
                Some(single_unit) => single_unit,
                None => return Err(InvalidUnitError),
            };
            mixed_units.push(*single_unit);
        }
        Ok(MixedMeasureUnit { mixed_units })
    }
}

#[cfg(test)]
mod tests {
    use crate::measure::provider::{
        si_prefix::{Base, SiPrefix},
        single_unit::SingleUnit,
    };

    use super::*;

    #[test]
    fn test_mixed_measure_unit_from_str() {
        // Meter
        let mixed_measure_unit = MixedMeasureUnit::try_from_str("meter").unwrap();
        assert_eq!(mixed_measure_unit.mixed_units.as_slice().len(), 1);
        assert_eq!(
            mixed_measure_unit.mixed_units.as_slice()[0],
            SingleUnit {
                power: 1,
                si_prefix: SiPrefix {
                    power: 0,
                    base: Base::Decimal,
                },
                unit_id: *crate::provider::Baked::UNIT_IDS_V1_UND_METER,
            }
        );

        // Foot and Inch
        let mixed_measure_unit = MixedMeasureUnit::try_from_str("foot-and-inch").unwrap();
        assert_eq!(mixed_measure_unit.mixed_units.as_slice().len(), 2);
        assert_eq!(
            mixed_measure_unit.mixed_units.as_slice()[0],
            SingleUnit {
                power: 1,
                si_prefix: SiPrefix {
                    power: 0,
                    base: Base::Decimal,
                },
                unit_id: *crate::provider::Baked::UNIT_IDS_V1_UND_FOOT,
            }
        );
        assert_eq!(
            mixed_measure_unit.mixed_units.as_slice()[1],
            SingleUnit {
                power: 1,
                si_prefix: SiPrefix {
                    power: 0,
                    base: Base::Decimal,
                },
                unit_id: *crate::provider::Baked::UNIT_IDS_V1_UND_INCH,
            }
        );

        // Kilometer and Meter
        let mixed_measure_unit = MixedMeasureUnit::try_from_str("kilometer-and-meter").unwrap();
        assert_eq!(mixed_measure_unit.mixed_units.as_slice().len(), 2);
        assert_eq!(
            mixed_measure_unit.mixed_units.as_slice()[0],
            SingleUnit {
                power: 1,
                si_prefix: SiPrefix {
                    power: 3,
                    base: Base::Decimal,
                },
                unit_id: *crate::provider::Baked::UNIT_IDS_V1_UND_METER,
            }
        );
        assert_eq!(
            mixed_measure_unit.mixed_units.as_slice()[1],
            SingleUnit {
                power: 1,
                si_prefix: SiPrefix {
                    power: 0,
                    base: Base::Decimal,
                },
                unit_id: *crate::provider::Baked::UNIT_IDS_V1_UND_METER,
            }
        );
    }

    #[test]
    fn test_invalid_mixed_measure_unit_from_str() {
        let mixed_measure_unit =
            MixedMeasureUnit::try_from_str("meter-per-second-and-mile-per-hour");
        assert!(mixed_measure_unit.is_err());
    }
}
