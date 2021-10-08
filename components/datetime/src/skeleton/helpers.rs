// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;

use crate::{
    fields::{self, Field, FieldLength, FieldSymbol},
    options::{components, length},
    pattern::{hour_cycle, reference::Pattern, PatternItem},
    provider::gregory::{
        patterns::{LengthPatternsV1, PatternV1},
        DateSkeletonPatternsV1,
    },
};

// The following scalar values are for testing the suitability of a skeleton's field for the
// given input. Per UTS 35, the better the fit of a pattern, the "lower the distance". In this
// implementation each distance type is separated by an order of magnitiude. This magnitude needs
// to be at minimum a multiple of the max length of fields. As of CLDR 38 (2021-01), the max length
// of a skeleton in the "availableFormats" contained a total of 4 fields. The scores use a multiple
// of 10, as a number that will contain the range, and be easy to reason with.
//
// The only exception is on the largest magnitude of values (MISSING_OR_SKELETON_EXTRA_SYMBOL). The
// missing or extra count BOTH the requested fields and skeleton fields. This is fine since there
// is no higher magnitude.

const MAX_SKELETON_FIELDS: u32 = 10;

// Per the skeleton matching algorithm:
// https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons

// > 1. "Input skeleton symbols" are replaced with the best match for a given locale.
// >   - Hour: j → {H, k, h, K} + {a, b, B}
// >           J → {H, k, h, K}
// >           C → j + day period

// The components::Bag does not support step 1

// > 2. For fields with symbols representing the same type (year, month, day, etc):
// >   A. Most symbols have a small distance from each other.
// >     - Months: M ≅ L           (9 ≅ 9)  conjuction, vs stand-alone
// >       Week:   E ≅ c           (Tue ≅ 2)
// >       Period: a ≅ b ≅ B       (am. ≅ mid. ≅ at night)
// >       Hour:   H ≅ k ≅ h ≅ K   (23, 24, 12, 11)

// For step 2, the components::Bag will not produce "stand-alone" months, as no skeletons
// contain stand-alone months.

const NO_DISTANCE: u32 = 0;

// B. Width differences among fields, other than those marking text vs numeric, are given small
// distance from each other.
// - MMM ≅ MMMM  (Sep ≅ September)
//   MM ≅ M      (09 ≅ 9)
const WIDTH_MISMATCH_DISTANCE: u32 = 1;

// C. Numeric and text fields are given a larger distance from each other.
// - MMM ≈ MM    (Sep ≈ 09)
//   MMM
const TEXT_VS_NUMERIC_DISTANCE: u32 = 10;

// D. Symbols representing substantial differences (week of year vs week of month) are given much
// larger a distances from each other.
// - d ≋ D;     (12 ≋ 345) Day of month vs Day of year
const SUBSTANTIAL_DIFFERENCES_DISTANCE: u32 = 100;

// A skeleton had more symbols than what was requested.
const SKELETON_EXTRA_SYMBOL: u32 = 1000;

// A requested symbol is missing in the skeleton. Note that this final value can be more than
// MAX_SKELETON_FIELDS, as it's counting the missing requested fields, which can be longer than
// the stored skeletons. There cannot be any cases higher than this one.
const REQUESTED_SYMBOL_MISSING: u32 = 10000;

/// According to the [UTS 35 skeleton matching algorithm](https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons)
/// there will be a guaranteed match for a skeleton. However, with this initial implementation,
/// there is no attempt to add on missing fields. This enum encodes the variants for the current
/// search for a best skeleton.
#[derive(Debug, PartialEq, Clone)]
pub enum BestSkeleton<T> {
    AllFieldsMatch(T),
    MissingOrExtraFields(T),
    NoMatch,
}

