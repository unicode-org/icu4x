// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Borrow;
use std::ops::Deref;

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
pub struct Keywords(Option<Box<[(Key, Value)]>>);

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
        Self(None)
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
        if input.is_empty() {
            Self(None)
        } else {
            Self(Some(input.into_boxed_slice()))
        }
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
            if let Some(ref mut data) = self.0 {
                Some(&mut data[idx].1)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl_writeable_for_key_value!(Keywords, "ca", "islamic-civil", "aa", "aa");

impl Deref for Keywords {
    type Target = [(Key, Value)];

    fn deref(&self) -> &Self::Target {
        if let Some(ref data) = self.0 {
            data
        } else {
            &[]
        }
    }
}
