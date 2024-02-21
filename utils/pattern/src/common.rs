// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Error;
use writeable::Writeable;

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, borrow::ToOwned};

/// A borrowed item in a [`Pattern`]. Items are either string literals or placeholders.
///
/// [`Pattern`]: crate::Pattern
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_enums)] // Part of core data model
pub enum PatternItem<'a, T> {
    /// A placeholder of the type specified on this [`PatternItem`].
    Placeholder(T),
    /// A string literal. This can occur in one of three places:
    ///
    /// 1. Between the start of the string and the first placeholder (prefix)
    /// 2. Between two placeholders (infix)
    /// 3. Between the final placeholder and the end of the string (suffix)
    Literal(&'a str),
}

/// A borrowed-or-owned item in a [`Pattern`]. Items are either string literals or placeholders.
///
/// ✨ *Enabled with the `alloc` Cargo feature.*
///
/// [`Pattern`]: crate::Pattern
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_enums)] // Part of core data model
#[cfg(feature = "alloc")]
pub enum PatternItemCow<'a, T> {
    /// A placeholder of the type specified on this [`PatternItemCow`].
    Placeholder(T),
    /// A string literal. This can occur in one of three places:
    ///
    /// 1. Between the start of the string and the first placeholder (prefix)
    /// 2. Between two placeholders (infix)
    /// 3. Between the final placeholder and the end of the string (suffix)
    Literal(Cow<'a, str>),
}

/// Types that implement backing data models for [`Pattern`] implement this trait.
///
/// The trait has no public methods and is not implementable outside of this crate.
///
/// [`Pattern`]: crate::Pattern
pub trait PatternBackend: crate::private::Sealed {
    /// The type to be used as the placeholder key in code.
    type PlaceholderKey;

    /// The unsized type of the store required for this backend, usually `str` or `[u8]`.
    type Store: ?Sized;

    /// The iterator type returned by [`Self::try_from_items`].
    #[doc(hidden)] // TODO(#4467): Should be internal
    type Iter<'a>: Iterator<Item = PatternItem<'a, Self::PlaceholderKey>>
    where
        Self: 'a;

    /// Checks a store for validity, returning an error if invalid.
    #[doc(hidden)] // TODO(#4467): Should be internal
    fn validate_store(store: &Self::Store) -> Result<(), Error>;

    /// Constructs a store from pattern items.
    #[doc(hidden)]
    // TODO(#4467): Should be internal
    // Note: it is not good practice to feature-gate trait methods, but this trait is sealed
    #[cfg(feature = "alloc")]
    fn try_from_items<
        'a,
        I: Iterator<Item = Result<PatternItemCow<'a, Self::PlaceholderKey>, Error>>,
    >(
        items: I,
    ) -> Result<<Self::Store as ToOwned>::Owned, Error>
    where
        Self: 'a,
        Self::Store: ToOwned;

    /// Iterates over the pattern items in a store.
    #[doc(hidden)] // TODO(#4467): Should be internal
    fn iter_items(store: &Self::Store) -> Self::Iter<'_>;
}

pub trait PlaceholderValueProvider<K> {
    type W<'a>: Writeable
    where
        Self: 'a;

    /// Returns the [`Writeable`] to substitute for the given placeholder.
    fn value_for(&self, key: K) -> Self::W<'_>;
}

impl<'b, K, T> PlaceholderValueProvider<K> for &'b T
where
    T: PlaceholderValueProvider<K> + ?Sized,
{
    type W<'a> = T::W<'a> where T: 'a, 'b: 'a;
    fn value_for(&self, key: K) -> Self::W<'_> {
        (*self).value_for(key)
    }
}
