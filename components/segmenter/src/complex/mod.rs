// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
#[cfg(feature = "unstable")]
use crate::scaffold::PotentiallyIllFormedUtf8;
use crate::scaffold::{RuleBreakType, Utf8, Utf16};
use alloc::vec::Vec;
use icu_provider::prelude::*;

mod dictionary;
use dictionary::*;
mod script;
use script::*;
#[cfg(feature = "lstm")]
mod lstm;
#[cfg(feature = "lstm")]
use lstm::*;

#[derive(Debug)]
#[doc(hidden)]
pub struct ComplexIterator<
    'data,
    's,
    G: AbstractGraphemeClusterSegmenterBorrowed<'data>,
    R: RuleBreakType + 'static,
>(ComplexIteratorInner<'data, 's, G, R>, usize);

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum ComplexIteratorInner<
    'data,
    's,
    G: AbstractGraphemeClusterSegmenterBorrowed<'data>,
    R: RuleBreakType + 'static,
> {
    Dictionary(DictionaryBreakIterator<'data, 's, G, R>),
    #[cfg(feature = "lstm")]
    Lstm(LstmSegmenterIterator<'data, 's, R>),
}

impl<'data, G: AbstractGraphemeClusterSegmenterBorrowed<'data>, R: RuleBreakType> Iterator
    for ComplexIterator<'data, '_, G, R>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ComplexIteratorInner::Dictionary(ref mut iter) => iter.next(),
            #[cfg(feature = "lstm")]
            ComplexIteratorInner::Lstm(ref mut iter) => iter.next(),
        }
        .map(|n| n + self.1)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "lstm", expect(clippy::large_enum_variant))]
enum ComplexPayload {
    Dict(DataPayload<UCharDictionaryBreakDataV1>),
    #[cfg(feature = "lstm")]
    Lstm(DataPayload<SegmenterLstmAutoV1>),
}

#[derive(Debug, Clone, Copy)]
pub enum ComplexPayloadBorrowed<'data> {
    Dict(&'data UCharDictionaryBreakData<'data>),
    #[cfg(feature = "lstm")]
    Lstm(&'data LstmData<'data>),
}

impl<'data> ComplexPayloadBorrowed<'data> {
    pub(crate) fn segment_str<'s, G: AbstractGraphemeClusterSegmenterBorrowed<'data>>(
        self,
        input: &'s str,
        grapheme: G,
        offset: usize,
    ) -> ComplexIterator<'data, 's, G, Utf8> {
        ComplexIterator(
            match self {
                Self::Dict(dict) => ComplexIteratorInner::Dictionary(
                    DictionarySegmenter::new(dict, grapheme).segment_str(input),
                ),
                #[cfg(feature = "lstm")]
                Self::Lstm(lstm) => ComplexIteratorInner::Lstm(
                    LstmSegmenter::new(lstm, grapheme).segment_str(input),
                ),
            },
            offset,
        )
    }

    #[cfg(feature = "unstable")]
    pub(crate) fn segment_utf8<'s, G: AbstractGraphemeClusterSegmenterBorrowed<'data>>(
        self,
        input: &'s [u8],
        grapheme: G,
        offset: usize,
    ) -> ComplexIterator<'data, 's, G, PotentiallyIllFormedUtf8> {
        ComplexIterator(
            match self {
                Self::Dict(dict) => ComplexIteratorInner::Dictionary(
                    DictionarySegmenter::new(dict, grapheme).segment_utf8(input),
                ),
                #[cfg(feature = "lstm")]
                Self::Lstm(lstm) => ComplexIteratorInner::Lstm(
                    LstmSegmenter::new(lstm, grapheme).segment_utf8(input),
                ),
            },
            offset,
        )
    }

    pub(crate) fn segment_utf16<'s, G: AbstractGraphemeClusterSegmenterBorrowed<'data>>(
        self,
        input: &'s [u16],
        grapheme: G,
        offset: usize,
    ) -> ComplexIterator<'data, 's, G, Utf16> {
        ComplexIterator(
            match self {
                Self::Dict(dict) => ComplexIteratorInner::Dictionary(
                    DictionarySegmenter::new(dict, grapheme).segment_utf16(input),
                ),
                #[cfg(feature = "lstm")]
                Self::Lstm(lstm) => ComplexIteratorInner::Lstm(
                    LstmSegmenter::new(lstm, grapheme).segment_utf16(input),
                ),
            },
            offset,
        )
    }
}

