// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Set of `Display` implementations for reference and runtime `Pattern`.

use super::{
    super::{GenericPatternItem, PatternItem},
    generic::{GenericPattern, GenericPatternIterator},
    Pattern,
};
use alloc::string::String;
use core::fmt::{self, Write};

/// A helper function optimized to dump string buffers into `Pattern`
/// serialization wrapping minimal chunks of the buffer in escaping `'`
/// literals to produce valid UTF35 pattern string.
pub(crate) fn write_buffer<W: fmt::Write>(literal: &str, writer: &mut W) -> fmt::Result {
    if literal.is_empty() {
        return Ok(());
    }
    // Determine if the literal contains any characters that would need to be escaped.
    let mut needs_escaping = false;
    for ch in literal.chars() {
        if ch.is_ascii_alphabetic() || ch == '\'' {
            needs_escaping = true;
            break;
        }
    }

    if needs_escaping {
        let mut ch_iter = literal.trim_end().chars().peekable();

        // Do not escape the leading whitespace.
        while let Some(ch) = ch_iter.peek() {
            if ch.is_whitespace() {
                writer.write_char(*ch)?;
                ch_iter.next();
            } else {
                break;
            }
        }

        // Wrap in "'" and escape "'".
        writer.write_char('\'')?;
        for ch in ch_iter {
            if ch == '\'' {
                // Escape a single quote.
                writer.write_char('\\')?;
            }
            writer.write_char(ch)?;
        }
        writer.write_char('\'')?;

        // Add the trailing whitespace
        for ch in literal.chars().rev() {
            if ch.is_whitespace() {
                writer.write_char(ch)?;
            } else {
                break;
            }
        }
    } else {
        writer.write_str(literal)?;
    }
    Ok(())
}

/// This trait is implemented in order to provide the machinery to convert a [`Pattern`] to a UTS 35
/// pattern string. It could also be implemented as the Writeable trait, but at the time of writing
/// this was not done, as this code would need to implement the [`write_len()`] method, which would
/// need to duplicate the branching logic of the [`fmt`](std::fmt) method here. This code is used in generating
/// the data providers and is not as performance sensitive.
impl fmt::Display for Pattern {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for pattern_item in self.items.iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    write_buffer(&buffer, formatter)?;
                    buffer.clear();
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        formatter.write_char(ch)?;
                    }
                }
                PatternItem::Literal(ch) => {
                    buffer.push(*ch);
                }
            }
        }
        write_buffer(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}

impl fmt::Display for GenericPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for pattern_item in &self.items {
            match pattern_item {
                GenericPatternItem::Date => {
                    write_buffer(&buffer, formatter)?;
                    buffer.clear();
                    formatter.write_char('{')?;
                    formatter.write_char('0')?;
                    formatter.write_char('}')?;
                }
                GenericPatternItem::Time => {
                    write_buffer(&buffer, formatter)?;
                    buffer.clear();
                    formatter.write_char('{')?;
                    formatter.write_char('1')?;
                    formatter.write_char('}')?;
                }
                GenericPatternItem::Literal(ch) => {
                    buffer.push(*ch);
                }
            }
        }
        write_buffer(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}

impl GenericPatternIterator<'_> {
    pub fn into_string(self) -> String {
        let mut result = String::new();
        let mut buffer = String::new();
        for pattern_item in self {
            match pattern_item {
                PatternItem::Field(field) => {
                    write_buffer(&buffer, &mut result).expect("Failed to write to buffer");
                    buffer.clear();
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        result.write_char(ch).expect("Failed to write to buffer");
                    }
                }
                PatternItem::Literal(ch) => {
                    buffer.push(ch);
                }
            }
        }
        write_buffer(&buffer, &mut result).expect("Failed to write to buffer");
        buffer.clear();
        result
    }
}
