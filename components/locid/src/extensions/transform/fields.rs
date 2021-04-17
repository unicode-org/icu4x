// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Borrow;
use std::ops::Deref;

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
/// let fields = Fields::from_vec_unchecked(vec![(key, value)]);
///
/// assert_eq!(&fields.to_string(), "h0-hybrid");
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Fields(Option<Box<[(Key, Value)]>>);

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
        Self(None)
    }

    /// A constructor which takes a pre-sorted list of `(`[`Key`]`, `[`Value`]`)` tuples.
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
    /// let fields = Fields::from_vec_unchecked(vec![(key, value)]);
    ///
    /// assert_eq!(&fields.to_string(), "h0-hybrid");
    /// ```
    pub fn from_vec_unchecked(input: Vec<(Key, Value)>) -> Self {
        if input.is_empty() {
            Self(None)
        } else {
            Self(Some(input.into_boxed_slice()))
        }
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
    /// let mut fields = Fields::from_vec_unchecked(vec![(key, value)]);
    ///
    /// assert_eq!(&fields.to_string(), "h0-hybrid");
    ///
    /// fields.clear();
    ///
    /// assert_eq!(&fields.to_string(), "");
    /// ```
    pub fn clear(&mut self) {
        self.0 = None;
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
    /// let mut fields = Fields::from_vec_unchecked(vec![(key, value)]);
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// assert!(&fields.contains_key(&key));
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
    /// use icu::locid::extensions::transform::{Fields, Key, Value};
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// let value: Value = "hybrid".parse()
    ///     .expect("Failed to parse a Value.");
    /// let mut fields = Fields::from_vec_unchecked(vec![(key, value)]);
    ///
    /// let key: Key = "h0".parse()
    ///     .expect("Failed to parse a Key.");
    /// assert_eq!(
    ///     fields.get(&key).map(|v| v.to_string()),
    ///     Some("hybrid".to_string())
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
}

impl_writeable_for_key_value!(Fields, "h0", "hybrid", "m0", "m0-true");

impl Deref for Fields {
    type Target = [(Key, Value)];

    fn deref(&self) -> &Self::Target {
        if let Some(ref data) = self.0 {
            data
        } else {
            &[]
        }
    }
}
