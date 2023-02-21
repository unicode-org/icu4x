// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_datetime::pattern::runtime::{PatternPlurals, PluralPattern};
use icu_datetime::provider::calendar::*;
use icu_datetime::skeleton::reference::Skeleton;
use icu_datetime::skeleton::SkeletonError;
use icu_plurals::PluralCategory;
use std::collections::HashMap;
use std::convert::TryFrom;

impl From<&cldr_serde::ca::Dates> for DateSkeletonPatternsV1<'_> {
    fn from(other: &cldr_serde::ca::Dates) -> Self {
        let mut patterns: HashMap<String, HashMap<String, String>> = HashMap::new();

        // The CLDR keys for available_formats can have duplicate skeletons with either
        // an additional variant, or with multiple variants for different plurals.
        for (skeleton_str, pattern_str) in other.datetime_formats.available_formats.0.iter() {
            let (skeleton_str, plural_form_str) = match skeleton_str.split_once("-count-") {
                Some((s, v)) => (s, v),
                None => (skeleton_str.as_ref(), "other"),
            };

            patterns
                .entry(skeleton_str.to_string())
                .or_default()
                .insert(plural_form_str.to_string(), pattern_str.to_string());
        }

        let skeletons = patterns
            .iter()
            .filter_map(|(skeleton_str, patterns)| {
                let skeleton = match Skeleton::try_from(skeleton_str.as_str()) {
                    Ok(s) => s,
                    Err(SkeletonError::SymbolUnimplemented(_)) => return None,
                    Err(SkeletonError::SkeletonHasVariant) => return None,
                    Err(err) => panic!(
                        "Unexpected skeleton error while parsing skeleton {skeleton_str:?} {err}"
                    ),
                };
                let pattern_str = patterns.get("other").expect("Other variant must exist");
                let pattern = pattern_str.parse().expect("Unable to parse a pattern");

                let mut pattern_plurals = if patterns.len() == 1 {
                    PatternPlurals::SinglePattern(pattern)
                } else {
                    let mut plural_pattern =
                        PluralPattern::new(pattern).expect("Unable to construct PluralPattern");

                    for (key, pattern_str) in patterns {
                        if key == "other" {
                            continue;
                        }
                        let cat = PluralCategory::get_for_cldr_string(key)
                            .expect("Failed to retrieve plural category");
                        let pattern = pattern_str.parse().expect("Unable to parse a pattern");
                        plural_pattern.maybe_set_variant(cat, pattern);
                    }
                    PatternPlurals::MultipleVariants(plural_pattern)
                };
                // In some cases we may encounter duplicated patterns, which will
                // get deduplicated and result in a single-variant `MultiVariant` branch
                // here. The following `normalize` will turn those cases to `SingleVariant`.
                pattern_plurals.normalize();

                Some((SkeletonV1(skeleton), pattern_plurals))
            })
            .collect();

        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self(skeletons)
    }
}
