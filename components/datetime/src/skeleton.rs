// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

//! Skeletons are used for pattern matching. See the [`Skeleton`] struct for more information.

use std::convert::TryFrom;

use crate::{
    fields::{self, Field},
    pattern::Pattern,
    provider::gregory::{
        patterns::{SkeletonFieldsV1, SkeletonTupleV1},
        DatesV1,
    },
};

#[derive(Debug, PartialEq)]
struct FieldIndex(usize);

#[derive(Debug)]
pub enum SkeletonError {
    FieldOutOfOrder,
    DuplicateField,
}

/// A `Skeleton` is used to represent what types of `Field`s are present in a `Pattern`. The
/// ordering of the `Skeleton`'s `Field`s have no bearing on the ordering of the `Field`s and
/// `Literal`s in the `Pattern`.
///
/// A `Skeleton` is a `Vec<Field>`, but with the invariant that it is sorted according to the canonical
/// sort order. This order is sorted according to the most significant `Field` to the least significant.
/// For example, a field with a `Minute` symbol would preceed a field with a `Second` symbol.
/// This order is documented as the order of fields as presented in the
/// [UTS 35 Date Field Symbol Table](https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
///
/// The `Field`s are only sorted in the `Skeleton` in order to provide a deterministic
/// serialization strategy, and to provide a faster `Skeleton` matching operation.
///
/// The [`Copy`] trait is implemented as this is only a wrapper around a reference.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Skeleton<'a> {
    fields: &'a SkeletonFieldsV1,
}

impl<'a> Skeleton<'a> {
    fn fields_iter(&'a self) -> impl Iterator<Item = &Field> + 'a {
        self.fields.0.iter()
    }

    fn fields_len(&self) -> usize {
        self.fields.0.len()
    }
}

impl<'a> From<&'a SkeletonFieldsV1> for Skeleton<'a> {
    fn from(fields: &'a SkeletonFieldsV1) -> Self {
        Skeleton { fields }
    }
}

/// The `AvailableFormatPattern` represents a specific pattern that is available for a given locale.
/// A [`Skeleton`] is used to match against to find the best pattern.
///
/// This struct implements the [`Copy`] trait, as it's a collection of two references, which should
/// be fairly cheap to copy.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AvailableFormatPattern<'a> {
    /// The skeleton is used to match against.
    skeleton: Skeleton<'a>,
    pub pattern: &'a Pattern,
}

impl<'a> TryFrom<&'a SkeletonTupleV1> for AvailableFormatPattern<'a> {
    type Error = Error;
    fn try_from(tuple: &'a SkeletonTupleV1) -> Result<Self, Self::Error> {
        let SkeletonTupleV1(skeleton_fields, pattern_v1) = tuple;

        Ok(AvailableFormatPattern {
            skeleton: Skeleton::from(skeleton_fields),
            pattern: &pattern_v1.0,
        })
    }
}

#[derive(Debug)]
pub enum Error {
    FieldLengthTooLong,
    SymbolUnknown(char),
    SymbolInvalid(char),
    UnimplementedField(char),
    SkeletonFieldOutOfOrder,
    Skeleton(SkeletonError),
    Fields(fields::Error),
}

impl From<SkeletonError> for Error {
    fn from(skeleton_error: SkeletonError) -> Self {
        Error::Skeleton(skeleton_error)
    }
}

impl From<fields::Error> for Error {
    fn from(fields_error: fields::Error) -> Self {
        Error::Fields(fields_error)
    }
}

impl From<fields::LengthError> for Error {
    fn from(_: fields::LengthError) -> Self {
        Error::FieldLengthTooLong
    }
}

impl From<fields::SymbolError> for Error {
    fn from(symbol_error: fields::SymbolError) -> Self {
        match symbol_error {
            fields::SymbolError::Unknown(byte) => Error::SymbolUnknown(byte as char),
            fields::SymbolError::Invalid(ch) => Error::SymbolInvalid(ch),
        }
    }
}

