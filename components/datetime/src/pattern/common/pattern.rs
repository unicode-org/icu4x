// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use super::super::{PatternItem, TimeGranularity};
use icu_provider::yoke::{self, Yokeable, ZeroCopyFrom};

pub(crate) trait PatternType: Yokeable + ZeroCopyFrom {
    type Iter: Iterator<Item = PatternItem>;

    fn time_granularity(&self) -> TimeGranularity;
    fn iter(&self) -> Self::Iter;
}
