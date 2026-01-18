// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use icu_plurals::PluralElements;

use crate::{
    options::SubsecondDigits,
    provider::{
        fields::{self, components, Field, FieldLength, FieldSymbol},
        pattern::{naively_apply_hour_cycle, runtime, PatternItem, TimeGranularity},
        skeleton::{reference, FullLongMediumShort, GenericLengthPatterns},
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
// >     - Months: M ≅ L           (9 ≅ 9)  conjunction, vs stand-alone
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

// If a glue pattern is required, give a small penalty.
const GLUE_DISTANCE: u32 = 10;

// C. Numeric and text fields are given a larger distance from each other.
// - MMM ≈ MM    (Sep ≈ 09)
//   MMM
const TEXT_VS_NUMERIC_DISTANCE: u32 = 100;

// D. Symbols representing substantial differences (week of year vs week of month) are given much
// larger a distances from each other.
// - d ≋ D;     (12 ≋ 345) Day of month vs Day of year
const SUBSTANTIAL_DIFFERENCES_DISTANCE: u32 = 1000;

// A skeleton had more symbols than what was requested.
const SKELETON_EXTRA_SYMBOL: u32 = 10000;

// A requested symbol is missing in the skeleton. Note that this final value can be more than
// MAX_SKELETON_FIELDS, as it's counting the missing requested fields, which can be longer than
// the stored skeletons. There cannot be any cases higher than this one.
const REQUESTED_SYMBOL_MISSING: u32 = 100000;

/// The best skeleton found, alongside information on how well it matches.
///
/// According to the [UTS 35 skeleton matching algorithm](https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons)
/// there will be a guaranteed match for a skeleton. However, with this initial implementation,
/// there is no attempt to add on missing fields. This enum encodes the variants for the current
/// search for a best skeleton.
///
/// The patterns are paired with a measure of their quality.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub enum BestSkeleton<T> {
    AllFieldsMatch(T, SkeletonQuality),
    MissingOrExtraFields(T, SkeletonQuality),
    NoMatch,
}

/// A measure of the quality of a skeleton.
///
/// Internally, this is a u32, a "distance" value. This value is highly
/// unstable and should not be compared across versions. It should be used
/// only for comparing against other distances in the same version of ICU4X.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkeletonQuality(u32);

impl SkeletonQuality {
    /// Returns the worst possible quality measure.
    pub fn worst() -> SkeletonQuality {
        SkeletonQuality(u32::MAX)
    }
    /// Returns the best possible quality measure.
    pub fn best() -> SkeletonQuality {
        SkeletonQuality(0)
    }
    /// Returns whether this is an "excellent" match by an arbitrary definition.
    pub fn is_excellent_match(self) -> bool {
        self.0 < GLUE_DISTANCE
    }
}

/// This function swaps out the time zone name field for the appropriate one. Skeleton matching
/// only needs to find a single "v" field, and then the time zone name can expand from there.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
fn naively_apply_time_zone_name(
    pattern: &mut runtime::Pattern,
    time_zone_name: Option<components::TimeZoneName>,
) {
    // If there is a preference overriding the hour cycle, apply it now.
    if let Some(time_zone_name) = time_zone_name {
        runtime::helpers::maybe_replace_first(pattern, |item| {
            if let PatternItem::Field(fields::Field {
                symbol: fields::FieldSymbol::TimeZone(_),
                length: _,
            }) = item
            {
                Some(PatternItem::Field((time_zone_name).into()))
            } else {
                None
            }
        });
    }
}

// Note: This could return a Cow<'a, Pattern>, but it affects every other part of the API to
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
///   the desired fields, even if the provider data doesn't completely match. This
///   configuration option makes it so that the final pattern won't have additional work
///   done to mutate it to match the fields. It will prefer the actual matched pattern.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
pub fn create_best_pattern_for_fields<'data>(
    skeletons: &BTreeMap<reference::Skeleton, PluralElements<runtime::Pattern<'data>>>,
    length_patterns: &GenericLengthPatterns<'data>,
    fields: &[Field],
    components: &components::Bag,
    prefer_matched_pattern: bool,
) -> BestSkeleton<PluralElements<runtime::Pattern<'data>>> {
    let first_pattern_match =
        get_best_available_format_pattern(skeletons, fields, prefer_matched_pattern);

    // Try to match a skeleton to all of the fields.
    if let BestSkeleton::AllFieldsMatch(mut pattern, d) = first_pattern_match {
        pattern.for_each_mut(|pattern| {
            naively_apply_hour_cycle(pattern, components.hour_cycle);
            naively_apply_time_zone_name(pattern, components.time_zone_name);
            apply_subseconds(pattern, components.subsecond);
        });
        return BestSkeleton::AllFieldsMatch(pattern, d);
    }

    let FieldsByType { date, time } = group_fields_by_type(fields);

    if date.is_empty() || time.is_empty() {
        return match first_pattern_match {
            BestSkeleton::AllFieldsMatch(_, _) => {
                unreachable!("Logic error in implementation. AllFieldsMatch handled above.")
            }
            BestSkeleton::MissingOrExtraFields(mut pattern, d) => {
                if date.is_empty() {
                    pattern.for_each_mut(|pattern| {
                        naively_apply_hour_cycle(pattern, components.hour_cycle);
                        naively_apply_time_zone_name(pattern, components.time_zone_name);
                        apply_subseconds(pattern, components.subsecond);
                    });
                }
                BestSkeleton::MissingOrExtraFields(pattern, d)
            }
            BestSkeleton::NoMatch => BestSkeleton::NoMatch,
        };
    }

    // Match the date and time, and then simplify the combinatorial logic of the results into
    // an optional values of the results, and a boolean value.
    let (date_patterns, date_missing_or_extra, date_distance) =
        match get_best_available_format_pattern(skeletons, &date, prefer_matched_pattern) {
            BestSkeleton::MissingOrExtraFields(fields, d) => (Some(fields), true, d),
            BestSkeleton::AllFieldsMatch(fields, d) => (Some(fields), false, d),
            BestSkeleton::NoMatch => (None, true, SkeletonQuality(REQUESTED_SYMBOL_MISSING)),
        };

    let (time_patterns, time_missing_or_extra, time_distance) =
        match get_best_available_format_pattern(skeletons, &time, prefer_matched_pattern) {
            BestSkeleton::MissingOrExtraFields(fields, d) => (Some(fields), true, d),
            BestSkeleton::AllFieldsMatch(fields, d) => (Some(fields), false, d),
            BestSkeleton::NoMatch => (None, true, SkeletonQuality(REQUESTED_SYMBOL_MISSING)),
        };
    let time_pattern: Option<runtime::Pattern<'data>> = time_patterns.map(|pattern| {
        #[allow(clippy::unwrap_used)] // only date patterns can contain plural variants
        let mut pattern = pattern.try_into_other().unwrap();
        naively_apply_hour_cycle(&mut pattern, components.hour_cycle);
        naively_apply_time_zone_name(&mut pattern, components.time_zone_name);
        apply_subseconds(&mut pattern, components.subsecond);
        pattern
    });

    // Determine how to combine the date and time.
    let patterns = match (date_patterns, time_pattern) {
        (Some(mut date_patterns), Some(time_pattern)) => {
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
                    FieldLength::Four => {
                        let weekday = fields
                            .iter()
                            .find(|f| matches!(f.symbol, FieldSymbol::Weekday(_)));

                        if weekday.is_some() {
                            FullLongMediumShort::Full
                        } else {
                            FullLongMediumShort::Long
                        }
                    }
                    FieldLength::Three => FullLongMediumShort::Medium,
                    _ => FullLongMediumShort::Short,
                },
                None => FullLongMediumShort::Short,
            };

            use crate::provider::pattern::runtime::GenericPattern;
            let dt_pattern: &GenericPattern<'data> = match length {
                FullLongMediumShort::Full => &length_patterns.full,
                FullLongMediumShort::Long => &length_patterns.long,
                FullLongMediumShort::Medium => &length_patterns.medium,
                FullLongMediumShort::Short => &length_patterns.short,
            };

            date_patterns.for_each_mut(|pattern| {
                let date = pattern.clone();
                let time = time_pattern.clone();

                // TODO(#2626) - Since this is fallible, we should make this method fallible.
                #[expect(clippy::expect_used)] // Generic pattern combination should never fail.
                let dt = dt_pattern
                    .clone()
                    .combined(date, time)
                    .expect("Failed to combine date and time");
                *pattern = dt;
            });
            Some(date_patterns)
        }
        (None, Some(pattern)) => Some(PluralElements::new(pattern)),
        (Some(patterns), None) => Some(patterns),
        (None, None) => None,
    };

    let distance = SkeletonQuality(
        date_distance
            .0
            .saturating_add(time_distance.0)
            .saturating_add(GLUE_DISTANCE),
    );
    match patterns {
        Some(patterns) => {
            if date_missing_or_extra || time_missing_or_extra {
                BestSkeleton::MissingOrExtraFields(patterns, distance)
            } else {
                BestSkeleton::AllFieldsMatch(patterns, distance)
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
            FieldSymbol::Era
            | FieldSymbol::Year(_)
            | FieldSymbol::Month(_)
            | FieldSymbol::Week(_)
            | FieldSymbol::Day(_)
            | FieldSymbol::Weekday(_) => date.push(*field),

            // Time components:
            FieldSymbol::DayPeriod(_)
            | FieldSymbol::Hour(_)
            | FieldSymbol::Minute
            | FieldSymbol::Second(_)
            | FieldSymbol::TimeZone(_)
            | FieldSymbol::DecimalSecond(_) => time.push(*field),
            // Other components
            // TODO(#486)
            // FieldSymbol::Era(_) => other.push(*field),
            // Plus others...
        };
    }

    FieldsByType { date, time }
}

