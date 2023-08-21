// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader::*;

use core::borrow::Borrow;

#[cfg(feature = "alloc")]
use crate::{builder::bytestr::ByteStr, builder::nonconst::ZeroTrieBuilder, error::Error};
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, collections::BTreeMap, collections::VecDeque, string::String, vec::Vec};
#[cfg(feature = "litemap")]
use litemap::LiteMap;

/// A data structure that compactly maps from byte sequences to integers.
///
/// There are several variants of `ZeroTrie` which are very similar but are optimized
/// for different use cases:
///
/// - [`ZeroTrieSimpleAscii`] is the most compact structure. Very fast for small data.
///   Only stores ASCII-encoded strings. Can be const-constructed!
/// - [`ZeroTriePerfectHash`] is also compact, but it also supports arbitrary binary
///   strings. It also scales better to large data. Cannot be const-constructed.
/// - [`ZeroTrieExtendedCapacity`] can be used if more than 2^32 bytes are required.
///
/// You can create a `ZeroTrie` directly, in which case the most appropriate
/// backing implementation will be chosen.
///
/// # Backing Store
///
/// The data structure has a flexible backing data store. The only requirement for most
/// functionality is that it implement `AsRef<[u8]>`. All of the following are valid
/// ZeroTrie types:
///
/// - `ZeroTrie<[u8]>` (dynamically sized type: must be stored in a reference or Box)
/// - `ZeroTrie<&[u8]>` (borrows its data from a u8 buffer)
/// - `ZeroTrie<Vec<u8>>` (fully owned data)
/// - `ZeroTrie<ZeroVec<u8>>` (the recommended borrowed-or-owned signature)
/// - `Cow<ZeroTrie<[u8]>>` (another borrowed-or-owned signature)
/// - `ZeroTrie<Cow<[u8]>>` (another borrowed-or-owned signature)
///
/// # Examples
///
/// ```
/// use zerotrie::ZeroTrie;
/// use litemap::LiteMap;
///
/// let mut map = LiteMap::<&[u8], usize>::new_vec();
/// map.insert("foo".as_bytes(), 1);
/// map.insert("bar".as_bytes(), 2);
/// map.insert("bazzoo".as_bytes(), 3);
///
/// let trie = ZeroTrie::try_from(&map)?;
///
/// assert_eq!(trie.get("foo"), Some(1));
/// assert_eq!(trie.get("bar"), Some(2));
/// assert_eq!(trie.get("bazzoo"), Some(3));
/// assert_eq!(trie.get("unknown"), None);
///
/// # Ok::<_, zerotrie::ZeroTrieError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroTrie<Store>(pub(crate) ZeroTrieFlavor<Store>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ZeroTrieFlavor<Store> {
    SimpleAscii(ZeroTrieSimpleAscii<Store>),
    PerfectHash(ZeroTriePerfectHash<Store>),
    ExtendedCapacity(ZeroTrieExtendedCapacity<Store>),
}

/// A data structure that compactly maps from ASCII strings to integers.
///
/// For more information, see [`ZeroTrie`].
///
/// # Examples
///
/// ```
/// use zerotrie::ZeroTrieSimpleAscii;
/// use litemap::LiteMap;
///
/// let mut map = LiteMap::new_vec();
/// map.insert(&b"foo"[..], 1);
/// map.insert(b"bar", 2);
/// map.insert(b"bazzoo", 3);
///
/// let trie = ZeroTrieSimpleAscii::try_from(&map)?;
///
/// assert_eq!(trie.get(b"foo"), Some(1));
/// assert_eq!(trie.get(b"bar"), Some(2));
/// assert_eq!(trie.get(b"bazzoo"), Some(3));
/// assert_eq!(trie.get(b"unknown"), None);
///
/// # Ok::<_, zerotrie::ZeroTrieError>(())
/// ```
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ZeroTrieSimpleAscii<Store: ?Sized> {
    pub(crate) store: Store,
}