/// This function swaps out the the time zone name field for the appropriate one. Skeleton matching
/// only needs to find a single "v" field, and then the time zone name can expand from there.
fn naively_apply_time_zone_name(
    pattern: &mut PatternV1,
    time_zone_name: &Option<components::TimeZoneName>,
) {
    // If there is a preference overiding the hour cycle, apply it now.
    if let Some(time_zone_name) = time_zone_name {
        for item in pattern.0.items_mut() {
            if let PatternItem::Field(fields::Field {
                symbol: fields::FieldSymbol::TimeZone(_),
                length: _,
            }) = item
            {
                *item = PatternItem::Field((*time_zone_name).into());
            }
        }
    }
}

// TODO - This could return a Cow<'a, Pattern>, but it affects every other part of the API to
// add a lifetime here. The pattern returned here could be one that we've already constructed in
// the CLDR as an exotic type, or it could be one that was modified to meet the requirements of
// the components bag.

/// Given a set of fields (which represents a skeleton), try to create a best localized pattern
// for those fields.
///
/// * `skeletons` - The skeletons that will be matched against
/// * `length_patterns` - Contains information on how to combine date and time patterns.
/// * `fields` - The desired fields to match against.
/// * `prefer_matched_pattern` - This algorithm does some extra steps of trying to respect
///         the desired fields, even if the provider data doesn't completely match. This
///         configuration option makes it so that the final pattern won't have additional work
///         done to mutate it to match the fields. It will prefer the actual matched pattern.
pub fn create_best_pattern_for_fields<'a>(
    skeletons: &'a DateSkeletonPatternsV1,
    length_patterns: &LengthPatternsV1,
    fields: &[Field],
    components: &components::Bag,
    prefer_matched_pattern: bool,
) -> BestSkeleton<PatternV1> {
    let first_pattern_match =
        get_best_available_format_pattern(skeletons, fields, prefer_matched_pattern);

    // Try to match a skeleton to all of the fields.
    if let BestSkeleton::AllFieldsMatch(mut pattern) = first_pattern_match {
        hour_cycle::naively_apply_preferences(&mut pattern.0, &components.preferences);
        naively_apply_time_zone_name(&mut pattern, &components.time_zone_name);
        return BestSkeleton::AllFieldsMatch(pattern);
    }

    let FieldsByType { date, time } = group_fields_by_type(fields);

    if date.is_empty() || time.is_empty() {
        return match first_pattern_match {
            BestSkeleton::AllFieldsMatch(_) => {
                unreachable!("Logic error in implementation. AllFieldsMatch handled above.")
            }
            BestSkeleton::MissingOrExtraFields(mut pattern) => {
                if date.is_empty() {
                    hour_cycle::naively_apply_preferences(&mut pattern.0, &components.preferences);
                    naively_apply_time_zone_name(&mut pattern, &components.time_zone_name);
                }
                BestSkeleton::MissingOrExtraFields(pattern)
            }
            BestSkeleton::NoMatch => BestSkeleton::NoMatch,
        };
    }

    // Match the date and time, and then simplify the combinatorial logic of the results into
    // an optional values of the results, and a boolean value.
    let (date_pattern, date_missing_or_extra) =
        match get_best_available_format_pattern(skeletons, &date, prefer_matched_pattern) {
            BestSkeleton::MissingOrExtraFields(fields) => (Some(fields), true),
            BestSkeleton::AllFieldsMatch(fields) => (Some(fields), false),
            BestSkeleton::NoMatch => (None, true),
        };

    let (mut time_pattern, time_missing_or_extra) =
        match get_best_available_format_pattern(skeletons, &time, prefer_matched_pattern) {
            BestSkeleton::MissingOrExtraFields(fields) => (Some(fields), true),
            BestSkeleton::AllFieldsMatch(fields) => (Some(fields), false),
            BestSkeleton::NoMatch => (None, true),
        };

    if let Some(ref mut pattern) = time_pattern {
        hour_cycle::naively_apply_preferences(&mut pattern.0, &components.preferences);
        naively_apply_time_zone_name(pattern, &components.time_zone_name);
    }

    // Determine how to combine the date and time.
    let pattern: Option<PatternV1> = match (date_pattern, time_pattern) {
        (Some(date_pattern), Some(time_pattern)) => {
            let month_field = fields
                .iter()
                .find(|f| matches!(f.symbol, FieldSymbol::Month(_)));

            // Per UTS-35, choose a "length" pattern for combining the date and time.
            // https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons
            //
            // 1. If the requested date fields include Wide month and weekday name of any length, use length::Date::Full
            // 2. Otherwise, if the requested date fields include wide month, use length::Date::Long
            // 3. Otherwise, if the requested date fields include abbreviated month, use length::Date::Medium
            // 4. Otherwise use length::Date::Short
            let length = match month_field {
                Some(field) => match field.length {
                    FieldLength::Wide => {
                        let weekday = fields
                            .iter()
                            .find(|f| matches!(f.symbol, FieldSymbol::Weekday(_)));

                        if weekday.is_some() {
                            length::Date::Full
                        } else {
                            length::Date::Long
                        }
                    }
                    FieldLength::Abbreviated => length::Date::Medium,
                    _ => length::Date::Short,
                },
                None => length::Date::Short,
            };

            let bytes = match length {
                length::Date::Full => &length_patterns.full,
                length::Date::Long => &length_patterns.long,
                length::Date::Medium => &length_patterns.medium,
                length::Date::Short => &length_patterns.short,
            };

            Some(
                Pattern::from_bytes_combination(bytes, date_pattern.0, time_pattern.0)
                    .expect("Failed to create a Pattern from bytes")
                    .into(),
            )
        }
        (None, Some(pattern)) => Some(pattern),
        (Some(pattern), None) => Some(pattern),
        (None, None) => None,
    };

    match pattern {
        Some(pattern) => {
            if date_missing_or_extra || time_missing_or_extra {
                BestSkeleton::MissingOrExtraFields(pattern)
            } else {
                BestSkeleton::AllFieldsMatch(pattern)
            }
        }
        None => BestSkeleton::NoMatch,
    }
}