/// Alters given Pattern so that its fields have the same length as 'fields'.
///
///  For example the "d MMM y" pattern will be changed to "d MMMM y" given fields ["y", "MMMM", "d"].
fn adjust_pattern_field_lengths(fields: &[Field], pattern: &mut runtime::Pattern) {
    runtime::helpers::maybe_replace(pattern, |item| {
        if let PatternItem::Field(pattern_field) = item {
            if let Some(requested_field) = fields
                .iter()
                .find(|field| field.symbol.skeleton_cmp(pattern_field.symbol).is_eq())
            {
                if requested_field.length != pattern_field.length
                    && requested_field.get_length_type() == pattern_field.get_length_type()
                {
                    let length = requested_field.length;
                    let length = if requested_field.symbol.is_at_least_abbreviated() {
                        length.numeric_to_abbr()
                    } else {
                        length
                    };
                    return Some(PatternItem::Field(Field {
                        length,
                        ..*pattern_field
                    }));
                }
            }
        }
        None
    })
}

/// Alters given Pattern so that it will have a fractional second field if it was requested.
///
/// If the requested skeleton included both seconds and fractional seconds and the dateFormatItem
/// skeleton included seconds but not fractional seconds, then the seconds field of the corresponding
/// pattern should be adjusted by appending the locale’s decimal separator, followed by the sequence
/// of ‘S’ characters from the requested skeleton.
/// (see <https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons>)
fn apply_subseconds(pattern: &mut runtime::Pattern, subseconds: Option<SubsecondDigits>) {
    if let Some(subseconds) = subseconds {
        let mut items = pattern.items.to_vec();
        for item in items.iter_mut() {
            if let PatternItem::Field(
                ref mut field @ Field {
                    symbol:
                        FieldSymbol::Second(fields::Second::Second) | FieldSymbol::DecimalSecond(_),
                    ..
                },
            ) = item
            {
                field.symbol = FieldSymbol::from_subsecond_digits(subseconds);
            };
        }
        *pattern = runtime::Pattern::from(items);
        pattern
            .metadata
            .set_time_granularity(TimeGranularity::Nanoseconds);
    }
}

