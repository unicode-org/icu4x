// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod difference;
pub(crate) mod formatter;
pub(crate) mod formatter_impl;
pub(crate) mod write;

pub use formatter::{DateRangeFormatter, FixedCalendarDateRangeFormatter};
pub use write::{DATE_RANGE_PART_SOURCE_CATEGORY, DateRangePartSource, FormattedDateRange};

/// A range formatter optimized for time and time zone formatting, when a calendar is not needed.
pub type NoCalendarRangeFormatter<FSet> = FixedCalendarDateRangeFormatter<(), FSet>;