struct FieldsByType {
    pub date: Vec<Field>,
    pub time: Vec<Field>,
}

fn group_fields_by_type(fields: &[Field]) -> FieldsByType {
    let mut date = Vec::new();
    let mut time = Vec::new();

    for field in fields {
        match field.symbol {
            // Date components:
            // Note: Weekdays are included in both time and date skeletons.
            //  - Time examples: "EBhm" "EBhms" "Ed" "Ehm" "EHm" "Ehms" "EHms"
            //  - Date examples: "GyMMMEd" "MEd" "MMMEd" "MMMMEd" "yMEd" "yMMMEd"
            //  - Solo example: "E"
            FieldSymbol::Year(_)
            | FieldSymbol::Month(_)
            | FieldSymbol::Day(_)
            | FieldSymbol::Weekday(_) => date.push(*field),

            // Time components:
            FieldSymbol::DayPeriod(_)
            | FieldSymbol::Hour(_)
            | FieldSymbol::Minute
            | FieldSymbol::Second(_)
            | FieldSymbol::TimeZone(_) => time.push(*field),
            // Other components
            // TODO(#486)
            // FieldSymbol::Era(_) => other.push(*field),
            // Plus others...
        };
    }

    FieldsByType { date, time }
}

/// A partial implementation of the [UTS 35 skeleton matching algorithm](https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons).
///
/// The following is implemented:
///
///  * Compute a score based on the best possible match for the given fields.
///  * Select the skeleton with highest score.
///  * Modify the resulting pattern to have fields of the same length. For example requesting
///      a skeleton "yMMMMd" can have a best match of ["yMMMd", "d MMM y"]. This pattern should
///      then be modified to use the requested length to produce a pattern "d MMMM y".
///      However, fields should not be changed from numeric to text.
///
/// The following is not implemented:
///
///  * 2.6.2.2 Missing Skeleton Fields
///    - TODO(#586) - Using the CLDR appendItems field. Note: There is not agreement yet on how
///      much of this step to implement. See the issue for more information.
pub fn get_best_available_format_pattern(
    skeletons: &DateSkeletonPatternsV1,
    fields: &[Field],
    prefer_matched_pattern: bool,
) -> BestSkeleton<PatternV1> {
    let mut closest_format_pattern = None;
    let mut closest_distance: u32 = u32::MAX;
    let mut closest_missing_fields = 0;

    for (skeleton, pattern) in skeletons.0.iter() {
        debug_assert!(
            skeleton.0.fields_len() <= MAX_SKELETON_FIELDS as usize,
            "The distance mechanism assumes skeletons are less than MAX_SKELETON_FIELDS in length."
        );
        let mut missing_fields = 0;
        let mut distance: u32 = 0;
        // The distance should fit into a u32.

        let mut requested_fields = fields.iter().peekable();
        let mut skeleton_fields = skeleton.0.fields_iter().peekable();
        loop {
            let next = (requested_fields.peek(), skeleton_fields.peek());

            // Try to find matching symbols.
            match next {
                (Some(requested_field), Some(skeleton_field)) => {
                    debug_assert!(
                        // As of the time of this writing, stand-alone months are not in the CLDR
                        // skeleton data. The components::Bag could produce stand-alone month fields,
                        // but since the CLDR does not have them, only Month::Format symbols are
                        // used for matching.
                        skeleton_field.symbol != FieldSymbol::Month(fields::Month::StandAlone)
                    );

                    if skeleton_field.symbol > requested_field.symbol {
                        // Keep searching for a matching skeleton field.
                        skeleton_fields.next();
                        distance += SKELETON_EXTRA_SYMBOL;
                        continue;
                    }

                    if skeleton_field.symbol < requested_field.symbol {
                        // The requested field symbol is missing from the skeleton.
                        distance += REQUESTED_SYMBOL_MISSING;
                        missing_fields += 1;
                        requested_fields.next();
                        continue;
                    }

                    distance += if requested_field == skeleton_field {
                        NO_DISTANCE
                    } else if requested_field.symbol != skeleton_field.symbol {
                        SUBSTANTIAL_DIFFERENCES_DISTANCE
                    } else if requested_field.get_length_type() != skeleton_field.get_length_type()
                    {
                        TEXT_VS_NUMERIC_DISTANCE
                    } else {
                        WIDTH_MISMATCH_DISTANCE
                    };

                    requested_fields.next();
                    skeleton_fields.next();
                }
                (None, Some(_)) => {
                    // The skeleton has additional fields that we are not matching.
                    distance += SKELETON_EXTRA_SYMBOL;
                    skeleton_fields.next();
                }
                (Some(_), None) => {
                    // The skeleton is missing requested fields.
                    distance += REQUESTED_SYMBOL_MISSING;
                    requested_fields.next();
                    missing_fields += 1;
                }
                (None, None) => {
                    break;
                }
            }
        }

        if distance < closest_distance {
            closest_format_pattern = Some(pattern);
            closest_distance = distance;
            closest_missing_fields = missing_fields;
        }
    }

    let mut closest_format_pattern = closest_format_pattern
        .expect("At least one closest format pattern will always be found.")
        .clone();

    if closest_missing_fields == fields.len() {
        return BestSkeleton::NoMatch;
    }

    if closest_distance == NO_DISTANCE {
        return BestSkeleton::AllFieldsMatch(closest_format_pattern);
    }

    // Modify the resulting pattern to have fields of the same length.
    if prefer_matched_pattern {
        #[cfg(not(feature = "provider_transform_internals"))]
        panic!("This code branch should only be run when transforming provider code.");
    } else {
        closest_format_pattern.0.items.iter_mut().for_each(|item| {
            if let PatternItem::Field(pattern_field) = item {
                if let Some(requested_field) = fields
                    .iter()
                    .find(|field| field.symbol == pattern_field.symbol)
                {
                    if requested_field.length != pattern_field.length
                        && requested_field.get_length_type() == pattern_field.get_length_type()
                    {
                        *item = PatternItem::Field(*requested_field);
                    }
                }
            }
        });
    }

    if closest_distance >= SKELETON_EXTRA_SYMBOL {
        return BestSkeleton::MissingOrExtraFields(closest_format_pattern);
    }

    BestSkeleton::AllFieldsMatch(closest_format_pattern)
}
