// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "databake")]
mod databake;
#[cfg(feature = "serde")]
mod serde;

use core::{
    fmt::{self, Write},
    marker::PhantomData,
};

use writeable::{PartsWrite, Writeable};

use crate::common::*;
use crate::Error;

#[cfg(feature = "alloc")]
use crate::{Parser, ParserOptions};
#[cfg(feature = "alloc")]
use alloc::{borrow::ToOwned, str::FromStr, string::String};

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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "yoke", derive(yoke::Yokeable))]
#[cfg_attr(
    feature = "zerofrom",
    derive(zerofrom::ZeroFrom),
    zerofrom(may_borrow(Store))
)]
pub struct Pattern<Backend, Store: ?Sized> {
    _backend: PhantomData<Backend>,
    store: Store,
}

impl<Backend, Store> Pattern<Backend, Store> {
    pub fn take_store(self) -> Store {
        self.store
    }

    /// # Safety
    ///
    /// The store must come from a valid `Pattern` with this `Backend`,
    /// such as by calling [`Pattern::take_store()`].
    pub const unsafe fn from_store_unchecked(store: Store) -> Self {
        Self {
            _backend: PhantomData,
            store,
        }
    }
}

impl<B, Store> Pattern<B, Store>
where
    B: PatternBackend,
    Store: AsRef<B::Store>,
{
    /// Creates a pattern from a serialized backing store.
    ///
    /// To parse a pattern string, use [`Self::try_from_str()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_pattern::Pattern;
    /// use icu_pattern::SinglePlaceholder;
    ///
    /// // Create a pattern from a valid store:
    /// Pattern::<SinglePlaceholder, _>::try_from_store("\x01 days")
    ///     .expect("valid pattern");
    ///
    /// // Error on an invalid pattern:
    /// Pattern::<SinglePlaceholder, _>::try_from_store("\x09 days")
    ///     .expect_err("9 is out of bounds");
    /// ```
    pub fn try_from_store(store: Store) -> Result<Self, Error> {
        B::validate_store(store.as_ref())?;
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

#[cfg(feature = "alloc")]
impl<B> Pattern<B, <B::Store as ToOwned>::Owned>
where
    B: PatternBackend,
    B::Store: ToOwned,
{
    /// Creates a pattern from an iterator of pattern items.
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_pattern::Pattern;
    /// use icu_pattern::PatternItemCow;
    /// use icu_pattern::SinglePlaceholder;
    /// use icu_pattern::SinglePlaceholderKey;
    /// use std::borrow::Cow;
    ///
    /// Pattern::<SinglePlaceholder, _>::try_from_items(
    ///     [
    ///         PatternItemCow::Placeholder(SinglePlaceholderKey::Singleton),
    ///         PatternItemCow::Literal(Cow::Borrowed(" days")),
    ///     ]
    ///     .into_iter(),
    /// )
    /// .expect("valid pattern items");
    /// ```
    pub fn try_from_items<'a, I>(items: I) -> Result<Self, Error>
    where
        B: 'a,
        I: Iterator<Item = PatternItemCow<'a, B::PlaceholderKey>>,
    {
        let store = B::try_from_items(items.map(Ok))?;
        #[cfg(debug_assertions)]
        match B::validate_store(core::borrow::Borrow::borrow(&store)) {
            Ok(()) => (),
            Err(e) => {
                debug_assert!(false, "{:?}", e);
            }
        };
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

#[cfg(feature = "alloc")]
impl<B> Pattern<B, <B::Store as ToOwned>::Owned>
where
    B: PatternBackend,
    B::PlaceholderKey: FromStr,
    B::Store: ToOwned,
    <B::PlaceholderKey as FromStr>::Err: fmt::Debug,
{
    /// Creates a pattern by parsing a syntax string.
    ///
    /// To construct from a serialized pattern string, use [`Self::try_from_store()`].
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_pattern::Pattern;
    /// use icu_pattern::SinglePlaceholder;
    ///
    /// // Create a pattern from a valid string:
    /// Pattern::<SinglePlaceholder, _>::try_from_str("{0} days")
    ///     .expect("valid pattern");
    ///
    /// // Error on an invalid pattern:
    /// Pattern::<SinglePlaceholder, _>::try_from_str("{0 days")
    ///     .expect_err("mismatched braces");
    /// ```
    pub fn try_from_str(pattern: &str) -> Result<Self, Error> {
        let parser = Parser::new(
            pattern,
            ParserOptions {
                allow_raw_letters: true,
            },
        );
        let store = B::try_from_items(parser)?;
        #[cfg(debug_assertions)]
        match B::validate_store(core::borrow::Borrow::borrow(&store)) {
            Ok(()) => (),
            Err(e) => {
                debug_assert!(false, "{:?} for pattern {:?}", e, pattern);
            }
        };
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

#[cfg(feature = "alloc")]
impl<B> FromStr for Pattern<B, <B::Store as ToOwned>::Owned>
where
    B: PatternBackend,
    B::PlaceholderKey: FromStr,
    B::Store: ToOwned,
    <B::PlaceholderKey as FromStr>::Err: fmt::Debug,
{
    type Err = Error;
    fn from_str(pattern: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(pattern)
    }
}

impl<B, Store> Pattern<B, Store>
where
    B: PatternBackend,
    Store: AsRef<B::Store> + ?Sized,
{
    /// Returns an iterator over the [`PatternItem`]s in this pattern.
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

    #[cfg(feature = "alloc")]
    /// Interpolates the pattern directly to a string.
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
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
    /// let pattern =
    ///     Pattern::<SinglePlaceholder, _>::try_from_str("Hello, {0}!").unwrap();
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
    ///     pattern.interpolate_with_parts(["Alice"], LITERAL_PART, ELEMENT_PART),
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
        let it = B::iter_items(self.store);
        #[cfg(debug_assertions)]
        let (size_hint, mut actual_len) = (it.size_hint(), 0);
        for item in it {
            match item {
                PatternItem::Literal(s) => {
                    sink.write_str(s)?;
                }
                PatternItem::Placeholder(key) => {
                    let element_writeable = self.value_provider.value_for(key);
                    element_writeable.write_to(sink)?;
                }
            }
            #[cfg(debug_assertions)]
            {
                actual_len += 1;
            }
        }
        #[cfg(debug_assertions)]
        {
            debug_assert!(actual_len >= size_hint.0);
            if let Some(max_len) = size_hint.1 {
                debug_assert!(actual_len <= max_len);
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
