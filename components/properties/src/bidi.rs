// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module exposes tooling for running the [unicode bidi algorithm](https://unicode.org/reports/tr9/) using ICU4X data.
//!
//! `BidiClassAdapter` enables ICU4X to provide data to [`unicode-bidi`].
//!
//! # Examples
//!
//!```
//! use icu_properties::bidi::BidiClassAdapter;
//! use icu_properties::{maps, BidiClass};
//! use unicode_bidi::BidiClass as DataSourceBidiClass;
//! use unicode_bidi::BidiDataSource;
//! use unicode_bidi::BidiInfo;
//! // This example text is defined using `concat!` because some browsers
//! // and text editors have trouble displaying bidi strings.
//! let text = concat![
//!   "א",
//!   "ב",
//!   "ג",
//!   "a",
//!   "b",
//!   "c",
//! ];
//!
//! // Create an adapter to provide the data to `BidiInfo`.
//! let provider = icu_testdata::get_provider();
//!
//!  let adapter = BidiClassAdapter::try_new(&provider).expect("Loading data failed");
//! // Resolve embedding levels within the text.  Pass `None` to detect the
//! // paragraph level automatically.
//!
//! let bidi_info = BidiInfo::new_with_data_source(&adapter, &text, None);
//!
//! // This paragraph has embedding level 1 because its first strong character is RTL.
//! assert_eq!(bidi_info.paragraphs.len(), 1);
//! let para = &bidi_info.paragraphs[0];
//! assert_eq!(para.level.number(), 1);
//! assert_eq!(para.level.is_rtl(), true);
//!
//! // Re-ordering is done after wrapping each paragraph into a sequence of
//! // lines. For this example, I'll just use a single line that spans the
//! // entire paragraph.
//! let line = para.range.clone();
//!
//! let display = bidi_info.reorder_line(para, line);
//! assert_eq!(display, concat![
//!   "a",
//!   "b",
//!   "c",
//!   "ג",
//!   "ב",
//!   "א",
//! ]);
//! ```

use crate::error::PropertiesError;
use crate::maps;
use crate::props::BidiClass;
use crate::provider::*;
use icu_provider::prelude::*;
use unicode_bidi::data_source::BidiDataSource;
use unicode_bidi::BidiClass as DataSourceBidiClass;

/// An adapter to convert from icu4x `BidiClass` to `unicode_bidi::BidiClass`.
///
/// # Example
///
/// ```
/// use icu_properties::bidi::BidiClassAdapter;
/// use icu_properties::{maps, BidiClass};
/// use unicode_bidi::BidiClass as DataSourceBidiClass;
/// use unicode_bidi::BidiDataSource;
///
/// let provider = icu_testdata::get_provider();
///
/// let adapter = BidiClassAdapter::try_new(&provider).expect("Loading data failed");
/// assert_eq!(adapter.bidi_class('a'), DataSourceBidiClass::L);
/// assert_eq!(adapter.bidi_class('ع'), DataSourceBidiClass::AL);
/// ```
pub struct BidiClassAdapter {
    bidi_trie: DataPayload<UnicodePropertyMapV1Marker<BidiClass>>,
}

impl BidiClassAdapter {
    /// Creates new instance of `BidiClassAdapter`.
    pub fn try_new<D>(provider: &D) -> Result<Self, PropertiesError>
    where
        D: DynProvider<UnicodePropertyMapV1Marker<BidiClass>> + ?Sized,
    {
        Ok(Self {
            bidi_trie: maps::get_bidi_class(provider)?,
        })
    }
}

impl BidiDataSource for BidiClassAdapter {
    /// Returns a [`DataSourceBidiClass`] given a unicode character.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::bidi::BidiClassAdapter;
    /// use icu_properties::{maps, BidiClass};
    /// use unicode_bidi::BidiClass as DataSourceBidiClass;
    /// use unicode_bidi::BidiDataSource;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let adapter = BidiClassAdapter::try_new(&provider).expect("Loading data failed");
    /// assert_eq!(adapter.bidi_class('a'), DataSourceBidiClass::L);
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    fn bidi_class(&self, c: char) -> DataSourceBidiClass {
        let bidi_class = self.bidi_trie.get().code_point_trie.get(c as u32);
        match bidi_class {
            BidiClass::LeftToRight => DataSourceBidiClass::L,
            BidiClass::RightToLeft => DataSourceBidiClass::R,
            BidiClass::EuropeanNumber => DataSourceBidiClass::EN,
            BidiClass::EuropeanSeparator => DataSourceBidiClass::ES,
            BidiClass::EuropeanTerminator => DataSourceBidiClass::ET,
            BidiClass::ArabicNumber => DataSourceBidiClass::AN,
            BidiClass::CommonSeparator => DataSourceBidiClass::CS,
            BidiClass::ParagraphSeparator => DataSourceBidiClass::B,
            BidiClass::SegmentSeparator => DataSourceBidiClass::S,
            BidiClass::WhiteSpace => DataSourceBidiClass::WS,
            BidiClass::OtherNeutral => DataSourceBidiClass::ON,
            BidiClass::LeftToRightEmbedding => DataSourceBidiClass::LRE,
            BidiClass::LeftToRightOverride => DataSourceBidiClass::LRO,
            BidiClass::ArabicLetter => DataSourceBidiClass::AL,
            BidiClass::RightToLeftEmbedding => DataSourceBidiClass::RLE,
            BidiClass::RightToLeftOverride => DataSourceBidiClass::RLO,
            BidiClass::PopDirectionalFormat => DataSourceBidiClass::PDF,
            BidiClass::NonspacingMark => DataSourceBidiClass::NSM,
            BidiClass::BoundaryNeutral => DataSourceBidiClass::BN,
            BidiClass::FirstStrongIsolate => DataSourceBidiClass::FSI,
            BidiClass::LeftToRightIsolate => DataSourceBidiClass::LRI,
            BidiClass::RightToLeftIsolate => DataSourceBidiClass::RLI,
            BidiClass::PopDirectionalIsolate => DataSourceBidiClass::PDI,
            _ =>
            // This must not happen.
            {
                DataSourceBidiClass::ON
            }
        }
    }
}
