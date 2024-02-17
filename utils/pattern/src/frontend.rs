// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{fmt, marker::PhantomData, str::FromStr};

use alloc::borrow::Cow;
use writeable::Writeable;

use crate::{
    Parser, ParserOptions, PatternError, PatternItem, PatternItemCow,
};

use super::PatternBackend;

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
    pub fn try_from_store(store: Store) -> Result<Self, PatternError> {
        B::validate_store(store.as_ref())?;
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

impl<'a, B> Pattern<B, Cow<'a, B::Store>>
where
    B: PatternBackend + 'a,
{
    pub fn try_from_items<I: Iterator<Item = PatternItemCow<'a, B::PlaceholderKey>>>(
        items: I,
    ) -> Result<Self, PatternError> {
        let store = B::try_from_items(items.map(Ok))?;
        Ok(Self {
            _backend: PhantomData,
            store,
        })
    }
}

impl<'a, B> Pattern<B, Cow<'a, B::Store>>
where
    B: PatternBackend + 'a,
    B::PlaceholderKey: FromStr,
    <B::PlaceholderKey as FromStr>::Err: fmt::Debug,
{
    pub fn try_from_str(pattern: &'a str) -> Result<Self, PatternError> {
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
}

pub trait PlaceholderValueProvider<K> {
    type W<'a>: Writeable
    where
        Self: 'a;

    /// Returns the [`Writeable`] to substitute for the given placeholder.
    fn value_for<'a>(&'a self, key: K) -> Self::W<'a>;
}

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
