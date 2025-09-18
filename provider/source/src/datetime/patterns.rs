// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(test)]
use super::legacy::DateLengths;
use super::legacy::{LengthPatterns, TimeLengths};
use crate::cldr_serde;
use icu::datetime::provider::pattern;
use icu::datetime::provider::pattern::CoarseHourCycle;
use icu::datetime::provider::skeleton::*;
use icu_provider::DataLocale;

impl From<&cldr_serde::ca::LengthPatterns> for LengthPatterns<'_> {
    fn from(other: &cldr_serde::ca::LengthPatterns) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other
                .full
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            long: other
                .long
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

#[cfg(test)]
impl From<&cldr_serde::ca::DateTimeFormats> for LengthPatterns<'_> {
    fn from(other: &cldr_serde::ca::DateTimeFormats) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other
                .full
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            long: other
                .long
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

impl From<&cldr_serde::ca::DateTimeFormats> for GenericLengthPatterns<'_> {
    fn from(other: &cldr_serde::ca::DateTimeFormats) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other
                .full
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            long: other
                .long
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

#[cfg(test)]
impl From<&cldr_serde::ca::Dates> for DateLengths<'_> {
    fn from(other: &cldr_serde::ca::Dates) -> Self {
        let length_combinations_v1 = GenericLengthPatterns::from(&other.datetime_formats);

        Self {
            date: (&other.date_skeletons).into(),
            length_combinations: length_combinations_v1,
        }
    }
}

impl TimeLengths<'_> {
    pub(crate) fn from_serde(other: &cldr_serde::ca::Dates, locale: &DataLocale) -> Self {
        let length_combinations_v1 = GenericLengthPatterns::from(&other.datetime_formats);
        let skeletons_v1 = DateSkeletonPatterns::from(&other.datetime_formats.available_formats);

        // Note: TimeLengths is only used for preferred_hour_cycle, we don't really use
        // the rest of the pattern here.
        let pattern_str_full = other.time_skeletons.full.get_pattern();
        let pattern_str_long = other.time_skeletons.long.get_pattern();
        let pattern_str_medium = other.time_skeletons.medium.get_pattern();
        let pattern_str_short = other.time_skeletons.short.get_pattern();

        let pattern_full = pattern_str_full
            .parse()
            .expect("Failed to create a full Pattern from bytes.");
        let pattern_long = pattern_str_long
            .parse()
            .expect("Failed to create a long Pattern from bytes.");
        let pattern_medium = pattern_str_medium
            .parse()
            .expect("Failed to create a medium Pattern from bytes.");
        let pattern_short = pattern_str_short
            .parse()
            .expect("Failed to create a short Pattern from bytes.");

        let mut preferred_hour_cycle: Option<CoarseHourCycle> = None;
        let arr = [
            pattern::CoarseHourCycle::determine(&pattern_full),
            pattern::CoarseHourCycle::determine(&pattern_long),
            pattern::CoarseHourCycle::determine(&pattern_medium),
            pattern::CoarseHourCycle::determine(&pattern_short),
        ];
        let iter = arr.iter().flatten();
        for hour_cycle in iter {
            if let Some(preferred_hour_cycle) = preferred_hour_cycle {
                if *hour_cycle != preferred_hour_cycle {
                    log::warn!("{locale:?} contained a mix of coarse hour cycle types ({hour_cycle:?}, {preferred_hour_cycle:?})");
                }
            } else {
                preferred_hour_cycle = Some(*hour_cycle);
            }
        }

        let preferred_hour_cycle =
            preferred_hour_cycle.expect("Could not find a preferred hour cycle.");
        let alt_hour_cycle = if preferred_hour_cycle == CoarseHourCycle::H11H12 {
            CoarseHourCycle::H23
        } else {
            CoarseHourCycle::H11H12
        };

        let (time_h11_h12, time_h23_h24) = {
            let time = (&other.time_skeletons).into();
            let alt_time = LengthPatterns {
                full: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_full,
                        pattern_full,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a full pattern.")
                    .into(),
                long: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_long,
                        pattern_long,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a long pattern.")
                    .into(),
                medium: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_medium,
                        pattern_medium,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a medium pattern.")
                    .into(),
                short: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_short,
                        pattern_short,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a short pattern.")
                    .into(),
            };

            match preferred_hour_cycle {
                CoarseHourCycle::H11H12 => (time, alt_time),
                CoarseHourCycle::H23 => (alt_time, time),
            }
        };

        Self {
            time_h11_h12,
            time_h23_h24,
            preferred_hour_cycle,
        }
    }
}
