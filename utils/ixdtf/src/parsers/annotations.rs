// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing for `TimeZoneAnnotations` and `KeyValueAnnotations`.

use crate::{
    assert_syntax,
    parsers::{
        grammar::{
            is_a_key_char, is_a_key_leading_char, is_annotation_close,
            is_annotation_key_value_separator, is_annotation_open, is_annotation_value_component,
            is_critical_flag, is_hyphen,
        },
        records::{Annotation, TimeZoneAnnotation},
        timezone, Cursor,
    },
    ParseError, ParserResult,
};

/// Strictly a parsing intermediary for the checking the common annotation backing.
pub(crate) struct AnnotationSet<'a> {
    pub(crate) tz: Option<TimeZoneAnnotation<'a>>,
    pub(crate) calendar: Option<&'a [u8]>,
}

/// Parse a `TimeZoneAnnotation` `Annotations` set
pub(crate) fn parse_annotation_set<'a>(
    cursor: &mut Cursor<'a>,
    handler: impl FnMut(Annotation<'a>) -> Option<Annotation<'a>>,
) -> ParserResult<AnnotationSet<'a>> {
    // Parse the first annotation.
    let tz_annotation = timezone::parse_ambiguous_tz_annotation(cursor)?;

    // Parse any `Annotations`
    let annotations = cursor.check_or(false, is_annotation_open);

    if annotations {
        let calendar = parse_annotations(cursor, handler)?;
        return Ok(AnnotationSet {
            tz: tz_annotation,
            calendar,
        });
    }

    Ok(AnnotationSet {
        tz: tz_annotation,
        calendar: None,
    })
}

/// Parse any number of `KeyValueAnnotation`s
pub(crate) fn parse_annotations<'a>(
    cursor: &mut Cursor<'a>,
    mut handler: impl FnMut(Annotation<'a>) -> Option<Annotation<'a>>,
) -> ParserResult<Option<&'a [u8]>> {
    let mut calendar: Option<Annotation<'a>> = None;

    while cursor.check_or(false, is_annotation_open) {
        let annotation = handler(parse_kv_annotation(cursor)?);

        match annotation {
            // Check if the key is the registered key "u-ca".
            Some(kv) if kv.key == "u-ca".as_bytes() => {
                // Check the calendar
                match calendar {
                    Some(calendar)
                        // if calendars do not match and one of them is critical
                        if calendar.value != kv.value && (calendar.critical || kv.critical) =>
                    {
                        return Err(ParseError::CriticalDuplicateCalendar)
                    }
                    // If there is not yet a calendar, save it.
                    None => {
                        calendar = Some(kv);
                    }
                    _ => {}
                }
            }
            Some(unknown_kv) => {
                // Throw an error on any unrecognized annotations that are marked as critical.
                if unknown_kv.critical {
                    return Err(ParseError::UnrecognizedCritical);
                }
            }
            None => {}
        }
    }

    Ok(calendar.map(|a| a.value))
}

/// Parse an annotation with an `AnnotationKey`=`AnnotationValue` pair.
fn parse_kv_annotation<'a>(cursor: &mut Cursor<'a>) -> ParserResult<Annotation<'a>> {
    assert_syntax!(
        is_annotation_open(cursor.next_or(ParseError::AnnotationOpen)?),
        AnnotationOpen
    );

    let critical = cursor.check_or(false, is_critical_flag);
    cursor.advance_if(critical);

    // Parse AnnotationKey.
    let annotation_key = parse_annotation_key(cursor)?;
    assert_syntax!(
        is_annotation_key_value_separator(cursor.next_or(ParseError::AnnotationKeyValueSeparator)?),
        AnnotationKeyValueSeparator,
    );

    // Parse AnnotationValue.
    let annotation_value = parse_annotation_value(cursor)?;
    assert_syntax!(
        is_annotation_close(cursor.next_or(ParseError::AnnotationClose)?),
        AnnotationClose
    );

    Ok(Annotation {
        critical,
        key: annotation_key,
        value: annotation_value,
    })
}

/// Parse an `AnnotationKey`.
fn parse_annotation_key<'a>(cursor: &mut Cursor<'a>) -> ParserResult<&'a [u8]> {
    let key_start = cursor.pos();
    assert_syntax!(
        is_a_key_leading_char(cursor.next_or(ParseError::AnnotationKeyLeadingChar)?),
        AnnotationKeyLeadingChar,
    );

    while let Some(potential_key_char) = cursor.next() {
        // End of key.
        if cursor.check_or(false, is_annotation_key_value_separator) {
            // Return found key
            return cursor
                .slice(key_start, cursor.pos())
                .ok_or(ParseError::ImplAssert);
        }

        assert_syntax!(is_a_key_char(potential_key_char), AnnotationKeyChar);
    }

    Err(ParseError::AnnotationChar)
}

/// Parse an `AnnotationValue`.
fn parse_annotation_value<'a>(cursor: &mut Cursor<'a>) -> ParserResult<&'a [u8]> {
    let value_start = cursor.pos();
    cursor.advance();
    while let Some(potential_value_char) = cursor.next() {
        if cursor.check_or(false, is_annotation_close) {
            // Return the determined AnnotationValue.
            return cursor
                .slice(value_start, cursor.pos())
                .ok_or(ParseError::ImplAssert);
        }

        if is_hyphen(potential_value_char) {
            assert_syntax!(
                cursor.peek().map_or(false, is_annotation_value_component),
                AnnotationValueCharPostHyphen,
            );
            cursor.advance();
            continue;
        }

        assert_syntax!(
            is_annotation_value_component(potential_value_char),
            AnnotationValueChar,
        );
    }

    Err(ParseError::AnnotationValueChar)
}
