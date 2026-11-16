// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing for Time Zone and Offset data.

use super::{
    grammar::{
        is_a_key_leading_char, is_annotation_close, is_annotation_key_value_separator,
        is_annotation_open, is_ascii_sign, is_critical_flag, is_time_separator, is_tz_char,
        is_tz_leading_char, is_tz_name_separator, is_utc_designator,
    },
    time::{parse_fraction, parse_hour, parse_minute_second},
    Cursor,
};
use crate::{
    assert_syntax,
    encoding::EncodingType,
    records::{
        FullPrecisionOffset, MinutePrecisionOffset, Sign, TimeZoneAnnotation, TimeZoneRecord,
        UtcOffsetRecord, UtcOffsetRecordOrZ,
    },
    ParseError, ParserResult,
};

// NOTE: critical field on time zones is captured but not handled.

// ==== Time Zone Annotation Parsing ====

/// We support two kinds of annotations here: annotations (e.g. `[u-ca=foo]`)
/// and "time zone annotations" (`[UTC]` or `[+05:30]`)
///
/// When parsing bracketed contents, we need to figure out which one we're dealing with.
///
/// This function returns a time zone annotation if we are dealing with a time zone,
/// otherwise it returns None (and the caller must handle non-tz annotations).
pub(crate) fn parse_ambiguous_tz_annotation<'a, T: EncodingType>(
    cursor: &mut Cursor<'a, T>,
) -> ParserResult<Option<TimeZoneAnnotation<'a, T>>> {
    cursor.speculative_parse(parse_tz_annotation)
}

fn parse_tz_annotation<'a, T: EncodingType>(
    cursor: &mut Cursor<'a, T>,
) -> ParserResult<Option<TimeZoneAnnotation<'a, T>>> {
    if cursor.next_if(is_annotation_open)?.is_none() {
        return Ok(None);
    }

    let critical = cursor.next_if(is_critical_flag)?.is_some();

    // Ambiguous start values when lowercase alpha that is shared between `TzLeadingChar` and `KeyLeadingChar`.
    let mut peek = *cursor;
    let leading_char = peek.next_or(ParseError::AbruptEnd {
        location: "AmbiguousAnnotation",
    })?;

    if is_a_key_leading_char(leading_char) {
        // Go through looking for `=` or `]`
        loop {
            let ch = peek.next_or(ParseError::AbruptEnd {
                location: "AmbiguousAnnotation",
            })?;
            if is_annotation_key_value_separator(ch) {
                // We have an `=` sign, this is a non-tz annotation
                return Ok(None);
            } else if is_annotation_close(ch) {
                // We found a `]` without an `=`, this is a time zone
                break;
            }
        }
    }

    let tz = parse_time_zone(cursor)?;

    assert_syntax!(
        cursor.next_if(is_annotation_close)?.is_some(),
        AnnotationClose
    );

    Ok(Some(TimeZoneAnnotation { critical, tz }))
}

/// Parses the [`TimeZoneIdentifier`][tz] node.
///
/// [tz]: https://tc39.es/proposal-temporal/#prod-TimeZoneIdentifier
pub(crate) fn parse_time_zone<'a, T: EncodingType>(
    cursor: &mut Cursor<'a, T>,
) -> ParserResult<TimeZoneRecord<'a, T>> {
    if let Some(offset) = parse_utc_offset_minute_precision_strict(cursor)? {
        return Ok(TimeZoneRecord::Offset(offset));
    }
    Ok(TimeZoneRecord::Name(parse_tz_iana_name(cursor)?))
}

/// Parse a `TimeZoneIANAName` Parse Node
pub(crate) fn parse_tz_iana_name<'a, T: EncodingType>(
    cursor: &mut Cursor<'a, T>,
) -> ParserResult<&'a [T::CodeUnit]> {
    let tz_name_start = cursor.pos();
    let leading_char = cursor.next_or(ParseError::AbruptEnd {
        location: "TimeZoneAnnotation",
    })?;
    assert_syntax!(is_tz_leading_char(leading_char), TzLeadingChar);
    while let Some(potential_value_char) = cursor.next_if(|ch| !is_annotation_close(ch))? {
        if is_tz_name_separator(potential_value_char) {
            assert_syntax!(cursor.next_if(is_tz_char)?.is_some(), IanaCharPostSeparator,);
            continue;
        }

        assert_syntax!(is_tz_char(potential_value_char), IanaChar);
    }

    cursor
        .slice(tz_name_start, cursor.pos())
        .ok_or(ParseError::ImplAssert)
}

