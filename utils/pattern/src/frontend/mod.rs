// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "databake")]
mod databake;
#[cfg(feature = "serde")]
mod serde;

use core::{
    convert::Infallible,
    fmt::{self, Write},
    marker::PhantomData,
};

use writeable::{adapters::TryWriteableInfallibleAsWriteable, PartsWrite, TryWriteable, Writeable};

use crate::Error;
use crate::{common::*, SinglePlaceholder};

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
/// # Format to Parts
///
/// [`Pattern`] supports interpolating with [writeable::Part]s, annotations for whether the
/// substring was a placeholder or a literal.
///
/// By default, the substrings are annotated with [`PATTERN_LITERAL_PART`] and
/// [`PATTERN_PLACEHOLDER_PART`]. This can be customized with [`PlaceholderValueProvider`].
///
/// # Examples
///
/// Interpolating a [`SinglePlaceholder`] pattern with parts:
///
/// ```
/// use core::str::FromStr;
/// use icu_pattern::Pattern;
/// use icu_pattern::SinglePlaceholder;
/// use writeable::assert_writeable_parts_eq;
///
/// let pattern =
///     Pattern::<SinglePlaceholder, _>::from_str("Hello, {0}!").unwrap();
///
/// assert_writeable_parts_eq!(
///     pattern.interpolate(["Alice"]),
///     "Hello, Alice!",
///     [
///         (0, 7, icu_pattern::PATTERN_LITERAL_PART),
///         (7, 12, icu_pattern::PATTERN_PLACEHOLDER_PART),
///         (12, 13, icu_pattern::PATTERN_LITERAL_PART),
///     ]
/// );
/// ```
///
/// [`SinglePlaceholder`]: crate::SinglePlaceholder
#[derive(Debug, Clone, PartialEq)]
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

    /// Creates a pattern from a serialized backing store without checking invariants.
    /// Most users should prefer [`Pattern::try_from_store()`].
    ///
    /// The store is expected to come from a valid `Pattern` with this `Backend`,
    /// such as by calling [`Pattern::take_store()`]. If the store is not valid,
    /// unexpected behavior may occur.
    ///
    /// To parse a pattern string, use [`FromStr::from_str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use core::str::FromStr;
    /// use icu_pattern::Pattern;
    /// use icu_pattern::SinglePlaceholder;
    /// use writeable::assert_writeable_eq;
    ///
    /// // Create a pattern from a valid string:
    /// let allocated_pattern =
    ///     Pattern::<SinglePlaceholder, String>::from_str("{0} days")
    ///         .expect("valid pattern");
    ///
    /// // Transform the store and create a new Pattern. This is valid because
    /// // we call `.take_store()` and `.from_store_unchecked()` on patterns
    /// // with the same backend (`SinglePlaceholder`).
    /// let store = allocated_pattern.take_store();
    /// let borrowed_pattern: Pattern<SinglePlaceholder, &str> =
    ///     Pattern::from_store_unchecked(&store);
    ///
    /// assert_writeable_eq!(borrowed_pattern.interpolate([5]), "5 days");
    /// ```
    pub const fn from_store_unchecked(store: Store) -> Self {
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
    /// To parse a pattern string, use [`FromStr::from_str`].
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
        I: Iterator<Item = PatternItemCow<'a, B::PlaceholderKeyCow<'a>>>,
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

    /// Creates a pattern from a UTF-8 encoded string.
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
    /// Pattern::<SinglePlaceholder, _>::
    ///                 try_from_utf8("{0} days".as_bytes())
    ///                             .expect("valid pattern items");
    /// ```
    pub fn try_from_utf8(bytes: &[u8]) -> Result<Self, Error> {
        let store = B::try_store_from_utf8(bytes).map_err(|e| Error::from(e))?;
        #[cfg(debug_assertions)]
        match B::validate_store(core::borrow::Borrow::borrow(&store)) {
            Ok(()) => (),
            Err(e) => {
                debug_assert!(false, "{:?}", e);
            }
        };

        Ok(Self::from_store_unchecked(store.to_owned()))
    }
}

