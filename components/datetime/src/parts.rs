// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parts of a formatted date/time.
//!
//! # Examples
//!
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::{Date, Time};
/// use icu::datetime::DateTimeWriteError;
/// use icu::datetime::parts;
/// use icu::datetime::fieldsets;
/// use icu::datetime::DateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::IxdtfParser;
/// use writeable::assert_writeable_parts_eq;
///
/// let dtf = DateTimeFormatter::try_new(
///     locale!("en-u-ca-buddhist").into(),
///     fieldsets::YMDTZ::medium(),
/// )
/// .unwrap();
///
/// let dtz = IxdtfParser::new().try_from_str("2023-11-20T11:35:03+00:00[Europe/London]").unwrap();
///
/// // Missing data is filled in on a best-effort basis, and an error is signaled.
/// assert_writeable_parts_eq!(
///     dtf.format_any_calendar(&dtz),
///     "Nov 20, 2566 BE, 11:35:03 AM GMT",
///     [
///         (0, 3, parts::MONTH),
///         (4, 6, parts::DAY),
///         (8, 12, parts::YEAR),
///         (13, 15, parts::ERA),
///         (17, 19, parts::HOUR),
///         (20, 22, parts::MINUTE),
///         (23, 25, parts::SECOND),
///         // note: from 25 to 28 is a NNBSP
///         (28, 30, parts::DAY_PERIOD),
///         (31, 34, parts::TIME_ZONE_NAME),
///     ]
/// );
/// ```
use writeable::Part;

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const ERA: Part = Part {
    category: "datetime",
    value: "era",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const YEAR: Part = Part {
    category: "datetime",
    value: "year",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const RELATED_YEAR: Part = Part {
    category: "datetime",
    value: "relatedYear",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const YEAR_NAME: Part = Part {
    category: "datetime",
    value: "yearName",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const MONTH: Part = Part {
    category: "datetime",
    value: "month",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const DAY: Part = Part {
    category: "datetime",
    value: "day",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const WEEKDAY: Part = Part {
    category: "datetime",
    value: "weekday",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const DAY_PERIOD: Part = Part {
    category: "datetime",
    value: "dayPeriod",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const HOUR: Part = Part {
    category: "datetime",
    value: "hour",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const MINUTE: Part = Part {
    category: "datetime",
    value: "minute",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const SECOND: Part = Part {
    category: "datetime",
    value: "second",
};

/// A [`Part`] used by [`FormattedDateTime`](super::FormattedDateTime).
pub const TIME_ZONE_NAME: Part = Part {
    category: "datetime",
    value: "timeZoneName",
};
