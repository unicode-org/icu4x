// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use icu_provider::prelude::*;
use writeable::{impl_display_with_writeable, Writeable};
use zerotrie::cursor::ZeroAsciiIgnoreCaseTrieCursor;
use zerovec::vecs::{VarZeroSliceIter, ZeroSliceIter};

use crate::{
    provider::iana::{
        Bcp47ToIanaMap, Bcp47ToIanaMapV1, IanaToBcp47Map, IanaToBcp47MapV3, NON_REGION_CITY_PREFIX,
    },
    TimeZone,
};

/// A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
///
/// This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
/// It also supports normalizing and canonicalizing the IANA strings.
///
/// There are approximately 600 IANA identifiers and 450 BCP-47 identifiers.
///
/// BCP-47 time zone identifiers are 8 ASCII characters or less and currently
/// average 5.1 characters long. Current IANA time zone identifiers are less than
/// 40 ASCII characters and average 14.2 characters long.
///
/// These lists grow very slowly; in a typical year, 2-3 new identifiers are added.
///
/// # Normalization vs Canonicalization
///
/// Multiple IANA time zone identifiers can refer to the same BCP-47 time zone. For example, the
/// following three IANA identifiers all map to `"usind"`:
///
/// - "America/Fort_Wayne"
/// - "America/Indiana/Indianapolis"
/// - "America/Indianapolis"
/// - "US/East-Indiana"
///
/// There is only one canonical identifier, which is "America/Indiana/Indianapolis". The
/// *canonicalization* operation returns the canonical identifier. You should canonicalize if
/// you need to compare time zones for equality. Note that the canonical identifier can change
/// over time. For example, the identifier "Europe/Kiev" was renamed to the newly-added
/// identifier "Europe/Kyiv" in 2022.
///
/// The *normalization* operation, on the other hand, keeps the input identifier but normalizes
/// the casing. For example, "AMERICA/FORT_WAYNE" normalizes to "America/Fort_Wayne".
/// Normalization is a data-driven operation because there are no algorithmic casing rules that
/// work for all IANA time zone identifiers.
///
/// Normalization is a cheap operation, but canonicalization might be expensive, since it might
/// require searching over all IANA IDs to find the canonicalization. If you need
/// canonicalization that is reliably fast, use [`IanaParserExtended`].
///
/// # Examples
///
/// ```
/// use icu::time::TimeZone;
/// use icu::time::zone::IanaParser;
/// use tinystr::tinystr;
///
/// let mapper = IanaParser::new();
///
/// // The IANA zone "Australia/Melbourne" is the BCP-47 zone "aumel":
/// assert_eq!(
///     mapper.iana_to_bcp47("Australia/Melbourne"),
///     TimeZone(tinystr!(8, "aumel"))
/// );
///
/// // Lookup is ASCII-case-insensitive:
/// assert_eq!(
///     mapper.iana_to_bcp47("australia/melbourne"),
///     TimeZone(tinystr!(8, "aumel"))
/// );
///
/// // The IANA zone "Australia/Victoria" is an alias:
/// assert_eq!(
///     mapper.iana_to_bcp47("Australia/Victoria"),
///     TimeZone(tinystr!(8, "aumel"))
/// );
///
/// // The IANA zone "Australia/Boing_Boing" does not exist
/// // (maybe not *yet*), so it produces the special unknown
/// // timezone in order for this operation to be infallible:
/// assert_eq!(
///     mapper.iana_to_bcp47("Australia/Boing_Boing"),
///     TimeZone::unknown()
/// );
///
/// // We can recover the canonical identifier from the mapper:
/// assert_eq!(
///     mapper.canonicalize_iana("Australia/Victoria").unwrap().0,
///     "Australia/Melbourne"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct IanaParser {
    data: DataPayload<IanaToBcp47MapV3>,
    checksum: u64,
}

