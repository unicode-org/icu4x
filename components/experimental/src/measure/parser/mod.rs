// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod ids;
mod power;
mod si_prefix;

use crate::measure::measureunit::MeasureUnit;
use displaydoc::Display;
use ids::CLDR_IDS_TRIE;
use power::POWERS_TRIE;
use si_prefix::get_si_prefix;

use super::provider::si_prefix::{Base, SiPrefix};
use super::provider::single_unit::SingleUnit;
use super::single_unit_vec::SingleUnitVec;

#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[displaydoc("The unit is not valid")]
/// The unit is not valid.
/// This can occur if the unit ID does not adhere to the CLDR specification.
/// For example, `meter` is a valid unit ID, but `metre` is not.
#[non_exhaustive]
pub struct InvalidUnitError;

impl MeasureUnit {
    /// Parses a CLDR unit identifier and returns a [`MeasureUnit`].
    /// Examples include: `meter`, `foot`, `meter-per-second`, `meter-per-square-second`, `meter-per-square-second-per-second`, etc.
    /// Returns:
    ///    - `Ok(MeasureUnit)` if the identifier is valid.
    ///    - `Err(InvalidUnitError)` if the identifier is invalid.
    #[inline]
    pub fn try_from_str(s: &str) -> Result<MeasureUnit, InvalidUnitError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    pub fn try_from_utf8(code_units: &[u8]) -> Result<MeasureUnit, InvalidUnitError> {
        if code_units.starts_with(b"-") || code_units.ends_with(b"-") {
            return Err(InvalidUnitError);
        }

        let mut constant_denominator = 1;
        let mut single_units = SingleUnitVec::Empty;
        let mut power: i8 = 1;

        for part in code_units.split(|c| *c == b'-') {
            if part.is_empty() {
                return Err(InvalidUnitError);
            }

            if let Some(p) = POWERS_TRIE.get(part) {
                power = power.checked_mul(p as i8).ok_or(InvalidUnitError)?;
                continue;
            }

            if part == b"per" {
                if power == 1 {
                    power = -1;
                } else {
                    return Err(InvalidUnitError);
                }
                continue;
            }

            // special case: the whole part is a unit id without SI prefix.
            // We need to check this because we cannot strip kilo from kilogram
            // or deca from decade.
            if let Some(unit_id) = CLDR_IDS_TRIE.get(part) {
                single_units.push(SingleUnit {
                    power,
                    si_prefix: SiPrefix {
                        power: 0,
                        base: Base::Decimal,
                    },
                    unit_id: unit_id as u16,
                });
                power = 1;
                continue;
            }

            if let Some(c) = core::str::from_utf8(part)
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .and_then(|num| {
                    if num > u64::MAX as f64 {
                        None
                    } else {
                        Some(num as u64)
                    }
                })
            {
                if constant_denominator == 1 && power == -1 {
                    constant_denominator = c;
                } else {
                    return Err(InvalidUnitError);
                }
                continue;
            }

            let (si_prefix, rest) = get_si_prefix(part);
            let unit_id = CLDR_IDS_TRIE.get(rest).ok_or(InvalidUnitError)? as u16;

            single_units.push(SingleUnit {
                    power,
                    si_prefix,
                    unit_id,
                });
            power = 1;
        }

        // TODO: shall we allow units without any single units?
        // There is no unit without any valid single units.
        if single_units.as_slice().is_empty() {
            return Err(InvalidUnitError);
        }

        Ok(MeasureUnit {
            id: None,
            single_units,
            constant_denominator,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::measure::measureunit::MeasureUnit;

    #[test]
    fn test_parser_cases() {
        let test_cases = vec![
            ("meter-per-square-cubic-second", 2, 1),
            ("meter-per-square-second", 2, 1),
            ("portion-per-1e9", 1, 1_000_000_000),
            ("portion-per-1000000000", 1, 1_000_000_000),
            ("liter-per-100-kilometer", 2, 100),
        ];

        for (input, expected_len, expected_denominator) in test_cases {
            let measure_unit = MeasureUnit::try_from_str(input).unwrap();
            assert_eq!(measure_unit.single_units().len(), expected_len);
            assert_eq!(measure_unit.constant_denominator, expected_denominator);
        }
    }

    #[test]
    fn test_invlalid_unit_ids() {
        let test_cases = vec![
            "meter-per-square-100",
            "kilo",
            "kilokilo",
            "onekilo",
            "meterkilo",
            "meter-kilo",
            "k",
            "meter-",
            "meter+",
            "-meter",
            "+meter",
            "-kilometer",
            "+kilometer",
            "-pow2-meter",
            "+pow2-meter",
            "p2-meter",
            "p4-meter",
            "+",
            "-",
            "-mile",
            "-and-mile",
            "-per-mile",
            "one",
            "one-one",
            "one-per-mile",
            "one-per-cubic-centimeter",
            "square--per-meter",
            "metersecond", // Must have a compound part between single units
            // Negative powers not supported in mixed units yet. TODO(CLDR-13701).
            "per-hour-and-hertz",
            "hertz-and-per-hour",
            // Compound units not supported in mixed units yet. TODO(CLDR-13701).
            "kilonewton-meter-and-newton-meter",
            // Invalid units due to invalid constant denominator
            "meter-per--20-second",
            "meter-per-1000-1e9-second",
            "per-1000",
            "meter-per-1000-1000",
            "meter-per-1000-second-1000-kilometer",
            "1000-meter",
            "meter-1000",
            "meter-per-1000-1000",
            "meter-per-1000-second-1000-kilometer",
            "per-1000-and-per-1000",
            "liter-per-kilometer-100",
        ];

        for input in test_cases {
            let measure_unit = MeasureUnit::try_from_str(input);
            assert!(measure_unit.is_err(), "{input}");
        }
    }
}