impl ComplexPayload {
    fn as_borrowed(&self) -> ComplexPayloadBorrowed<'_> {
        match self {
            ComplexPayload::Dict(dict) => ComplexPayloadBorrowed::Dict(dict.get()),
            #[cfg(feature = "lstm")]
            ComplexPayload::Lstm(lstm) => ComplexPayloadBorrowed::Lstm(lstm.get()),
        }
    }
}

impl ComplexPayloadBorrowed<'static> {
    fn static_to_owned(self) -> ComplexPayload {
        match self {
            ComplexPayloadBorrowed::Dict(dict) => {
                ComplexPayload::Dict(DataPayload::from_static_ref(dict))
            }
            #[cfg(feature = "lstm")]
            ComplexPayloadBorrowed::Lstm(lstm) => {
                ComplexPayload::Lstm(DataPayload::from_static_ref(lstm))
            }
        }
    }
}

pub(crate) trait AbstractGraphemeClusterSegmenter {
    type Borrowed<'data>: AbstractGraphemeClusterSegmenterBorrowed<'data>
    where
        Self: 'data;

    fn as_borrowed<'data>(&'data self) -> Self::Borrowed<'data>;
}

#[doc(hidden)]
pub trait AbstractGraphemeClusterSegmenterBorrowed<'data>: core::fmt::Debug + Copy {
    type Iter<'d, 's, R: RuleBreakType + 'static>: Iterator<Item = usize> + core::fmt::Debug;

    fn segment_str<'s>(self, input: &'s str) -> Self::Iter<'data, 's, Utf8>;
    #[cfg(feature = "unstable")]
    fn segment_utf8<'s>(self, input: &'s [u8]) -> Self::Iter<'data, 's, PotentiallyIllFormedUtf8>;
    fn segment_utf16<'s>(self, input: &'s [u16]) -> Self::Iter<'data, 's, Utf16>;
}

impl AbstractGraphemeClusterSegmenter for crate::GraphemeClusterSegmenter {
    type Borrowed<'data> = crate::GraphemeClusterSegmenterBorrowed<'data>;

    fn as_borrowed<'data>(&'data self) -> Self::Borrowed<'data> {
        self.as_borrowed()
    }
}

impl<'data> AbstractGraphemeClusterSegmenterBorrowed<'data>
    for crate::grapheme::GraphemeClusterSegmenterBorrowed<'data>
{
    type Iter<'d, 's, R: RuleBreakType + 'static> =
        crate::grapheme::GraphemeClusterBreakIterator<'data, 's, R>;

    fn segment_str<'s>(self, input: &'s str) -> Self::Iter<'data, 's, Utf8> {
        self.segment_str(input)
    }

    #[cfg(feature = "unstable")]
    fn segment_utf8<'s>(self, input: &'s [u8]) -> Self::Iter<'data, 's, PotentiallyIllFormedUtf8> {
        self.segment_utf8(input)
    }

    fn segment_utf16<'s>(self, input: &'s [u16]) -> Self::Iter<'data, 's, Utf16> {
        self.segment_utf16(input)
    }
}

#[cfg(feature = "unstable")]
impl AbstractGraphemeClusterSegmenter for crate::neo::GraphemeClusterSegmenter {
    type Borrowed<'data> = crate::neo::GraphemeClusterSegmenterBorrowed<'data>;

    fn as_borrowed<'data>(&'data self) -> Self::Borrowed<'data> {
        self.as_borrowed()
    }
}

#[cfg(feature = "unstable")]
impl<'data> AbstractGraphemeClusterSegmenterBorrowed<'data>
    for crate::neo::GraphemeClusterSegmenterBorrowed<'data>
{
    type Iter<'d, 's, R: RuleBreakType + 'static> =
        crate::neo::GraphemeClusterBreakIterator<'d, 's, R>;

    fn segment_str<'s>(self, input: &'s str) -> Self::Iter<'data, 's, Utf8> {
        self.segment_str(input)
    }

    #[cfg(feature = "unstable")]
    fn segment_utf8<'s>(self, input: &'s [u8]) -> Self::Iter<'data, 's, PotentiallyIllFormedUtf8> {
        self.segment_utf8(input)
    }

    fn segment_utf16<'s>(self, input: &'s [u16]) -> Self::Iter<'data, 's, Utf16> {
        self.segment_utf16(input)
    }
}

