// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The `bundle` module provides data structures for working directly with the
//! contents of a resource bundle.
//!
//! WARNING: This module is not suitable for use at runtime due to its reliance
//! on `std` and `alloc` and therefore not intended for general deserialization
//! of resource bundles. Rather, it is intended to be used in development-time
//! tools for working with bundles.

extern crate alloc;
use alloc::{
    borrow::Cow,
    collections::{btree_map, BTreeMap},
};

use crate::MASK_28_BIT;

/// A tree-like collection of data [`Resource`]s primarily intended for storing
/// locale and other internationalization data for [ICU] (International
/// Components for Unicode).
///
/// [ICU]: https://icu.unicode.org/
#[derive(Debug)]
pub struct ResourceBundle<'a> {
    name: Cow<'a, str>,
    root: Resource<'a>,
    is_locale_fallback_enabled: bool,
}

impl<'a> ResourceBundle<'a> {
    /// Makes a new resource bundle with the specified resource at its root.
    pub fn new(name: Cow<'a, str>, root: Resource<'a>, is_locale_fallback_enabled: bool) -> Self {
        Self {
            name,
            root,
            is_locale_fallback_enabled,
        }
    }

    /// Gets the name of the resource bundle.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the root resource in the resource tree.
    pub fn root(&self) -> &Resource {
        &self.root
    }

    /// Returns `true` if fallback is enabled for this resource bundle.
    ///
    /// A resource bundle storing locale data may omit some data in order to
    /// reduce duplication, allowing fallback to more general locales which
    /// use the same values.
    pub fn is_locale_fallback_enabled(&self) -> bool {
        self.is_locale_fallback_enabled
    }
}

/// A data resource within a [`ResourceBundle`].
#[derive(Debug)]
#[non_exhaustive]
pub enum Resource<'a> {
    /// A well-formed UTF-8 string.
    String(Cow<'a, str>),

    /// A heterogeneous list of resources, ordered by insertion.
    Array(Vec<Resource<'a>>),

    /// A set of key-resource pairs, sorted lexically by key.
    Table(Table<'a>),

    /// A slice of arbitrary binary data.
    Binary(Cow<'a, [u8]>),

    /// A 28-bit integer.
    ///
    /// May be interpreted as either signed or unsigned depending on consumer
    /// expectations. See [`Int28`] for further details.
    Integer(Int28),

    /// A list of 32-bit integers, ordered by insertion.
    IntVector(Vec<u32>),
}

/// A table of [`Resource`]s indexed by a string [`Key`].
///
/// Resources may be accessed either by key or by integer index.
#[derive(Debug)]
pub struct Table<'a> {
    // Table entries are sorted by key, so a BTreeMap is preferred over a
    // HashMap, which makes no ordering guarantees.
    map: BTreeMap<Key<'a>, Resource<'a>>,
}

impl<'a> Table<'a> {
    /// Makes a new, empty table.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    /// Returns a reference to the resource corresponding to the key, if any.
    pub fn get(&self, k: &Key<'a>) -> Option<&Resource> {
        self.map.get(k)
    }

    /// Inserts a new resource into the table.
    ///
    /// Returns `None` if this key was not already present in the table.
    /// Otherwise, updates the resource in the table and returns the previous
    /// resource.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use icu_resb::bundle::{Resource, Table};
    ///
    /// let mut table = Table::new();
    /// let previous = table.insert("foo".into(), Resource::String("bar".into()));
    /// assert!(previous.is_none());
    ///
    /// let previous = table.insert("foo".into(), Resource::String("baz".into()));
    /// if let Some(Resource::String(inner)) = previous {
    ///     assert_eq!(String::from(inner), "bar");
    /// } else {
    ///     panic!();
    /// }
    /// ```
    pub fn insert(&mut self, k: Key<'a>, v: Resource<'a>) -> Option<Resource> {
        self.map.insert(k, v)
    }

    /// Returns `true` if there are no resources in the table.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use icu_resb::bundle::{Resource, Table};
    ///
    /// let mut table = Table::new();
    /// assert!(table.is_empty());
    /// table.insert("foo".into(), Resource::String("bar".into()));
    /// assert!(!table.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Gets an iterator over the entries in the table, sorted by key.
    pub fn iter(&self) -> TableIter {
        TableIter {
            iter: self.map.iter(),
        }
    }

    /// Gets an iterator over the sorted keys in the table.
    pub fn keys(&self) -> Keys {
        Keys {
            iter: self.map.keys(),
        }
    }

    /// Returns the number of resources in the table.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Gets an iterator over the resources in the table, sorted by key.
    pub fn values(&self) -> Values {
        Values {
            iter: self.map.values(),
        }
    }
}

impl Default for Table<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Table<'a> {
    type Item = (&'a Key<'a>, &'a Resource<'a>);

    type IntoIter = TableIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A key for a [`Resource`] within a [`Table`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Key<'a>(Cow<'a, str>);

impl Key<'_> {
    /// Converts the string representing the key into a slice of UTF-8 bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<'a> From<&'a str> for Key<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> From<String> for Key<'a> {
    fn from(value: String) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> From<Key<'a>> for String {
    fn from(value: Key<'a>) -> Self {
        value.0.into_owned()
    }
}

/// An iterator over the sorted [`Key`]s of a [`Table`].
#[derive(Debug)]
pub struct Keys<'a> {
    iter: btree_map::Keys<'a, Key<'a>, Resource<'a>>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a Key<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> DoubleEndedIterator for Keys<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

/// An iterator over the [`Resource`] values of a [`Table`], sorted by [`Key`].
#[derive(Debug)]
pub struct Values<'a> {
    iter: btree_map::Values<'a, Key<'a>, Resource<'a>>,
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a Resource<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> DoubleEndedIterator for Values<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

/// An iterator over the entries of a `Table`, sorted by [`Key`].
///
/// This `struct` is created by the [`iter`] method on [`Table`]. See its
/// documentation for more.
///
/// [`iter`]: Table::iter
#[derive(Debug)]
pub struct TableIter<'a> {
    iter: btree_map::Iter<'a, Key<'a>, Resource<'a>>,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = (&'a Key<'a>, &'a Resource<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> DoubleEndedIterator for TableIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

/// A 28-bit integer of undetermined signedness.
///
/// [`Resource`]s may include 28-bit integers whose signedness is determined at
/// runtime by consumers. Because these integers are stored in a 32-bit value,
/// negative values in signed integers require special handling, provided by
/// this newtype wrapper.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Int28(u32);

impl From<Int28> for i32 {
    fn from(value: Int28) -> Self {
        ((value.0 as i32) << 4) >> 4
    }
}

impl From<Int28> for u32 {
    fn from(value: Int28) -> Self {
        value.0
    }
}

impl From<i32> for Int28 {
    fn from(value: i32) -> Self {
        Self::from(value as u32)
    }
}

impl From<u32> for Int28 {
    fn from(value: u32) -> Self {
        Self(value & MASK_28_BIT)
    }
}