// The following scalar values are for testing the suitability of a skeleton's field for the
// given input. Per UTS 35, the better the fit of a pattern, the "lower the distance". In this
// implementation each distance type is separated by an order of magnitiude. This magnitude needs
// to be at minimum a multiple of the max length of fields. As of CLDR 38 (2021-01), the max length
// of a skeleton in the "availableFormats" contained a total of 4 fields. The scores use a multiple
// of 10, as a number that will contain the range, and be easy to reason with.

const MAX_FIELDS: u16 = 10;

// Per the skeleton matching algorithm:
// https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons

// > 1. "Input skeleton symbols" are replaced with the best match for a given locale.
// >   - Hour: j → {H, k, h, K} + {a, b, B}
// >           J → {H, k, h, K}
// >           C → j + day period

// The components::Bag does not support step 1

// > 2. For fields with symbols representing the same type (year, month, day, etc):
// >   A. Most symbols have a small distance from each other.
// >     - Months: M ≅ L           (9 ≅ 9)  conjuction, vs standalone
// >       Week:   E ≅ c           (Tue ≅ 2)
// >       Period: a ≅ b ≅ B       (am. ≅ mid. ≅ at night)
// >       Hour:   H ≅ k ≅ h ≅ K   (23, 24, 12, 11)

const NO_DISTANCE: u16 = 0;

// B. Width differences among fields, other than those marking text vs numeric, are given small
// distance from each other.
// - MMM ≅ MMMM  (Sep ≅ September)
//   MM ≅ M      (09 ≅ 9)
const WIDTH_MISMATCH_DISTANCE: u16 = 1;

// C. Numeric and text fields are given a larger distance from each other.
// - MMM ≈ MM    (Sep ≈ 09)
//   MMM
const TEXT_VS_NUMERIC_DISTANCE: u16 = 10;

// D. Symbols representing substantial differences (week of year vs week of month) are given much
// larger a distances from each other.
// - d ≋ D;     (12 ≋ 345) Day of month vs Day of year
const SUBSTANTIAL_DIFFERENCES_DISTANCE: u16 = 100;

// Finally, missing symbols are the biggest distance.
const MISSING_SYMBOL_DISTANCE: u16 = 1000;

