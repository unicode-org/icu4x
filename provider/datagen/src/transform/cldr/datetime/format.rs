// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::convert::TryFrom;

use icu_datetime::provider::calendar::{
    patterns::{GenericLengthPatternsV1, LengthPatternsV1},
    DateSkeletonPatternsV1, SkeletonV1,
};
use icu_datetime_bag_v2::provider::FormatLengthsV1;

use crate::transform::cldr::cldr_serde;

impl From<&cldr_serde::ca::Dates> for FormatLengthsV1<'_> {
    fn from(other: &cldr_serde::ca::Dates) -> Self {
        struct Format<'data> {
            patterns: LengthPatternsV1<'data>,
            skeletons: LengthSkeletons,
        }
        struct LengthSkeletons {
            full: SkeletonV1,
            long: SkeletonV1,
            medium: SkeletonV1,
            short: SkeletonV1,
        }

        impl From<&cldr_serde::ca::LengthSkeletons> for LengthSkeletons {
            fn from(skeletons: &cldr_serde::ca::LengthSkeletons) -> Self {
                Self {
                    full: SkeletonV1::try_from(&*skeletons.full)
                        .expect("unable to parse a skeleton"),
                    long: SkeletonV1::try_from(&*skeletons.long)
                        .expect("unable to parse a skeleton"),
                    medium: SkeletonV1::try_from(&*skeletons.medium)
                        .expect("unable to parse a skeleton"),
                    short: SkeletonV1::try_from(&*skeletons.short)
                        .expect("unable to parse a skeleton"),
                }
            }
        }

        let length_combinations = GenericLengthPatternsV1::from(&other.datetime_formats);
        let skeleton_patterns = DateSkeletonPatternsV1::from(other);
        let date_formats = Format {
            patterns: LengthPatternsV1::from(&other.date_formats),
            skeletons: LengthSkeletons::from(&other.date_skeletons),
        };
        let time_formats = Format {
            patterns: LengthPatternsV1::from(&other.time_formats),
            skeletons: LengthSkeletons::from(&other.time_skeletons),
        };
        todo!()
    }
}
