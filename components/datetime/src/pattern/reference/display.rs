// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "datagen")]

//! Set of `Display` implementations for reference and runtime `Pattern`.

use super::{
    super::{GenericPatternItem, PatternItem},
    GenericPattern, Pattern,
};
use alloc::string::String;
use core::fmt::{self, Write};

/// A helper function optimized to dump string buffers into `Pattern`
/// serialization wrapping minimal chunks of the buffer in escaping `'`
/// literals to produce valid UTF35 pattern string.
fn dump_buffer_into_formatter(literal: &str, formatter: &mut fmt::Formatter) -> fmt::Result {
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
                formatter.write_char(*ch)?;
                ch_iter.next();
            } else {
                break;
            }
        }

        // Wrap in "'" and escape "'".
        formatter.write_char('\'')?;
        for ch in ch_iter {
            if ch == '\'' {
                // Escape a single quote.
                formatter.write_char('\\')?;
            }
            formatter.write_char(ch)?;
        }
        formatter.write_char('\'')?;

        // Add the trailing whitespace
        for ch in literal.chars().rev() {
            if ch.is_whitespace() {
                formatter.write_char(ch)?;
            } else {
                break;
            }
        }
    } else {
        formatter.write_str(literal)?;
    }
    Ok(())
}

/// This trait is implemented in order to provide the machinery to convert a [`Pattern`] to a UTS 35
/// pattern string. It could also be implemented as the Writeable trait, but at the time of writing
/// this was not done, as this code would need to implement the [`writeable_length_hint()`] method,
/// which would need to duplicate the branching logic of the [`fmt`](std::fmt) method here. This code
/// is used in generating the data providers and is not as performance sensitive.
impl fmt::Display for Pattern {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for pattern_item in self.items.iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    dump_buffer_into_formatter(&buffer, formatter)?;
                    buffer.clear();
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length.to_len() {
                        formatter.write_char(ch)?;
                    }
                }
                PatternItem::Literal(ch) => {
                    buffer.push(*ch);
                }
            }
        }
        dump_buffer_into_formatter(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}

impl fmt::Display for GenericPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = alloc::string::String::new();
        for pattern_item in self.items.iter() {
            match pattern_item {
                GenericPatternItem::Placeholder(idx) => {
                    dump_buffer_into_formatter(&buffer, formatter)?;
                    buffer.clear();
                    write!(formatter, "{{{idx}}}")?;
                }
                GenericPatternItem::Literal(ch) => {
                    buffer.push(*ch);
                }
            }
        }
        dump_buffer_into_formatter(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}
