// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::si_prefix::SiPrefix;

/// Represents a single unit in a measure unit.
/// For example, the MeasureUnit `kilometer-per-square-second` contains two single units:
///    1. `kilometer` with power 1 and prefix 3 with base 10.
///    2. `second` with power -2 and prefix power equal to 0.
#[zerovec::make_ule(SingleUnitULE)]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::measure::provider::single_unit))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SingleUnit {
    /// The power of the unit.
    pub power: i8,

    /// The si base of the unit.
    pub si_prefix: SiPrefix,

    /// The id of the unit.
    pub unit_id: u16,
}

/// Returns a string representation of the `SingleUnit`.
///
/// The format of the string is as follows:
///  - The power is prefixed with "P" followed by the power value.
///  - The SI prefix is represented by its base character ('D' for Decimal, 'B' for Binary) followed by the prefix power value.
///  - The unit ID is prefixed with "I" and appended to the string.
///
/// NOTE:
///  - If the power is 1, the power is not included in the string representation.
///  - If the SI prefix power is 0, the SI prefix is not included in the string representation.
///
/// # Returns
/// A `String` representing the `SingleUnit` in the format described above.
///
/// # Examples
///
/// ```
/// use icu_experimental::measure::provider::single_unit::SingleUnit;
/// use icu_experimental::measure::provider::si_prefix::SiPrefix;
/// use icu_experimental::measure::provider::si_prefix::Base;
/// use icu_experimental::measure::parser::MeasureUnitParser;
///
///
/// let parser = MeasureUnitParser::new();
///
/// let full_unit = "meter";
/// let measure_unit = parser.try_from_str(full_unit).unwrap();
/// let single_unit = measure_unit.single_units().first().unwrap();
/// let string_representation = single_unit.to_string();
/// assert_eq!(string_representation, "I85", "{}", full_unit);
/// assert_eq!(full_unit.len() - string_representation.len(), 2, "{}", full_unit);
///
///
/// let full_unit = "square-millimeter";
/// let measure_unit = parser.try_from_str(full_unit).unwrap();
/// let single_unit = measure_unit.single_units().last().unwrap();
/// let string_representation = single_unit.to_string();
/// assert_eq!(string_representation, "P2D-3I85", "{}", full_unit);
/// assert_eq!(full_unit.len() - string_representation.len(), 9, "{}", full_unit);
///
/// ```
impl core::fmt::Display for SingleUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.power != 1 {
            write!(f, "P{}", self.power)?;
        }

        if self.si_prefix.power != 0 {
            write!(f, "{}", self.si_prefix)?;
        }

        write!(f, "I{}", self.unit_id)
    }
}
