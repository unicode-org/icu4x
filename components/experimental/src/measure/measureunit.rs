// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{provider::single_unit::SingleUnit, single_unit_vec::SingleUnitVec};
use alloc::string::String;
use alloc::string::ToString;

// TODO NOTE: the MeasureUnitParser takes the trie and the ConverterFactory takes the full payload and an instance of MeasureUnitParser.
/// The [`MeasureUnit`] struct represents a processed CLDR compound unit.
/// Examples include:
///  1. `meter-per-second`
///  2. `square-meter`
///  3. `liter-per-100-kilometer`
///  4. `portion-per-1e9`
///  5. `square-meter` (Note: a single unit is a special case of a compound unit containing only one single unit.)
///
/// To construct a [`MeasureUnit`] from a CLDR unit identifier, use the [`crate::measure::parser::MeasureUnitParser`].
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MeasureUnit {
    /// Contains the processed units.
    pub(crate) single_units: SingleUnitVec,

    /// Represents the constant denominator of this measure unit.
    ///
    /// Examples:
    ///   - For the unit `meter-per-second`, the constant denominator is `0`, because there is no denominator.
    ///   - For the unit `liter-per-100-kilometer`, the constant denominator is `100`.
    ///   - For the unit `portion-per-1e9`, the constant denominator is `1_000_000_000`.
    ///
    /// NOTE:
    ///   If the constant denominator is not set, the value defaults to `0`.
    pub(crate) constant_denominator: u64,
}

impl MeasureUnit {
    /// Returns a slice of the single units contained within this measure unit.
    pub fn single_units(&self) -> &[SingleUnit] {
        self.single_units.as_slice()
    }

    /// Returns the constant denominator of this measure unit.
    ///
    /// NOTE:
    ///   If the constant denominator is not set, a value of `0` is returned.
    pub fn constant_denominator(&self) -> u64 {
        self.constant_denominator
    }

    /// Returns a short representation of this measure unit as follows:
    /// 1. Each single unit will be represented by its short representation.
    /// 2. The constant denominator will be represented by its value prefixed with `C`.
    ///    2.1 If the constant denominator is greater than or equal to 1000 and has more than 3 trailing zeros, it will be represented in scientific notation.
    /// 3. The division (per) will be represented by the `R` symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_experimental::measure::parser::MeasureUnitParser;
    /// use icu_experimental::measure::measureunit::MeasureUnit;
    ///
    ///
    /// let parser = MeasureUnitParser::new();
    ///
    /// let full_unit = "meter";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "I85", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 2, "{}", full_unit);
    ///
    /// let full_unit = "foot";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "I50", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 1);
    ///
    /// let full_unit = "inch";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "I66", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 1);
    ///
    /// let full_unit = "square-meter";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "P2I85", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 7, "{}", full_unit);
    ///
    /// let full_unit = "square-millimeter";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "P2D-3I85", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 9, "{}", full_unit);
    ///
    /// let full_unit = "micrometer";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "D-6I85", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 4, "{}", full_unit);
    ///
    /// let full_unit = "meter-per-second";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "I85#R#P-1I127", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 3, "{}", full_unit);
    ///
    /// let full_unit = "liter-per-100-kilometer";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "I82#R#C100#P-1D3I85", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 4, "{}", full_unit);
    ///
    /// let full_unit = "portion-per-1e9";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "I113#R#C1E9", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 4, "{}", full_unit);
    ///
    /// let full_unit = "per-10000000000-portion";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "R#C1E10#P-1I113", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 8, "{}", full_unit);
    ///
    /// let full_unit = "liter-per-240000000000-kilometer";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "I82#R#C24E10#P-1D3I85", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 11, "{}", full_unit);
    ///
    /// let full_unit = "millimeter-per-square-microsecond";
    /// let measure_unit = parser.try_from_str(full_unit).unwrap();
    /// let short_representation = measure_unit.generate_short_representation();
    /// assert_eq!(short_representation, "D-3I85#R#P-2D-6I127", "{}", full_unit);
    /// assert_eq!(full_unit.len() - short_representation.len(), 14, "{}", full_unit);
    ///
    /// ```
    pub fn generate_short_representation(&self) -> String {
        // Convert the constant to scientific notation if it is a power of 10 with more than 3 trailing zeros
        fn power_of_10_to_scientific(n: u64) -> String {
            if n < 1000 {
                return n.to_string();
            }

            let result = n.to_string();
            let zeros_count = result.chars().rev().take_while(|&c| c == '0').count();

            if zeros_count > 3 {
                let significant_digits = &result.split_at(result.len() - zeros_count).0;
                return [
                    significant_digits.to_string(),
                    "E".to_string(),
                    zeros_count.to_string(),
                ]
                .concat();
            }

            result
        }

        // Split the single units into two groups based on their SI prefix power:
        // 1. Single units with a non-negative SI prefix power (positive or zero).
        // 2. Single units with a negative SI prefix power.
        // Since single units with non-negative powers are expected to appear first,
        // we can efficiently partition them by finding the first unit with a negative power.
        let split_index = self
            .single_units
            .as_slice()
            .iter()
            .position(|unit| unit.power < 0)
            .unwrap_or_else(|| self.single_units.as_slice().len());
        let (positive_prefix_units, negative_prefix_units) =
            self.single_units.as_slice().split_at(split_index);

        // Generate the first part of the short representation
        let positive_part =
            positive_prefix_units
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (index, unit)| {
                    if index < positive_prefix_units.len() - 1 {
                        acc + &unit.to_string() + "#"
                    } else {
                        acc + &unit.to_string()
                    }
                });

        // Generate the second part of the short representation
        let mut negative_part =
            negative_prefix_units
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (index, unit)| {
                    if index < negative_prefix_units.len() - 1 {
                        acc + &unit.to_string() + "#"
                    } else {
                        acc + &unit.to_string()
                    }
                });

        // add constant part to the negative part
        if self.constant_denominator > 0 {
            if !negative_part.is_empty() {
                negative_part.insert(0, '#');
            }
            negative_part.insert_str(0, &power_of_10_to_scientific(self.constant_denominator));
            negative_part.insert(0, 'C');
        }

        // Glue the positive and negative parts together to get the final short representation
        let mut short_representation = positive_part;
        if !short_representation.is_empty() && !negative_part.is_empty() {
            short_representation.push_str("#R#"); // --> -per- representation
        } else if !negative_part.is_empty() {
            short_representation.push_str("R#"); // --> per- representation
        }

        short_representation.push_str(&negative_part);
        short_representation
    }
}