impl IanaParser {
    /// Creates a new [`IanaParser`] using compiled data.
    ///
    /// See [`IanaParser`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> IanaParserBorrowed<'static> {
        IanaParserBorrowed::new()
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
                        try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: DataProvider<IanaToBcp47MapV3> + ?Sized,
    {
        let response = provider.load(Default::default())?;
        Ok(Self {
            data: response.payload,
            checksum: response.metadata.checksum.ok_or_else(|| {
                DataError::custom("Missing checksum")
                    .with_req(IanaToBcp47MapV3::INFO, Default::default())
            })?,
        })
    }

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential indirection cost when querying the mapper.
    pub fn as_borrowed(&self) -> IanaParserBorrowed {
        IanaParserBorrowed {
            data: self.data.get(),
            checksum: self.checksum,
        }
    }
}

impl AsRef<IanaParser> for IanaParser {
    #[inline]
    fn as_ref(&self) -> &IanaParser {
        self
    }
}

/// A borrowed wrapper around the time zone ID mapper, returned by
/// [`IanaParser::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct IanaParserBorrowed<'a> {
    data: &'a IanaToBcp47Map<'a>,
    checksum: u64,
}

#[cfg(feature = "compiled_data")]
impl Default for IanaParserBorrowed<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl IanaParserBorrowed<'static> {
    /// Creates a new [`IanaParserBorrowed`] using compiled data.
    ///
    /// See [`IanaParserBorrowed`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new() -> Self {
        Self {
            data: crate::provider::Baked::SINGLETON_IANA_TO_BCP47_MAP_V3,
            checksum: crate::provider::Baked::SINGLETON_IANA_TO_BCP47_MAP_V3_CHECKSUM,
        }
    }

    /// Cheaply converts a [`IanaParserBorrowed<'static>`] into a [`IanaParser`].
    ///
    /// Note: Due to branching and indirection, using [`IanaParser`] might inhibit some
    /// compile-time optimizations that are possible with [`IanaParserBorrowed`].
    pub fn static_to_owned(&self) -> IanaParser {
        IanaParser {
            data: DataPayload::from_static_ref(self.data),
            checksum: self.checksum,
        }
    }
}

