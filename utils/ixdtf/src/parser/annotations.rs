// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//! Parsing for Parser's `Annotations`.
use crate::{
    assert_syntax,
    parser::{
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

use alloc::string::String;

/// A `KeyValueAnnotation` Parse Node.
#[derive(Debug, Clone)]
pub(crate) struct KeyValueAnnotation {
    /// An `Annotation`'s Key.
    pub(crate) key: String,
    /// An `Annotation`'s value.
    pub(crate) value: String,
    /// Whether the annotation was flagged as critical.
    pub(crate) critical: bool,
}

/// Strictly a Parsing Intermediary for the checking the common annotation backing.
pub(crate) struct AnnotationSet {
    pub(crate) tz: Option<TimeZoneAnnotation>,
    pub(crate) calendar: Option<String>,
}

/// Parse a `TimeZoneAnnotation` `Annotations` set
pub(crate) fn parse_annotation_set(
    zoned: bool,
    cursor: &mut Cursor,
) -> ParserResult<AnnotationSet> {
    // Parse the first annotation.
    let tz_annotation = time_zone::parse_ambiguous_tz_annotation(cursor)?;
    if tz_annotation.is_none() && zoned {
        return Err(ParserError::MissingRequiredTzAnnotation);
    }

    // Parse any `Annotations`
    let annotations = cursor.check_or(false, is_annotation_open);

    if annotations {
        let annotations = parse_annotations(cursor)?;
        return Ok(AnnotationSet {
            tz: tz_annotation,
            calendar: annotations.calendar,
        });
    }

    Ok(AnnotationSet {
        tz: tz_annotation,
        calendar: None,
    })
}

/// An internal crate type to house any recognized annotations that are found.
#[derive(Default)]
pub(crate) struct RecognizedAnnotations {
    pub(crate) calendar: Option<String>,
}

/// Parse any number of `KeyValueAnnotation`s
pub(crate) fn parse_annotations(cursor: &mut Cursor) -> ParserResult<RecognizedAnnotations> {
    let mut annotations = RecognizedAnnotations::default();

    let mut calendar_crit = false;
    while cursor.check_or(false, is_annotation_open) {
        let kv = parse_kv_annotation(cursor)?;

        if &kv.key == "u-ca" {
            if annotations.calendar.is_none() {
                annotations.calendar = Some(kv.value);
                calendar_crit = kv.critical;
                continue;
            }

            if calendar_crit || kv.critical {
                return Err(ParserError::CriticalDuplicateCalendar);
            }
        } else if kv.critical {
            return Err(ParserError::UnrecognizedCritical);
        }
    }

    Ok(annotations)
}

/// Parse an annotation with an `AnnotationKey`=`AnnotationValue` pair.
fn parse_kv_annotation(cursor: &mut Cursor) -> ParserResult<KeyValueAnnotation> {
    assert_syntax!(is_annotation_open(cursor.abrupt_next()?), AnnotationOpen,);

    let critical = cursor.check_or(false, is_critical_flag);
    cursor.advance_if(critical);

    // Parse AnnotationKey.
    let annotation_key = parse_annotation_key(cursor)?;
    assert_syntax!(
        is_annotation_key_value_separator(cursor.abrupt_next()?),
        AnnotationKeyValueSeparator,
    );

    // Parse AnnotationValue.
    let annotation_value = parse_annotation_value(cursor)?;
    assert_syntax!(is_annotation_close(cursor.abrupt_next()?), AnnotationClose,);

    Ok(KeyValueAnnotation {
        key: annotation_key,
        value: annotation_value,
        critical,
    })
}

/// Parse an `AnnotationKey`.
fn parse_annotation_key(cursor: &mut Cursor) -> ParserResult<String> {
    let key_start = cursor.pos();
    assert_syntax!(
        is_a_key_leading_char(cursor.abrupt_next()?),
        AnnotationKeyLeadingChar,
    );

    while let Some(potential_key_char) = cursor.next() {
        // End of key.
        if cursor.check_or(false, is_annotation_key_value_separator) {
            // Return found key
            return Ok(cursor.slice(key_start, cursor.pos()));
        }

        assert_syntax!(is_a_key_char(potential_key_char), AnnotationKeyChar,);
    }

    Err(ParserError::abrupt_end())
}

/// Parse an `AnnotationValue`.
fn parse_annotation_value(cursor: &mut Cursor) -> ParserResult<String> {
    let value_start = cursor.pos();
    cursor.advance();
    while let Some(potential_value_char) = cursor.next() {
        if cursor.check_or(false, is_annotation_close) {
            // Return the determined AnnotationValue.
            return Ok(cursor.slice(value_start, cursor.pos()));
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

    Err(ParserError::AbruptEnd)
}
