// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde::units::data::Patterns;
use crate::cldr_serde::units::info::Constant;
use crate::cldr_serde::units::preferences::{CategorizedUnitsList, Supplemental, UnitType};
use core::str::FromStr;
use icu::experimental::measure::measureunit::MeasureUnit;
use icu::experimental::measure::provider::single_unit::UnitID;
use icu::experimental::units::provider::{ConversionInfo, Exactness, Sign};
use icu::experimental::units::ratio::IcuRatio;
use icu::plurals::provider::PluralElementsPackedCow;
use icu::plurals::PluralElements;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::{DataError, DataErrorKind};
use num_traits::One;
use num_traits::Signed;
use std::collections::{BTreeMap, HashSet, VecDeque};
use zerovec::ZeroVec;

/// Represents a scientific number that contains only clean numerator and denominator terms.
/// NOTE:
///   clean means that there is no constant in the numerator or denominator.
///   For example, `["1.2"]` is clean, but `["1.2", "ft_to_m"]` is not clean.
pub(crate) struct ScientificNumber {
    /// Contains numerator terms that are represented as scientific numbers
    pub(crate) clean_num: Vec<String>,
    /// Contains denominator terms that are represented as scientific numbers
    pub(crate) clean_den: Vec<String>,

    /// Indicates if the constant is exact or approximate
    pub(crate) exactness: Exactness,
}

/// Represents a general constant which contains scientific and non scientific numbers.
#[derive(Debug)]
struct GeneralNonScientificNumber {
    /// Contains numerator terms that are represented as scientific numbers
    clean_num: Vec<String>,
    /// Contains denominator terms that are represented as scientific numbers
    clean_den: Vec<String>,
    /// Contains numerator terms that are not represented as scientific numbers
    non_scientific_num: VecDeque<String>,
    /// Contains denominator terms that are not represented as scientific numbers
    non_scientific_den: VecDeque<String>,
    /// Indicates if the constant is exact or approximate
    exactness: Exactness,
}

impl GeneralNonScientificNumber {
    fn new(num: &[String], den: &[String], exactness: Exactness) -> Self {
        let mut constant = GeneralNonScientificNumber {
            clean_num: Vec::new(),
            clean_den: Vec::new(),
            non_scientific_num: VecDeque::new(),
            non_scientific_den: VecDeque::new(),
            exactness,
        };

        for n in num {
            if is_scientific_number(n) {
                constant.clean_num.push(n.clone());
            } else {
                constant.non_scientific_num.push_back(n.clone());
            }
        }

        for d in den {
            if is_scientific_number(d) {
                constant.clean_den.push(d.clone());
            } else {
                constant.non_scientific_den.push_back(d.clone());
            }
        }
        constant
    }

    /// Determines if the constant is free of any `non_scientific` elements.
    fn is_free_of_non_scientific(&self) -> bool {
        self.non_scientific_num.is_empty() && self.non_scientific_den.is_empty()
    }
}

pub(crate) fn process_factor_part(
    factor_part: &str,
    cons_map: &BTreeMap<&str, ScientificNumber>,
) -> Result<ScientificNumber, DataError> {
    if factor_part.contains('/') {
        return Err(DataError::custom("the factor part is fractional number"));
    }

    let mut result = ScientificNumber {
        clean_num: Vec::new(),
        clean_den: Vec::new(),
        exactness: Exactness::Exact,
    };

    let factor_parts = factor_part.split('*');
    for factor in factor_parts {
        if let Some(cons) = cons_map.get(factor.trim()) {
            result.clean_num.extend(cons.clean_num.clone());
            result.clean_den.extend(cons.clean_den.clone());
            if cons.exactness == Exactness::Approximate {
                result.exactness = Exactness::Approximate;
            }
        } else {
            result.clean_num.push(factor.trim().to_string());
        }
    }

    Ok(result)
}

