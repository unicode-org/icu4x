// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Error;
use alloc::borrow::Cow;
use writeable::Writeable;

/// A borrowed item in a [`Pattern`]. Items are either string literals or placeholders.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternItem<'a, T> {
    /// A placeholder of the type specified on this [`PatternItemCow`].
    Placeholder(T),
    /// A string literal. This can occur in one of three places:
    ///
    /// 1. Between the start of the string and the first placeholder (prefix)
    /// 2. Between two placeholders (infix)
    /// 3. Between the final placeholder and the end of the string (suffix)
    Literal(&'a str),
}

/// A borrowed-or-owned item in a [`Pattern`]. Items are either string literals or placeholders.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
#[allow(private_bounds)]
pub trait PatternBackend: crate::Sealed {
    /// The type to be used as the placeholder key in code.
    type PlaceholderKey;

    /// The unsized type of the store required for this backend, usually `str` or `[u8]`.
    type Store: ?Sized;

    #[doc(hidden)] // TODO(#4467): Should be internal
    type Iter<'a>: Iterator<Item = PatternItem<'a, Self::PlaceholderKey>>
    where
        Self: 'a;

    #[doc(hidden)] // TODO(#4467): Should be internal
    fn validate_store(store: &Self::Store) -> Result<(), Error>;

    #[doc(hidden)] // TODO(#4467): Should be internal
    fn try_from_items<
        'a,
        I: Iterator<Item = Result<PatternItemCow<'a, Self::PlaceholderKey>, Error>>,
    >(
        items: I,
    ) -> Result<<Self::Store as ToOwned>::Owned, Error>
    where
        Self: 'a,
        Self::Store: ToOwned;

    #[doc(hidden)] // TODO(#4467): Should be internal
    fn iter_items<'a>(store: &'a Self::Store) -> Self::Iter<'a>;
}

pub trait PlaceholderValueProvider<K> {
    type W<'a>: Writeable
    where
        Self: 'a;

    /// Returns the [`Writeable`] to substitute for the given placeholder.
    fn value_for<'a>(&'a self, key: K) -> Self::W<'a>;
}

#[cfg(disabled)]
impl<W> PlaceholderValueProvider<bool> for (W, W)
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    fn value_for<'a>(&'a self, key: bool) -> Self::W<'a> {
        if key {
            &self.1
        } else {
            &self.0
        }
    }
}

#[cfg(disabled)]
impl<W> PlaceholderValueProvider<bool> for [W; 2]
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    fn value_for<'a>(&'a self, key: bool) -> Self::W<'a> {
        let [v0, v1] = self;
        if key {
            &v1
        } else {
            &v0
        }
    }
}

impl<'b, K, T> PlaceholderValueProvider<K> for &'b T
where
    T: PlaceholderValueProvider<K> + ?Sized,
{
    type W<'a> = T::W<'a> where T: 'a, 'b: 'a;
    fn value_for<'a>(&'a self, key: K) -> Self::W<'a> {
        (*self).value_for(key)
    }
}
