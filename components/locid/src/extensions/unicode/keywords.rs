// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::borrow::Borrow;
use core::ops::Deref;

use super::Key;
use super::Value;

/// A list of [`Key`]-[`Value`] pairs representing functional information
/// about locale's internationnalization preferences.
///
/// Here are examples of fields used in Unicode:
/// - `hc` - Hour Cycle (`h11`, `h12`, `h23`, `h24`)
/// - `ca` - Calendar (`buddhist`, `gregory`, ...)
/// - `fw` - First Day Of the Week (`sun`, `mon`, `sat`, ...)
///
/// You can find the full list in [`Unicode BCP 47 U Extension`] section of LDML.
///
/// [`Unicode BCP 47 U Extension`]: https://unicode.org/reports/tr35/tr35.html#Key_And_Type_Definitions_
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::unicode::{Keywords, Key, Value};
///
/// let key: Key = "hc".parse()
///     .expect("Failed to parse a Key.");
/// let value: Value = "h23".parse()
///     .expect("Failed to parse a Value.");
/// let keywords = Keywords::from_vec_unchecked(vec![(key, value)]);
///
/// assert_eq!(&keywords.to_string(), "hc-h23");
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Keywords(Vec<(Key, Value)>);

impl Keywords {
    /// Returns a new empty list of key-value pairs. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Keywords;
    ///
    /// assert_eq!(Keywords::new(), Keywords::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// A constructor which takes a pre-sorted list of `(`[`Key`]`, `[`Value`]`)` tuples.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Keywords, Key, Value};
    ///
    /// let key: Key = "ca".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "buddhist".parse()
    ///     .expect("Failed to parse a Value.");
    /// let keywords = Keywords::from_vec_unchecked(vec![(key, value)]);
    ///
    /// assert_eq!(&keywords.to_string(), "ca-buddhist");
    /// ```
    pub fn from_vec_unchecked(input: Vec<(Key, Value)>) -> Self {
        Self(input)
    }

    /// Returns `true` if the list contains a [`Value`] for the specified [`Key`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Keywords, Key, Value};
    ///
    /// let key: Key = "ca".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "gregory".parse()
    ///     .expect("Failed to parse a Value.");
    /// let mut keywords = Keywords::from_vec_unchecked(vec![(key, value)]);
    ///
    /// let key: Key = "ca".parse()
    ///     .expect("Failed to parse a Key.");
    /// assert!(&keywords.contains_key(&key));
    /// ```
    pub fn contains_key<Q>(&self, key: Q) -> bool
    where
        Q: Borrow<Key>,
    {
        self.binary_search_by_key(key.borrow(), |(key, _)| *key)
            .is_ok()
    }

    /// Returns a reference to the [`Value`] corresponding to the [`Key`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Keywords, Key, Value};
    ///
    /// let key: Key = "ca".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "buddhist".parse()
    ///     .expect("Failed to parse a Value.");
    /// let mut keywords = Keywords::from_vec_unchecked(vec![(key, value)]);
    ///
    /// let key: Key = "ca".parse()
    ///     .expect("Failed to parse a Key.");
    /// assert_eq!(
    ///     keywords.get(&key).map(|v| v.to_string()),
    ///     Some("buddhist".to_string())
    /// );
    /// ```
    pub fn get<Q>(&self, key: Q) -> Option<&Value>
    where
        Q: Borrow<Key>,
    {
        if let Ok(idx) = self.binary_search_by_key(key.borrow(), |(key, _)| *key) {
            self.deref().get(idx).map(|(_, v)| v)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the [`Value`] corresponding to the [`Key`].
    ///
    /// Returns `None` if the key doesn't exist or if the key has no value.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Keywords, Key, Value};
    ///
    /// let key: Key = "ca".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "buddhist".parse()
    ///     .expect("Failed to parse a Value.");
    /// let mut keywords = Keywords::from_vec_unchecked(vec![(key, value)]);
    ///
    /// let key: Key = "ca".parse()
    ///     .expect("Failed to parse a Key.");
    /// if let Some(value) = keywords.get_mut(key) {
    ///     *value = "gregory".parse()
    ///         .expect("Failed to parse a Value.");
    /// }
    /// assert_eq!(
    ///     keywords.get(&key).map(|v| v.to_string()),
    ///     Some("gregory".to_string())
    /// );
    /// ```
    pub fn get_mut<Q>(&mut self, key: Q) -> Option<&mut Value>
    where
        Q: Borrow<Key>,
    {
        if let Ok(idx) = self.binary_search_by_key(key.borrow(), |(key, _)| *key) {
            // Won't panic because the index was given to us by binary_search
            #[allow(clippy::indexing_slicing)]
            Some(&mut self.0[idx].1)
        } else {
            None
        }
    }

    /// Clears all Unicode extension keywords, leaving Unicode attributes.
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    /// use icu::locid::Locale;
    ///
    /// let mut loc: Locale = "und-u-hello-ca-buddhist-hc-h12".parse().unwrap();
    /// loc.extensions.unicode.keywords.clear();
    /// assert_eq!(loc, "und-u-hello");
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Retains a subset of keywords as specified by the predicate function.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// let mut loc: Locale = "und-u-ca-buddhist-hc-h12-ms-metric".parse().unwrap();
    ///
    /// loc.extensions.unicode.keywords.retain_by_key(|k| k == "hc");
    /// assert_eq!(loc, "und-u-hc-h12");
    ///
    /// loc.extensions.unicode.keywords.retain_by_key(|k| k == "ms");
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
        for (k, v) in self.iter() {
            f(k.as_str())?;
            v.for_each_subtag_str(f)?;
        }
        Ok(())
    }
}

impl_writeable_for_key_value!(Keywords, "ca", "islamic-civil", "aa", "aa");

impl Deref for Keywords {
    type Target = [(Key, Value)];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
