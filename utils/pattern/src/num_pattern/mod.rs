// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::{self, Write};
use writeable::{PartsWrite, Writeable};

#[cfg(feature = "alloc")]
use crate::parser::{Parser, ParserOptions, PatternToken};
#[cfg(feature = "alloc")]
use alloc::string::String;

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
/// use icu_pattern::NumericPlaceholderPattern;
/// use writeable::assert_writeable_eq;
///
/// // Create a pattern from the string syntax:
/// let pattern = NumericPlaceholderPattern::from_str("Hello, {0} and {1}!").unwrap();
///
/// // Interpolate some values into the pattern:
/// assert_writeable_eq!(
///     pattern.interpolate(["Alice", "Bob"]),
///     "Hello, Alice and Bob!"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericPlaceholderPattern<Store: ?Sized, const N: usize> {
    store: Store,
}

/// An item in a [`NumericPlaceholderPattern`]. Items are either string literals or placeholders.
#[allow(clippy::exhaustive_enums)] // documented structure
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(feature = "alloc")]
impl<'a> From<&'a PatternToken<'_, usize>> for NumericPlaceholderPatternItem<'a> {
    fn from(value: &'a PatternToken<usize>) -> Self {
        match value {
            PatternToken::Literal { content, .. } => {
                NumericPlaceholderPatternItem::Literal(&content)
            }
            PatternToken::Placeholder(number) => {
                NumericPlaceholderPatternItem::Placeholder(*number)
            }
        }
    }
}

impl<Store, const N: usize> NumericPlaceholderPattern<Store, N> {
    pub fn from_store(store: Store) -> Self {
        Self { store }
    }
    pub fn take_store(self) -> Store {
        self.store
    }
}

impl<Store: ?Sized, const N: usize> NumericPlaceholderPattern<Store, N> {
    pub fn borrow_store(&self) -> &Store {
        &self.store
    }
}

impl<Store, const N: usize> NumericPlaceholderPattern<Store, N>
where
    Store: ?Sized + AsRef<str>,
{
    /// Iterate over the contents of this pattern string, returning instances of
    /// [`NumericPlaceholderPatternItem`].
    pub fn iter(&self) -> impl Iterator<Item = NumericPlaceholderPatternItem> + '_ {
        todo!();
        *&[].into_iter() // required for type resolution
    }

    /// Convert this pattern's store to `&str`.
    pub fn as_borrowed_store(&self) -> NumericPlaceholderPattern<&str, N> {
        NumericPlaceholderPattern {
            store: self.store.as_ref(),
        }
    }
}

/// A type that returns [`Writeable`]s for interpolating into a [`NumericPlaceholderPattern`].
///
/// This trait is implemented on slices of [`Writeable`]s, including `[W]`, `[W; N]`, and `&[W]`.
///
/// # Examples
///
/// Interpolating a slice of `i32` (`i32` implements [`Writeable`]):
///
/// ```
/// use icu_pattern::NumericPlaceholderPattern;
/// use writeable::assert_writeable_eq;
///
/// let pattern = NumericPlaceholderPattern::from_store("Your lucky numbers are: \x01, \x02, and \x03");
///
/// assert_writeable_eq!(
///     pattern.interpolate(&[55, 46, 91] as &[i32]),
///     "Your lucky numbers are: 55, 46, and 91"
/// );
/// ```
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
    T: NumericPlaceholderProvider + ?Sized,
{
    type W<'a> = T::W<'a> where T: 'a, 'b: 'a;
    fn replacement_for<'a>(&'a self, number: usize) -> Option<Self::W<'a>> {
        (*self).replacement_for(number)
    }
}

struct WriteableNumericPlaceholderPattern<'a, P, const N: usize> {
    pattern: NumericPlaceholderPattern<&'a str, N>,
    replacements: P,
}

impl<P, const N: usize> Writeable for WriteableNumericPlaceholderPattern<'_, P, N>
where
    P: NumericPlaceholderProvider,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        for item in self.pattern.iter() {
            match item {
                NumericPlaceholderPatternItem::Literal(s) => {
                    sink.write_str(s)?;
                }
                NumericPlaceholderPatternItem::Placeholder(number) => {
                    let Some(element_writeable) = self.replacements.replacement_for(number) else {
                        debug_assert!(false, "could not load placeholder value {number}");
                        return Err(fmt::Error);
                    };
                    element_writeable.write_to(sink)?;
                }
            }
        }
        Ok(())
    }
}

impl<P, const N: usize> fmt::Display for WriteableNumericPlaceholderPattern<'_, P, N>
where
    P: NumericPlaceholderProvider,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

struct WriteableNumericPlaceholderPatternWithParts<'a, P, const N: usize> {
    pattern: NumericPlaceholderPattern<&'a str, N>,
    replacements: P,
    literal_part: writeable::Part,
    element_part: writeable::Part,
}

impl<P, const N: usize> Writeable for WriteableNumericPlaceholderPatternWithParts<'_, P, N>
where
    P: NumericPlaceholderProvider,
{
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        for item in self.pattern.iter() {
            match item {
                NumericPlaceholderPatternItem::Literal(s) => {
                    sink.with_part(self.literal_part, |w| w.write_str(s))?;
                }
                NumericPlaceholderPatternItem::Placeholder(number) => {
                    let Some(element_writeable) = self.replacements.replacement_for(number) else {
                        debug_assert!(false, "could not load placeholder value {number}");
                        return Err(fmt::Error);
                    };
                    sink.with_part(self.element_part, |w| element_writeable.write_to_parts(w))?;
                }
            }
        }
        Ok(())
    }
}

impl<P, const N: usize> fmt::Display for WriteableNumericPlaceholderPatternWithParts<'_, P, N>
where
    P: NumericPlaceholderProvider,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}
