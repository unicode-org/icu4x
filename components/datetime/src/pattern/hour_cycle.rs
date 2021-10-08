// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{reference::Pattern, PatternItem};
use crate::{fields, options::preferences};
#[cfg(feature = "provider_transform_internals")]
use crate::{provider, skeleton};

/// Used to represent either H11/H12, or H23/H24. Skeletons only store these
/// hour cycles as H12 or H23.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum CoarseHourCycle {
    /// Can either be fields::Hour::H11 or fields::Hour::H12
    H11H12,
    /// Can either be fields::Hour::H23 or fields::Hour::H24
    H23H24,
}

/// Default is required for serialization. H23H24 is the more locale-agnostic choice, as it's
/// less likely to have a day period in it.
impl Default for CoarseHourCycle {
    fn default() -> Self {
        CoarseHourCycle::H23H24
    }
}

impl CoarseHourCycle {
    /// Figure out the coarse hour cycle given a pattern, which is useful for generating the provider
    /// patterns for `length::Bag`.
    pub fn determine(pattern: &Pattern) -> Option<Self> {
        for item in pattern.items.iter() {
            if let PatternItem::Field(fields::Field {
                symbol: fields::FieldSymbol::Hour(pattern_hour),
                length: _,
            }) = item
            {
                return Some(match pattern_hour {
                    fields::Hour::H11 | fields::Hour::H12 => CoarseHourCycle::H11H12,
                    fields::Hour::H23 | fields::Hour::H24 => CoarseHourCycle::H23H24,
                });
            }
        }

        None
    }

    /// Invoke the pattern matching machinery to transform the hour cycle of a pattern. This provides
    /// a safe mapping from a h11/h12 to h23/h24 for transforms.
    #[doc(hidden)]
    #[cfg(feature = "provider_transform_internals")]
    pub fn apply_on_pattern(
        &self,
        date_time: &provider::gregory::patterns::LengthPatternsV1,
        skeletons: &provider::gregory::DateSkeletonPatternsV1,
        pattern_str: &str,
        mut pattern: Pattern,
    ) -> Option<String> {
        for item in pattern.items_mut() {
            if let PatternItem::Field(fields::Field { symbol, length: _ }) = item {
                if let fields::FieldSymbol::Hour(pattern_hour) = symbol {
                    if match self {
                        CoarseHourCycle::H11H12 => match pattern_hour {
                            fields::Hour::H11 | fields::Hour::H12 => true,
                            fields::Hour::H23 | fields::Hour::H24 => false,
                        },
                        CoarseHourCycle::H23H24 => match pattern_hour {
                            fields::Hour::H11 | fields::Hour::H12 => false,
                            fields::Hour::H23 | fields::Hour::H24 => true,
                        },
                    } {
                        // The preference hour cycle matches the pattern, bail out early and
                        // return the current pattern.
                        return Some(pattern_str.into());
                    } else {
                        // Mutate the pattern with the new symbol, so that it can be matched against.
                        *symbol = fields::FieldSymbol::Hour(match self {
                            CoarseHourCycle::H11H12 => fields::Hour::H12,
                            CoarseHourCycle::H23H24 => fields::Hour::H23,
                        });
                        break;
                    }
                }
            }
        }

        let skeleton = skeleton::reference::Skeleton::from(&pattern);

        match skeleton::create_best_pattern_for_fields(
            skeletons,
            date_time,
            skeleton.as_slice(),
            &Default::default(),
            // Prefer using the matched pattern directly, rather than mutating it to match the
            // requested fields.
            true,
        ) {
            skeleton::BestSkeleton::AllFieldsMatch(pattern)
            | skeleton::BestSkeleton::MissingOrExtraFields(pattern) => {
                Some(format!("{}", pattern.0))
            }
            skeleton::BestSkeleton::NoMatch => None,
        }
    }
}

/// The hour cycle can be set by preferences. This function switches between h11 and h12,
/// and between h23 and h24. This function is naive as it is assumed that this application of
/// the hour cycle will not change between h1x to h2x.
pub(crate) fn naively_apply_preferences(
    pattern: &mut Pattern,
    preferences: &Option<preferences::Bag>,
) {
    // If there is a preference overiding the hour cycle, apply it now.
    if let Some(preferences::Bag {
        hour_cycle: Some(hour_cycle),
    }) = preferences
    {
        for item in pattern.items_mut() {
            if let PatternItem::Field(fields::Field { symbol, length: _ }) = item {
                if let fields::FieldSymbol::Hour(_) = symbol {
                    *symbol = fields::FieldSymbol::Hour(hour_cycle.field());
                }
            }
        }
    }
}
