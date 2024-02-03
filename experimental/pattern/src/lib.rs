// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::fmt::{self, Write};
use writeable::{PartsWrite, Writeable};

#[cfg(feature = "alloc")]
use alloc::string::String;

const MIN_PLACEHOLDER_CODE_POINT: usize = 1;
const MAX_PLACEHOLDER_CHAR: char = '\x06';
const MAX_PLACEHOLDER_CODE_POINT: usize = MAX_PLACEHOLDER_CHAR as usize;
const PLACEHOLDER_RANGE_SIZE: usize = MAX_PLACEHOLDER_CODE_POINT - MIN_PLACEHOLDER_CODE_POINT;

pub struct NumericPlaceholderPattern<Store: ?Sized> {
    store: Store,
}

pub enum NumericPlaceholderPatternItem<'a> {
    Literal(&'a str),
    Placeholder(usize),
}

impl<Store> NumericPlaceholderPattern<Store>
where
    Store: ?Sized + AsRef<str>,
{
    pub fn iter(&self) -> impl Iterator<Item = NumericPlaceholderPatternItem> + '_ {
        NumericPlaceholderPatternIterator {
            inner: self.store.as_ref().char_indices().peekable(),
            full_str: self.store.as_ref(),
        }
    }

    pub fn as_borrowed_store(&self) -> NumericPlaceholderPattern<&str> {
        NumericPlaceholderPattern {
            store: self.store.as_ref(),
        }
    }

    pub fn interpolate_with<P: NumericPlaceholderProvider>(
        &self,
        replacements: P,
        part: writeable::Part,
    ) -> WriteableNumericPlaceholderPatternWithPart<P> {
        WriteableNumericPlaceholderPatternWithPart {
            pattern: self.as_borrowed_store(),
            replacements,
            part,
        }
    }
}

struct NumericPlaceholderPatternIterator<'a> {
    // TODO: Make this better when `CharIndices::offset` is stabilized:
    // <https://github.com/rust-lang/rust/issues/83871>
    inner: core::iter::Peekable<core::str::CharIndices<'a>>,
    // This is accessible via `CharIndices::as_str` but we can't call that
    // function since the `CharIndices` is inside a `Peekable`
    full_str: &'a str,
}

impl<'a> Iterator for NumericPlaceholderPatternIterator<'a> {
    type Item = NumericPlaceholderPatternItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let &(start_index, start_ch) = self.inner.peek()?;
        if start_ch as usize > MAX_PLACEHOLDER_CODE_POINT {
            // Next item is a literal. Consume the string to the end or a placeholder.
            let end_index = loop {
                self.inner.next();
                match self.inner.peek() {
                    Some((_, ch)) if *ch as usize > MAX_PLACEHOLDER_CODE_POINT => {
                        // Consume another character
                        continue;
                    }
                    Some((index, _)) => {
                        // Reached a placeholder; pause here
                        break *index;
                    }
                    None => {
                        // Reached end of string
                        break self.full_str.len();
                    }
                }
            };
            self.full_str
                .get(start_index..end_index)
                .map(NumericPlaceholderPatternItem::Literal)
        } else {
            // Next item is a placeholder.
            let mut placeholder_number = 0;
            loop {
                let (_, ch) = self.inner.next()?;
                let ch_usize = ch as usize;
                debug_assert!(
                    ch_usize <= MAX_PLACEHOLDER_CODE_POINT,
                    "invalid code point in placeholder pattern: {:?}",
                    self.full_str.as_bytes()
                );
                placeholder_number += ch_usize - MIN_PLACEHOLDER_CODE_POINT;
                if ch_usize != MAX_PLACEHOLDER_CODE_POINT {
                    break;
                }
            }
            Some(NumericPlaceholderPatternItem::Placeholder(
                placeholder_number,
            ))
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a> FromIterator<NumericPlaceholderPatternItem<'a>> for NumericPlaceholderPattern<String> {
    fn from_iter<T: IntoIterator<Item = NumericPlaceholderPatternItem<'a>>>(iter: T) -> Self {
        let mut pattern_str = String::new();
        for item in iter {
            match item {
                NumericPlaceholderPatternItem::Literal(s) => {
                    pattern_str.push_str(s);
                }
                NumericPlaceholderPatternItem::Placeholder(number) => {
                    let mut remainder = number;
                    loop {
                        if remainder < MAX_PLACEHOLDER_CODE_POINT - MIN_PLACEHOLDER_CODE_POINT {
                            debug_assert!(
                                remainder - MIN_PLACEHOLDER_CODE_POINT <= u8::MAX as usize
                            );
                            let u8_to_push = (remainder - MIN_PLACEHOLDER_CODE_POINT) as u8;
                            pattern_str.push(char::from(u8_to_push));
                        } else {
                            pattern_str.push(MAX_PLACEHOLDER_CHAR);
                            remainder -= PLACEHOLDER_RANGE_SIZE;
                        }
                    }
                }
            }
        }
        NumericPlaceholderPattern { store: pattern_str }
    }
}

pub trait NumericPlaceholderProvider {
    type Writeable<'a>: Writeable
    where
        Self: 'a;

    fn replacement_for<'a>(&'a self, number: usize) -> Self::Writeable<'a>;
}

pub struct WriteableNumericPlaceholderPatternWithPart<'a, P> {
    pattern: NumericPlaceholderPattern<&'a str>,
    replacements: P,
    part: writeable::Part,
}

impl<P> Writeable for WriteableNumericPlaceholderPatternWithPart<'_, P>
where
    P: NumericPlaceholderProvider,
{
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        for item in self.pattern.iter() {
            match item {
                NumericPlaceholderPatternItem::Literal(s) => {
                    sink.with_part(self.part, |w| w.write_str(s))?;
                }
                NumericPlaceholderPatternItem::Placeholder(number) => {
                    self.replacements
                        .replacement_for(number)
                        .write_to_parts(sink)?;
                }
            }
        }
        Ok(())
    }
}