impl IanaParserBorrowed<'_> {
    /// Gets the BCP-47 time zone ID from an IANA time zone ID
    /// with a case-insensitive lookup.
    ///
    /// Returns [`TimeZone::unknown()`] if the IANA ID is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_time::TimeZone;
    /// use icu_time::zone::iana::IanaParser;
    ///
    /// let mapper = IanaParser::new();
    ///
    /// let result = mapper.iana_to_bcp47("Asia/CALCUTTA");
    ///
    /// assert_eq!(*result, "inccu");
    ///
    /// // Unknown IANA time zone ID:
    /// assert_eq!(
    ///     mapper.iana_to_bcp47("America/San_Francisco"),
    ///     TimeZone::unknown()
    /// );
    /// ```
    pub fn iana_to_bcp47(&self, iana_id: &str) -> TimeZone {
        self.iana_lookup_quick(iana_id)
            .and_then(|trie_value| self.data.bcp47_ids.get(trie_value.index()))
            .unwrap_or(TimeZone::unknown())
    }

    /// Same as [`Self::iana_to_bcp47()`] but works with potentially ill-formed UTF-8.
    pub fn iana_bytes_to_bcp47(&self, iana_id: &[u8]) -> TimeZone {
        self.iana_lookup_quick(iana_id)
            .and_then(|trie_value| self.data.bcp47_ids.get(trie_value.index()))
            .unwrap_or(TimeZone::unknown())
    }

    /// Normalizes the syntax of an IANA time zone ID.
    ///
    /// Also returns the BCP-47 time zone ID.
    ///
    /// Returns `None` if the IANA ID is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_time::TimeZone;
    /// use icu_time::zone::iana::IanaParser;
    /// use std::borrow::Cow;
    ///
    /// let mapper = IanaParser::new();
    ///
    /// let result = mapper.normalize_iana("Asia/CALCUTTA").unwrap();
    ///
    /// assert_eq!(result.0, "Asia/Calcutta");
    /// assert!(matches!(result.0, Cow::Owned(_)));
    /// assert_eq!(*result.1, "inccu");
    ///
    /// // Borrows when able:
    /// let result = mapper.normalize_iana("America/Chicago").unwrap();
    /// assert_eq!(result.0, "America/Chicago");
    /// assert!(matches!(result.0, Cow::Borrowed(_)));
    ///
    /// // Unknown IANA time zone ID:
    /// assert_eq!(mapper.normalize_iana("America/San_Francisco"), None);
    /// ```
    pub fn normalize_iana<'s>(&self, iana_id: &'s str) -> Option<(Cow<'s, str>, TimeZone)> {
        let (trie_value, string) = self.iana_lookup_with_normalization(iana_id, |_| {})?;
        let Some(bcp47_id) = self.data.bcp47_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return None;
        };
        Some((string, bcp47_id))
    }

    /// Returns the canonical, normalized identifier of the given IANA time zone.
    ///
    /// Also returns the BCP-47 time zone ID.
    ///
    /// Returns `None` if the IANA ID is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_time::TimeZone;
    /// use icu_time::zone::iana::IanaParser;
    /// use std::borrow::Cow;
    ///
    /// let mapper = IanaParser::new();
    ///
    /// let result = mapper.canonicalize_iana("Asia/CALCUTTA").unwrap();
    ///
    /// assert_eq!(result.0, "Asia/Kolkata");
    /// assert!(matches!(result.0, Cow::Owned(_)));
    /// assert_eq!(*result.1, "inccu");
    ///
    /// // Borrows when able:
    /// let result = mapper.canonicalize_iana("America/Chicago").unwrap();
    /// assert_eq!(result.0, "America/Chicago");
    /// assert!(matches!(result.0, Cow::Borrowed(_)));
    ///
    /// // Unknown IANA time zone ID:
    /// assert_eq!(mapper.canonicalize_iana("America/San_Francisco"), None);
    /// ```
    pub fn canonicalize_iana<'s>(&self, iana_id: &'s str) -> Option<(Cow<'s, str>, TimeZone)> {
        // Note: We collect the cursors into a stack so that we start probing
        // nearby the input IANA identifier. This should improve lookup time since
        // most renames share the same prefix like "Asia" or "Europe".
        let mut stack = Vec::with_capacity(iana_id.len());
        let (trie_value, mut string) = self.iana_lookup_with_normalization(iana_id, |cursor| {
            stack.push((cursor.clone(), 0, 1));
        })?;
        let Some(bcp47_id) = self.data.bcp47_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return None;
        };
        if trie_value.is_canonical() {
            return Some((string, bcp47_id));
        }
        // If we get here, we need to walk the trie to find the canonical IANA ID.
        let needle = trie_value.to_canonical();
        if !string.contains('/') {
            string.to_mut().insert(0, '_');
        }
        let Some(string) = self.iana_search(needle, string.into_owned(), stack) else {
            debug_assert!(false, "every time zone should have a canonical IANA ID");
            return None;
        };
        Some((Cow::Owned(string), bcp47_id))
    }

    /// Returns the canonical, normalized IANA ID of the given BCP-47 ID.
    ///
    /// This function performs a linear search over all IANA IDs. If this is problematic, consider one of the
    /// following functions instead:
    ///
    /// 1. [`IanaParserBorrowed::canonicalize_iana()`]
    ///    is faster if you have an IANA ID.
    /// 2. [`IanaParserExtendedBorrowed::canonical_iana_from_bcp47()`]
    ///    is faster, but it requires loading additional data
    ///    (see [`IanaParserExtended`]).
    ///
    /// Returns `None` if the BCP-47 ID is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_time::TimeZone;
    /// use icu_time::zone::iana::IanaParser;
    /// use std::borrow::Cow;
    /// use tinystr::tinystr;
    ///
    /// let mapper = IanaParser::new();
    ///
    /// let bcp47_id = TimeZone(tinystr!(8, "inccu"));
    /// let result = mapper.find_canonical_iana_from_bcp47(bcp47_id).unwrap();
    ///
    /// assert_eq!(result, "Asia/Kolkata");
    ///
    /// // Unknown BCP-47 time zone ID:
    /// let bcp47_id = TimeZone(tinystr!(8, "ussfo"));
    /// assert_eq!(mapper.find_canonical_iana_from_bcp47(bcp47_id), None);
    /// ```
    pub fn find_canonical_iana_from_bcp47(&self, bcp47_id: TimeZone) -> Option<String> {
        let index = self.data.bcp47_ids.binary_search(&bcp47_id).ok()?;
        let stack = alloc::vec![(self.data.map.cursor(), 0, 0)];
        let needle = IanaTrieValue::canonical_for_index(index);
        let string = self.iana_search(needle, String::new(), stack)?;
        Some(string)
    }

    /// Queries the data for `iana_id` without recording the normalized string.
    /// This is a fast, no-alloc lookup.
    fn iana_lookup_quick(self, iana_id: impl AsRef<[u8]>) -> Option<IanaTrieValue> {
        let mut cursor = self.data.map.cursor();
        let iana_id = iana_id.as_ref();
        if !iana_id.contains(&b'/') {
            cursor.step(NON_REGION_CITY_PREFIX);
        }
        for &b in iana_id {
            cursor.step(b);
        }
        cursor.take_value().map(IanaTrieValue)
    }

    /// Queries the data for `iana_id` while keeping track of the normalized string.
    /// This is a fast lookup, but it may require allocating memory.
    fn iana_lookup_with_normalization<'l, 's>(
        &'l self,
        iana_id: &'s str,
        mut cursor_fn: impl FnMut(&ZeroAsciiIgnoreCaseTrieCursor<'l>),
    ) -> Option<(IanaTrieValue, Cow<'s, str>)> {
        let mut cursor = self.data.map.cursor();
        if !iana_id.contains('/') {
            cursor_fn(&cursor);
            cursor.step(NON_REGION_CITY_PREFIX);
        }
        let mut string = Cow::Borrowed(iana_id);
        let mut i = 0;
        let trie_value = loop {
            cursor_fn(&cursor);
            let Some(&input_byte) = string.as_bytes().get(i) else {
                break cursor.take_value().map(IanaTrieValue);
            };
            let Some(matched_byte) = cursor.step(input_byte) else {
                break None;
            };
            if matched_byte != input_byte {
                // Safety: we write to input_byte farther down after performing safety checks.
                let Some(input_byte) = unsafe { string.to_mut().as_bytes_mut() }.get_mut(i) else {
                    debug_assert!(false, "the same index was just accessed earlier");
                    break None;
                };
                if !input_byte.is_ascii() {
                    debug_assert!(false, "non-ASCII input byte: {input_byte}");
                    break None;
                }
                if !matched_byte.is_ascii() {
                    debug_assert!(false, "non-ASCII matched byte: {matched_byte}");
                    break None;
                }
                // Safety: we just checked that both input_byte and matched_byte are ASCII,
                // so the buffer remains UTF-8 when we replace one with the other.
                *input_byte = matched_byte;
            }
            i += 1;
        }?;
        Some((trie_value, string))
    }

    /// Performs a reverse lookup by walking the trie with an optional start position.
    /// This is not a fast operation since it requires a linear search.
    fn iana_search(
        self,
        needle: IanaTrieValue,
        mut string: String,
        mut stack: Vec<(ZeroAsciiIgnoreCaseTrieCursor, usize, usize)>,
    ) -> Option<String> {
        loop {
            let Some((mut cursor, index, suffix_len)) = stack.pop() else {
                // Nothing left in the trie.
                return None;
            };
            // Check to see if there is a value at the current node.
            if let Some(candidate) = cursor.take_value().map(IanaTrieValue) {
                if candidate == needle {
                    // Success! Found what we were looking for.
                    return Some(string);
                }
            }
            // Now check for children of the current node.
            let mut sub_cursor = cursor.clone();
            if let Some(probe_result) = sub_cursor.probe(index) {
                // Found a child. Add the current byte edge to the string.
                if !probe_result.byte.is_ascii() {
                    debug_assert!(false, "non-ASCII probe byte: {}", probe_result.byte);
                    return None;
                }
                // Safety: the byte being added is ASCII as guarded above
                unsafe { string.as_mut_vec().push(probe_result.byte) };
                // Add the child to the stack, and also add back the current
                // node if there are more siblings to visit.
                if index + 1 < probe_result.total_siblings as usize {
                    stack.push((cursor, index + 1, suffix_len));
                    stack.push((sub_cursor, 0, 1));
                } else {
                    stack.push((sub_cursor, 0, suffix_len + 1));
                }
            } else {
                // No more children. Pop this node's bytes from the string.
                for _ in 0..suffix_len {
                    // Safety: we check that the bytes being removed are ASCII
                    let removed_byte = unsafe { string.as_mut_vec().pop() };
                    if let Some(removed_byte) = removed_byte {
                        if !removed_byte.is_ascii() {
                            debug_assert!(false, "non-ASCII removed byte: {removed_byte}");
                            // If we get here for some reason, `string` is not in a valid state,
                            // so to be extra safe, we can clear it.
                            string.clear();
                            return None;
                        }
                    } else {
                        debug_assert!(false, "could not remove another byte");
                        return None;
                    }
                }
            }
        }
    }

    /// Returns an iterator over BCP-47 time zone identifiers in alphabetical order.
    ///
    /// To iterate over canonical IANA time zone IDs, use
    /// [`IanaParserExtendedBorrowed::iter_canonical_iana()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::zone::IanaParser;
    ///
    /// let ids = IanaParser::new()
    ///     .iter_bcp47()
    ///     .skip(30)
    ///     .take(5)
    ///     .map(|id| id.to_string())
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     ids,
    ///     &[
    ///         "arush",
    ///         "asppg",
    ///         "atvie",
    ///         "auadl",
    ///         "aubhq",
    ///     ]
    /// );
    /// ```
    pub fn iter_bcp47(&self) -> TimeZoneBcp47Iter {
        TimeZoneBcp47Iter {
            inner: self.data.bcp47_ids.iter(),
        }
    }
}

