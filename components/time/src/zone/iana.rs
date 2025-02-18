// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO

use icu_provider::prelude::*;
use zerovec::vecs::{VarZeroSliceIter, ZeroSliceIter};

use crate::{
    provider::iana::{
        IanaNames, IanaToBcp47Map, TimeZoneIanaBasicV1, TimeZoneIanaExtendedV1,
        NON_REGION_CITY_PREFIX,
    },
    TimeZone,
};

/// A parser between IANA time zone identifiers and BCP-47 time zone identifiers.
///
/// This parser supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
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
/// let parser = IanaParser::new();
///
/// // The IANA zone "Australia/Melbourne" is the BCP-47 zone "aumel":
/// assert_eq!(
///     parser.parse("Australia/Melbourne"),
///     TimeZone(tinystr!(8, "aumel"))
/// );
///
/// // Lookup is ASCII-case-insensitive:
/// assert_eq!(
///     parser.parse("australia/melbourne"),
///     TimeZone(tinystr!(8, "aumel"))
/// );
///
/// // The IANA zone "Australia/Victoria" is an alias:
/// assert_eq!(
///     parser.parse("Australia/Victoria"),
///     TimeZone(tinystr!(8, "aumel"))
/// );
///
/// // The IANA zone "Australia/Boing_Boing" does not exist
/// // (maybe not *yet*), so it produces the special unknown
/// // timezone in order for this operation to be infallible:
/// assert_eq!(
///     parser.parse("Australia/Boing_Boing"),
///     TimeZone::unknown()
/// );
/// ```
#[derive(Debug, Clone)]
pub struct IanaParser {
    data: DataPayload<TimeZoneIanaBasicV1>,
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
        P: DataProvider<TimeZoneIanaBasicV1> + ?Sized,
    {
        let response = provider.load(Default::default())?;
        Ok(Self {
            data: response.payload,
            checksum: response.metadata.checksum.ok_or_else(|| {
                DataError::custom("Missing checksum")
                    .with_req(TimeZoneIanaBasicV1::INFO, Default::default())
            })?,
        })
    }

    /// Returns a borrowed version of the parser that can be queried.
    ///
    /// This avoids a small potential indirection cost when querying the parser.
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

/// A borrowed wrapper around the time zone ID parser, returned by
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
            data: crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_BASIC_V1,
            checksum: crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_BASIC_V1_CHECKSUM,
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
    /// let parser = IanaParser::new();
    ///
    /// let result = parser.parse("Asia/CALCUTTA");
    ///
    /// assert_eq!(*result, "inccu");
    ///
    /// // Unknown IANA time zone ID:
    /// assert_eq!(
    ///     parser.parse("America/San_Francisco"),
    ///     TimeZone::unknown()
    /// );
    /// ```
    pub fn parse(&self, iana_id: &str) -> TimeZone {
        self.parse_from_utf8(iana_id.as_bytes())
    }

    /// Same as [`Self::parse()`] but works with potentially ill-formed UTF-8.
    pub fn parse_from_utf8(&self, iana_id: &[u8]) -> TimeZone {
        let mut cursor = self.data.map.cursor();
        if !iana_id.contains(&b'/') {
            cursor.step(NON_REGION_CITY_PREFIX);
        }
        for &b in iana_id {
            cursor.step(b);
        }
        cursor
            .take_value()
            .map(IanaTrieValue)
            .and_then(|trie_value| self.data.bcp47_ids.get(trie_value.index()))
            .unwrap_or(TimeZone::unknown())
    }

    /// Returns an iterator over all known time zones.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::zone::IanaParser;
    /// use icu::time::zone::TimeZone;
    /// use std::collections::BTreeSet;
    /// use tinystr::tinystr;
    ///
    /// let parser = IanaParser::new();
    ///
    /// let ids = parser.iter().collect::<BTreeSet<_>>();
    ///
    /// assert!(ids.contains(&TimeZone(tinystr!(8, "uaiev"))));
    /// ```
    pub fn iter(&self) -> TimeZoneIter {
        TimeZoneIter {
            inner: self.data.bcp47_ids.iter(),
        }
    }
}

/// Returned by [`IanaParserBorrowed::iter()`]
#[derive(Debug)]
pub struct TimeZoneIter<'a> {
    inner: ZeroSliceIter<'a, TimeZone>,
}

impl Iterator for TimeZoneIter<'_> {
    type Item = TimeZone;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// A parser that supplements [`IanaParser`] with about 8 KB of additional data to
