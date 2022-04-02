// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::borrow::Borrow;
use core::iter::FromIterator;
use litemap::LiteMap;

use super::Key;
use super::Value;

/// A list of [`Key`]-[`Value`] pairs representing functional information
/// about content transformations.
///
/// Here are examples of fields used in Unicode:
/// - `s0`, `d0` - Transform source/destination
/// - `t0` - Machine Translation
/// - `h0` - Hybrid Locale Identifiers
///
/// You can find the full list in [`Unicode BCP 47 T Extension`] section of LDML.
///
/// [`Unicode BCP 47 T Extension`]: https://unicode.org/reports/tr35/tr35.html#BCP47_T_Extension
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::transform::{Fields, Key, Value};
///
/// let key: Key = "h0".parse()
///     .expect("Failed to parse a Key.");
/// let value: Value = "hybrid".parse()
///     .expect("Failed to parse a Value.");
/// let fields: Fields = vec![(key, value)].into_iter().collect();
///
/// assert_eq!(&fields.to_string(), "h0-hybrid");
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Fields(LiteMap<Key, Value>);

impl Fields {
    /// Returns a new empty list of key-value pairs. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::transform::Fields;
    ///
    /// assert_eq!(Fields::new(), Fields::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self(LiteMap::new())
    }

    /// Returns `true` if there are no fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::extensions::transform::Fields;
    ///
    /// let loc1 = Locale::from_bytes(b"und-t-h0-hybrid").unwrap();
    /// let loc2 = Locale::from_bytes(b"und-u-ca-buddhist").unwrap();
    ///
    /// assert!(!loc1.extensions.transform.fields.is_empty());
    /// assert!(loc2.extensions.transform.fields.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Empties the [`Fields`] list.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::transform::{Fields, Key, Value};
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "hybrid".parse()
    ///     .expect("Failed to parse a Value.");
    /// let mut fields: Fields = vec![(key, value)].into_iter().collect();
    ///
    /// assert_eq!(&fields.to_string(), "h0-hybrid");
    ///
    /// fields.clear();
    ///
    /// assert_eq!(&fields.to_string(), "");
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns `true` if the list contains a [`Value`] for the specified [`Key`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::transform::{Fields, Key, Value};
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "hybrid".parse()
    ///     .expect("Failed to parse a Value.");
    /// let mut fields: Fields = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// assert!(&fields.contains_key(&key));
    /// ```
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.contains_key(key)
    }

    /// Returns a reference to the [`Value`] corresponding to the [`Key`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::transform::{Fields, Key, Value};
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "hybrid".parse()
    ///     .expect("Failed to parse a Value.");
    /// let mut fields: Fields = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// assert_eq!(
    ///     fields.get(&key).map(|v| v.to_string()),
    ///     Some("hybrid".to_string())
    /// );
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.get(key)
    }

    /// Retains a subset of fields as specified by the predicate function.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// let mut loc: Locale = "und-t-h0-hybrid-d0-hex-m0-xml".parse().unwrap();
    ///
    /// loc.extensions.transform.fields.retain_by_key(|k| k == "h0");
    /// assert_eq!(loc, "und-t-h0-hybrid");
    ///
    /// loc.extensions.transform.fields.retain_by_key(|k| k == "d0");
    /// assert_eq!(loc, "und");
    /// ```
    pub fn retain_by_key<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&Key) -> bool,
    {
        self.0.retain(|(k, _)| predicate(k))
    }

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        for (k, v) in self.0.iter() {
            f(k.as_str())?;
            v.for_each_subtag_str(f)?;
        }
        Ok(())
    }

    /// This needs to be its own method to help with type inference in helpers.rs
    #[cfg(test)]
    pub(crate) fn from_tuple_vec(v: Vec<(Key, Value)>) -> Self {
        v.into_iter().collect()
    }
}

impl From<LiteMap<Key, Value>> for Fields {
    fn from(map: LiteMap<Key, Value>) -> Self {
        Self(map)
    }
}

impl FromIterator<(Key, Value)> for Fields {
    fn from_iter<I: IntoIterator<Item = (Key, Value)>>(iter: I) -> Self {
        LiteMap::from_iter(iter).into()
    }
}

impl_writeable_for_key_value!(Fields, "h0", "hybrid", "m0", "m0-true");
