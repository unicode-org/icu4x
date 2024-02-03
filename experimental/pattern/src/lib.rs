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

/// A string pattern with simple numeric placeholders, like `"{0} and {1}"`
///
/// # Backing Store
///
/// The data structure has a flexible backing data store. The only requirement for most
/// functionality is that it implement `AsRef<str>`.
///
/// # Pattern String Encoding Details
///
/// The pattern is stored as a string with the following encoding:
///
/// - String literals are stored as regular UTF-8.
/// - Placeholder numbers are stored in the context where they occur as varints.
///
/// The placeholder varint encoding is as follows:
///
/// - Code point `\x06` = add 5 and read the next code point.
/// - Code points `\x01 - \x05` = add the code point value minus 1 and stop.
///
/// For example, the placeholder 12 is encoded as `\x06\x06\x03`, or 5+5+2.
///
/// Consequences of this encoding:
///
/// 1. String literals cannot contain code points in the range `\x01` through `\x06`. If they do,
///    GIGO behavior will occur.
/// 2. Small numeric placeholder values are stored much more efficiently than large ones.
/// 3. The same numeric placeholder value could occur multiple times in the same pattern.
///
/// # Examples
///
/// ```
/// use icu_pattern_2::NumericPlaceholderPattern;
/// use writeable::assert_writeable_eq;
///
/// let pattern = NumericPlaceholderPattern::from_store("Hello, \x01 and \x02!");
///
/// assert_writeable_eq!(
///     pattern.interpolate_with(&["Alice", "Bob"], writeable::Part { category: "foo", value: "bar" }),
///     "Hello, Alice and Bob!"
/// );
/// ```
pub struct NumericPlaceholderPattern<Store: ?Sized> {
    store: Store,
}

/// An item in a [`NumericPlaceholderPattern`]. Items are either string literals or placeholders.
#[allow(clippy::exhaustive_enums)] // documented structure
pub enum NumericPlaceholderPatternItem<'a> {
    /// A string literal. This can occur in one of three places:
    ///
    /// 1. Between the start of the string and the first placeholder (prefix)
    /// 2. Between two placeholders (infix)
    /// 3. Between the final placeholder and the end of the string (suffix)
    Literal(&'a str),
    /// A numeric placeholder. The `usize` is the placeholder number.
    Placeholder(usize),
}

impl<Store> NumericPlaceholderPattern<Store> {
    pub fn from_store(store: Store) -> Self {
        Self { store }
    }
    pub fn take_store(self) -> Store {
        self.store
    }
}

impl<Store> NumericPlaceholderPattern<Store>
where
    Store: ?Sized + AsRef<str>,
{
    /// Iterate over the contents of this pattern string, returning instances of
    /// [`NumericPlaceholderPatternItem`].
    pub fn iter(&self) -> impl Iterator<Item = NumericPlaceholderPatternItem> + '_ {
        NumericPlaceholderPatternIterator {
            inner: self.store.as_ref().char_indices().peekable(),
            full_str: self.store.as_ref(),
        }
    }

    /// Convert this pattern's store to `&str`.
    pub fn as_borrowed_store(&self) -> NumericPlaceholderPattern<&str> {
        NumericPlaceholderPattern {
            store: self.store.as_ref(),
        }
    }

    /// Returns a [`Writeable`] that interpolates items from the given replacement provider
    /// into this pattern string.
    pub fn interpolate_with<'a, P>(
        &'a self,
        replacements: P,
        part: writeable::Part,
    ) -> impl Writeable + fmt::Display + 'a
    where
        P: NumericPlaceholderProvider + 'a,
    {
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
                    "invalid code point in numeric placeholder pattern: {:?}",
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
                    if cfg!(debug_assertions) {
                        for ch in s.chars() {
                            let ch_usize = ch as usize;
                            debug_assert!(
                                ch_usize < MIN_PLACEHOLDER_CODE_POINT || ch_usize > MAX_PLACEHOLDER_CODE_POINT,
                                "invalid code point in string being converted to numeric placeholder pattern: {:?}",
                                s.as_bytes()
                            );
                        }
                    }
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

/// A type that returns [`Writeable`]s for interpolating into a [`NumericPlaceholderPattern`].
pub trait NumericPlaceholderProvider {
    type W<'a>: Writeable
    where
        Self: 'a;

    /// Returns the [`Writeable`] to substitute for the given placeholder number.
    fn replacement_for<'a>(&'a self, number: usize) -> Option<Self::W<'a>>;
}

impl<W> NumericPlaceholderProvider for [W]
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    fn replacement_for<'a>(&'a self, number: usize) -> Option<Self::W<'a>> {
        self.get(number)
    }
}

impl<W, const N: usize> NumericPlaceholderProvider for [W; N]
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    fn replacement_for<'a>(&'a self, number: usize) -> Option<Self::W<'a>> {
        self.get(number)
    }
}

impl<'b, T> NumericPlaceholderProvider for &'b T
where
    T: NumericPlaceholderProvider,
{
    type W<'a> = T::W<'a> where T: 'a, 'b: 'a;
    fn replacement_for<'a>(&'a self, number: usize) -> Option<Self::W<'a>> {
        (*self).replacement_for(number)
    }
}

struct WriteableNumericPlaceholderPatternWithPart<'a, P> {
    pattern: NumericPlaceholderPattern<&'a str>,
    replacements: P,
    // TODO: Include the part for the element?
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
                    let Some(element_writeable) = self.replacements.replacement_for(number) else {
                        debug_assert!(false, "could not load placeholder value {number}");
                        return Err(fmt::Error);
                    };
                    element_writeable.write_to_parts(sink)?;
                }
            }
        }
        Ok(())
    }
}

impl<P> fmt::Display for WriteableNumericPlaceholderPatternWithPart<'_, P>
where
    P: NumericPlaceholderProvider,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}