#[derive(Debug)]
pub(crate) struct ComplexPayloads<G: AbstractGraphemeClusterSegmenter> {
    grapheme: G,
    my: Option<ComplexPayload>,
    km: Option<ComplexPayload>,
    lo: Option<ComplexPayload>,
    th: Option<ComplexPayload>,
    ja: Option<ComplexPayload>,
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct ComplexPayloadsBorrowed<'data, G: AbstractGraphemeClusterSegmenterBorrowed<'data>> {
    pub(crate) grapheme: G,
    my: Option<ComplexPayloadBorrowed<'data>>,
    km: Option<ComplexPayloadBorrowed<'data>>,
    lo: Option<ComplexPayloadBorrowed<'data>>,
    th: Option<ComplexPayloadBorrowed<'data>>,
    ja: Option<ComplexPayloadBorrowed<'data>>,
}

#[cfg(feature = "lstm")]
const MY_LSTM: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("Burmese_");
#[cfg(feature = "lstm")]
const KM_LSTM: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("Khmer_");
#[cfg(feature = "lstm")]
const LO_LSTM: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("Lao_");
#[cfg(feature = "lstm")]
const TH_LSTM: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("Thai_");

const MY_DICT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("burmesedict");
const KM_DICT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("khmerdict");
const LO_DICT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("laodict");
const TH_DICT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("thaidict");
const CJ_DICT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("cjdict");

impl<'data, G: AbstractGraphemeClusterSegmenterBorrowed<'data>> ComplexPayloadsBorrowed<'data, G> {
    pub(crate) fn select(
        &self,
        complex_script: ComplexScript,
    ) -> Option<ComplexPayloadBorrowed<'data>> {
        const ERR: DataError = DataError::custom("No segmentation model for complex script");
        match complex_script {
            ComplexScript::Myanmar => self.my.or_else(|| {
                ERR.with_display_context("Myanmar");
                None
            }),
            ComplexScript::Khmer => self.km.or_else(|| {
                ERR.with_display_context("Khmer");
                None
            }),
            ComplexScript::Lao => self.lo.or_else(|| {
                ERR.with_display_context("Lao");
                None
            }),
            ComplexScript::Thai => self.th.or_else(|| {
                ERR.with_display_context("Thai");
                None
            }),
            ComplexScript::ChineseOrJapanese => self.ja.or_else(|| {
                ERR.with_display_context("Chinese/Japanese");
                None
            }),
            ComplexScript::None => None,
        }
    }

    pub(crate) fn segment_str(&self, input: &str) -> Vec<usize> {
        let mut result = Vec::new();
        let mut offset = 0;
        for (slice, complex_script) in ComplexScriptIterator::new(input) {
            match self.select(complex_script) {
                Some(d) => result.extend(d.segment_str(slice, self.grapheme, offset)),
                None => result.push(offset + slice.len()),
            }
            offset += slice.len();
        }
        result
    }
    /// Return UTF-16 segment offset array using dictionary or lstm segmenter.
    pub(crate) fn segment_utf16(&self, input: &[u16]) -> Vec<usize> {
        let mut result = Vec::new();
        let mut offset = 0;
        for (slice, complex_script) in ComplexScriptIteratorUtf16::new(input) {
            match self.select(complex_script) {
                Some(d) => result.extend(d.segment_utf16(slice, self.grapheme, offset)),
                None => result.push(offset + slice.len()),
            }
            offset += slice.len();
        }
        result
    }
}