/// improve the performance of canonical IANA ID lookup.
///
/// The data in [`IanaParser`] is optimized for IANA to BCP-47 lookup; the reverse
/// requires a linear walk over all ~600 IANA identifiers. The data added here allows for
/// constant-time mapping from BCP-47 to IANA.
#[derive(Debug, Clone)]
pub struct IanaParserExtended<I> {
    inner: I,
    data: DataPayload<TimeZoneIanaExtendedV1>,
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
        P: DataProvider<TimeZoneIanaBasicV1> + DataProvider<TimeZoneIanaExtendedV1> + ?Sized,
    {
        let parser = IanaParser::try_new_unstable(provider)?;
        Self::try_new_with_parser_unstable(provider, parser)
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
    pub fn try_new_with_parser(parser: I) -> Result<Self, DataError> {
        if parser.as_ref().checksum
            != crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_EXTENDED_V1_CHECKSUM
        {
            return Err(DataErrorKind::InconsistentData(TimeZoneIanaBasicV1::INFO)
                .with_marker(TimeZoneIanaExtendedV1::INFO));
        }
        Ok(Self {
            inner: parser,
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_EXTENDED_V1,
            ),
        })
    }

    icu_provider::gen_buffer_data_constructors!((parser: I) -> error: DataError,
        functions: [
            try_new_with_parser: skip,
            try_new_with_parser_with_buffer_provider,
            try_new_with_parser_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_with_parser_unstable<P>(provider: &P, parser: I) -> Result<Self, DataError>
    where
        P: DataProvider<TimeZoneIanaBasicV1> + DataProvider<TimeZoneIanaExtendedV1> + ?Sized,
    {
        let response = provider.load(Default::default())?;
        if Some(parser.as_ref().checksum) != response.metadata.checksum {
            return Err(DataErrorKind::InconsistentData(TimeZoneIanaBasicV1::INFO)
                .with_marker(TimeZoneIanaExtendedV1::INFO));
        }
        Ok(Self {
            inner: parser,
            data: response.payload,
        })
    }

    /// Returns a borrowed version of the parser that can be queried.
    ///
    /// This avoids a small potential indirection cost when querying the parser.
    pub fn as_borrowed(&self) -> IanaParserExtendedBorrowed {
        IanaParserExtendedBorrowed {
            inner: self.inner.as_ref().as_borrowed(),
            data: self.data.get(),
        }
    }
}

/// A borrowed wrapper around the time zone ID parser, returned by
/// [`IanaParserExtended::as_borrowed()`]. More efficient to query.
#[derive(Debug, Copy, Clone)]
pub struct IanaParserExtendedBorrowed<'a> {
    inner: IanaParserBorrowed<'a>,
    data: &'a IanaNames<'a>,
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
            crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_BASIC_V1_CHECKSUM
                == crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_EXTENDED_V1_CHECKSUM,
        );
        Self {
            inner: IanaParserBorrowed::new(),
            data: crate::provider::Baked::SINGLETON_TIME_ZONE_IANA_EXTENDED_V1,
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
    /// Gets the time zone, the canonical IANA ID, and the normalized IANA ID from an IANA time zone ID.
    ///
    /// Returns [`(TimeZone::unknown(), "Etc/Unknown", "Etc/Unknown")`] if the IANA ID is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_time::TimeZone;
    /// use icu_time::zone::iana::IanaParserExtended;
    ///
    /// let parser = IanaParserExtended::new();
    ///
    /// let (tz, canonical, normalized) = parser.parse("Asia/CALCUTTA");
    ///
    /// assert_eq!(*tz, "inccu");
    /// assert_eq!(canonical, "Asia/Kolkata");
    /// assert_eq!(normalized, "Asia/Calcutta");
    ///
    /// // Unknown IANA time zone ID:
    /// assert_eq!(
    ///     parser.parse("America/San_Francisco"),
    ///     (TimeZone::unknown(), "Etc/Unknown", "Etc/Unknown".into())
    /// );
    /// ```
    pub fn parse(&self, iana_id: &str) -> (TimeZone, &'a str, &'a str) {
        self.parse_from_utf8(iana_id.as_bytes())
    }

    /// Same as [`Self::parse()`] but works with potentially ill-formed UTF-8.
    pub fn parse_from_utf8(&self, iana_id: &[u8]) -> (TimeZone, &'a str, &'a str) {
        const FAILURE: (TimeZone, &str, &str) = (TimeZone::unknown(), "Etc/Unknown", "Etc/Unknown");
        let mut cursor = self.inner.data.map.cursor();
        if !iana_id.contains(&b'/') {
            cursor.step(NON_REGION_CITY_PREFIX);
        }
        for &b in iana_id {
            cursor.step(b);
        }
        let Some(trie_value) = cursor.take_value().map(IanaTrieValue) else {
            return FAILURE;
        };
        let Some(bcp47_id) = self.inner.data.bcp47_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return FAILURE;
        };
        let Some(canonical) = self.data.normalized_iana_ids.get(trie_value.index()) else {
            debug_assert!(false, "index should be in range");
            return FAILURE;
        };
        let normalized = if trie_value.is_canonical() {
            canonical
        } else {
            let Some(Ok(index)) = self.data.normalized_iana_ids.binary_search_in_range_by(
                |a| {
                    a.as_bytes()
                        .iter()
                        .map(u8::to_ascii_lowercase)
                        .cmp(iana_id.iter().map(u8::to_ascii_lowercase))
                },
                self.inner.data.bcp47_ids.len()..self.data.normalized_iana_ids.len(),
            ) else {
                debug_assert!(
                    false,
                    "binary search should succeed if trie lookup succeeds"
                );
                return FAILURE;
            };
            let Some(normalized) = self
                .data
                .normalized_iana_ids
                .get(self.inner.data.bcp47_ids.len() + index)
            else {
                debug_assert!(false, "binary search returns valid index");
                return FAILURE;
            };
            normalized
        };
        (bcp47_id, canonical, normalized)
    }