/// An iterator over BCP-47 time zone identifiers.
///
/// See [`IanaParserBorrowed::iter_bcp47()`]
#[derive(Debug)]
pub struct TimeZoneBcp47Iter<'a> {
    inner: ZeroSliceIter<'a, TimeZone>,
}

impl Iterator for TimeZoneBcp47Iter<'_> {
    type Item = TimeZone;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// A mapper that supplements [`IanaParser`] with about 8 KB of additional data to
/// improve the performance of canonical IANA ID lookup.
///
/// The data in [`IanaParser`] is optimized for IANA to BCP-47 lookup; the reverse
/// requires a linear walk over all ~600 IANA identifiers. The data added here allows for
/// constant-time mapping from BCP-47 to IANA.
#[derive(Debug, Clone)]
pub struct IanaParserExtended<I> {
    inner: I,
    data: DataPayload<Bcp47ToIanaMapV1>,
}

impl IanaParserExtended<IanaParser> {
    /// Creates a new [`IanaParserExtended`] using compiled data.
    ///
    /// See [`IanaParserExtended`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> IanaParserExtendedBorrowed<'static> {
        IanaParserExtendedBorrowed::new()
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
                        try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: DataProvider<IanaToBcp47MapV3> + DataProvider<Bcp47ToIanaMapV1> + ?Sized,
    {
        let mapper = IanaParser::try_new_unstable(provider)?;
        Self::try_new_with_mapper_unstable(provider, mapper)
    }
}