/// A partial implementation of the [UTS 35 skeleton matching algorithm](https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons).
///
/// The following is implemented:
///
///  * Compute a score based on the best possible match for the given fields.
///  * Select the skeleton with highest score.
///  * Modify the resulting pattern to have fields of the same length. For example requesting
///    a skeleton "yMMMMd" can have a best match of ["yMMMd", "d MMM y"]. This pattern should
///    then be modified to use the requested length to produce a pattern "d MMMM y".
///    However, fields should not be changed from numeric to text.
///
/// The following is not implemented:
///
///  * 2.6.2.2 Missing Skeleton Fields
///    - TODO(#586) - Using the CLDR appendItems field. Note: There is not agreement yet on how
///      much of this step to implement. See the issue for more information.
///
/// # Panics
///
/// Panics if `prefer_matched_pattern` is set to true in a non-datagen mode.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
pub fn get_best_available_format_pattern<'data>(
    skeletons: &BTreeMap<reference::Skeleton, PluralElements<runtime::Pattern<'data>>>,
    fields: &[Field],
    prefer_matched_pattern: bool,
) -> BestSkeleton<PluralElements<runtime::Pattern<'data>>> {
    let mut closest_format_pattern = None;
    let mut closest_distance: u32 = u32::MAX;
    let mut closest_missing_fields = 0;

    for (skeleton, pattern) in skeletons.iter() {
        debug_assert!(
            skeleton.fields_len() <= MAX_SKELETON_FIELDS as usize,
            "The distance mechanism assumes skeletons are less than MAX_SKELETON_FIELDS in length."
        );
        let mut missing_fields = 0;
        let mut distance: u32 = 0;
        // The distance should fit into a u32.

        let mut requested_fields = fields.iter().peekable();
        let mut skeleton_fields = skeleton.fields_iter().peekable();

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

                    match skeleton_field.symbol.skeleton_cmp(requested_field.symbol) {
                        Ordering::Less => {
                            // Keep searching for a matching skeleton field.
                            skeleton_fields.next();
                            distance += SKELETON_EXTRA_SYMBOL;
                            continue;
                        }
                        Ordering::Greater => {
                            // The requested field symbol is missing from the skeleton.
                            distance += REQUESTED_SYMBOL_MISSING;
                            missing_fields += 1;
                            requested_fields.next();
                            continue;
                        }
                        _ => (),
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

    if !prefer_matched_pattern && closest_distance >= TEXT_VS_NUMERIC_DISTANCE {
        if let [field] = fields {
            // A single field was requested and the best pattern either includes extra fields or can't be adjusted to match
            // (e.g. text vs numeric). We return the field instead of the matched pattern.
            return BestSkeleton::AllFieldsMatch(
                PluralElements::new(runtime::Pattern::from(vec![PatternItem::Field(*field)])),
                SkeletonQuality(closest_distance),
            );
        }
    }

    let mut closest_format_pattern = if let Some(pattern) = closest_format_pattern {
        pattern.clone()
    } else {
        return BestSkeleton::NoMatch;
    };

    if closest_missing_fields == fields.len() {
        return BestSkeleton::NoMatch;
    }

    if closest_distance == NO_DISTANCE {
        return BestSkeleton::AllFieldsMatch(
            closest_format_pattern,
            SkeletonQuality(closest_distance),
        );
    }

    // Modify the resulting pattern to have fields of the same length.
    #[allow(clippy::panic)] // guards against running this branch in non-datagen mode.
    if prefer_matched_pattern {
        #[cfg(not(feature = "datagen"))]
        panic!("This code branch should only be run when transforming provider code.");
    } else {
        closest_format_pattern.for_each_mut(|pattern| {
            adjust_pattern_field_lengths(fields, pattern);
        });
    }

    if closest_distance >= SKELETON_EXTRA_SYMBOL {
        return BestSkeleton::MissingOrExtraFields(
            closest_format_pattern,
            SkeletonQuality(closest_distance),
        );
    }

    BestSkeleton::AllFieldsMatch(closest_format_pattern, SkeletonQuality(closest_distance))
}
