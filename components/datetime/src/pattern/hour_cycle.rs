use crate::fields;
use crate::pattern::{Pattern, PatternItem};
use crate::provider;
use crate::skeleton;

/// Used to represent either H11/H12, or H23/H24. Skeletons only store these
/// hour cycles as H12 or H24.
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

/// Default is required for serialization, arbitrarily pick one.
impl Default for CoarseHourCycle {
    fn default() -> Self {
        CoarseHourCycle::H11H12
    }
}

impl CoarseHourCycle {
    pub fn matches_field(&self, hour: &fields::Hour) -> bool {
        match self {
            CoarseHourCycle::H11H12 => match hour {
                fields::Hour::H11 | fields::Hour::H12 => true,
                fields::Hour::H23 | fields::Hour::H24 => false,
            },
            CoarseHourCycle::H23H24 => match hour {
                fields::Hour::H11 | fields::Hour::H12 => false,
                fields::Hour::H23 | fields::Hour::H24 => true,
            },
        }
    }

    /// Skeletons only use h12 and h24.
    fn to_skeleton_field_symbol(&self) -> fields::Hour {
        match self {
            CoarseHourCycle::H11H12 => fields::Hour::H12,
            CoarseHourCycle::H23H24 => fields::Hour::H24,
        }
    }
}

pub fn determine_coarse_hour_cycle(pattern: &Pattern) -> Option<CoarseHourCycle> {
    for item in pattern.items() {
        if let PatternItem::Field(fields::Field { symbol, length: _ }) = item {
            if let fields::FieldSymbol::Hour(pattern_hour) = symbol {
                return Some(match pattern_hour {
                    fields::Hour::H11 | fields::Hour::H12 => CoarseHourCycle::H11H12,
                    fields::Hour::H23 | fields::Hour::H24 => CoarseHourCycle::H23H24,
                });
            }
        }
    }

    None
}

/// Invoke the pattern matching machinery to transform the hour cycle of a pattern. This provides
/// a safe mapping from a h11/h12 to h23/h24
pub fn apply_coarse_hour_cycle(
    datetime: &provider::gregory::patterns::DateTimeFormatsV1,
    pattern_str: &str,
    mut pattern: Pattern,
    coarse_hour_cycle: CoarseHourCycle,
) -> Option<String> {
    for item in pattern.items_mut() {
        if let PatternItem::Field(fields::Field { symbol, length: _ }) = item {
            if let fields::FieldSymbol::Hour(pattern_hour) = symbol {
                if coarse_hour_cycle.matches_field(pattern_hour) {
                    // The preference hour cycle matches the pattern, bail out early and
                    // return the current pattern.
                    return Some(pattern_str.into());
                } else {
                    // Mutate the pattern with the new symbol, so that it can be matched against.
                    *symbol =
                        fields::FieldSymbol::Hour(coarse_hour_cycle.to_skeleton_field_symbol());
                    break;
                }
            }
        }
    }

    let skeleton = skeleton::Skeleton::from(&pattern);

    match skeleton::create_best_pattern_for_fields(
        &datetime.skeletons,
        &datetime.length_patterns,
        &skeleton.as_slice(),
    ) {
        skeleton::BestSkeleton::AllFieldsMatch(pattern)
        | skeleton::BestSkeleton::MissingOrExtraFields(pattern) => Some(format!("{}", pattern)),
        skeleton::BestSkeleton::NoMatch => None,
    }
}