/// Processes a factor in the form of a string and returns a `ScientificNumber`.
/// Examples:
///     `1` is converted to `ScientificNumber { clean_num: ["1"], clean_den: ["1"], exactness: Exact }`
///     `3 * ft_to_m` is converted to `ScientificNumber { clean_num: ["3", "ft_to_m"], clean_den: ["1"], exactness: Exact }`
/// NOTE:
///    If one of the constants in the factor is approximate, the whole factor is approximate.
pub(crate) fn process_factor(
    factor: &str,
    cons_map: &BTreeMap<&str, ScientificNumber>,
) -> Result<ScientificNumber, DataError> {
    let mut factor_parts = factor.split('/');
    let factor_num_str = factor_parts.next().unwrap_or("0").trim();
    let factor_den_str = factor_parts.next().unwrap_or("1").trim();
    if factor_parts.next().is_some() {
        return Err(DataError::custom(
            "the factor is not a valid scientific notation number",
        ));
    }

    let mut result = process_factor_part(factor_num_str, cons_map)?;
    let factor_den_scientific = process_factor_part(factor_den_str, cons_map)?;

    result.clean_num.extend(factor_den_scientific.clean_den);
    result.clean_den.extend(factor_den_scientific.clean_num);
    if factor_den_scientific.exactness == Exactness::Approximate {
        result.exactness = Exactness::Approximate;
    }

    Ok(result)
}

/// Extracts the conversion info from a base unit, factor and offset.
pub(crate) fn extract_conversion_info<'data>(
    unit_id: UnitID,
    base_unit: &str,
    factor: &ScientificNumber,
    offset: &ScientificNumber,
) -> Result<ConversionInfo<'data>, DataError> {
    let factor_fraction = convert_slices_to_fraction(&factor.clean_num, &factor.clean_den)?;
    let offset_fraction = convert_slices_to_fraction(&offset.clean_num, &offset.clean_den)?;

    let (factor_num, factor_den, factor_sign) = flatten_fraction(factor_fraction);
    let (offset_num, offset_den, offset_sign) = flatten_fraction(offset_fraction);

    let exactness = if factor.exactness == Exactness::Exact && offset.exactness == Exactness::Exact
    {
        Exactness::Exact
    } else {
        Exactness::Approximate
    };

    let base_unit = MeasureUnit::try_from_str(base_unit)
        .map_err(|_| DataError::custom("the base unit is not valid"))?;

    Ok(ConversionInfo {
        unit_id,
        basic_units: ZeroVec::from_iter(base_unit.single_units().iter().copied()),
        factor_num: factor_num.into(),
        factor_den: factor_den.into(),
        factor_sign,
        offset_num: offset_num.into(),
        offset_den: offset_den.into(),
        offset_sign,
        exactness,
    })
}

/// Processes the constants and return them in a numerator-denominator form.
pub(crate) fn process_constants<'a>(
    constants: &'a BTreeMap<String, Constant>,
) -> Result<BTreeMap<&'a str, ScientificNumber>, DataError> {
    let mut constants_with_non_scientific =
        VecDeque::<(&'a str, GeneralNonScientificNumber)>::new();
    let mut clean_constants_map = BTreeMap::<&str, GeneralNonScientificNumber>::new();
    for (cons_name, cons_value) in constants {
        let (num, den) = split_unit_term(&cons_value.value)?;
        let exactness = match cons_value.status.as_deref() {
            Some("approximate") => Exactness::Approximate,
            _ => Exactness::Exact,
        };
        let constant = GeneralNonScientificNumber::new(&num, &den, exactness);
        if constant.is_free_of_non_scientific() {
            clean_constants_map.insert(cons_name, constant);
        } else {
            constants_with_non_scientific.push_back((cons_name, constant));
        }
    }
    let mut no_update_count = 0;
    while !constants_with_non_scientific.is_empty() {
        let mut updated = false;
        let (constant_key, mut non_scientific_constant) = constants_with_non_scientific
            .pop_front()
            .ok_or(DataError::custom(
                "non scientific queue error: an element must exist",
            ))?;
        for _ in 0..non_scientific_constant.non_scientific_num.len() {
            if let Some(num) = non_scientific_constant.non_scientific_num.pop_front() {
                if let Some(clean_constant) = clean_constants_map.get(num.as_str()) {
                    non_scientific_constant
                        .clean_num
                        .extend(clean_constant.clean_num.clone());
                    non_scientific_constant
                        .clean_den
                        .extend(clean_constant.clean_den.clone());
                    updated = true;
                } else {
                    non_scientific_constant.non_scientific_num.push_back(num);
                }
            }
        }
        for _ in 0..non_scientific_constant.non_scientific_den.len() {
            if let Some(den) = non_scientific_constant.non_scientific_den.pop_front() {
                if let Some(clean_constant) = clean_constants_map.get(den.as_str()) {
                    non_scientific_constant
                        .clean_num
                        .extend(clean_constant.clean_den.clone());
                    non_scientific_constant
                        .clean_den
                        .extend(clean_constant.clean_num.clone());
                    updated = true;
                } else {
                    non_scientific_constant.non_scientific_den.push_back(den);
                }
            }
        }
        if non_scientific_constant.is_free_of_non_scientific() {
            clean_constants_map.insert(constant_key, non_scientific_constant);
        } else {
            constants_with_non_scientific.push_back((constant_key, non_scientific_constant));
        }
        no_update_count = if !updated { no_update_count + 1 } else { 0 };
        if no_update_count > constants_with_non_scientific.len() {
            return Err(DataError::custom(
                "A loop was detected in the CLDR constants data!",
            ));
        }
    }

    Ok(clean_constants_map
        .into_iter()
        .map(|(k, v)| {
            (k, {
                ScientificNumber {
                    clean_num: v.clean_num,
                    clean_den: v.clean_den,
                    exactness: v.exactness,
                }
            })
        })
        .collect())
}