impl<G: AbstractGraphemeClusterSegmenterBorrowed<'static>> ComplexPayloadsBorrowed<'static, G> {
    #[cfg(feature = "lstm")]
    #[cfg(feature = "compiled_data")]
    pub(crate) fn with_southeast_asian_lstms(&mut self) {
        #![expect(clippy::unwrap_used)]
        // try_load is infallible if the provider only returns `MissingLocale`.
        if self.my.is_none() {
            self.my = try_load_static::<SegmenterLstmAutoV1, _>(&Baked, MY_LSTM)
                .unwrap()
                .map(ComplexPayloadBorrowed::Lstm);
        }
        if self.km.is_none() {
            self.km = try_load_static::<SegmenterLstmAutoV1, _>(&Baked, KM_LSTM)
                .unwrap()
                .map(ComplexPayloadBorrowed::Lstm);
        }
        if self.lo.is_none() {
            self.lo = try_load_static::<SegmenterLstmAutoV1, _>(&Baked, LO_LSTM)
                .unwrap()
                .map(ComplexPayloadBorrowed::Lstm);
        }
        if self.th.is_none() {
            self.th = try_load_static::<SegmenterLstmAutoV1, _>(&Baked, TH_LSTM)
                .unwrap()
                .map(ComplexPayloadBorrowed::Lstm);
        }
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn with_japanese_dictionary(&mut self) {
        #![expect(clippy::unwrap_used)]
        // try_load is infallible if the provider only returns `MissingLocale`.
        if self.ja.is_none() {
            self.ja = try_load_static::<SegmenterDictionaryAutoV1, _>(&Baked, CJ_DICT)
                .unwrap()
                .map(ComplexPayloadBorrowed::Dict);
        }
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn with_southeast_asian_dictionaries(&mut self) {
        #![expect(clippy::unwrap_used)]
        // try_load is infallible if the provider only returns `MissingLocale`.
        if self.my.is_none() {
            self.my = try_load_static::<SegmenterDictionaryExtendedV1, _>(&Baked, MY_DICT)
                .unwrap()
                .map(ComplexPayloadBorrowed::Dict);
        }
        if self.km.is_none() {
            self.km = try_load_static::<SegmenterDictionaryExtendedV1, _>(&Baked, KM_DICT)
                .unwrap()
                .map(ComplexPayloadBorrowed::Dict);
        }
        if self.lo.is_none() {
            self.lo = try_load_static::<SegmenterDictionaryExtendedV1, _>(&Baked, LO_DICT)
                .unwrap()
                .map(ComplexPayloadBorrowed::Dict);
        }
        if self.th.is_none() {
            self.th = try_load_static::<SegmenterDictionaryExtendedV1, _>(&Baked, TH_DICT)
                .unwrap()
                .map(ComplexPayloadBorrowed::Dict);
        }
    }
}

impl ComplexPayloadsBorrowed<'static, crate::grapheme::GraphemeClusterSegmenterBorrowed<'static>> {
    #[cfg(feature = "compiled_data")]
    pub(crate) const fn new() -> Self {
        Self {
            grapheme: crate::grapheme::GraphemeClusterSegmenter::new(),
            my: None,
            km: None,
            lo: None,
            th: None,
            ja: None,
        }
    }

    pub(crate) fn static_to_owned(
        self,
    ) -> ComplexPayloads<crate::grapheme::GraphemeClusterSegmenter> {
        ComplexPayloads {
            grapheme: self.grapheme.static_to_owned(),
            my: self.my.map(ComplexPayloadBorrowed::static_to_owned),
            km: self.km.map(ComplexPayloadBorrowed::static_to_owned),
            lo: self.lo.map(ComplexPayloadBorrowed::static_to_owned),
            th: self.th.map(ComplexPayloadBorrowed::static_to_owned),
            ja: self.ja.map(ComplexPayloadBorrowed::static_to_owned),
        }
    }
}

#[cfg(feature = "unstable")]
impl ComplexPayloadsBorrowed<'static, crate::neo::GraphemeClusterSegmenterBorrowed<'static>> {
    #[cfg(feature = "compiled_data")]
    pub(crate) const fn new() -> Self {
        Self {
            grapheme: crate::neo::GraphemeClusterSegmenter::new(),
            my: None,
            km: None,
            lo: None,
            th: None,
            ja: None,
        }
    }

    pub(crate) fn static_to_owned(self) -> ComplexPayloads<crate::neo::GraphemeClusterSegmenter> {
        ComplexPayloads {
            grapheme: self.grapheme.static_to_owned(),
            my: self.my.map(ComplexPayloadBorrowed::static_to_owned),
            km: self.km.map(ComplexPayloadBorrowed::static_to_owned),
            lo: self.lo.map(ComplexPayloadBorrowed::static_to_owned),
            th: self.th.map(ComplexPayloadBorrowed::static_to_owned),
            ja: self.ja.map(ComplexPayloadBorrowed::static_to_owned),
        }
    }
}

impl<G: AbstractGraphemeClusterSegmenter> ComplexPayloads<G> {
    pub(crate) fn as_borrowed(&self) -> ComplexPayloadsBorrowed<'_, G::Borrowed<'_>> {
        ComplexPayloadsBorrowed {
            grapheme: self.grapheme.as_borrowed(),
            my: self.my.as_ref().map(ComplexPayload::as_borrowed),
            km: self.km.as_ref().map(ComplexPayload::as_borrowed),
            lo: self.lo.as_ref().map(ComplexPayload::as_borrowed),
            th: self.th.as_ref().map(ComplexPayload::as_borrowed),
            ja: self.ja.as_ref().map(ComplexPayload::as_borrowed),
        }
    }

    #[cfg(feature = "lstm")]
    pub(crate) fn with_southeast_asian_lstms<D>(&mut self, provider: &D) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterLstmAutoV1> + ?Sized,
    {
        if self.my.is_none() {
            self.my = try_load::<SegmenterLstmAutoV1, D>(provider, MY_LSTM)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Lstm);
        }
        if self.km.is_none() {
            self.km = try_load::<SegmenterLstmAutoV1, D>(provider, KM_LSTM)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Lstm);
        }
        if self.lo.is_none() {
            self.lo = try_load::<SegmenterLstmAutoV1, D>(provider, LO_LSTM)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Lstm);
        }
        if self.th.is_none() {
            self.th = try_load::<SegmenterLstmAutoV1, D>(provider, TH_LSTM)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Lstm);
        }
        Ok(())
    }

    pub(crate) fn with_japanese_dictionary<D>(&mut self, provider: &D) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterDictionaryAutoV1> + ?Sized,
    {
        self.ja = try_load::<SegmenterDictionaryAutoV1, D>(provider, CJ_DICT)?
            .map(DataPayload::cast)
            .map(ComplexPayload::Dict);
        Ok(())
    }

    pub(crate) fn with_southeast_asian_dictionaries<D>(
        &mut self,
        provider: &D,
    ) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterDictionaryExtendedV1> + ?Sized,
    {
        if self.my.is_none() {
            self.my = try_load::<SegmenterDictionaryExtendedV1, _>(provider, MY_DICT)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Dict);
        }
        if self.km.is_none() {
            self.km = try_load::<SegmenterDictionaryExtendedV1, _>(provider, KM_DICT)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Dict);
        }
        if self.lo.is_none() {
            self.lo = try_load::<SegmenterDictionaryExtendedV1, _>(provider, LO_DICT)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Dict);
        }
        if self.th.is_none() {
            self.th = try_load::<SegmenterDictionaryExtendedV1, _>(provider, TH_DICT)?
                .map(DataPayload::cast)
                .map(ComplexPayload::Dict);
        }
        Ok(())
    }
}