    /// Returns an iterator over all known time zones.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::zone::iana::IanaParserExtended;
    /// use icu::time::zone::TimeZone;
    /// use std::collections::BTreeSet;
    /// use tinystr::tinystr;
    ///
    /// let parser = IanaParserExtended::new();
    ///
    /// let ids = parser.iter().collect::<BTreeSet<_>>();
    ///
    /// assert!(ids.contains(&TimeZone(tinystr!(8, "uaiev"))));
    /// assert_eq!(ids.len(), 445);
    /// ```
    pub fn iter(&self) -> TimeZoneIter {
        self.inner.iter()
    }

    /// Returns an iterator over all IANA time zone identifiers that are canonical for some
    /// CLDR time zone.
    ///
    /// This iterator has the same length and ordering as [`Self::iter()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::zone::iana::IanaParserExtended;
    /// use icu::time::zone::TimeZone;
    /// use std::collections::BTreeSet;
    /// use tinystr::tinystr;
    ///
    /// let parser = IanaParserExtended::new();
    ///
    /// let ids = parser.iter_canonical().collect::<BTreeSet<_>>();
    ///
    /// assert!(ids.contains("Europe/Kyiv"));
    /// assert!(!ids.contains("Europe/Kiev"));
    /// assert_eq!(ids.len(), 445);
    ///
    /// assert_eq!(
    ///     parser.iter().zip(parser.iter_canonical()).nth(367).unwrap(), 
    ///     (TimeZone(tinystr!(8, "uaiev")), "Europe/Kyiv")
    /// );
    /// ```
    pub fn iter_canonical(&self) -> CanonicalIanaIter<'a> {
        CanonicalIanaIter(
            self.data
                .normalized_iana_ids
                .iter()
                .take(self.inner.data.bcp47_ids.len()),
        )
    }

    /// Returns an iterator over all IANA time zone identifiers in an arbitrary, unstable order.
    ///
    /// To iterate over CLDR time zones, use [`Self::iter()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::zone::iana::IanaParserExtended;
    ///
    /// let parser = IanaParserExtended::new();
    ///
    /// let ids = parser.iter_normalized().collect::<std::collections::BTreeSet<_>>();
    ///
    /// assert!(ids.contains("Europe/Kyiv"));
    /// assert!(ids.contains("Europe/Kiev"));
    /// assert_eq!(ids.len(), 598);
    /// ```
    pub fn iter_normalized(&self) -> NormalizedIanaIter<'a> {
        NormalizedIanaIter(self.data.normalized_iana_ids.iter())
    }
}

/// The iterator returned by [`IanaParserExtendedBorrowed::iter_canonical()`]
#[derive(Debug)]
pub struct CanonicalIanaIter<'a>(core::iter::Take<VarZeroSliceIter<'a, str>>);

impl<'a> Iterator for CanonicalIanaIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// The iterator returned by [`IanaParserExtendedBorrowed::iter_normalized()`]
#[derive(Debug)]
pub struct NormalizedIanaIter<'a>(VarZeroSliceIter<'a, str>);

impl<'a> Iterator for NormalizedIanaIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct IanaTrieValue(usize);

impl IanaTrieValue {
    #[inline]
    pub(crate) fn index(self) -> usize {
        self.0 >> 1
    }
    #[inline]
    pub(crate) fn is_canonical(self) -> bool {
        (self.0 & 0x1) != 0
    }
}
