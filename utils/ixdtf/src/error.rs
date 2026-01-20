// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! An error enum for representing `ixdtf` parsing errors.

use core::fmt;

#[non_exhaustive]
#[derive(PartialEq, Clone, Copy, Debug)]
/// The error returned by `ixdtf`'s parsers.
pub enum ParseError {
    ImplAssert,
    NonAsciiCodePoint,
    ParseFloat,
    AbruptEnd { location: &'static str },
    InvalidEnd,

    // Date related errors
    InvalidMonthRange,
    InvalidDayRange,
    DateYear,
    DateExtendedYear,
    DateMonth,
    DateDay,
    DateUnexpectedEnd,

    // Time Related errors
    TimeRequired,
    TimeHour,
    TimeMinuteSecond,
    TimeSecond,
    FractionPart,
    DateSeparator,
    TimeSeparator,
    DecimalSeparator,

    // Annotation Related Errors
    InvalidAnnotation,
    AnnotationOpen,
    AnnotationClose,
    AnnotationChar,
    AnnotationKeyValueSeparator,
    AnnotationKeyLeadingChar,
    AnnotationKeyChar,
    AnnotationValueCharPostHyphen,
    AnnotationValueChar,
    InvalidMinutePrecisionOffset,

    // Duplicate calendar with critical.
    CriticalDuplicateCalendar,
    UnrecognizedCritical,

    // Time Zone Errors
    TzLeadingChar,
    IanaCharPostSeparator,
    IanaChar,
    UtcTimeSeparator,
    OffsetNeedsSign,

    // MonthDay Errors
    MonthDayHyphen,

    // Duration Errors
    DurationDisgnator,
    DurationValueExceededRange,
    DateDurationPartOrder,
    TimeDurationPartOrder,
    TimeDurationDesignator,

    AmbiguousTimeMonthDay,
    AmbiguousTimeYearMonth,
    InvalidMonthDay,
}

impl core::error::Error for ParseError {}

impl ParseError {
    pub(crate) fn abrupt_end(location: &'static str) -> Self {
        Self::AbruptEnd { location }
    }

    /// Convert this error to a static string representation
    pub fn to_static_string(&self) -> &'static str {
        match *self {
            Self::ImplAssert => "Implementation error: this error must not throw.",

            Self::NonAsciiCodePoint => "Code point was not ASCII",

            Self::ParseFloat => "Invalid float while parsing fraction part.",

            Self::AbruptEnd { .. } => "Parsing ended abruptly.",

            Self::InvalidEnd => "Unexpected character found after parsing was completed.",
            // Date related errors
            Self::InvalidMonthRange => "Parsed month value not in a valid range.",

            Self::InvalidDayRange => "Parsed day value not in a valid range.",

            Self::DateYear => "Invalid chracter while parsing year value.",

            Self::DateExtendedYear => "Invalid character while parsing extended year value.",

            Self::DateMonth => "Invalid character while parsing month value.",

            Self::DateDay => "Invalid character while parsing day value.",

            Self::DateUnexpectedEnd => "Unexpected end while parsing a date value.",

            Self::TimeRequired => "Time is required.",

            Self::TimeHour => "Invalid character while parsing hour value.",

            Self::TimeMinuteSecond => {
                "Invalid character while parsing minute/second value in (0, 59] range."
            }

            Self::TimeSecond => "Invalid character while parsing second value in (0, 60] range.",

            Self::FractionPart => "Invalid character while parsing fraction part value.",

            Self::DateSeparator => "Invalid character while parsing date separator.",

            Self::TimeSeparator => "Invalid character while parsing time separator.",

            Self::DecimalSeparator => "Invalid character while parsing decimal separator.",
            // Annotation Related Errors
            Self::InvalidAnnotation => "Invalid annotation.",

            Self::AnnotationOpen => "Invalid annotation open character.",

            Self::AnnotationClose => "Invalid annotation close character.",

            Self::AnnotationChar => "Invalid annotation character.",

            Self::AnnotationKeyValueSeparator => {
                "Invalid annotation key-value separator character."
            }

            Self::AnnotationKeyLeadingChar => "Invalid annotation key leading character.",

            Self::AnnotationKeyChar => "Invalid annotation key character.",

            Self::AnnotationValueCharPostHyphen => {
                "Expected annotation value character must exist after hyphen."
            }

            Self::AnnotationValueChar => "Invalid annotation value character.",

            Self::InvalidMinutePrecisionOffset => "Offset must be minute precision",

            Self::CriticalDuplicateCalendar => {
                "Duplicate calendars cannot be provided when one is critical."
            }

            Self::UnrecognizedCritical => "Unrecognized annoation is marked as critical.",

            Self::TzLeadingChar => "Invalid time zone leading character.",

            Self::IanaCharPostSeparator => "Expected time zone character after '/'.",

            Self::IanaChar => "Invalid IANA time zone character after '/'.",

            Self::UtcTimeSeparator => "Invalid time zone character after '/'.",

            Self::OffsetNeedsSign => "UTC offset needs a sign",

            Self::MonthDayHyphen => "MonthDay must begin with a month or '--'",

            Self::DurationDisgnator => "Invalid duration designator.",

            Self::DurationValueExceededRange => {
                "Provided Duration field value exceeds supported range."
            }

            Self::DateDurationPartOrder => "Invalid date duration part order.",

            Self::TimeDurationPartOrder => "Invalid time duration part order.",

            Self::TimeDurationDesignator => "Invalid time duration designator.",

            Self::AmbiguousTimeMonthDay => "Time is ambiguous with MonthDay",

            Self::AmbiguousTimeYearMonth => "Time is ambiguous with YearMonth",

            Self::InvalidMonthDay => "MonthDay was not valid.",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Self::AbruptEnd { location } = *self {
            write!(f, "Parsing ended abruptly while parsing {location}")
        } else {
            f.write_str(self.to_static_string())
        }
    }
}