/// A data structure that compactly maps from byte strings to integers.
///
/// For more information, see [`ZeroTrie`].
///
/// # Examples
///
/// ```
/// use zerotrie::ZeroTriePerfectHash;
/// use litemap::LiteMap;
///
/// let mut map = LiteMap::<&[u8], usize>::new_vec();
/// map.insert("foo".as_bytes(), 1);
/// map.insert("bår".as_bytes(), 2);
/// map.insert("båzzøø".as_bytes(), 3);
///
/// let trie = ZeroTriePerfectHash::try_from(&map)?;
///
/// assert_eq!(trie.get("foo".as_bytes()), Some(1));
/// assert_eq!(trie.get("bår".as_bytes()), Some(2));
/// assert_eq!(trie.get("båzzøø".as_bytes()), Some(3));
/// assert_eq!(trie.get("bazzoo".as_bytes()), None);
///
/// # Ok::<_, zerotrie::ZeroTrieError>(())
/// ```
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ZeroTriePerfectHash<Store: ?Sized> {
    pub(crate) store: Store,
}

/// A data structure that maps from a large number of byte strings to integers.
///
/// For more information, see [`ZeroTrie`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ZeroTrieExtendedCapacity<Store: ?Sized> {
    pub(crate) store: Store,
}

macro_rules! impl_zerotrie_subtype {
    ($name:ident, $variant:ident, $getter_fn:path, $iter_ty:ty, $iter_fn:path, $cnv_fn:path) => {
        impl<Store> $name<Store> {
            /// Wrap this specific ZeroTrie variant into a ZeroTrie.
            #[inline]
            pub const fn into_zerotrie(self) -> ZeroTrie<Store> {
                ZeroTrie(ZeroTrieFlavor::$variant(self))
            }
            /// Create a trie directly from a store.
            ///
            /// If the store does not contain valid bytes, unexpected behavior may occur.
            #[inline]
            pub const fn from_store(store: Store) -> Self {
                Self { store }
            }
            /// Takes the byte store from this trie.
            #[inline]
            pub fn take_store(self) -> Store {
                self.store
            }
            /// Maps the store into another type.
            #[inline]
            #[cfg(feature = "serde")]
            pub(crate) fn map_store<X>(self, f: impl FnOnce(Store) -> X) -> $name<X> {
                $name::<X>::from_store(f(self.store))
            }
            #[inline]
            #[cfg(feature = "serde")]
            pub(crate) fn map_store_into_zerotrie<X>(self, f: impl FnOnce(Store) -> X) -> ZeroTrie<X> {
                $name::<X>::from_store(f(self.store)).into_zerotrie()
            }
        }
        impl<Store> $name<Store>
        where
        Store: AsRef<[u8]> + ?Sized,
        {
            /// Queries the trie for a string.
            #[inline]
            pub fn get<K>(&self, key: K) -> Option<usize> where K: AsRef<[u8]> {
                // TODO: Should this be AsRef or Borrow?
                $getter_fn(self.store.as_ref(), key.as_ref())
            }
            /// Returns `true` if the trie is empty.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.store.as_ref().is_empty()
            }
            /// Returns the size of the trie in number of bytes.
            ///
            /// To get the number of keys in the trie, use `.iter().count()`:
            ///
            /// ```
            #[doc = concat!("use zerotrie::", stringify!($name), ";")]
            ///
            /// // A trie with two values: "abc" and "abcdef"
            #[doc = concat!("let trie: &", stringify!($name), "<[u8]> = ", stringify!($name), "::from_bytes(b\"abc\\x80def\\x81\");")]
            ///
            /// assert_eq!(8, trie.byte_len());
            /// assert_eq!(2, trie.iter().count());
            /// ```
            #[inline]
            pub fn byte_len(&self) -> usize {
                self.store.as_ref().len()
            }
            /// Returns the bytes contained in the underlying store.
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                self.store.as_ref()
            }
            /// Returns this trie as a reference transparent over a byte slice.
            #[inline]
            pub fn as_borrowed(&self) -> &$name<[u8]> {
                $name::from_bytes(self.store.as_ref())
            }
        }
        #[cfg(feature = "alloc")]
        impl<Store> $name<Store>
        where
        Store: AsRef<[u8]> + ?Sized,
        {
            /// Converts a possibly-borrowed $name to an owned one.
            ///
            /// ***Enable this impl with the `"alloc"` feature.***
            ///
            /// # Examples
            ///
            /// ```
            /// use std::borrow::Cow;
            #[doc = concat!("use zerotrie::", stringify!($name), ";")]
            ///
            #[doc = concat!("let trie: &", stringify!($name), "<[u8]> = ", stringify!($name), "::from_bytes(b\"abc\\x85\");")]
            #[doc = concat!("let owned: ", stringify!($name), "<Vec<u8>> = trie.to_owned();")]
            ///
            /// assert_eq!(trie.get(b"abc"), Some(5));
            /// assert_eq!(owned.get(b"abc"), Some(5));
            /// ```
            #[inline]
            pub fn to_owned(&self) -> $name<Vec<u8>> {
                $name::from_store(
                    Vec::from(self.store.as_ref()),
                )
            }
            #[inline]
            pub fn iter(&self) -> impl Iterator<Item = ($iter_ty, usize)> + '_ {
                 $iter_fn(self.as_bytes())
            }
        }
        impl $name<[u8]> {
            /// Casts from a byte slice to a reference to a trie with the same lifetime.
            ///
            /// If the bytes are not a valid trie, unexpected behavior may occur.
            #[inline]
            pub fn from_bytes(trie: &[u8]) -> &Self {
                // Safety: Self is repr(transparent) over [u8]
                unsafe { core::mem::transmute(trie) }
            }
        }
        #[cfg(feature = "alloc")]
        impl $name<Vec<u8>> {
            pub(crate) fn try_from_tuple_slice(items: &[(&ByteStr, usize)]) -> Result<Self, Error> {
                ZeroTrieBuilder::<VecDeque<u8>>::from_sorted_tuple_slice(
                    items,
                    Self::BUILDER_OPTIONS,
                )
                .map(|s| Self {
                    store: s.to_bytes(),
                })
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a, K> FromIterator<(K, usize)> for $name<Vec<u8>>
        where
            K: AsRef<[u8]>
        {
            fn from_iter<T: IntoIterator<Item = (K, usize)>>(iter: T) -> Self {
                use crate::builder::nonconst::ZeroTrieBuilder;
                ZeroTrieBuilder::<VecDeque<u8>>::from_bytes_iter(
                    iter,
                    Self::BUILDER_OPTIONS
                )
                .map(|s| Self {
                    store: s.to_bytes(),
                })
                .unwrap()
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a, K> TryFrom<&'a BTreeMap<K, usize>> for $name<Vec<u8>>
        where
            K: Borrow<[u8]>
        {
            type Error = crate::error::Error;
            fn try_from(map: &'a BTreeMap<K, usize>) -> Result<Self, Self::Error> {
                let tuples: Vec<(&[u8], usize)> = map
                    .iter()
                    .map(|(k, v)| (k.borrow(), *v))
                    .collect();
                let byte_str_slice = ByteStr::from_byte_slice_with_value(&tuples);
                Self::try_from_tuple_slice(byte_str_slice)
            }
        }
        #[cfg(feature = "alloc")]
        impl<Store> $name<Store>
        where
            Store: AsRef<[u8]> + ?Sized
        {
            /// Exports the data from this ZeroTrie type into a BTreeMap.
            ///
            /// ***Enable this impl with the `"alloc"` feature.***
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use zerotrie::", stringify!($name), ";")]
            /// use std::collections::BTreeMap;
            ///
            #[doc = concat!("let trie = ", stringify!($name), "::from_bytes(b\"abc\\x81def\\x82\");")]
            /// let items = trie.to_btreemap();
            ///
            /// assert_eq!(items.len(), 2);
            ///
            #[doc = concat!("let recovered_trie: ", stringify!($name), "<Vec<u8>> = items")]
            ///     .into_iter()
            ///     .collect();
            /// assert_eq!(trie.as_bytes(), recovered_trie.as_bytes());
            /// ```
            pub fn to_btreemap(&self) -> BTreeMap<$iter_ty, usize> {
                self.iter().collect()
            }
            pub(crate) fn to_btreemap_bytes(&self) -> BTreeMap<Box<[u8]>, usize> {
                self.iter().map(|(k, v)| ($cnv_fn(k), v)).collect()
            }
        }
        #[cfg(feature = "alloc")]
        impl<Store> From<&$name<Store>> for BTreeMap<$iter_ty, usize>
        where
            Store: AsRef<[u8]> + ?Sized,
        {
            #[inline]
            fn from(other: &$name<Store>) -> Self {
                other.to_btreemap()
            }
        }
        #[cfg(feature = "litemap")]
        impl<'a, K, S> TryFrom<&'a LiteMap<K, usize, S>> for $name<Vec<u8>>
        where
            K: Borrow<[u8]>,
            S: litemap::store::StoreIterable<'a, K, usize>,
        {
            type Error = crate::error::Error;
            fn try_from(map: &'a LiteMap<K, usize, S>) -> Result<Self, Self::Error> {
                let tuples: Vec<(&[u8], usize)> = map
                    .iter()
                    .map(|(k, v)| (k.borrow(), *v))
                    .collect();
                let byte_str_slice = ByteStr::from_byte_slice_with_value(&tuples);
                Self::try_from_tuple_slice(byte_str_slice)
            }
        }
        #[cfg(feature = "litemap")]
        impl<Store> $name<Store>
        where
            Store: AsRef<[u8]> + ?Sized,
        {
            /// Exports the data from this ZeroTrie type into a LiteMap.
            ///
            /// ***Enable this function with the `"litemap"` feature.***
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use zerotrie::", stringify!($name), ";")]
            /// use litemap::LiteMap;
            ///
            #[doc = concat!("let trie = ", stringify!($name), "::from_bytes(b\"abc\\x81def\\x82\");")]
            ///
            /// let items = trie.to_litemap();
            /// assert_eq!(items.len(), 2);
            ///
            #[doc = concat!("let recovered_trie: ", stringify!($name), "<Vec<u8>> = items")]
            ///     .iter()
            ///     .map(|(k, v)| (k, *v))
            ///     .collect();
            /// assert_eq!(trie.as_bytes(), recovered_trie.as_bytes());
            /// ```
            pub fn to_litemap(&self) -> LiteMap<$iter_ty, usize> {
                self.iter().collect()
            }
            pub(crate) fn to_litemap_bytes(&self) -> LiteMap<Box<[u8]>, usize> {
                self.iter().map(|(k, v)| ($cnv_fn(k), v)).collect()
            }
        }
        #[cfg(feature = "litemap")]
        impl<Store> From<&$name<Store>> for LiteMap<$iter_ty, usize>
        where
            Store: AsRef<[u8]> + ?Sized,
        {
            #[inline]
            fn from(other: &$name<Store>) -> Self {
                other.to_litemap()
            }
        }
        #[cfg(feature = "litemap")]
        impl $name<Vec<u8>>
        {
            #[cfg(feature = "serde")]
            pub(crate) fn try_from_serde_litemap(items: &LiteMap<Box<ByteStr>, usize>) -> Result<Self, Error> {
                let lm_borrowed: LiteMap<&ByteStr, usize> = items.to_borrowed_keys();
                Self::try_from_tuple_slice(lm_borrowed.as_slice())
            }
        }
        // Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
        impl Borrow<$name<[u8]>> for $name<&[u8]> {
            #[inline]
            fn borrow(&self) -> &$name<[u8]> {
                self.as_borrowed()
            }
        }
        // Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
        #[cfg(feature = "alloc")]
        impl Borrow<$name<[u8]>> for $name<Box<[u8]>> {
            #[inline]
            fn borrow(&self) -> &$name<[u8]> {
                self.as_borrowed()
            }
        }
        // Note: Can't generalize this impl due to the `core::borrow::Borrow` blanket impl.
        #[cfg(feature = "alloc")]
        impl Borrow<$name<[u8]>> for $name<Vec<u8>> {
            #[inline]
            fn borrow(&self) -> &$name<[u8]> {
                self.as_borrowed()
            }
        }
        #[cfg(feature = "alloc")]
        impl alloc::borrow::ToOwned for $name<[u8]> {
            type Owned = $name<Box<[u8]>>;
            #[doc = concat!("This impl allows [`", stringify!($name), "`] to be used inside of a [`Cow`](alloc::borrow::Cow).")]
            ///
            #[doc = concat!("Note that it is also possible to use `", stringify!($name), "<ZeroVec<u8>>` for a similar result.")]
            ///
            /// ***Enable this impl with the `"alloc"` feature.***
            ///
            /// # Examples
            ///
            /// ```
            /// use std::borrow::Cow;
            #[doc = concat!("use zerotrie::", stringify!($name), ";")]
            ///
            #[doc = concat!("let trie: Cow<", stringify!($name), "<[u8]>> = Cow::Borrowed(", stringify!($name), "::from_bytes(b\"abc\\x85\"));")]
            /// assert_eq!(trie.get(b"abc"), Some(5));
            /// ```
            fn to_owned(&self) -> Self::Owned {
                let bytes: &[u8] = self.store.as_ref();
                $name::from_store(
                    Vec::from(bytes).into_boxed_slice(),
                )
            }
        }
        // TODO(#2778): Auto-derive these impls based on the repr(transparent).
        // Safety: $name is repr(transparent) over S, a VarULE
        #[cfg(feature = "zerovec")]
        unsafe impl<Store> zerovec::ule::VarULE for $name<Store>
        where
            Store: zerovec::ule::VarULE,
        {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
                Store::validate_byte_slice(bytes)
            }
            #[inline]
            unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
                core::mem::transmute(Store::from_byte_slice_unchecked(bytes))
            }
        }
    };
}

