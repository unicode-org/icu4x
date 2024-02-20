// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{
    fmt::{self, Write},
    marker::PhantomData,
    str::FromStr,
};

use writeable::{PartsWrite, Writeable};

use crate::common::*;
use crate::Error;
use crate::{Parser, ParserOptions};

/// A string pattern with placeholders.
///
/// There are 2 generic parameters: `Backend` and `Store`.
///
/// # Backend
///
/// This determines the nature of placeholders and serialized encoding of the pattern.
///
/// The following backends are available:
///
/// - [`SinglePlaceholder`] for patterns with one placeholder: `"{0} days ago"`
///
/// # Store
///
/// The data structure has a flexible backing data store. The only requirement for most
/// functionality is that it implement `AsRef<str>` (backend-dependent).
///
/// Example stores:
///
/// - `&str` for a fully borrowed pattern
/// - `String` for a fully owned pattern
/// - `Cow<str>` for an owned-or-borrowed pattern
///
/// [`SinglePlaceholder`]: crate::SinglePlaceholder
#[derive(Debug)]
pub struct Pattern<Backend, Store: ?Sized> {
    _backend: PhantomData<Backend>,
    store: Store,
}

impl<Backend, Store> Pattern<Backend, Store> {
    pub fn take_store(self) -> Store {
        self.store
    }
}

impl<B, Store> Pattern<B, Store>
where
    B: PatternBackend,
    Store: AsRef<B::Store>,
{
    pub fn try_from_store(store: Store) -> Result<Self, Error> {
        B::validate_store(store.as_ref())?;
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

impl<B> Pattern<B, <B::Store as ToOwned>::Owned>
where
    B: PatternBackend,
    B::Store: ToOwned,
{
    pub fn try_from_items<'a, I>(items: I) -> Result<Self, Error>
    where
        B: 'a,
        I: Iterator<Item = PatternItemCow<'a, B::PlaceholderKey>>,
    {
        let store = B::try_from_items(items.map(Ok))?;
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

impl<B> Pattern<B, <B::Store as ToOwned>::Owned>
where
    B: PatternBackend,
    B::PlaceholderKey: FromStr,
    B::Store: ToOwned,
    <B::PlaceholderKey as FromStr>::Err: fmt::Debug,
{
    pub fn try_from_str(pattern: &str) -> Result<Self, Error> {
        let parser = Parser::new(
            pattern,
            ParserOptions {
                allow_raw_letters: true,
            },
        );
        let store = B::try_from_items(parser)?;
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

impl<B, Store> Pattern<B, Store>
where
    B: PatternBackend,
    Store: AsRef<B::Store> + ?Sized,
{
    pub fn iter(&self) -> impl Iterator<Item = PatternItem<B::PlaceholderKey>> + '_ {
        B::iter_items(self.store.as_ref())
    }

    /// Returns a [`Writeable`] that interpolates items from the given replacement provider
    /// into this pattern string.
    pub fn interpolate<'a, P>(&'a self, value_provider: P) -> impl Writeable + fmt::Display + 'a
    where
        P: PlaceholderValueProvider<B::PlaceholderKey> + 'a,
    {
        WriteablePattern::<B, P> {
            store: self.store.as_ref(),
            value_provider,
        }
    }

    pub fn interpolate_to_string<P>(&self, value_provider: P) -> String
    where
        P: PlaceholderValueProvider<B::PlaceholderKey>,
    {
        self.interpolate(value_provider)
            .write_to_string()
            .into_owned()
    }

    /// Interpolates items with [writeable::Part]s.
    ///
    /// Two parts are used:
    ///
    /// 1. `literal_part` for [`PatternItem::Literal`]
    /// 2. `element_part` for [`PatternItem::Placeholder`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_pattern::Pattern;
    /// use icu_pattern::SinglePlaceholder;
    /// use writeable::assert_writeable_parts_eq;
    ///
    /// let pattern = Pattern::<SinglePlaceholder, _>::try_from_str("Hello, {0}!").unwrap();
    ///
    /// const LITERAL_PART: writeable::Part = writeable::Part {
    ///     category: "demo",
    ///     value: "literal",
    /// };
    /// const ELEMENT_PART: writeable::Part = writeable::Part {
    ///     category: "demo",
    ///     value: "element",
    /// };
    ///
    /// assert_writeable_parts_eq!(
    ///     pattern.interpolate_with_parts(
    ///         ["Alice"],
    ///         LITERAL_PART,
    ///         ELEMENT_PART
    ///     ),
    ///     "Hello, Alice!",
    ///     [
    ///         (0, 7, LITERAL_PART),
    ///         (7, 12, ELEMENT_PART),
    ///         (12, 13, LITERAL_PART),
    ///     ]
    /// );
    /// ```
    pub fn interpolate_with_parts<'a, P>(
        &'a self,
        value_provider: P,
        literal_part: writeable::Part,
        placeholder_value_part: writeable::Part,
    ) -> impl Writeable + fmt::Display + 'a
    where
        P: PlaceholderValueProvider<B::PlaceholderKey> + 'a,
    {
        WriteablePatternWithParts::<B, P> {
            store: self.store.as_ref(),
            value_provider,
            literal_part,
            element_part: placeholder_value_part,
        }
    }
}

struct WriteablePattern<'a, B: PatternBackend, P> {
    store: &'a B::Store,
    value_provider: P,
}

impl<B, P> Writeable for WriteablePattern<'_, B, P>
where
    B: PatternBackend,
    P: PlaceholderValueProvider<B::PlaceholderKey>,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        for item in B::iter_items(self.store) {
            match item {
                PatternItem::Literal(s) => {
                    sink.write_str(s)?;
                }
                PatternItem::Placeholder(key) => {
                    let element_writeable = self.value_provider.value_for(key);
                    element_writeable.write_to(sink)?;
                }
            }
        }
        Ok(())
    }
}

impl<B, P> fmt::Display for WriteablePattern<'_, B, P>
where
    B: PatternBackend,
    P: PlaceholderValueProvider<B::PlaceholderKey>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

struct WriteablePatternWithParts<'a, B: PatternBackend, P> {
    store: &'a B::Store,
    value_provider: P,
    literal_part: writeable::Part,
    element_part: writeable::Part,
}

impl<B, P> Writeable for WriteablePatternWithParts<'_, B, P>
where
    B: PatternBackend,
    P: PlaceholderValueProvider<B::PlaceholderKey>,
{
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        for item in B::iter_items(self.store) {
            match item {
                PatternItem::Literal(s) => {
                    sink.with_part(self.literal_part, |w| w.write_str(s))?;
                }
                PatternItem::Placeholder(key) => {
                    let element_writeable = self.value_provider.value_for(key);
                    sink.with_part(self.element_part, |w| element_writeable.write_to_parts(w))?;
                }
            }
        }
        Ok(())
    }
}

impl<B, P> fmt::Display for WriteablePatternWithParts<'_, B, P>
where
    B: PatternBackend,
    P: PlaceholderValueProvider<B::PlaceholderKey>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}
