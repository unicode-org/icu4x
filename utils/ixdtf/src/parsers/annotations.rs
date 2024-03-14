// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing for Parser's `Annotations`.

use crate::{
    assert_syntax,
    parsers::{
        grammar::{
            is_a_key_char, is_a_key_leading_char, is_annotation_close,
            is_annotation_key_value_separator, is_annotation_open, is_annotation_value_component,
            is_critical_flag, is_hyphen,
        },
        time_zone,
        time_zone::TimeZoneAnnotation,
        Cursor,
    },
    ParserError, ParserResult,
};

use alloc::vec::Vec;

use super::records::Annotation;

/// Strictly a parsing intermediary for the checking the common annotation backing.
pub(crate) struct AnnotationSet<'a> {
    pub(crate) tz: Option<TimeZoneAnnotation<'a>>,
    pub(crate) calendar: Option<&'a str>,
    pub(crate) annotations: Vec<Annotation<'a>>,
}

/// Parse a `TimeZoneAnnotation` `Annotations` set
pub(crate) fn parse_annotation_set<'a>(cursor: &mut Cursor<'a>) -> ParserResult<AnnotationSet<'a>> {
    // Parse the first annotation.
    let tz_annotation = time_zone::parse_ambiguous_tz_annotation(cursor)?;

    // Parse any `Annotations`
    let annotations = cursor.check_or(false, is_annotation_open);

    if annotations {
        let annotations = parse_annotations(cursor)?;
        return Ok(AnnotationSet {
            tz: tz_annotation,
            calendar: annotations.0,
            annotations: annotations.1,
        });
    }

    Ok(AnnotationSet {
        tz: tz_annotation,
        calendar: None,
        annotations: Vec::default(),
    })
}

/// Parse any number of `KeyValueAnnotation`s
pub(crate) fn parse_annotations<'a>(
    cursor: &mut Cursor<'a>,
) -> ParserResult<(Option<&'a str>, Vec<Annotation<'a>>)> {
    let mut annotations = Vec::default();
    let mut calendar = None;
    let mut calendar_crit = false;

    while cursor.check_or(false, is_annotation_open) {
        let kv = parse_kv_annotation(cursor)?;

        if kv.key == "u-ca" {
            if calendar.is_none() {
                calendar = Some(kv.value);
                calendar_crit = kv.critical;
                continue;
            }

            if calendar_crit || kv.critical {
                return Err(ParserError::CriticalDuplicateCalendar);
            }
        }

        annotations.push(kv);
    }

    Ok((calendar, annotations))
}

/// Parse an annotation with an `AnnotationKey`=`AnnotationValue` pair.
fn parse_kv_annotation<'a>(cursor: &mut Cursor<'a>) -> ParserResult<Annotation<'a>> {
    assert_syntax!(
        is_annotation_open(cursor.next_or(ParserError::AnnotationOpen)?),
        AnnotationOpen
    );

    let critical = cursor.check_or(false, is_critical_flag);
    cursor.advance_if(critical);

    // Parse AnnotationKey.
    let annotation_key = parse_annotation_key(cursor)?;
    assert_syntax!(
        is_annotation_key_value_separator(
            cursor.next_or(ParserError::AnnotationKeyValueSeparator)?
        ),
        AnnotationKeyValueSeparator,
    );

    // Parse AnnotationValue.
    let annotation_value = parse_annotation_value(cursor)?;
    assert_syntax!(
        is_annotation_close(cursor.next_or(ParserError::AnnotationClose)?),
        AnnotationClose
    );

    Ok(Annotation {
        critical,
        key: annotation_key,
        value: annotation_value,
    })
}

/// Parse an `AnnotationKey`.
fn parse_annotation_key<'a>(cursor: &mut Cursor<'a>) -> ParserResult<&'a str> {
    let key_start = cursor.pos();
    assert_syntax!(
        is_a_key_leading_char(cursor.next_or(ParserError::AnnotationKeyLeadingChar)?),
        AnnotationKeyLeadingChar,
    );

    while let Some(potential_key_char) = cursor.next() {
        // End of key.
        if cursor.check_or(false, is_annotation_key_value_separator) {
            // Return found key
            return cursor
                .slice(key_start, cursor.pos())
                .ok_or(ParserError::ImplAssert);
        }

        assert_syntax!(is_a_key_char(potential_key_char), AnnotationKeyChar);
    }

    Err(ParserError::AnnotationChar)
}

/// Parse an `AnnotationValue`.
fn parse_annotation_value<'a>(cursor: &mut Cursor<'a>) -> ParserResult<&'a str> {
    let value_start = cursor.pos();
    cursor.advance();
    while let Some(potential_value_char) = cursor.next() {
        if cursor.check_or(false, is_annotation_close) {
            // Return the determined AnnotationValue.
            return cursor
                .slice(value_start, cursor.pos())
                .ok_or(ParserError::ImplAssert);
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

    Err(ParserError::AnnotationValueChar)
}