impl<I> IanaParserExtended<I>
where
    I: AsRef<IanaParser>,
{
    /// Creates a new [`IanaParserExtended`] using compiled data
    /// and a pre-existing [`IanaParser`], which can be borrowed.
    ///
    /// See [`IanaParserExtended`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_mapper(mapper: I) -> Result<Self, DataError> {
        if mapper.as_ref().checksum
            != crate::provider::Baked::SINGLETON_BCP47_TO_IANA_MAP_V1_CHECKSUM
        {
            return Err(DataErrorKind::InconsistentData(IanaToBcp47MapV3::INFO)
                .with_marker(Bcp47ToIanaMapV1::INFO));
        }
        Ok(Self {
            inner: mapper,
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_BCP47_TO_IANA_MAP_V1,
            ),
        })
    }

    icu_provider::gen_buffer_data_constructors!((mapper: I) -> error: DataError,
        functions: [
            try_new_with_mapper: skip,
            try_new_with_mapper_with_buffer_provider,
            try_new_with_mapper_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_with_mapper_unstable<P>(provider: &P, mapper: I) -> Result<Self, DataError>
    where
        P: DataProvider<IanaToBcp47MapV3> + DataProvider<Bcp47ToIanaMapV1> + ?Sized,
    {
        let response = provider.load(Default::default())?;
        if Some(mapper.as_ref().checksum) != response.metadata.checksum {
            return Err(DataErrorKind::InconsistentData(IanaToBcp47MapV3::INFO)
                .with_marker(Bcp47ToIanaMapV1::INFO));
        }
        Ok(Self {
            inner: mapper,
            data: response.payload,
        })
    }

    /// Gets the inner [`IanaParser`] for performing queries.
    pub fn inner(&self) -> &IanaParser {
        self.inner.as_ref()
    }

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential indirection cost when querying the mapper.
    pub fn as_borrowed(&self) -> IanaParserExtendedBorrowed {
        IanaParserExtendedBorrowed {
            inner: self.inner.as_ref().as_borrowed(),
            data: self.data.get(),
        }
    }
}

/// A borrowed wrapper around the time zone ID mapper, returned by
/// [`IanaParserExtended::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct IanaParserExtendedBorrowed<'a> {
    inner: IanaParserBorrowed<'a>,
    data: &'a Bcp47ToIanaMap<'a>,
}