#[cfg(feature = "alloc")]
fn string_to_box_u8(input: String) -> Box<[u8]> {
    input.into_boxed_str().into_boxed_bytes()
}

impl_zerotrie_subtype!(
    ZeroTrieSimpleAscii,
    SimpleAscii,
    get_bsearch_only,
    String,
    get_iter_ascii_or_panic,
    string_to_box_u8
);
impl_zerotrie_subtype!(
    ZeroTriePerfectHash,
    PerfectHash,
    get_phf_limited,
    Vec<u8>,
    get_iter_phf,
    Vec::into_boxed_slice
);
impl_zerotrie_subtype!(
    ZeroTrieExtendedCapacity,
    ExtendedCapacity,
    get_phf_extended,
    Vec<u8>,
    get_iter_phf,
    Vec::into_boxed_slice
);

macro_rules! impl_dispatch {
    ($self:ident, $inner_fn:ident()) => {
        match $self.0 {
            ZeroTrieFlavor::SimpleAscii(subtype) => subtype.$inner_fn(),
            ZeroTrieFlavor::PerfectHash(subtype) => subtype.$inner_fn(),
            ZeroTrieFlavor::ExtendedCapacity(subtype) => subtype.$inner_fn(),
        }
    };
    (&$self:ident, $inner_fn:ident()) => {
        match &$self.0 {
            ZeroTrieFlavor::SimpleAscii(subtype) => subtype.$inner_fn(),
            ZeroTrieFlavor::PerfectHash(subtype) => subtype.$inner_fn(),
            ZeroTrieFlavor::ExtendedCapacity(subtype) => subtype.$inner_fn(),
        }
    };
    ($self:ident, $inner_fn:ident($arg:ident)) => {
        match $self.0 {
            ZeroTrieFlavor::SimpleAscii(subtype) => subtype.$inner_fn($arg),
            ZeroTrieFlavor::PerfectHash(subtype) => subtype.$inner_fn($arg),
            ZeroTrieFlavor::ExtendedCapacity(subtype) => subtype.$inner_fn($arg),
        }
    };
    (&$self:ident, $inner_fn:ident($arg:ident)) => {
        match &$self.0 {
            ZeroTrieFlavor::SimpleAscii(subtype) => subtype.$inner_fn($arg),
            ZeroTrieFlavor::PerfectHash(subtype) => subtype.$inner_fn($arg),
            ZeroTrieFlavor::ExtendedCapacity(subtype) => subtype.$inner_fn($arg),
        }
    };
}

