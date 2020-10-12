// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
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
/// [`Key`]: ./struct.Key.html
/// [`Value`]: ./struct.Value.html
/// [`Unicode BCP 47 U Extension`]: https://unicode.org/reports/tr35/tr35.html#Key_And_Type_Definitions_
///
/// # Examples
///
/// ```
/// use icu_locale::extensions::unicode::{Keywords, Key, Value};
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
pub struct Keywords(Box<[(Key, Value)]>);

impl Keywords {
    /// A constructor which takes a pre-sorted list of `(Key, Value)` tuples.
    ///
    /// [`Key`]: ./struct.Key.html
    /// [`Value`]: ./struct.Value.html
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::{Keywords, Key, Value};
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
        Self(input.into_boxed_slice())
    }

    /// Returns `true` if the list contains a [`Value`] for the specified [`Key`].
    ///
    /// [`Key`]: ./struct.Key.html
    /// [`Value`]: ./struct.Value.html
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::{Keywords, Key, Value};
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
    /// [`Key`]: ./struct.Key.html
    /// [`Value`]: ./struct.Value.html
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::{Keywords, Key, Value};
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
        if let Ok(idx) = self.0.binary_search_by_key(key.borrow(), |(key, _)| *key) {
            self.0.get(idx).map(|(_, v)| v)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Keywords {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (key, value) in self.iter() {
            write!(f, "{}-{}", key, value)?;
        }
        Ok(())
    }
}

impl Deref for Keywords {
    type Target = [(Key, Value)];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
