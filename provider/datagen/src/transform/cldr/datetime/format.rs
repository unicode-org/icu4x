use icu_datetime::provider::calendar::{
    patterns::{GenericLengthPatternsV1, LengthPatternsV1},
    DateSkeletonPatternsV1,
};
use icu_datetime_bag_v2::provider::FormatLengthsV1;

use crate::transform::cldr::cldr_serde;

impl From<&cldr_serde::ca::Dates> for FormatLengthsV1<'_> {
    fn from(other: &cldr_serde::ca::Dates) -> Self {
        let length_combinations = GenericLengthPatternsV1::from(&other.datetime_formats);
        let patterns = DateSkeletonPatternsV1::from(other);
        todo!()
    }
}
