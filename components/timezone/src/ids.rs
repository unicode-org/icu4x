// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use alloc::string::String;
use icu_provider::prelude::*;
use zerotrie::cursor::ZeroAsciiIgnoreCaseTrieCursor;

use crate::{
    provider::names::{IanaToBcp47MapV2, IanaToBcp47MapV2Marker},
    TimeZoneBcp47Id,
};

/// A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
///
/// This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
/// It also supports normalizing and canonicalizing the IANA strings.
///
/// # Examples
///
/// ```
/// use icu::timezone::TimeZoneIdMapper;
///
/// let mapper = TimeZoneIdMapper::new();
/// let mapper = mapper.as_borrowed();
///
/// // The IANA zone "Australia/Melbourne" is the BCP-47 zone "aumel":
/// assert_eq!(
///     mapper.iana_to_bcp47("Australia/Melbourne"),
///     Some("aumel".parse().unwrap())
/// );
///
/// // Lookup is ASCII-case insensitive:
/// assert_eq!(
///     mapper.iana_to_bcp47("australia/melbourne"),
///     Some("aumel".parse().unwrap())
/// );
///
/// // The IANA zone "Australia/Victoria" is an alias:
/// assert_eq!(
///     mapper.iana_to_bcp47("Australia/Victoria"),
///     Some("aumel".parse().unwrap())
/// );
///
/// // We can recover the canonical name from the mapper:
/// assert_eq!(
///     mapper
///         .canonicalize_iana("Australia/Victoria")
///         .unwrap()
///         .string,
///     "Australia/Melbourne"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TimeZoneIdMapper {
    data: DataPayload<IanaToBcp47MapV2Marker>,
}

impl TimeZoneIdMapper {
    /// Creates a new [`TimeZoneIdMapper`] using compiled data.
    ///
    /// See [`TimeZoneIdMapper`] for an example.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new() -> Self {
        Self {
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V2,
            ),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: skip, error: DataError,
        #[cfg(skip)]
        functions: [
            new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: DataProvider<IanaToBcp47MapV2Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.take_payload()?;
        Ok(Self { data })
    }

    /// Returns a borrowed version of the mapper that can be queried.
    ///
    /// This avoids a small potential cost of reading the data pointer.
    pub fn as_borrowed(&self) -> TimeZoneIdMapperBorrowed {
        TimeZoneIdMapperBorrowed {
            data: self.data.get(),
        }
    }
}

/// A borrowed wrapper around the time zone ID mapper, returned by
/// [`TimeZoneIdMapper::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct TimeZoneIdMapperBorrowed<'a> {
    data: &'a IanaToBcp47MapV2<'a>,
}

impl<'a> TimeZoneIdMapperBorrowed<'a> {
    /// Gets the BCP-47 time zone ID from an IANA time zone ID
    /// with a case-insensitive lookup.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_timezone::TimeZoneBcp47Id;
    /// use icu_timezone::TimeZoneIdMapper;
    /// use tinystr::tinystr;
    ///
    /// let mapper = TimeZoneIdMapper::new();
    /// let mapper = mapper.as_borrowed();
    ///
    /// assert_eq!(
    ///     mapper.iana_to_bcp47("AMERICA/chicago"),
    ///     Some(TimeZoneBcp47Id(tinystr!(8, "uschi")))
    /// );
    /// ```
    pub fn iana_to_bcp47(&self, iana_id: &str) -> Option<TimeZoneBcp47Id> {
        self.iana_lookup_quick(iana_id)
            .and_then(|trie_value| self.data.bcp47_ids.get(trie_value.index()))
    }