impl<Store> ZeroTrie<Store> {
    /// Takes the byte store from this trie.
    pub fn take_store(self) -> Store {
        impl_dispatch!(self, take_store())
    }
    /// Maps the store into another type.
    #[cfg(feature = "serde")]
    pub(crate) fn map_store<NewStore>(
        self,
        f: impl FnOnce(Store) -> NewStore,
    ) -> ZeroTrie<NewStore> {
        impl_dispatch!(self, map_store_into_zerotrie(f))
    }
}

impl<Store> ZeroTrie<Store>
where
    Store: AsRef<[u8]>,
{
    /// Queries the trie for a string.
    pub fn get<K>(&self, key: K) -> Option<usize>
    where
        K: AsRef<[u8]>,
    {
        impl_dispatch!(&self, get(key))
    }
    /// Returns `true` if the trie is empty.
    pub fn is_empty(&self) -> bool {
        impl_dispatch!(&self, is_empty())
    }
    /// Returns the size of the trie in number of bytes.
    ///
    /// To get the number of keys in the trie, use `.iter().count()`.
    pub fn byte_len(&self) -> usize {
        impl_dispatch!(&self, byte_len())
    }
}

#[cfg(feature = "alloc")]
impl<Store> ZeroTrie<Store>
where
    Store: AsRef<[u8]>,
{
    /// Exports the data from this ZeroTrie into a BTreeMap.
    pub fn to_btreemap(&self) -> BTreeMap<Box<[u8]>, usize> {
        impl_dispatch!(&self, to_btreemap_bytes())
    }
}

