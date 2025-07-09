// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Transition, Zone};
use icu_time::zone::UtcOffset;

#[derive(Clone, Copy, Debug)]
pub struct ChronoOffset<'a>(Transition, Zone<'a>);

impl core::ops::Deref for ChronoOffset<'_> {
    type Target = UtcOffset;

    fn deref(&self) -> &Self::Target {
        &self.0.offset
    }
}

impl ChronoOffset<'_> {
    pub fn rule_applies(&self) -> bool {
        self.0.rule_applies
    }
}

#[cfg(feature = "chrono")]
impl<'a> chrono::Offset for ChronoOffset<'a> {
    fn fix(&self) -> chrono::FixedOffset {
        chrono::FixedOffset::east_opt(self.0.offset.to_seconds()).unwrap()
    }
}

#[cfg(feature = "chrono")]
impl<'a> chrono::TimeZone for Zone<'a> {
    type Offset = ChronoOffset<'a>;

    fn from_offset(offset: &Self::Offset) -> Self {
        offset.1
    }

    fn offset_from_local_date(
        &self,
        _local: &chrono::NaiveDate,
    ) -> chrono::MappedLocalTime<Self::Offset> {
        todo!()
    }

    fn offset_from_local_datetime(
        &self,
        _local: &chrono::NaiveDateTime,
    ) -> chrono::MappedLocalTime<Self::Offset> {
        todo!()
    }

    fn offset_from_utc_date(&self, utc: &chrono::NaiveDate) -> Self::Offset {
        ChronoOffset(
            self.previous_transition(utc.and_time(chrono::NaiveTime::MIN).and_utc().timestamp()),
            *self,
        )
    }

    fn offset_from_utc_datetime(&self, utc: &chrono::NaiveDateTime) -> Self::Offset {
        ChronoOffset(self.previous_transition(utc.and_utc().timestamp()), *self)
    }
}
