// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::iter::FromIterator;
use litemap::LiteMap;

use super::Key;
use super::Value;
use crate::helpers::ShortVec;
use crate::ordering::SubtagOrderingResult;

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
/// Manually build up a [`Keywords`] object:
///
/// ```
/// use icu::locid::extensions::unicode::{Key, Keywords, Value};
///
/// let key: Key = "hc".parse().expect("Failed to parse a Key.");
/// let value: Value = "h23".parse().expect("Failed to parse a Value.");
/// let keywords: Keywords = vec![(key, value)].into_iter().collect();
///
/// assert_eq!(&keywords.to_string(), "hc-h23");
/// ```
///
/// Access a [`Keywords`] object from a [`Locale`]:
///
/// ```
/// use icu::locid::{extensions_unicode_key as key, extensions_unicode_value as value, Locale};
///
/// let loc: Locale = "und-u-hc-h23-kc-true".parse().expect("Valid BCP-47");
///
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&key!("ca")),
///     None
/// );
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&key!("hc")),
///     Some(&value!("h23"))
/// );
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&key!("kc")),
///     Some(&value!("true"))
/// );
///
/// assert_eq!(loc.extensions.unicode.keywords.to_string(), "hc-h23-kc");
/// ```
///
/// [`Locale`]: crate::Locale
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Keywords(LiteMap<Key, Value, ShortVec<(Key, Value)>>);

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
        Self(LiteMap::new())
    }

    /// Returns `true` if there are no keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Keywords;
    /// use icu::locid::Locale;
    ///
    /// let loc1 = Locale::from_bytes(b"und-t-h0-hybrid").unwrap();
    /// let loc2 = Locale::from_bytes(b"und-u-ca-buddhist").unwrap();
    ///
    /// assert!(loc1.extensions.unicode.keywords.is_empty());
    /// assert!(!loc2.extensions.unicode.keywords.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the list contains a [`Value`] for the specified [`Key`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Key, Keywords, Value};
    /// use litemap::LiteMap;
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// let value: Value = "gregory".parse().expect("Failed to parse a Value.");
    /// let keywords: Keywords = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// assert!(&keywords.contains_key(&key));
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
    /// use icu::locid::extensions::unicode::{Key, Keywords, Value};
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// let value: Value = "buddhist".parse().expect("Failed to parse a Value.");
    /// let keywords: Keywords = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// assert_eq!(
    ///     keywords.get(&key).map(|v| v.to_string()),
    ///     Some("buddhist".to_string())
    /// );
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.get(key)
    }

    /// Returns a mutable reference to the [`Value`] corresponding to the [`Key`].
    ///
    /// Returns `None` if the key doesn't exist or if the key has no value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Key, Keywords, Value};
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// let value: Value = "buddhist".parse().expect("Failed to parse a Value.");
    /// let mut keywords: Keywords = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// if let Some(value) = keywords.get_mut(&key) {
    ///     *value = "gregory".parse().expect("Failed to parse a Value.");
    /// }
    /// assert_eq!(
    ///     keywords.get(&key).map(|v| v.to_string()),
    ///     Some("gregory".to_string())
    /// );
    /// ```
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.get_mut(key)
    }

    /// Sets the specified keyword, returning the old value if it already existed.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Key;
    /// use icu::locid::extensions::unicode::Value;
    /// use icu::locid::extensions_unicode_key as key;
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    /// use std::string::ToString;
    ///
    /// const CA_KEY: Key = key!("ca");
    /// let japanese = Value::from_str("japanese").expect("valid extension subtag");
    /// let buddhist = Value::from_str("buddhist").expect("valid extension subtag");
    ///
    /// let mut loc: Locale = "und-u-hello-ca-buddhist-hc-h12"
    ///     .parse()
    ///     .expect("valid BCP-47 identifier");
    /// let old_value = loc.extensions.unicode.keywords.set(CA_KEY, japanese);
    ///
    /// assert_eq!(old_value, Some(buddhist));
    /// assert_eq!(loc, "und-u-hello-ca-japanese-hc-h12".parse().unwrap());
    /// ```
    pub fn set(&mut self, key: Key, value: Value) -> Option<Value> {
        self.0.insert(key, value)
    }

    /// Removes the specified keyword, returning the old value if it existed.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Key;
    /// use icu::locid::extensions_unicode_key as key;
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// const CA_KEY: Key = key!("ca");
    ///
    /// let mut loc: Locale = "und-u-hello-ca-buddhist-hc-h12"
    ///     .parse()
    ///     .expect("valid BCP-47 identifier");
    /// loc.extensions.unicode.keywords.remove(&CA_KEY);
    /// assert_eq!(loc, "und-u-hello-hc-h12".parse().unwrap());
    /// ```
    pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.remove(key)
    }

    /// Clears all Unicode extension keywords, leaving Unicode attributes.
    ///
    /// Returns the old Unicode extension keywords.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// let mut loc: Locale = "und-u-hello-ca-buddhist-hc-h12".parse().unwrap();
    /// loc.extensions.unicode.keywords.clear();
    /// assert_eq!(loc, "und-u-hello".parse().unwrap());
    /// ```
    pub fn clear(&mut self) -> Self {
        core::mem::take(self)
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
    /// assert_eq!(loc, "und-u-hc-h12".parse().unwrap());
    ///
    /// loc.extensions.unicode.keywords.retain_by_key(|k| k == "ms");
    /// assert_eq!(loc, Locale::UND);
    /// ```
    pub fn retain_by_key<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&Key) -> bool,
    {
        self.0.retain(|k, _| predicate(k))
    }

    /// Compare this [`Keywords`] with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// [`Keywords`] to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`] is `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::extensions::unicode::Keywords;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&str] = &[
    ///     "ca-hebrew",
    ///     "ca-japanese",
    ///     "ca-japanese-nu-latn",
    ///     "nu-latn",
    /// ];
    ///
    /// for ab in bcp47_strings.windows(2) {
    ///     let a = ab[0];
    ///     let b = ab[1];
    ///     assert!(a.cmp(b) == Ordering::Less);
    ///     let a_kwds = format!("und-u-{}", a).parse::<Locale>().unwrap().extensions.unicode.keywords;
    ///     assert_eq!(a, a_kwds.to_string());
    ///     assert!(a_kwds.strict_cmp(a.as_bytes()) == Ordering::Equal);
    ///     assert!(a_kwds.strict_cmp(b.as_bytes()) == Ordering::Less);
    /// }
    /// ```
    pub fn strict_cmp(&self, other: &[u8]) -> Ordering {
        self.strict_cmp_iter(other.split(|b| *b == b'-')).end()
    }

    /// Compare this [`Keywords`] with an iterator of BCP-47 subtags.
    ///
    /// This function has the same equality semantics as [`Keywords::strict_cmp`]. It is intended as
    /// a more modular version that allows multiple subtag iterators to be chained together.
    ///
    /// For an additional example, see [`SubtagOrderingResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::extensions::unicode::Keywords;
    /// use std::cmp::Ordering;
    ///
    /// let subtags: &[&[u8]] = &[&*b"ca", &*b"buddhist"];
    ///
    /// let kwds = "und-u-ca-buddhist".parse::<Locale>().unwrap().extensions.unicode.keywords;
    /// assert_eq!(
    ///     Ordering::Equal,
    ///     kwds.strict_cmp_iter(subtags.iter().copied()).end()
    /// );
    ///
    /// let kwds = "und".parse::<Locale>().unwrap().extensions.unicode.keywords;
    /// assert_eq!(
    ///     Ordering::Less,
    ///     kwds.strict_cmp_iter(subtags.iter().copied()).end()
    /// );
    ///
    /// let kwds = "und-u-nu-latn".parse::<Locale>().unwrap().extensions.unicode.keywords;
    /// assert_eq!(
    ///     Ordering::Greater,
    ///     kwds.strict_cmp_iter(subtags.iter().copied()).end()
    /// );
    /// ```
    pub fn strict_cmp_iter<'l, I>(&self, mut subtags: I) -> SubtagOrderingResult<I>
    where
        I: Iterator<Item = &'l [u8]>,
    {
        let r = self.for_each_subtag_str(&mut |subtag| {
            if let Some(other) = subtags.next() {
                match subtag.as_bytes().cmp(other) {
                    Ordering::Equal => Ok(()),
                    not_equal => Err(not_equal),
                }
            } else {
                Err(Ordering::Greater)
            }
        });
        match r {
            Ok(_) => SubtagOrderingResult::Subtags(subtags),
            Err(o) => SubtagOrderingResult::Ordering(o),
        }
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

impl From<LiteMap<Key, Value, ShortVec<(Key, Value)>>> for Keywords {
    fn from(map: LiteMap<Key, Value, ShortVec<(Key, Value)>>) -> Self {
        Self(map)
    }
}

impl FromIterator<(Key, Value)> for Keywords {
    fn from_iter<I: IntoIterator<Item = (Key, Value)>>(iter: I) -> Self {
        LiteMap::from_iter(iter).into()
    }
}

impl_writeable_for_key_value!(Keywords, "ca", "islamic-civil", "mm", "mm");