    /// Normalizes the syntax of an IANA time zone ID.
    ///
    /// Also returns the BCP-47 time zone ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_timezone::TimeZoneBcp47Id;
    /// use icu_timezone::TimeZoneIdMapper;
    /// use tinystr::tinystr;
    ///
    /// let mapper = TimeZoneIdMapper::new();
    /// let mapper = mapper.as_borrowed();
    ///
    /// let normalized = mapper.normalize_iana("AMERICA/chicago").unwrap();
    ///
    /// assert_eq!(normalized.string, "America/Chicago");
    /// assert_eq!(normalized.bcp47_id, TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// ```
    pub fn normalize_iana<'s>(&self, iana_id: &'s str) -> Option<NormalizedIana<'s>> {
        let Some((trie_value, string)) = self.iana_lookup_with_normalization(iana_id, |_| {})
        else {
            return None;
        };
        let Some(bcp47_id) = self.data.bcp47_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return None;
        };
        Some(NormalizedIana { string, bcp47_id })
    }

    /// Returns the canonical, normalized name of the given IANA time zone.
    ///
    /// Also returns the BCP-47 time zone ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_timezone::TimeZoneBcp47Id;
    /// use icu_timezone::TimeZoneIdMapper;
    /// use tinystr::tinystr;
    ///
    /// let mapper = TimeZoneIdMapper::new();
    /// let mapper = mapper.as_borrowed();
    ///
    /// let canonicalized = mapper.canonicalize_iana("Asia/Calcutta").unwrap();
    ///
    /// assert_eq!(canonicalized.string, "Asia/Kolkata");
    /// assert_eq!(
    ///     canonicalized.bcp47_id,
    ///     TimeZoneBcp47Id(tinystr!(8, "inccu"))
    /// );
    /// ```
    pub fn canonicalize_iana<'s>(&self, iana_id: &'s str) -> Option<NormalizedIana<'s>> {
        // Note: We collect the cursors into a stack so that we start probing
        // nearby the input IANA name. This should improve lookup time since
        // most renames share the same prefix like "Asia" or "Europe".
        let mut stack = Vec::with_capacity(iana_id.len());
        let Some((trie_value, mut string)) =
            self.iana_lookup_with_normalization(iana_id, |cursor| {
                stack.push((cursor.clone(), 0, 1));
            })
        else {
            return None;
        };
        let Some(bcp47_id) = self.data.bcp47_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return None;
        };
        if trie_value.is_canonical() {
            return Some(NormalizedIana { string, bcp47_id });
        }
        // If we get here, we need to walk the trie to find the canonical IANA ID.
        loop {
            let Some((mut cursor, index, suffix_len)) = stack.pop() else {
                // Nothing left in the trie.
                debug_assert!(false, "every time zone should have a canonical IANA ID");
                return None;
            };
            // Check to see if there is a value at the current node.
            if let Some(candidate) = cursor.take_value().map(IanaTrieValue) {
                if candidate.index() == trie_value.index() && candidate.is_canonical() {
                    // Success! Found what we were looking for.
                    break;
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
                unsafe { string.to_mut().as_mut_vec().push(probe_result.byte) };
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
                    let removed_byte = unsafe { string.to_mut().as_mut_vec().pop() };
                    if let Some(removed_byte) = removed_byte {
                        if !removed_byte.is_ascii() {
                            debug_assert!(false, "non-ASCII removed byte: {removed_byte}");
                            return None;
                        }
                    } else {
                        debug_assert!(false, "could not remove another byte");
                        return None;
                    }
                }
            }
        }
        Some(NormalizedIana { string, bcp47_id })
    }

    /// Returns the canonical, normalized IANA ID of the given BCP-47 ID.
    ///
    /// Only use this function if you don't have the IANA ID. [`Self::canonicalize_iana()`]
    /// is much faster in the common case.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_timezone::TimeZoneBcp47Id;
    /// use icu_timezone::TimeZoneIdMapper;
    /// use tinystr::tinystr;
    ///
    /// let mapper = TimeZoneIdMapper::new();
    /// let mapper = mapper.as_borrowed();
    ///
    /// let canonicalized = mapper.canonicalize_iana("Asia/Calcutta").unwrap();
    ///
    /// assert_eq!(canonicalized.string, "Asia/Kolkata");
    /// assert_eq!(
    ///     canonicalized.bcp47_id,
    ///     TimeZoneBcp47Id(tinystr!(8, "inccu"))
    /// );
    /// ```
    pub fn bcp47_to_iana_search(&self, bcp47_id: TimeZoneBcp47Id) -> Option<String> {
        // TODO: This is not as efficient as .probe() since it allocates a string each time.
        for (string, raw_value) in self.data.map.iter() {
            let trie_value = IanaTrieValue(raw_value);
            if !trie_value.is_canonical() {
                continue;
            }
            let Some(candidate_bcp47_id) = self.data.bcp47_ids.get(trie_value.index()) else {
                debug_assert!(false, "index should be in range");
                return None;
            };
            if candidate_bcp47_id == bcp47_id {
                return Some(string);
            }
        }
        None
    }

    fn iana_lookup_quick(&self, iana_id: &str) -> Option<IanaTrieValue> {
        self.data.map.get(iana_id).map(IanaTrieValue)
    }

    fn iana_lookup_with_normalization<'l, 's>(
        &'l self,
        iana_id: &'s str,
        mut cursor_fn: impl FnMut(&ZeroAsciiIgnoreCaseTrieCursor<'l>),
    ) -> Option<(IanaTrieValue, Cow<'s, str>)> {
        let mut cursor = self.data.map.cursor();
        let mut string = Cow::Borrowed(iana_id);
        let mut i = 0;
        let trie_value = loop {
            cursor_fn(&cursor);
            let Some(input_byte) = string.as_bytes().get(i).copied() else {
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
}

/// A wrapper around a syntax-normalized IANA time zone identifier string
/// and its corresponding BCP-47 time zone identifier.
#[derive(Debug)]
pub struct NormalizedIana<'s> {
    /// The syntax-normalized IANA time zone identifier string.
    pub string: Cow<'s, str>,
    /// The corresponding BCP-47 time zone identifier.
    pub bcp47_id: TimeZoneBcp47Id,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct IanaTrieValue(usize);

impl IanaTrieValue {
    #[inline]
    pub fn index(self) -> usize {
        self.0 >> 1
    }
    #[inline]
    pub fn is_canonical(self) -> bool {
        (self.0 & 0x1) != 0
    }
}