impl ComplexPayloads<crate::grapheme::GraphemeClusterSegmenter> {
    pub(crate) fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakGraphemeClusterV1> + ?Sized,
    {
        Ok(Self {
            grapheme: crate::grapheme::GraphemeClusterSegmenter::try_new_unstable(provider)?,
            my: None,
            km: None,
            lo: None,
            th: None,
            ja: None,
        })
    }
}

#[cfg(feature = "unstable")]
impl ComplexPayloads<crate::neo::GraphemeClusterSegmenter> {
    pub(crate) fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakGraphemeClusterV2> + ?Sized,
    {
        Ok(Self {
            grapheme: crate::neo::GraphemeClusterSegmenter::try_new_unstable(provider)?,
            my: None,
            km: None,
            lo: None,
            th: None,
            ja: None,
        })
    }
}

fn try_load<M: DataMarker, P: DataProvider<M> + ?Sized>(
    provider: &P,
    model: &'static DataMarkerAttributes,
) -> Result<Option<DataPayload<M>>, DataError> {
    provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes(model),
            metadata: {
                let mut m = DataRequestMetadata::default();
                m.silent = true;
                m.attributes_prefix_match = true;
                m
            },
        })
        .allow_identifier_not_found()
        .map(|r| r.map(|r| r.payload))
}