/// Determines if a string contains any alphabetic characters.
/// Returns true if the string contains at least one alphabetic character, false otherwise.
/// Examples:
/// - `1` returns false
/// - `ft_to_m` returns true
/// - `1E2` returns true
/// - `1.5E-2` returns true
pub(crate) fn contains_alphabetic_chars(s: &str) -> bool {
    s.chars().any(char::is_alphabetic)
}

#[test]
fn test_contains_alphabetic_chars() {
    let input = "1";
    let expected = false;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);

    let input = "ft_to_m";
    let expected = true;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);

    let input = "1E2";
    let expected = true;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);

    let input = "1.5E-2";
    let expected = true;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);
}

/// Checks if a string is a valid scientific notation number.
/// Returns true if the string is a valid scientific notation number, false otherwise.  
pub(crate) fn is_scientific_number(s: &str) -> bool {
    let mut parts = s.split('E');
    let base = parts.next().unwrap_or("0");
    let exponent = parts.next().unwrap_or("0");
    if parts.next().is_some() {
        return false;
    }

    !contains_alphabetic_chars(base) && !contains_alphabetic_chars(exponent)
}

/// Converts a fractional number into its byte representation for numerator and denominator, along with its sign.
pub(crate) fn flatten_fraction(fraction: IcuRatio) -> (Vec<u8>, Vec<u8>, Sign) {
    let fraction = fraction.get_ratio();
    let numer_bytes = fraction.numer().to_bytes_le().1;
    let denom_bytes = fraction.denom().to_bytes_le().1;
    let sign = if fraction.is_negative() {
        Sign::Negative
    } else {
        Sign::Positive
    };

    (numer_bytes, denom_bytes, sign)
}

/// Converts slices of numerator and denominator strings to a fraction.
/// Examples:
/// - ["1"], ["2"] is converted to 1/2
/// - ["1", "2"], ["3", "1E2"] is converted to 1*2/(3*1E2) --> 2/300
/// - ["1", "2"], ["3", "1E-2"] is converted to 1*2/(3*1E-2) --> 200/3
/// - ["1", "2"], ["3", "1E-2.5"] is an invalid scientific notation number
/// - ["1E2"], ["2"] is converted to 1E2/2 --> 100/2 --> 50/1
/// - ["1E2", "2"], ["3", "1E2"] is converted to 1E2*2/(3*1E2) --> 2/3
pub(crate) fn convert_slices_to_fraction(
    numerator_strings: &[String],
    denominator_strings: &[String],
) -> Result<IcuRatio, DataError> {
    numerator_strings
        .iter()
        .try_fold(IcuRatio::one(), |result, num| {
            IcuRatio::from_str(num.as_str())
                .map_err(|_| {
                    DataError::custom("The numerator is not a valid scientific notation number")
                })
                .map(|num_fraction| result * num_fraction)
        })
        .and_then(|num_product| {
            denominator_strings
                .iter()
                .try_fold(num_product, |result, den| {
                    IcuRatio::from_str(den.as_str())
                        .map_err(|_| {
                            DataError::custom(
                                "The denominator is not a valid scientific notation number",
                            )
                        })
                        .map(|den_fraction| result / den_fraction)
                })
        })
}