#[cfg(feature = "alloc")]
impl<'a, B> FromStr for Pattern<B, <B::Store as ToOwned>::Owned>
where
    B: PatternBackend,
    B::PlaceholderKeyCow<'a>: FromStr,
    B::Store: ToOwned,
    <B::PlaceholderKeyCow<'a> as FromStr>::Err: fmt::Debug,
{
    type Err = Error;
    /// Creates a pattern by parsing a syntax string.
    ///
    /// To construct from a serialized pattern string, use [`Self::try_from_store()`].
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use core::str::FromStr;
    /// use icu_pattern::Pattern;
    /// use icu_pattern::SinglePlaceholder;
    ///
    /// // Create a pattern from a valid string:
    /// Pattern::<SinglePlaceholder, _>::from_str("{0} days")
    ///     .expect("valid pattern");
    ///
    /// // Error on an invalid pattern:
    /// Pattern::<SinglePlaceholder, _>::from_str("{0 days")
    ///     .expect_err("mismatched braces");
    /// ```
    fn from_str(pattern: &str) -> Result<Self, Self::Err> {
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

impl<B, Store> Pattern<B, Store>
where
    B: PatternBackend,
    Store: AsRef<B::Store> + ?Sized,
{
    /// Returns an iterator over the [`PatternItem`]s in this pattern.
    pub fn iter(&self) -> impl Iterator<Item = PatternItem<B::PlaceholderKey<'_>>> + '_ {
        B::iter_items(self.store.as_ref())
    }

    /// Returns a [`TryWriteable`] that interpolates items from the given replacement provider
    /// into this pattern string.
    pub fn try_interpolate<'a, P>(
        &'a self,
        value_provider: P,
    ) -> impl TryWriteable<Error = B::Error<'a>> + fmt::Display + 'a
    where
        P: PlaceholderValueProvider<B::PlaceholderKey<'a>, Error = B::Error<'a>> + 'a,
    {
        WriteablePattern::<B, P> {
            store: self.store.as_ref(),
            value_provider,
        }
    }

    #[cfg(feature = "alloc")]
    /// Interpolates the pattern directly to a string, returning the string or an error.
    ///
    /// In addition to the error, the lossy fallback string is returned in the failure case.
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
    pub fn try_interpolate_to_string<'a, P>(
        &'a self,
        value_provider: P,
    ) -> Result<String, (B::Error<'a>, String)>
    where
        P: PlaceholderValueProvider<B::PlaceholderKey<'a>, Error = B::Error<'a>> + 'a,
    {
        self.try_interpolate(value_provider)
            .try_write_to_string()
            .map(|s| s.into_owned())
            .map_err(|(e, s)| (e, s.into_owned()))
    }
}

impl<B, Store> Pattern<B, Store>
where
    for<'b> B: PatternBackend<Error<'b> = Infallible>,
    Store: AsRef<B::Store> + ?Sized,
{
    /// Returns a [`Writeable`] that interpolates items from the given replacement provider
    /// into this pattern string.
    pub fn interpolate<'a, P>(&'a self, value_provider: P) -> impl Writeable + fmt::Display + 'a
    where
        P: PlaceholderValueProvider<B::PlaceholderKey<'a>, Error = B::Error<'a>> + 'a,
    {
        TryWriteableInfallibleAsWriteable(WriteablePattern::<B, P> {
            store: self.store.as_ref(),
            value_provider,
        })
    }

    #[cfg(feature = "alloc")]
    /// Interpolates the pattern directly to a string.
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
    pub fn interpolate_to_string<'a, P>(&'a self, value_provider: P) -> String
    where
        P: PlaceholderValueProvider<B::PlaceholderKey<'a>, Error = B::Error<'a>> + 'a,
    {
        self.interpolate(value_provider)
            .write_to_string()
            .into_owned()
    }
}

struct WriteablePattern<'a, B: PatternBackend, P> {
    store: &'a B::Store,
    value_provider: P,
}

impl<'a, B, P> TryWriteable for WriteablePattern<'a, B, P>
where
    B: PatternBackend,
    P: PlaceholderValueProvider<B::PlaceholderKey<'a>, Error = B::Error<'a>>,
{
    type Error = B::Error<'a>;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        let mut error = None;
        let it = B::iter_items(self.store);
        #[cfg(debug_assertions)]
        let (size_hint, mut actual_len) = (it.size_hint(), 0);
        for item in it {
            match item {
                PatternItem::Literal(s) => {
                    sink.with_part(P::LITERAL_PART, |sink| sink.write_str(s))?;
                }
                PatternItem::Placeholder(key) => {
                    let (element_writeable, part) = self.value_provider.value_for(key);
                    sink.with_part(part, |sink| {
                        if let Err(e) = element_writeable.try_write_to_parts(sink)? {
                            // Keep the first error if there was one
                            error.get_or_insert(e);
                        }
                        Ok(())
                    })?;
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
        if let Some(e) = error {
            Ok(Err(e))
        } else {
            Ok(Ok(()))
        }
    }
}

impl<'a, B, P> fmt::Display for WriteablePattern<'a, B, P>
where
    B: PatternBackend,
    P: PlaceholderValueProvider<B::PlaceholderKey<'a>, Error = B::Error<'a>>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Discard the TryWriteable error (lossy mode)
        self.try_write_to(f).map(|_| ())
    }
}

#[test]
fn test_try_from_str_inference() {
    use crate::SinglePlaceholder;
    let _: Pattern<SinglePlaceholder, String> = Pattern::from_str("{0} days").unwrap();
    let _ = Pattern::<SinglePlaceholder, String>::from_str("{0} days").unwrap();
}

#[test]
fn test_try_from_utf8() {
    let x = Pattern::<SinglePlaceholder, _>::try_from_utf8("{0} days".as_bytes());

    match x {
        Ok(p) => {
            let y = p.interpolate([1]);
            println!("{y}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}