#[cfg(feature = "compiled_data")]
fn try_load_static<M: DataMarker, P: DataProvider<M> + ?Sized>(
    provider: &P,
    model: &'static DataMarkerAttributes,
) -> Result<Option<&'static <M::DataStruct as yoke::Yokeable<'static>>::Output>, DataError> {
    provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes(model),
            metadata: {
                let mut m = DataRequestMetadata::default();
                m.silent = true;
                m.attributes_prefix_match = true;
                m
            },
        })
        .allow_identifier_not_found()
        .map(|r| r.and_then(|r| r.payload.get_static()))
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;

    #[track_caller]
    fn check_complex<'data, G: AbstractGraphemeClusterSegmenterBorrowed<'data>>(
        s: &str,
        expected: &[&str],
        segmenter: ComplexPayloadsBorrowed<'data, G>,
    ) {
        use itertools::Itertools;

        let segments = [0]
            .into_iter()
            .chain(segmenter.segment_str(s))
            .tuple_windows()
            .map(|(a, b)| &s[a..b])
            .collect::<Vec<_>>();
        assert_eq!(segments, expected, "{s}");

        // let segments = segmenter
        //     .segment_utf8(s.as_bytes())
        //     .tuple_windows()
        //     .map(|(a, b)| &s[a..b])
        //     .collect::<Vec<_>>();
        // assert_eq!(segments, expected, "{s}");

        let utf16: Vec<u16> = s.encode_utf16().collect();
        let expected = expected
            .iter()
            .copied()
            .map(|s| s.encode_utf16().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let iter = [0]
            .into_iter()
            .chain(segmenter.segment_utf16(&utf16))
            .tuple_windows()
            .map(|(a, b)| &utf16[a..b])
            .collect::<Vec<_>>();
        assert_eq!(iter, expected, "{s}");
    }

    #[test]
    fn thai() {
        let mut lstm = ComplexPayloadsBorrowed::<crate::GraphemeClusterSegmenterBorrowed>::new();
        lstm.with_southeast_asian_lstms();
        let mut dict = ComplexPayloadsBorrowed::<crate::GraphemeClusterSegmenterBorrowed>::new();
        dict.with_southeast_asian_dictionaries();

        check_complex("ภาษาไทยภาษาไทย", &["ภาษา", "ไทย", "ภาษา", "ไทย"], lstm);
        check_complex("ภาษาไทยภาษาไทย", &["ภาษา", "ไทย", "ภาษา", "ไทย"], dict);
    }

    #[test]
    fn thai_neo() {
        let mut lstm =
            ComplexPayloadsBorrowed::<crate::neo::GraphemeClusterSegmenterBorrowed>::new();
        lstm.with_southeast_asian_lstms();
        let mut dict =
            ComplexPayloadsBorrowed::<crate::neo::GraphemeClusterSegmenterBorrowed>::new();
        dict.with_southeast_asian_dictionaries();

        check_complex("ภาษาไทยภาษาไทย", &["ภาษา", "ไทย", "ภาษา", "ไทย"], lstm);
        check_complex("ภาษาไทยภาษาไทย", &["ภาษา", "ไทย", "ภาษา", "ไทย"], dict);
    }

    #[test]
    fn mixed() {
        let mut lstm = ComplexPayloadsBorrowed::<crate::GraphemeClusterSegmenterBorrowed>::new();
        lstm.with_southeast_asian_lstms();
        lstm.with_japanese_dictionary();

        let mut dict = ComplexPayloadsBorrowed::<crate::GraphemeClusterSegmenterBorrowed>::new();
        dict.with_southeast_asian_dictionaries();
        dict.with_japanese_dictionary();

        check_complex("ภาษาไทย龟山岛", &["ภาษา", "ไทย", "龟山岛"], lstm);
        check_complex("ภาษาไทย龟山岛", &["ภาษา", "ไทย", "龟山岛"], dict);

        check_complex(
            "こんにちは世界ภาษาไทย",
            &["こんにちは", "世界", "ภาษา", "ไทย"],
            lstm,
        );
        check_complex(
            "こんにちは世界ภาษาไทย",
            &["こんにちは", "世界", "ภาษา", "ไทย"],
            dict,
        );
    }

    #[test]
    fn mixed_neo() {
        let mut lstm =
            ComplexPayloadsBorrowed::<crate::neo::GraphemeClusterSegmenterBorrowed>::new();
        lstm.with_southeast_asian_lstms();
        lstm.with_japanese_dictionary();

        let mut dict =
            ComplexPayloadsBorrowed::<crate::neo::GraphemeClusterSegmenterBorrowed>::new();
        dict.with_southeast_asian_dictionaries();
        dict.with_japanese_dictionary();

        check_complex("ภาษาไทย龟山岛", &["ภาษา", "ไทย", "龟山岛"], lstm);
        check_complex("ภาษาไทย龟山岛", &["ภาษา", "ไทย", "龟山岛"], dict);

        check_complex(
            "こんにちは世界ภาษาไทย",
            &["こんにちは", "世界", "ภาษา", "ไทย"],
            lstm,
        );
        check_complex(
            "こんにちは世界ภาษาไทย",
            &["こんにちは", "世界", "ภาษา", "ไทย"],
            dict,
        );
    }
}