#[cfg(feature = "compiled_data")]
impl Default for IanaParserExtendedBorrowed<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl IanaParserExtendedBorrowed<'static> {
    /// Creates a new [`IanaParserExtendedBorrowed`] using compiled data.
    ///
    /// See [`IanaParserExtendedBorrowed`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new() -> Self {
        const _: () = assert!(
            crate::provider::Baked::SINGLETON_IANA_TO_BCP47_MAP_V3_CHECKSUM
                == crate::provider::Baked::SINGLETON_BCP47_TO_IANA_MAP_V1_CHECKSUM,
        );
        Self {
            inner: IanaParserBorrowed::new(),
            data: crate::provider::Baked::SINGLETON_BCP47_TO_IANA_MAP_V1,
        }
    }

    /// Cheaply converts a [`IanaParserExtendedBorrowed<'static>`] into a [`IanaParserExtended`].
    ///
    /// Note: Due to branching and indirection, using [`IanaParserExtended`] might inhibit some
    /// compile-time optimizations that are possible with [`IanaParserExtendedBorrowed`].
    pub fn static_to_owned(&self) -> IanaParserExtended<IanaParser> {
        IanaParserExtended {
            inner: self.inner.static_to_owned(),
            data: DataPayload::from_static_ref(self.data),
        }
    }
}