// ==== Utc Offset Parsing ====

/// Parses a potentially full precision UTC offset or Z
pub(crate) fn parse_date_time_utc_offset<T: EncodingType>(
    cursor: &mut Cursor<T>,
) -> ParserResult<Option<UtcOffsetRecordOrZ>> {
    if cursor.next_if(is_utc_designator)?.is_some() {
        return Ok(Some(UtcOffsetRecordOrZ::Z));
    }

    let utc_offset = parse_utc_offset(cursor)?;
    Ok(utc_offset.map(UtcOffsetRecordOrZ::Offset))
}

/// Parse a potentially full precision `UtcOffset`
pub(crate) fn parse_utc_offset<T: EncodingType>(
    cursor: &mut Cursor<T>,
) -> ParserResult<Option<UtcOffsetRecord>> {
    let Some((minute_precision_offset, separated)) = parse_utc_offset_minute_precision(cursor)?
    else {
        return Ok(None);
    };

    let second_separated = cursor.next_if(is_time_separator)?.is_some();
    let second = match (separated, second_separated) {
        (true, true) | (false, false) => parse_minute_second(cursor, false, second_separated)?,
        (false, true) => return Err(ParseError::UtcTimeSeparator),
        (true, false) => {
            assert_syntax!(
                cursor.next_if(|ch| ch.is_ascii_digit())?.is_none(),
                UtcTimeSeparator
            );
            None
        }
    };

    // If `UtcOffsetWithSubMinuteComponents`, continue parsing.
    let Some(second) = second else {
        return Ok(Some(UtcOffsetRecord::MinutePrecision(
            minute_precision_offset,
        )));
    };

    let fraction = parse_fraction(cursor)?;

    Ok(Some(UtcOffsetRecord::FullPrecisionOffset(
        FullPrecisionOffset {
            minute_precision_offset,
            second,
            fraction,
        },
    )))
}

pub(crate) fn parse_utc_offset_minute_precision_strict<T: EncodingType>(
    cursor: &mut Cursor<T>,
) -> ParserResult<Option<MinutePrecisionOffset>> {
    let Some((offset, _)) = parse_utc_offset_minute_precision(cursor)? else {
        return Ok(None);
    };
    if cursor
        .next_if(|ch| is_time_separator(ch) || ch.is_ascii_digit())?
        .is_some()
    {
        return Err(ParseError::InvalidMinutePrecisionOffset);
    }
    Ok(Some(offset))
}

/// Parse an `UtcOffsetMinutePrecision` node
///
/// Returns the offset and whether the utc parsing includes a minute separator.
pub(crate) fn parse_utc_offset_minute_precision<T: EncodingType>(
    cursor: &mut Cursor<T>,
) -> ParserResult<Option<(MinutePrecisionOffset, bool)>> {
    // https://www.rfc-editor.org/rfc/rfc3339#section-5.6
    let Some(sign) = cursor.next_if(is_ascii_sign)? else {
        return Ok(None);
    };
    let sign = Sign::from(sign == b'+');

    let hour = parse_hour(cursor)?;

    // Advance cursor beyond any TimeSeparator
    let separated = cursor.next_if(is_time_separator)?.is_some();

    // If at the end of the utc, then return.
    let Some(minute) = parse_minute_second(cursor, false, separated)? else {
        let offset = MinutePrecisionOffset {
            sign,
            hour,
            minute: 0,
        };
        return Ok(Some((offset, false)));
    };

    Ok(Some((
        MinutePrecisionOffset { sign, hour, minute },
        separated,
    )))
}