/// According to the [UTS 35 skeleton matching algorithm](https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons)
/// there will be a guaranteed match for a skeleton. However, with this initial implementation,
/// there is no attempt to add on missing fields. This enum encodes the variants for the current
/// search for a best skeleton.
#[derive(Debug)]
pub enum BestSkeleton<'a> {
    AllFieldsMatch(AvailableFormatPattern<'a>),
    MissingFields(AvailableFormatPattern<'a>),
    NoMatch,
}

/// A partial implementation of the [UTS 35 skeleton matching algorithm](https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons).
///
/// The following is implemented:
///
///  * Compute a score based on the best possible match for the given fields.
///  * Select the skeleton with highest score.
///
/// The following is not implemented:
///
///  * 2.6.2.1 Matching Skeletons
///    - TODO - Modify the resulting pattern to have fields of the same length. For example requesting
///      a skeleton "yMMMMd" can have a best match of ["yMMMd", "d MMM y"]. This pattern should
///      then be modified to use the requested length to produce a pattern "d MMMM y".
///      However, fields should not be changed from numeric to text.
///  * 2.6.2.2 Missing Skeleton Fields
///    - TODO - The mechanism to combine a date pattern and a time pattern.
///    - Using the CLDR appendItems field. Note: This step is currently not planned to be implemented
///      as there is some consensus within the ICU4X team that this leads to low-quality and
///      non-sensical results.
pub fn get_best_available_format_pattern<'a>(
    available_format_patterns: impl Iterator<Item = AvailableFormatPattern<'a>> + 'a,
    fields: &[Field],
) -> BestSkeleton<'a> {
    let mut closest_distance: u16 = u16::MAX;
    let mut closest_format_pattern = None;

    for available_format_pattern in available_format_patterns {
        let skeleton = &available_format_pattern.skeleton;
        debug_assert!(
            skeleton.fields_len() <= MAX_FIELDS as usize,
            "The distance mechanism assumes skeletons are less than MAX_FIELDS in length."
        );

        let mut distance: u16 = 0;
        // The distance should fit into a u16.
        #[cfg(test)]
        static_assertions::const_assert!(MAX_FIELDS * MISSING_SYMBOL_DISTANCE < u16::MAX);

        let mut requested_fields = fields.iter().peekable();
        let mut skeleton_fields = skeleton.fields_iter().peekable();
        loop {
            let next = (requested_fields.peek(), skeleton_fields.peek());

            // Try to find matching symbols.
            if let (Some(requested_field), Some(skeleton_field)) = next {
                // TODO - Should the canonical order be cached? It would use more more memory, and
                // be a slightly more complex implementation, but could save on performance. This
                // should be evaluated.
                let requested_order = requested_field.symbol.get_canonical_order();
                let skeleton_order = skeleton_field.symbol.get_canonical_order();

                if skeleton_order > requested_order {
                    // Keep searching for a matching skeleton field.
                    skeleton_fields.next();
                    continue;
                }

                if skeleton_order < requested_order {
                    // The requested field is missing from the skeleton.
                    distance += MISSING_SYMBOL_DISTANCE;
                    requested_fields.next();
                    continue;
                }

                distance += if requested_field == skeleton_field {
                    NO_DISTANCE
                } else if requested_field.symbol != skeleton_field.symbol {
                    SUBSTANTIAL_DIFFERENCES_DISTANCE
                } else if requested_field.get_length_type() != skeleton_field.get_length_type() {
                    TEXT_VS_NUMERIC_DISTANCE
                } else {
                    WIDTH_MISMATCH_DISTANCE
                };

                requested_fields.next();
                skeleton_fields.next();
            } else if next.0.is_some() {
                // There are still requested fields we haven't matched.
                distance += MISSING_SYMBOL_DISTANCE;
                requested_fields.next();
            } else {
                break;
            }
        }

        if distance < closest_distance {
            closest_format_pattern = Some(available_format_pattern);
            closest_distance = distance;
        }
    }

    match closest_format_pattern {
        Some(available_format_pattern) => {
            if closest_distance < MISSING_SYMBOL_DISTANCE {
                BestSkeleton::AllFieldsMatch(available_format_pattern)
            } else {
                BestSkeleton::MissingFields(available_format_pattern)
            }
        }
        None => BestSkeleton::NoMatch,
    }
}

pub fn get_available_format_patterns<'a>(
    data: &'a DatesV1,
) -> impl Iterator<Item = AvailableFormatPattern> + 'a {
    data.patterns
        .date_time
        .skeletons
        .iter()
        .map(AvailableFormatPattern::try_from)
        .map(|available_format_pattern| {
            available_format_pattern
                .expect("The provider data should only contain valid available format patterns.")
        })
}

#[cfg(all(test, feature = "provider_serde"))]
mod test {

    use icu_locid_macros::langid;
    use icu_provider::DataProvider;
    use icu_provider::{DataRequest, ResourceOptions, ResourcePath};
    use std::borrow::Cow;

    use crate::{
        options::components,
        provider::{gregory::DatesV1, key::GREGORY_V1},
    };

    use super::*;

    fn get_data_provider() -> Cow<'static, DatesV1> {
        let provider = icu_testdata::get_provider();
        let langid = langid!("en");
        provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap()
    }

    /// This is an initial smoke test to verify the skeleton machinery is working. For more in-depth
    /// testing see components/datetime/tests/fixtures/tests/components.json
    #[test]
    fn test_skeleton_matching() {
        let components = components::Bag::default();
        let requested_fields = components.to_vec_fields();
        let data_provider = get_data_provider();
        let available_format_patterns = get_available_format_patterns(&data_provider);

        match get_best_available_format_pattern(available_format_patterns, &requested_fields) {
            BestSkeleton::AllFieldsMatch(available_format_pattern)
            | BestSkeleton::MissingFields(available_format_pattern) => {
                assert_eq!(
                    String::from(available_format_pattern.pattern),
                    String::from("MMM d, y")
                )
            }
            BestSkeleton::NoMatch => {
                panic!("No skeleton was found.")
            }
        };
    }
}
