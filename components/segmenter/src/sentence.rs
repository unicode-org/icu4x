// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use icu_locale_core::LanguageIdentifier;
use icu_provider::prelude::*;

use crate::indices::{Latin1Indices, Utf16Indices};
use crate::provider::*;
use crate::rule_segmenter::*;
use utf8_iter::Utf8CharIndices;

/// Options to tailor sentence breaking behavior.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct SentenceBreakOptions<'a> {
    /// Content locale for sentence segmenter.
    ///
    /// If you know the language of the text being segmented, provide it here in order to produce
    /// higher quality breakpoints.
    ///
    /// # Examples
    ///
    /// Normally, a semicolon character ';' is not a sentence separator:
    ///
    /// ```rust
    /// use icu::segmenter::SentenceSegmenter;
    ///
    /// let segmenter = SentenceSegmenter::new(Default::default());
    ///
    /// let breakpoints: Vec<usize> = segmenter.segment_str("hello; world").collect();
    /// assert_eq!(&breakpoints, &[0, 12]);
    /// ```
    ///
    /// But not in Greek, where it is used as a question mark:
    ///
    /// ```rust
    /// use icu::locale::langid;
    /// use icu::segmenter::options::SentenceBreakOptions;
    /// use icu::segmenter::SentenceSegmenter;
    ///
    /// let mut options = SentenceBreakOptions::default();
    /// let langid = &langid!("el");
    /// options.content_locale = Some(langid);
    /// let segmenter = SentenceSegmenter::try_new(options).unwrap();
    ///
    /// let breakpoints: Vec<usize> =
    ///     segmenter.as_borrowed().segment_str("hello; world").collect();
    /// assert_eq!(&breakpoints, &[0, 7, 12]);
    /// ```
    pub content_locale: Option<&'a LanguageIdentifier>,
    /// Options independent of the locale
    pub invariant_options: SentenceBreakInvariantOptions,
}

/// Locale-independent options to tailor sentence breaking behavior
///
/// Currently empty but may grow in the future
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct SentenceBreakInvariantOptions {}

/// Implements the [`Iterator`] trait over the sentence boundaries of the given string.
///
/// Lifetimes:
///
/// - `'data` = lifetime of the segmenter object from which this iterator was created
/// - `'s` = lifetime of the string being segmented
///
/// The [`Iterator::Item`] is an [`usize`] representing index of a code unit
/// _after_ the boundary (for a boundary at the end of text, this index is the length
/// of the [`str`] or array of code units).
///
/// For examples of use, see [`SentenceSegmenter`].
#[derive(Debug)]
pub struct SentenceBreakIterator<'data, 's, Y: RuleBreakType>(
    SentenceBreakIteratorInner<'data, 's, Y>,
);

#[derive(Debug)]
enum SentenceBreakIteratorInner<'data, 's, Y: RuleBreakType> {
    Legacy(RuleBreakIterator<'data, 's, Y>),
    #[cfg(feature = "unstable")]
    Neo(crate::neo::RuleBreakIterator<'data, 's, Y, crate::neo::NoComplexHandler>),
}

impl<Y: RuleBreakType> Iterator for SentenceBreakIterator<'_, '_, Y> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            SentenceBreakIteratorInner::Legacy(iter) => iter.next(),
            #[cfg(feature = "unstable")]
            SentenceBreakIteratorInner::Neo(iter) => iter.next(),
        }
    }
}

/// Supports loading sentence break data, and creating sentence break iterators for different string
/// encodings.
///
/// Most segmentation methods live on [`SentenceSegmenterBorrowed`], which can be obtained via
/// [`SentenceSegmenter::new()`] or [`SentenceSegmenter::as_borrowed()`].
///
/// Sentence segmenter is currently compatible with [Unicode Standard Annex #29][UAX29] (Version 17.0.0).
///
/// [UAX29]: https://www.unicode.org/reports/tr29/tr29-47.html
///
/// # Content Locale
///
/// You can optionally provide a _content locale_ to the [`SentenceSegmenter`] constructor. If you
/// have information on the language of the text being segmented, providing this hint can
/// produce higher-quality results.
///
/// If you have a content locale, use [`SentenceBreakOptions`] and a constructor begining with `new`.
/// If you do not have a content locale use [`SentenceBreakInvariantOptions`] and a constructor
/// beginning with `try_new`.
///
/// # Examples
///
/// Segment a string:
///
/// ```rust
/// use icu::segmenter::SentenceSegmenter;
///
/// let segmenter = SentenceSegmenter::new(Default::default());
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 11]);
/// ```
///
/// Segment a Latin1 byte string with a content locale:
///
/// ```rust
/// use icu::locale::langid;
/// use icu::segmenter::options::SentenceBreakOptions;
/// use icu::segmenter::SentenceSegmenter;
///
/// let mut options = SentenceBreakOptions::default();
/// let langid = &langid!("en");
/// options.content_locale = Some(langid);
/// let segmenter = SentenceSegmenter::try_new(options).unwrap();
///
/// let breakpoints: Vec<usize> = segmenter
///     .as_borrowed()
///     .segment_latin1(b"Hello World")
///     .collect();
/// assert_eq!(&breakpoints, &[0, 11]);
/// ```
///
/// Successive boundaries can be used to retrieve the sentences.
/// In particular, the first boundary is always 0, and the last one is the
/// length of the segmented text in code units.
///
/// ```rust
/// # use icu::segmenter::{SentenceSegmenter, options::SentenceBreakInvariantOptions};
/// # let segmenter = SentenceSegmenter::new(SentenceBreakInvariantOptions::default());
/// use itertools::Itertools;
/// let text = "Ceci tuera cela. Le livre tuera l’édifice.";
/// let sentences: Vec<&str> = segmenter
///     .segment_str(text)
///     .tuple_windows()
///     .map(|(i, j)| &text[i..j])
///     .collect();
/// assert_eq!(
///     &sentences,
///     &["Ceci tuera cela. ", "Le livre tuera l’édifice."]
/// );
/// ```
#[derive(Debug)]
pub struct SentenceSegmenter(SentenceSegmenterInner);