#[cfg(feature = "litemap")]
impl<Store> ZeroTrie<Store>
where
    Store: AsRef<[u8]>,
{
    /// Exports the data from this ZeroTrie into a LiteMap.
    pub fn to_litemap(&self) -> LiteMap<Box<[u8]>, usize> {
        impl_dispatch!(&self, to_litemap_bytes())
    }
}

#[cfg(feature = "alloc")]
impl ZeroTrie<Vec<u8>> {
    pub(crate) fn try_from_tuple_slice(items: &[(&ByteStr, usize)]) -> Result<Self, Error> {
        let is_all_ascii = items.iter().all(|(s, _)| s.is_all_ascii());
        if is_all_ascii && items.len() < 512 {
            ZeroTrieSimpleAscii::try_from_tuple_slice(items).map(|x| x.into_zerotrie())
        } else {
            ZeroTriePerfectHash::try_from_tuple_slice(items).map(|x| x.into_zerotrie())
        }
    }
}

#[cfg(feature = "alloc")]
impl<K> FromIterator<(K, usize)> for ZeroTrie<Vec<u8>>
where
    K: AsRef<[u8]>,
{
    fn from_iter<T: IntoIterator<Item = (K, usize)>>(iter: T) -> Self {
        // We need two Vecs because the first one anchors the `K`s that the second one borrows.
        let items = Vec::from_iter(iter);
        let mut items: Vec<(&[u8], usize)> = items.iter().map(|(k, v)| (k.as_ref(), *v)).collect();
        items.sort();
        let byte_str_slice = ByteStr::from_byte_slice_with_value(&items);
        #[allow(clippy::unwrap_used)] // FromIterator is panicky
        Self::try_from_tuple_slice(byte_str_slice).unwrap()
    }
}
