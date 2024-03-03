// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//! An error enum for representing `ixdtf` parsing errors.

#[non_exhaustive]
#[derive(PartialEq, Clone, Debug)]
pub enum ParserError {
    ParseFloat,
    AbruptEnd,
    InvalidEnd,
    InvalidMonthRange,
    InvalidDayRange,
    InvalidYearRange,
    DateYear,
    DateExtendedYear,
    DateFourDigitYear,
    DateMonth,
    DateDay,
    DateUnexpectedEnd,
    TimeHour,
    TimeMinute,
    TimeSecond,
    FractionPart,
    DateSeparator,
    TimeSeparator,
    DecimalSeparator,

    // Missing Required components.
    MissingRequiredTzAnnotation,
    MissingRequiredTime,
    MissingUtcOffset,

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
    UnrecognizedCritical,
    CriticalDuplicateCalendar,

    // Time Zone Errors
    TzLeadingChar,
    IanaCharPostSeparator,
    IanaChar,
    UtcTimeSeparator,

    // Duration Errors
    DurationDisgnator,
    DateDurationPartOrder,
    TimeDurationPartOrder,
    TimeDurationDesignator,
}

impl ParserError {
    pub(crate) fn abrupt_end() -> Self {
        ParserError::AbruptEnd
    }
}

impl core::fmt::Display for ParserError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            ParserError::ParseFloat => "Invalid float while parsing fraction part.",
            ParserError::AbruptEnd => "Parsing ended abruptly.",
            ParserError::InvalidEnd => "Invalid chars beyond parsing targets.",
            ParserError::InvalidMonthRange => "Invalid month value provided.",
            ParserError::InvalidDayRange => "Invalid day value for provided month.",
            ParserError::InvalidYearRange => "Invalid year value",
            ParserError::DateExtendedYear => "Invalid extended year value. ",
            ParserError::TimeHour => "Invalid hour value provided.",
            ParserError::TimeMinute => "Invalid minute value provided.",
            ParserError::TimeSecond => "Invalid second value provided.",
            ParserError::FractionPart => "Invalid fraction part provided.",
            ParserError::DateSeparator => "Invalid DateSeparator",
            ParserError::TimeSeparator => "Invalid TimeSeparator",
            ParserError::MissingRequiredTzAnnotation => "Missing required time zone annotation.",
            ParserError::MissingRequiredTime => "Missing required time value.",
            ParserError::MissingUtcOffset => "Missing required UTC offset value.",
            ParserError::InvalidAnnotation => "Invalid annotation found.",
            ParserError::AnnotationOpen => "Invalid AnnotationOpen character provided.",
            ParserError::AnnotationClose => "Invalid AnnotationClosing character provided.",
            ParserError::AnnotationChar => "Invalid annotation character provided.",
            ParserError::AnnotationKeyValueSeparator => {
                "Invalid Annotation KeyValueSeparator found."
            }
            ParserError::AnnotationKeyLeadingChar => {
                "Invalid leading character of an annotation key."
            }
            ParserError::AnnotationKeyChar => "Invalid annotation key character found.",
            ParserError::AnnotationValueCharPostHyphen => {
                "Expected annotation value character after '-'"
            }
            ParserError::AnnotationValueChar => "Invalid annotation value character.",
            ParserError::UnrecognizedCritical => {
                "Unrecognized annotations cannot be flagged as critical."
            }
            ParserError::CriticalDuplicateCalendar => {
                "Duplicate calendar annotations cannot be flagged as critical."
            }
            ParserError::TzLeadingChar => "Invalid time zone leading character.",
            ParserError::IanaCharPostSeparator => "Invalid IANA character found post '/'",
            ParserError::IanaChar => "Invalid IANA character found.",
            ParserError::UtcTimeSeparator => "Invalid UTC TimeSeparator provided.",
            ParserError::DurationDisgnator => {
                "Duration must begin with a valid DurationDesignator character."
            }
            ParserError::DateDurationPartOrder => "DateDuration part was provided out of order.",
            ParserError::TimeDurationPartOrder => "TimeDuration part was provided out of order.",
            ParserError::TimeDurationDesignator => {
                "No values provided after TimeDurationDesignator value."
            }
            _ => "",
        };
        f.write_str(msg)
    }
}
