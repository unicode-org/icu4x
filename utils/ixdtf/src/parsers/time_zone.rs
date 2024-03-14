// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing for Time Zone and Offset data.

use super::{
    grammar::{
        is_a_key_char, is_a_key_leading_char, is_annotation_close,
        is_annotation_key_value_separator, is_annotation_open, is_critical_flag, is_sign,
        is_time_separator, is_tz_char, is_tz_leading_char, is_tz_name_separator, is_utc_designator,
    },
    records::{TimeZone, UTCOffset},
    time::{parse_fraction_component, parse_hour, parse_minute_second},
    Cursor,
};
use crate::{assert_syntax, ParserError, ParserResult};

// NOTE: critical field on time zones is captured but not handled.
/// A `TimeZoneAnnotation` is an internal annotation struct.
#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct TimeZoneAnnotation<'a> {
    /// Critical Flag for the annotation.
    pub(crate) critical: bool,
    /// TimeZone Data
    pub(crate) tz: TimeZone<'a>,
}

// ==== Time Zone Annotation Parsing ====

pub(crate) fn parse_ambiguous_tz_annotation<'a>(
    cursor: &mut Cursor<'a>,
) -> ParserResult<Option<TimeZoneAnnotation<'a>>> {
    // Peek position + 1 to check for critical flag.
    let mut current_peek = 1;
    let critical = cursor
        .peek_n_char(current_peek)
        .map(is_critical_flag)
        .ok_or(ParserError::abrupt_end("AmbiguousAnnotation"))?;

    // Advance cursor if critical flag present.
    if critical {
        current_peek += 1;
    }

    let leading_char = cursor
        .peek_n_char(current_peek)
        .ok_or(ParserError::abrupt_end("AmbiguousAnnotation"))?;

    if is_tz_leading_char(leading_char) || is_sign(leading_char) {
        // Ambigious start values when lowercase alpha that is shared between `TzLeadingChar` and `KeyLeadingChar`.
        if is_a_key_leading_char(leading_char) {
            let mut peek_pos = current_peek + 1;
            while let Some(ch) = cursor.peek_n_char(peek_pos) {
                if is_tz_name_separator(ch) || (is_tz_char(ch) && !is_a_key_char(ch)) {
                    let tz = parse_tz_annotation(cursor)?;
                    return Ok(Some(tz));
                } else if is_annotation_key_value_separator(ch)
                    || (is_a_key_char(ch) && !is_tz_char(ch))
                {
                    return Ok(None);
                } else if is_annotation_close(ch) {
                    return Err(ParserError::InvalidAnnotation);
                }

                peek_pos += 1;
            }
            return Err(ParserError::abrupt_end("AmbiguousAnnotation"));
        }
        let tz = parse_tz_annotation(cursor)?;
        return Ok(Some(tz));
    }

    if is_a_key_leading_char(leading_char) {
        return Ok(None);
    };

    Err(ParserError::AnnotationChar)
}

fn parse_tz_annotation<'a>(cursor: &mut Cursor<'a>) -> ParserResult<TimeZoneAnnotation<'a>> {
    assert_syntax!(
        is_annotation_open(cursor.next_or(ParserError::AnnotationOpen)?),
        AnnotationOpen
    );

    let critical = cursor.check_or(false, is_critical_flag);
    cursor.advance_if(critical);

    let tz = parse_time_zone(cursor)?;

    assert_syntax!(
        is_annotation_close(cursor.next_or(ParserError::AnnotationClose)?),
        AnnotationClose
    );

    Ok(TimeZoneAnnotation { critical, tz })
}

/// Parses the [`TimeZoneIdentifier`][tz] node.
///
/// [tz]: https://tc39.es/proposal-temporal/#prod-TimeZoneIdentifier
pub(crate) fn parse_time_zone<'a>(cursor: &mut Cursor<'a>) -> ParserResult<TimeZone<'a>> {
    let is_iana = cursor
        .check(is_tz_leading_char)
        .ok_or(ParserError::abrupt_end("TimeZoneAnnotation"))?;
    let is_offset = cursor.check_or(false, is_sign);

    if is_iana {
        return parse_tz_iana_name(cursor);
    } else if is_offset {
        let (offset, _) = parse_utc_offset_minute_precision(cursor)?;
        return Ok(TimeZone {
            name: None,
            offset: Some(offset),
        });
    }

    Err(ParserError::TzLeadingChar)
}