// TODO: move some of these tests to the comment above.
#[test]
fn test_convert_array_of_strings_to_fraction() {
    use num_bigint::BigInt;
    let numerator = vec!["1".to_string()];
    let denominator = vec!["2".to_string()];
    let expected = IcuRatio::from_big_ints(BigInt::from(1i32), BigInt::from(2i32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E2".to_string()];
    let expected = IcuRatio::from_big_ints(BigInt::from(2i32), BigInt::from(300i32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E-2".to_string()];
    let expected = IcuRatio::from_big_ints(BigInt::from(200i32), BigInt::from(3i32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E-2.5".to_string()];
    let actual = convert_slices_to_fraction(&numerator, &denominator);
    assert!(actual.is_err());
    let numerator = vec!["1E2".to_string()];
    let denominator = vec!["2".to_string()];
    let expected = IcuRatio::from_big_ints(BigInt::from(50i32), BigInt::from(1i32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1E2".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E2".to_string()];
    let expected = IcuRatio::from_big_ints(BigInt::from(2i32), BigInt::from(3i32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);
}

/// Splits a constant string into a tuple of (numerator, denominator).
/// The numerator and denominator are represented as arrays of strings.
/// Examples:
/// - `1/2` is split into `(["1"], ["2"])`
/// - `1 * 2 / 3 * ft_to_m` is split into `(["1", "2"], ["3" , "ft_to_m"])`
/// - `/2` is split into `(["1"], ["2"])`
/// - `2` is split into `(["2"], ["1"])`
/// - `2/` is split into `(["2"], ["1"])`
/// - `1E2` is split into `(["1E2"], ["1"])`
/// - `1 2 * 3` is an invalid constant string
pub(crate) fn split_unit_term(
    constant_string: &str,
) -> Result<(Vec<String>, Vec<String>), DataError> {
    let split: Vec<&str> = constant_string.split('/').collect();
    if split.len() > 2 {
        return Err(DataError::custom("Invalid constant string"));
    }

    // Define a closure to process each part of the split string
    let process_string = |s: &str| -> Vec<String> {
        if s.is_empty() {
            vec!["1".to_string()]
        } else {
            s.split('*').map(|s| s.trim().to_string()).collect()
        }
    };

    // Process the numerator and denominator parts
    let numerator_values = process_string(split.first().unwrap_or(&"1"));
    let denominator_values = process_string(split.get(1).unwrap_or(&"1"));

    // If any part contains internal white spaces, return an error
    if numerator_values
        .iter()
        .any(|s| s.chars().any(char::is_whitespace))
        || denominator_values
            .iter()
            .any(|s| s.chars().any(char::is_whitespace))
    {
        return Err(DataError::custom(
            "The constant string contains internal white spaces",
        ));
    }

    Ok((numerator_values, denominator_values))
}
// TODO: move this to the comment above.
#[test]
fn test_convert_constant_to_num_denom_strings() {
    let input = "1/2";
    let expected = (vec!["1".to_string()], vec!["2".to_string()]);
    let actual = split_unit_term(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1 * 2 / 3 * ft_to_m";
    let expected = (
        vec!["1".to_string(), "2".to_string()],
        vec!["3".to_string(), "ft_to_m".to_string()],
    );
    let actual = split_unit_term(input).unwrap();
    assert_eq!(expected, actual);

    let input = "/2";
    let expected = (vec!["1".to_string()], vec!["2".to_string()]);
    let actual = split_unit_term(input).unwrap();
    assert_eq!(expected, actual);

    let input = "2";
    let expected = (vec!["2".to_string()], vec!["1".to_string()]);
    let actual = split_unit_term(input).unwrap();
    assert_eq!(expected, actual);

    let input = "2/";
    let expected = (vec!["2".to_string()], vec!["1".to_string()]);
    let actual = split_unit_term(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1E2";
    let expected = (vec!["1E2".to_string()], vec!["1".to_string()]);
    let actual = split_unit_term(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1 2 * 3";
    let actual = split_unit_term(input);
    assert!(actual.is_err());
}

impl Patterns {
    pub(crate) fn try_into_plural_elements_packed_cow(
        &self,
    ) -> Result<PluralElementsPackedCow<'static, SinglePlaceholderPattern>, DataError> {
        let other_pattern = self.other.as_deref().ok_or_else(|| {
            DataErrorKind::IdentifierNotFound
                .into_error()
                .with_debug_context(self)
        })?;

        Ok(PluralElements::new(other_pattern)
            .with_zero_value(self.zero.as_deref())
            .with_one_value(self.one.as_deref())
            .with_two_value(self.two.as_deref())
            .with_few_value(self.few.as_deref())
            .with_many_value(self.many.as_deref())
            .with_explicit_one_value(self.explicit_one.as_deref())
            .with_explicit_zero_value(self.explicit_zero.as_deref())
            .into())
    }
}

impl Supplemental {
    /// Helper function to check if a target unit matches any unit in the set,
    /// including as a subunit in compound units (e.g., "foot" matches "foot-and-inch")
    fn unit_matches_any(target_unit: &str, units: &HashSet<String>) -> bool {
        units.iter().any(|u| u.contains(target_unit))
    }

    /// Returns a map from each category (e.g., "length", "mass", "duration") to a map from each region to a set of units.
    /// This excludes usages; it only checks if the unit is present for a given country in a specific category and region.
    /// NOTE:
    ///   The units can be present as compound units (e.g., "square-meter") or mixed units (e.g., "foot-and-inch").
    pub(crate) fn categorized_units_list(&self) -> CategorizedUnitsList {
        let mut result = BTreeMap::new();

        // Iterate through each category in the unit preference data
        for (category, usage_prefs) in &self.unit_preference_data {
            let mut region_units_map = BTreeMap::new();

            // For each usage in this category (e.g., "default", "person", etc.)
            for region_prefs in usage_prefs.values() {
                // For each region in this usage
                for (region_str, unit_prefs) in region_prefs {
                    // Get or create the units set for this region (using string key)
                    let units_set = region_units_map
                        .entry(region_str.clone())
                        .or_insert_with(HashSet::new);

                    // Add all units for this region (HashSet automatically handles duplicates)
                    for unit_pref in unit_prefs {
                        units_set.insert(unit_pref.unit.clone());
                    }
                }
            }

            result.insert(category.clone(), region_units_map);
        }

        result
    }

    /// Determines the type of unit based on the region and category.
    /// If the unit is present in the unit preferences and the region is present in the unit preferences, it is a core unit.
    /// If the unit is present in the unit preferences but not in the region preferences, it is an extended unit.
    /// If the unit is not present in the unit preferences, it is an outlier unit.
    ///
    /// If the specific region is not found, falls back to region "001" (world/default region).
    ///
    /// The matching includes subunit matching: a unit like "foot" will match compound units like "foot-and-inch".
    pub(crate) fn unit_type(
        category: &str,
        unit: &str,
        region: &str,
        categorized_units_list: &CategorizedUnitsList,
    ) -> UnitType {
        let region_units = match categorized_units_list.get(category) {
            Some(units) => units,
            None => return UnitType::Outlier,
        };

        let found_in_region = region_units
            .get(region)
            .or_else(|| region_units.get("001"))
            .is_some_and(|units| Self::unit_matches_any(unit, units));

        let found_anywhere = region_units
            .values()
            .any(|units| Self::unit_matches_any(unit, units));

        if found_in_region {
            UnitType::Core
        } else if found_anywhere {
            UnitType::Extended
        } else {
            UnitType::Outlier
        }
    }
}