#[derive(Debug)]
enum SentenceSegmenterInner {
    Legacy {
        payload: DataPayload<SegmenterBreakSentenceV1>,
        payload_locale_override: Option<DataPayload<SegmenterBreakSentenceOverrideV1>>,
    },
    #[cfg(feature = "unstable")]
    Neo {
        payload: DataPayload<SegmenterBreakSentenceV2>,
        tailoring: Option<DataPayload<SegmenterBreakSentenceOverrideV2>>,
    },
}

/// Segments a string into sentences (borrowed version).
///
/// See [`SentenceSegmenter`] for examples.
#[derive(Clone, Debug, Copy)]
pub struct SentenceSegmenterBorrowed<'data>(SentenceSegmenterBorrowedInner<'data>);

#[derive(Clone, Debug, Copy)]
enum SentenceSegmenterBorrowedInner<'data> {
    Legacy {
        data: &'data RuleBreakData<'data>,
        locale_override: Option<&'data RuleBreakDataOverride<'data>>,
    },
    #[cfg(feature = "unstable")]
    Neo {
        data: &'data SegmenterStateMachine<'data>,
        tailoring: Option<&'data SegmenterStateMachineOverride<'data>>,
    },
}

impl SentenceSegmenter {
    /// Constructs a [`SentenceSegmenterBorrowed`] with an invariant locale and compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[expect(clippy::new_ret_no_self)]
    pub const fn new(
        _options: SentenceBreakInvariantOptions,
    ) -> SentenceSegmenterBorrowed<'static> {
        SentenceSegmenterBorrowed(SentenceSegmenterBorrowedInner::Legacy {
            data: Baked::SINGLETON_SEGMENTER_BREAK_SENTENCE_V1,
            locale_override: None,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (options: SentenceBreakOptions) -> error: DataError,
        /// Constructs a [`SentenceSegmenter`] for a given options and using compiled data.
        functions: [
            try_new,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        options: SentenceBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakSentenceV1>
            + DataProvider<SegmenterBreakSentenceOverrideV1>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.payload;
        let payload_locale_override = if let Some(locale) = options.content_locale {
            let locale = DataLocale::from(locale);
            let req = DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                metadata: {
                    let mut metadata = DataRequestMetadata::default();
                    metadata.silent = true;
                    metadata
                },
            };
            provider
                .load(req)
                .allow_identifier_not_found()?
                .map(|r| r.payload)
        } else {
            None
        };

        Ok(Self(SentenceSegmenterInner::Legacy {
            payload,
            payload_locale_override,
        }))
    }

    /// Constructs a [`SentenceSegmenterBorrowed`] with an invariant locale and compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "unstable")]
    pub const fn new_neo(
        _options: SentenceBreakInvariantOptions,
    ) -> SentenceSegmenterBorrowed<'static> {
        SentenceSegmenterBorrowed(SentenceSegmenterBorrowedInner::Neo {
            data: Baked::SINGLETON_SEGMENTER_BREAK_SENTENCE_V2,
            tailoring: None,
        })
    }

    #[cfg(feature = "unstable")]
    icu_provider::gen_buffer_data_constructors!(
        (options: SentenceBreakOptions) -> error: DataError,
        /// Constructs a [`SentenceSegmenter`] for a given options and using compiled data.
        functions: [
            try_new_neo,
            try_new_neo_with_buffer_provider,
            try_new_neo_unstable,
            Self
        ]
    );