impl<'a> IanaParserExtendedBorrowed<'a> {
    /// Gets the inner [`IanaParserBorrowed`] for performing queries.
    pub fn inner(&self) -> IanaParserBorrowed<'a> {
        self.inner
    }

    /// Returns the canonical, normalized identifier of the given IANA time zone.
    ///
    /// Also returns the BCP-47 time zone ID.
    ///
    /// This is a faster version of [`IanaParserBorrowed::canonicalize_iana()`]
    /// and it always returns borrowed IANA strings, but it requires loading additional data
    /// (see [`IanaParserExtended`]).
    ///
    /// Returns `None` if the IANA ID is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_time::TimeZone;
    /// use icu_time::zone::iana::IanaParserExtended;
    /// use std::borrow::Cow;
    ///
    /// let mapper = IanaParserExtended::new();
    ///
    /// let result = mapper.canonicalize_iana("Asia/CALCUTTA").unwrap();
    ///
    /// // The Cow is always returned borrowed:
    /// assert_eq!(result.0, "Asia/Kolkata");
    /// assert_eq!(*result.1, "inccu");
    ///
    /// // Unknown IANA time zone ID:
    /// assert_eq!(mapper.canonicalize_iana("America/San_Francisco"), None);
    /// ```
    pub fn canonicalize_iana(&self, iana_id: &str) -> Option<(TimeZoneIanaIdBorrowed, TimeZone)> {
        let trie_value = self.inner.iana_lookup_quick(iana_id)?;
        let Some(bcp47_id) = self.inner.data.bcp47_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return None;
        };
        let Some(string) = self.data.canonical_iana_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return None;
        };
        Some((TimeZoneIanaIdBorrowed(string), bcp47_id))
    }

    /// Returns the canonical, normalized IANA ID of the given BCP-47 ID.
    ///
    /// This is a faster version of [`IanaParserBorrowed::find_canonical_iana_from_bcp47()`]
    /// and it always returns borrowed IANA strings, but it requires loading additional data
    /// (see [`IanaParserExtended`]).
    ///
    /// Returns `None` if the BCP-47 ID is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_time::TimeZone;
    /// use icu_time::zone::iana::IanaParserExtended;
    /// use std::borrow::Cow;
    /// use tinystr::tinystr;
    ///
    /// let mapper = IanaParserExtended::new();
    ///
    /// let bcp47_id = TimeZone(tinystr!(8, "inccu"));
    /// let result = mapper.canonical_iana_from_bcp47(bcp47_id).unwrap();
    ///
    /// // The Cow is always returned borrowed:
    /// assert_eq!(result, "Asia/Kolkata");
    ///
    /// // Unknown BCP-47 time zone ID:
    /// let bcp47_id = TimeZone(tinystr!(8, "ussfo"));
    /// assert_eq!(mapper.canonical_iana_from_bcp47(bcp47_id), None);
    /// ```
    pub fn canonical_iana_from_bcp47(&self, bcp47_id: TimeZone) -> Option<TimeZoneIanaIdBorrowed> {
        let index = self.inner.data.bcp47_ids.binary_search(&bcp47_id).ok()?;
        let Some(string) = self.data.canonical_iana_ids.get(index) else {
            debug_assert!(false, "index should be in range");
            return None;
        };
        Some(TimeZoneIanaIdBorrowed(string))
    }

    /// Returns an iterator over all canonical IANA time zone identifiers in an arbitrary, unstable order.
    ///
    /// To iterate over BCP-47 IDs, use [`IanaParserBorrowed::iter_bcp47()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::zone::iana::IanaParserExtended;
    ///
    /// let ids = IanaParserExtended::new()
    ///     .iter_canonical_iana()
    ///     .skip(30)
    ///     .take(5)
    ///     .map(|id| id.to_string())
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     ids,
    ///     &[
    ///         "America/Argentina/Ushuaia",
    ///         "Pacific/Pago_Pago",
    ///         "Europe/Vienna",
    ///         "Australia/Adelaide",
    ///         "Australia/Broken_Hill",
    ///     ]
    /// );
    /// ```
    pub fn iter_canonical_iana(&self) -> TimeZoneCanonicalIanaIter {
        TimeZoneCanonicalIanaIter {
            inner: self.data.canonical_iana_ids.iter(),
        }
    }
}

/// An iterator over canonical IANA time zone identifiers.
///
/// See [`IanaParserExtendedBorrowed::iter_canonical_iana()`]
#[derive(Debug)]
pub struct TimeZoneCanonicalIanaIter<'a> {
    inner: VarZeroSliceIter<'a, str>,
}

impl<'a> Iterator for TimeZoneCanonicalIanaIter<'a> {
    type Item = TimeZoneIanaIdBorrowed<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(TimeZoneIanaIdBorrowed)
    }
}

/// A time zone IANA ID.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TimeZoneIanaIdBorrowed<'a>(&'a str);

impl Writeable for TimeZoneIanaIdBorrowed<'_> {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        self.0.write_to(sink)
    }
    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        Cow::Borrowed(self.0)
    }
}

impl_display_with_writeable!(TimeZoneIanaIdBorrowed<'_>);

impl PartialEq<&str> for TimeZoneIanaIdBorrowed<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct IanaTrieValue(usize);

impl IanaTrieValue {
    #[inline]
    pub(crate) fn to_canonical(self) -> Self {
        Self(self.0 | 1)
    }
    #[inline]
    pub(crate) fn canonical_for_index(index: usize) -> Self {
        Self(index << 1).to_canonical()
    }
    #[inline]
    pub(crate) fn index(self) -> usize {
        self.0 >> 1
    }
    #[inline]
    pub(crate) fn is_canonical(self) -> bool {
        (self.0 & 0x1) != 0
    }
}
