// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! ISO8601 specific grammar checks.

/// Checks if char is a `AKeyLeadingChar`.
#[inline]
pub(crate) fn is_a_key_leading_char(ch: &[u8]) -> bool {
    is_ascii_lowercase(ch) || ch == [b'_']
}

/// Checks if char is an `AKeyChar`.
#[inline]
pub(crate) fn is_a_key_char(ch: &[u8]) -> bool {
    is_a_key_leading_char(ch) || is_ascii_digit(ch) || ch == [b'-']
}

/// Checks if char is an `AnnotationValueComponent`.
pub(crate) fn is_annotation_value_component(ch: &[u8]) -> bool {
    is_ascii_digit(ch) || is_ascii_alphabetic(ch)
}

/// Checks if char is a `TZLeadingChar`.
#[inline]
pub(crate) fn is_tz_leading_char(ch: &[u8]) -> bool {
    is_ascii_alphabetic(ch) || ch == [b'_'] || ch == [b'.']
}

/// Checks if char is a `TZChar`.
#[inline]
pub(crate) fn is_tz_char(ch: &[u8]) -> bool {
    is_tz_leading_char(ch) || is_ascii_digit(ch) || ch == [b'-'] || ch == [b'+']
}

/// Checks if char is a `TimeZoneIANAName` Separator.
#[inline]
pub(crate) fn is_tz_name_separator(ch: &[u8]) -> bool {
    ch == [b'/']
}

/// Checks if char is an ascii sign.
#[inline]
pub(crate) fn is_ascii_sign(ch: &[u8]) -> bool {
    ch == [b'+'] || ch == [b'-']
}

/// Checks if char is an ascii sign or U+2212
#[inline]
pub(crate) fn is_sign(ch: &[u8]) -> bool {
    is_ascii_sign(ch) || ch == [0xE2, 0x88, 0x92]
}

/// Checks if char is a `TimeSeparator`.
#[inline]
pub(crate) fn is_time_separator(ch: &[u8]) -> bool {
    ch == [b':']
}

/// Checks if char is a `TimeDesignator`.
#[inline]
pub(crate) fn is_time_designator(ch: &[u8]) -> bool {
    ch == [b'T'] || ch == [b't']
}

#[inline]
/// Checks if char is a space.
pub(crate) fn is_space(ch: &[u8]) -> bool {
    ch == [b' ']
}

/// Checks if char is a `DateTimeSeparator`.
#[inline]
pub(crate) fn is_date_time_separator(ch: &[u8]) -> bool {
    is_time_designator(ch) || is_space(ch)
}

/// Checks if char is a `UtcDesignator`.
#[inline]
pub(crate) fn is_utc_designator(ch: &[u8]) -> bool {
    ch == [b'Z'] || ch == [b'z']
}

/// Checks if char is a `DurationDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_duration_designator(ch: &[u8]) -> bool {
    ch == [b'P'] || ch == [b'p']
}

/// Checks if char is a `YearDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_year_designator(ch: &[u8]) -> bool {
    ch == [b'Y'] || ch == [b'y']
}

/// Checks if char is a `MonthsDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_month_designator(ch: &[u8]) -> bool {
    ch == [b'M'] || ch == [b'm']
}

/// Checks if char is a `WeekDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_week_designator(ch: &[u8]) -> bool {
    ch == [b'W'] || ch == [b'w']
}

/// Checks if char is a `DayDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_day_designator(ch: &[u8]) -> bool {
    ch == [b'D'] || ch == [b'd']
}

/// checks if char is a `DayDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_hour_designator(ch: &[u8]) -> bool {
    ch == [b'H'] || ch == [b'h']
}

/// Checks if char is a `MinuteDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_minute_designator(ch: &[u8]) -> bool {
    is_month_designator(ch)
}

/// checks if char is a `SecondDesignator`.
#[inline]
#[cfg(feature = "duration")]
pub(crate) fn is_second_designator(ch: &[u8]) -> bool {
    ch == [b'S'] || ch == [b's']
}

/// Checks if char is a `DecimalSeparator`.
#[inline]
pub(crate) fn is_decimal_separator(ch: &[u8]) -> bool {
    ch == [b'.'] || ch == [b',']
}

/// Checks if char is an `AnnotationOpen`.
#[inline]
pub(crate) fn is_annotation_open(ch: &[u8]) -> bool {
    ch == [b'[']
}

/// Checks if char is an `AnnotationClose`.
#[inline]
pub(crate) fn is_annotation_close(ch: &[u8]) -> bool {
    ch == [b']']
}

/// Checks if char is an `CriticalFlag`.
#[inline]
pub(crate) fn is_critical_flag(ch: &[u8]) -> bool {
    ch == [b'!']
}

/// Checks if char is the `AnnotationKeyValueSeparator`.
#[inline]
pub(crate) fn is_annotation_key_value_separator(ch: &[u8]) -> bool {
    ch == [b'=']
}

/// Checks if char is a hyphen. Hyphens are used as a Date separator
/// and as a `AttributeValueComponent` separator.
#[inline]
pub(crate) fn is_hyphen(ch: &[u8]) -> bool {
    ch == [b'-']
}

/// Checks if the char is an ascii digit.
pub(crate) fn is_ascii_digit(ch: &[u8]) -> bool {
    ch.first().map_or(false, |d| d.is_ascii_digit())
}

/// Checks if the char is an ascii alpha char.
pub(crate) fn is_ascii_alphabetic(ch: &[u8]) -> bool {
    ch.first().map_or(false, |d| d.is_ascii_alphabetic())
}

/// Checks if the char is an ascii lowercase alpha char.
pub(crate) fn is_ascii_lowercase(ch: &[u8]) -> bool {
    ch.first().map_or(false, |d| d.is_ascii_lowercase())
}