    #[cfg(feature = "unstable")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_neo)]
    pub fn try_new_neo_unstable<D>(
        provider: &D,
        options: SentenceBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakSentenceV2>
            + DataProvider<SegmenterBreakSentenceOverrideV2>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.payload;
        let tailoring = if let Some(locale) = options.content_locale {
            provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&DataLocale::from(locale)),
                    metadata: {
                        let mut metadata = DataRequestMetadata::default();
                        metadata.silent = true;
                        metadata
                    },
                })
                .allow_identifier_not_found()?
                .map(|r| r.payload)
        } else {
            None
        };

        Ok(Self(SentenceSegmenterInner::Neo { payload, tailoring }))
    }

    /// Constructs a borrowed version of this type for more efficient querying.
    ///
    /// Most useful methods for segmentation are on this type.
    pub fn as_borrowed(&self) -> SentenceSegmenterBorrowed<'_> {
        match &self.0 {
            SentenceSegmenterInner::Legacy {
                payload,
                payload_locale_override,
            } => SentenceSegmenterBorrowed(SentenceSegmenterBorrowedInner::Legacy {
                data: payload.get(),
                locale_override: payload_locale_override.as_ref().map(|p| p.get()),
            }),
            #[cfg(feature = "unstable")]
            SentenceSegmenterInner::Neo { payload, tailoring } => {
                SentenceSegmenterBorrowed(SentenceSegmenterBorrowedInner::Neo {
                    data: payload.get(),
                    tailoring: tailoring.as_ref().map(|p| p.get()),
                })
            }
        }
    }
}

impl<'data> SentenceSegmenterBorrowed<'data> {
    fn segment<'s, Y: RuleBreakType>(
        self,
        iter: Y::IterAttr<'s>,
        len: usize,
    ) -> SentenceBreakIterator<'data, 's, Y> {
        match self.0 {
            SentenceSegmenterBorrowedInner::Legacy {
                data,
                locale_override,
            } => SentenceBreakIterator(SentenceBreakIteratorInner::Legacy(RuleBreakIterator {
                iter,
                len,
                current_pos_data: None,
                result_cache: Vec::new(),
                data,
                complex: None,
                boundary_property: 0,
                locale_override,
                handle_complex: empty_handle_complex,
            })),
            #[cfg(feature = "unstable")]
            SentenceSegmenterBorrowedInner::Neo { data, tailoring } => {
                SentenceBreakIterator(SentenceBreakIteratorInner::Neo(
                    crate::neo::RuleBreakIterator::new(iter, data, tailoring, None),
                ))
            }
        }
    }

    /// Creates a sentence break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'s>(self, input: &'s str) -> SentenceBreakIterator<'data, 's, Utf8> {
        self.segment(input.char_indices(), input.len())
    }
    /// Creates a sentence break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf8<'s>(
        self,
        input: &'s [u8],
    ) -> SentenceBreakIterator<'data, 's, PotentiallyIllFormedUtf8> {
        self.segment(Utf8CharIndices::new(input), input.len())
    }
    /// Creates a sentence break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'s>(self, input: &'s [u8]) -> SentenceBreakIterator<'data, 's, Latin1> {
        self.segment(Latin1Indices::new(input), input.len())
    }

    /// Creates a sentence break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'s>(self, input: &'s [u16]) -> SentenceBreakIterator<'data, 's, Utf16> {
        self.segment(Utf16Indices::new(input), input.len())
    }
}

impl SentenceSegmenterBorrowed<'static> {
    /// Cheaply converts a [`SentenceSegmenterBorrowed<'static>`] into a [`SentenceSegmenter`].
    ///
    /// Note: Due to branching and indirection, using [`SentenceSegmenter`] might inhibit some
    /// compile-time optimizations that are possible with [`SentenceSegmenterBorrowed`].
    pub const fn static_to_owned(self) -> SentenceSegmenter {
        SentenceSegmenter(match self.0 {
            SentenceSegmenterBorrowedInner::Legacy {
                data,
                locale_override,
            } => SentenceSegmenterInner::Legacy {
                payload: DataPayload::from_static_ref(data),
                payload_locale_override: if let Some(d) = locale_override {
                    Some(DataPayload::from_static_ref(d))
                } else {
                    None
                },
            },
            #[cfg(feature = "unstable")]
            SentenceSegmenterBorrowedInner::Neo { data, tailoring } => {
                SentenceSegmenterInner::Neo {
                    payload: DataPayload::from_static_ref(data),
                    tailoring: if let Some(d) = tailoring {
                        Some(DataPayload::from_static_ref(d))
                    } else {
                        None
                    },
                }
            }
        })
    }
}

#[test]
fn empty_string() {
    let segmenter = SentenceSegmenter::new(Default::default());
    let breaks: Vec<usize> = segmenter.segment_str("").collect();
    assert_eq!(breaks, [0]);
}

#[test]
fn empty_string_neo() {
    let segmenter = SentenceSegmenter::new_neo(Default::default());
    let breaks: Vec<usize> = segmenter.segment_str("").collect();
    assert_eq!(breaks, [0]);
}