/// Parse a `TimeZoneIANAName` Parse Node
fn parse_tz_iana_name<'a>(cursor: &mut Cursor<'a>) -> ParserResult<TimeZone<'a>> {
    let tz_name_start = cursor.pos();
    while let Some(potential_value_char) = cursor.next() {
        if cursor.check_or(false, is_annotation_close) {
            // Return the valid TimeZoneIANAName
            return Ok(TimeZone {
                name: cursor.slice(tz_name_start, cursor.pos()),
                offset: None,
            });
        }

        if is_tz_name_separator(potential_value_char) {
            assert_syntax!(cursor.check_or(false, is_tz_char), IanaCharPostSeparator,);
            continue;
        }

        assert_syntax!(is_tz_char(potential_value_char), IanaChar,);
    }

    Err(ParserError::abrupt_end("IANATimeZoneName"))
}

// ==== Utc Offset Parsing ====

/// Parse a full precision `UtcOffset`
pub(crate) fn parse_date_time_utc<'a>(cursor: &mut Cursor) -> ParserResult<TimeZone<'a>> {
    if cursor.check_or(false, is_utc_designator) {
        cursor.advance();
        return Ok(TimeZone {
            name: Some("UTC"),
            offset: None,
        });
    }

    let separated = cursor.peek_n_char(3).map_or(false, is_time_separator);

    let (mut utc_to_minute, parsed_minute) = parse_utc_offset_minute_precision(cursor)?;

    if cursor.check_or(false, is_time_separator) && !separated {
        return Err(ParserError::UtcTimeSeparator);
    }
    cursor.advance_if(cursor.check_or(false, is_time_separator));

    // Return early on only hour offset parse, None, or AnnotationOpen.
    if cursor.check_or(true, is_annotation_open) || !parsed_minute {
        return Ok(TimeZone {
            name: None,
            offset: Some(utc_to_minute),
        });
    }

    // If `UtcOffsetWithSubMinuteComponents`, continue parsing.
    utc_to_minute.second = parse_minute_second(cursor, true)?;

    let (millisecond, microsecond, nanosecond) = parse_fraction_component(cursor)?;

    utc_to_minute.millisecond = millisecond;
    utc_to_minute.microsecond = microsecond;
    utc_to_minute.nanosecond = nanosecond;

    Ok(TimeZone {
        name: None,
        offset: Some(utc_to_minute),
    })
}

/// Parse an `UtcOffsetMinutePrecision` node
///
/// Returns the offset and whether the utc parsing includes a minute.
pub(crate) fn parse_utc_offset_minute_precision(
    cursor: &mut Cursor,
) -> ParserResult<(UTCOffset, bool)> {
    let sign = if cursor.check_or(false, is_sign) {
        if cursor.next_or(ParserError::ImplAssert)? == '+' {
            1
        } else {
            -1
        }
    } else {
        1
    };
    let hour = parse_hour(cursor)?;

    // If at the end of the utc, then return.
    if !cursor.check_or(false, |ch| ch.is_ascii_digit() || is_time_separator(ch)) {
        return Ok((
            UTCOffset {
                sign,
                hour,
                minute: 0,
                second: 0,
                millisecond: 0,
                microsecond: 0,
                nanosecond: 0,
            },
            false,
        ));
    }
    // Advance cursor beyond any TimeSeparator
    cursor.advance_if(cursor.check_or(false, is_time_separator));

    let minute = parse_minute_second(cursor, false)?;

    Ok((
        UTCOffset {
            sign,
            hour,
            minute,
            second: 0,
            millisecond: 0,
            microsecond: 0,
            nanosecond: 0,
        },
        true,
    ))
}
