// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub use super::errors::ParserError;
use crate::parser::{get_subtag_iterator, SubtagIterator};
use crate::subtags;
use crate::LanguageIdentifier;
use alloc::vec::Vec;

#[derive(PartialEq, Clone, Copy)]
pub enum ParserMode {
    LanguageIdentifier,
    Locale,
    Partial,
}

#[derive(PartialEq, Clone, Copy)]
enum ParserPosition {
    Script,
    Region,
    Variant,
}

pub fn parse_language_identifier_from_iter(
    iter: &mut SubtagIterator,
    mode: ParserMode,
) -> Result<LanguageIdentifier, ParserError> {
    let language;
    let mut script = None;
    let mut region = None;
    let mut variants = Vec::new();

    if let Some(subtag) = iter.next() {
        language = subtags::Language::from_bytes(subtag)?;
    } else {
        return Err(ParserError::InvalidLanguage);
    }

    let mut position = ParserPosition::Script;

    while let Some(subtag) = iter.peek() {
        if mode != ParserMode::LanguageIdentifier && subtag.len() == 1 {
            break;
        }

        if position == ParserPosition::Script {
            if let Ok(s) = subtags::Script::from_bytes(subtag) {
                script = Some(s);
                position = ParserPosition::Region;
            } else if let Ok(s) = subtags::Region::from_bytes(subtag) {
                region = Some(s);
                position = ParserPosition::Variant;
            } else if let Ok(v) = subtags::Variant::from_bytes(subtag) {
                if let Err(idx) = variants.binary_search(&v) {
                    variants.insert(idx, v);
                }
                position = ParserPosition::Variant;
            } else if mode == ParserMode::Partial {
                break;
            } else {
                return Err(ParserError::InvalidSubtag);
            }
        } else if position == ParserPosition::Region {
            if let Ok(s) = subtags::Region::from_bytes(subtag) {
                region = Some(s);
                position = ParserPosition::Variant;
            } else if let Ok(v) = subtags::Variant::from_bytes(subtag) {
                if let Err(idx) = variants.binary_search(&v) {
                    variants.insert(idx, v);
                }
                position = ParserPosition::Variant;
            } else if mode == ParserMode::Partial {
                break;
            } else {
                return Err(ParserError::InvalidSubtag);
            }
        } else if let Ok(v) = subtags::Variant::from_bytes(subtag) {
            if let Err(idx) = variants.binary_search(&v) {
                variants.insert(idx, v);
            } else {
                return Err(ParserError::InvalidSubtag);
            }
        } else if mode == ParserMode::Partial {
            break;
        } else {
            return Err(ParserError::InvalidSubtag);
        }
        iter.next();
    }

    Ok(LanguageIdentifier {
        language,
        script,
        region,
        variants: subtags::Variants::from_vec_unchecked(variants),
    })
}

pub fn parse_language_identifier(
    t: &[u8],
    mode: ParserMode,
) -> Result<LanguageIdentifier, ParserError> {
    let mut iter = get_subtag_iterator(t);
    parse_language_identifier_from_iter(&mut iter, mode)
}

pub const fn parse_language_identifier_without_variants_from_iter(
    mut iter: SubtagIterator,
    mode: ParserMode,
) -> Result<
    (
        subtags::Language,
        Option<subtags::Script>,
        Option<subtags::Region>,
    ),
    ParserError,
> {
    let language;
    let mut script = None;
    let mut region = None;

    if let (i, Some((t, start, end))) = iter.next_manual() {
        iter = i;
        match subtags::Language::from_bytes_manual_slice(t, start, end) {
            Ok(l) => language = l,
            Err(e) => return Err(e),
        }
    } else {
        return Err(ParserError::InvalidLanguage);
    }

    let mut position = ParserPosition::Script;

    while let Some((t, start, end)) = iter.peek_manual() {
        if !matches!(mode, ParserMode::LanguageIdentifier) && start - end == 1 {
            break;
        }

        if matches!(position, ParserPosition::Script) {
            if let Ok(s) = subtags::Script::from_bytes_manual_slice(t, start, end) {
                script = Some(s);
                position = ParserPosition::Region;
            } else if let Ok(s) = subtags::Region::from_bytes_manual_slice(t, start, end) {
                region = Some(s);
                position = ParserPosition::Variant;
            } else if subtags::Variant::from_bytes_manual_slice(t, start, end).is_ok() {
                // We cannot handle variants in a const context
                return Err(ParserError::InvalidSubtag);
            } else if matches!(mode, ParserMode::Partial) {
                break;
            } else {
                return Err(ParserError::InvalidSubtag);
            }
        } else if matches!(position, ParserPosition::Region) {
            if let Ok(s) = subtags::Region::from_bytes_manual_slice(t, start, end) {
                region = Some(s);
                position = ParserPosition::Variant;
            } else if subtags::Variant::from_bytes_manual_slice(t, start, end).is_ok() {
                // We cannot handle variants in a const context
                return Err(ParserError::InvalidSubtag);
            } else if matches!(mode, ParserMode::Partial) {
                break;
            } else {
                return Err(ParserError::InvalidSubtag);
            }
        } else if subtags::Variant::from_bytes_manual_slice(t, start, end).is_ok() {
            // We cannot handle variants in a const context
            return Err(ParserError::InvalidSubtag);
        } else if matches!(mode, ParserMode::Partial) {
            break;
        } else {
            return Err(ParserError::InvalidSubtag);
        }

        iter = iter.next_manual().0;
    }

    Ok((language, script, region))
}

pub const fn parse_language_identifier_without_variants(
    t: &[u8],
    mode: ParserMode,
) -> Result<
    (
        subtags::Language,
        Option<subtags::Script>,
        Option<subtags::Region>,
    ),
    ParserError,
> {
    let iter = get_subtag_iterator(t);
    parse_language_identifier_without_variants_from_iter(iter, mode)
}
