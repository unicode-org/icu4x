// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use alloc::vec::Vec;
use either::Either;
use icu_provider::prelude::*;

mod dictionary;
use dictionary::*;
mod language;
use language::*;
#[cfg(feature = "lstm")]
mod lstm;
#[cfg(feature = "lstm")]
use lstm::*;

#[cfg(not(feature = "lstm"))]
type DictOrLstm = Either<DataPayload<UCharDictionaryBreakDataV1>, core::convert::Infallible>;
#[cfg(not(feature = "lstm"))]
type DictOrLstmBorrowed<'a> = Either<&'a UCharDictionaryBreakData<'a>, core::convert::Infallible>;

#[cfg(feature = "lstm")]
type DictOrLstm = Either<DataPayload<UCharDictionaryBreakDataV1>, DataPayload<SegmenterLstmAutoV1>>;
#[cfg(feature = "lstm")]
type DictOrLstmBorrowed<'a> = Either<&'a UCharDictionaryBreakData<'a>, &'a LstmData<'a>>;

fn borrow_dictor(dict_or: &DictOrLstm) -> DictOrLstmBorrowed<'_> {
    match dict_or {
        Either::Left(dict) => Either::Left(dict.get()),
        #[cfg(feature = "lstm")]
        Either::Right(lstm) => Either::Right(lstm.get()),
        #[cfg(not(feature = "lstm"))]
        Either::Right(infallible) => Either::Right(*infallible),
    }
}

fn fromstatic_dictor(dict_or: DictOrLstmBorrowed<'static>) -> DictOrLstm {
    match dict_or {
        Either::Left(dict) => Either::Left(DataPayload::from_static_ref(dict)),
        #[cfg(feature = "lstm")]
        Either::Right(lstm) => Either::Right(DataPayload::from_static_ref(lstm)),
        #[cfg(not(feature = "lstm"))]
        Either::Right(infallible) => Either::Right(infallible),
    }
}

#[derive(Debug)]
pub(crate) struct ComplexPayloads {
    grapheme: DataPayload<SegmenterBreakGraphemeClusterV1>,
    my: Option<DictOrLstm>,
    km: Option<DictOrLstm>,
    lo: Option<DictOrLstm>,
    th: Option<DictOrLstm>,
    ja: Option<DataPayload<UCharDictionaryBreakDataV1>>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ComplexPayloadsBorrowed<'data> {
    grapheme: &'data RuleBreakData<'data>,
    my: Option<DictOrLstmBorrowed<'data>>,
    km: Option<DictOrLstmBorrowed<'data>>,
    lo: Option<DictOrLstmBorrowed<'data>>,
    th: Option<DictOrLstmBorrowed<'data>>,
    ja: Option<&'data UCharDictionaryBreakData<'data>>,
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

impl<'data> ComplexPayloadsBorrowed<'data> {
    fn select(&self, language: Language) -> Option<DictOrLstmBorrowed<'data>> {
        const ERR: DataError = DataError::custom("No segmentation model for language");
        match language {
            Language::Burmese => self.my.or_else(|| {
                ERR.with_display_context("my");
                None
            }),
            Language::Khmer => self.km.or_else(|| {
                ERR.with_display_context("km");
                None
            }),
            Language::Lao => self.lo.or_else(|| {
                ERR.with_display_context("lo");
                None
            }),
            Language::Thai => self.th.or_else(|| {
                ERR.with_display_context("th");
                None
            }),
            Language::ChineseOrJapanese => self.ja.map(Either::Left).or_else(|| {
                ERR.with_display_context("ja");
                None
            }),
            Language::Unknown => None,
        }
    }
    pub(crate) fn complex_language_segment_str(&self, input: &str) -> Vec<usize> {
        let mut result = Vec::new();
        let mut offset = 0;
        for (slice, lang) in LanguageIterator::new(input) {
            match self.select(lang) {
                Some(Either::Left(dict)) => {
                    let seg = DictionarySegmenter::new(dict, self.grapheme);
                    result.extend(seg.segment_str(slice).map(|n| offset + n));
                }
                #[cfg(feature = "lstm")]
                Some(Either::Right(lstm)) => {
                    let seg = LstmSegmenter::new(lstm, self.grapheme);
                    result.extend(seg.segment_str(slice).map(|n| offset + n));
                }
                #[cfg(not(feature = "lstm"))]
                Some(Either::Right(_infallible)) => {} // should be refutable
                None => {
                    result.push(offset + slice.len());
                }
            }
            offset += slice.len();
        }
        result
    }
    /// Return UTF-16 segment offset array using dictionary or lstm segmenter.
    pub(crate) fn complex_language_segment_utf16(&self, input: &[u16]) -> Vec<usize> {
        let mut result = Vec::new();
        let mut offset = 0;
        for (slice, lang) in LanguageIteratorUtf16::new(input) {
            match self.select(lang) {
                Some(Either::Left(dict)) => {
                    let seg = DictionarySegmenter::new(dict, self.grapheme);
                    result.extend(seg.segment_utf16(slice).map(|n| offset + n));
                }
                #[cfg(feature = "lstm")]
                Some(Either::Right(lstm)) => {
                    let seg = LstmSegmenter::new(lstm, self.grapheme);
                    result.extend(seg.segment_utf16(slice).map(|n| offset + n));
                }
                #[cfg(not(feature = "lstm"))]
                Some(Either::Right(_infallible)) => {} // should be refutable
                None => {
                    result.push(offset + slice.len());
                }
            }
            offset += slice.len();
        }
        result
    }
}
impl ComplexPayloadsBorrowed<'static> {
    #[cfg(feature = "lstm")]
    #[cfg(feature = "compiled_data")]
    pub(crate) fn new_lstm() -> Self {
        #[allow(clippy::unwrap_used)]
        // try_load is infallible if the provider only returns `MissingLocale`.
        Self {
            grapheme: crate::provider::Baked::SINGLETON_SEGMENTER_BREAK_GRAPHEME_CLUSTER_V1,
            my: try_load_static::<SegmenterLstmAutoV1, _>(&crate::provider::Baked, MY_LSTM)
                .unwrap()
                .map(Either::Right),
            km: try_load_static::<SegmenterLstmAutoV1, _>(&crate::provider::Baked, KM_LSTM)
                .unwrap()
                .map(Either::Right),
            lo: try_load_static::<SegmenterLstmAutoV1, _>(&crate::provider::Baked, LO_LSTM)
                .unwrap()
                .map(Either::Right),
            th: try_load_static::<SegmenterLstmAutoV1, _>(&crate::provider::Baked, TH_LSTM)
                .unwrap()
                .map(Either::Right),
            ja: None,
        }
    }
    #[cfg(feature = "auto")]
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::unwrap_used)]
    pub(crate) fn new_auto() -> Self {
        let mut this = Self::new_lstm();
        this.ja = try_load_static::<SegmenterDictionaryAutoV1, _>(&crate::provider::Baked, CJ_DICT)
            .unwrap();
        this
    }
    #[cfg(feature = "compiled_data")]
    pub(crate) fn new_dict() -> Self {
        #[allow(clippy::unwrap_used)]
        // try_load is infallible if the provider only returns `MissingLocale`.
        Self {
            grapheme: crate::provider::Baked::SINGLETON_SEGMENTER_BREAK_GRAPHEME_CLUSTER_V1,
            my: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                MY_DICT,
            )
            .unwrap()
            .map(Either::Left),
            km: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                KM_DICT,
            )
            .unwrap()
            .map(Either::Left),
            lo: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                LO_DICT,
            )
            .unwrap()
            .map(Either::Left),
            th: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                TH_DICT,
            )
            .unwrap()
            .map(Either::Left),
            ja: try_load_static::<SegmenterDictionaryAutoV1, _>(&crate::provider::Baked, CJ_DICT)
                .unwrap(),
        }
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn new_southeast_asian() -> Self {
        #[allow(clippy::unwrap_used)]
        // try_load is infallible if the provider only returns `MissingLocale`.
        Self {
            grapheme: crate::provider::Baked::SINGLETON_SEGMENTER_BREAK_GRAPHEME_CLUSTER_V1,
            my: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                MY_DICT,
            )
            .unwrap()
            .map(Either::Left),
            km: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                KM_DICT,
            )
            .unwrap()
            .map(Either::Left),
            lo: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                LO_DICT,
            )
            .unwrap()
            .map(Either::Left),
            th: try_load_static::<SegmenterDictionaryExtendedV1, _>(
                &crate::provider::Baked,
                TH_DICT,
            )
            .unwrap()
            .map(Either::Left),
            ja: None,
        }
    }

    pub(crate) fn static_to_owned(self) -> ComplexPayloads {
        ComplexPayloads {
            grapheme: DataPayload::from_static_ref(self.grapheme),
            my: self.my.map(fromstatic_dictor),
            km: self.km.map(fromstatic_dictor),
            lo: self.lo.map(fromstatic_dictor),
            th: self.th.map(fromstatic_dictor),
            ja: self.ja.map(DataPayload::from_static_ref),
        }
    }
}

impl ComplexPayloads {
    pub(crate) fn as_borrowed(&self) -> ComplexPayloadsBorrowed<'_> {
        ComplexPayloadsBorrowed {
            grapheme: self.grapheme.get(),
            my: self.my.as_ref().map(borrow_dictor),
            km: self.km.as_ref().map(borrow_dictor),
            lo: self.lo.as_ref().map(borrow_dictor),
            th: self.th.as_ref().map(borrow_dictor),
            ja: self.ja.as_ref().map(|p| p.get()),
        }
    }

    #[cfg(feature = "lstm")]
    pub(crate) fn try_new_lstm<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakGraphemeClusterV1>
            + DataProvider<SegmenterLstmAutoV1>
            + ?Sized,
    {
        Ok(Self {
            grapheme: provider.load(Default::default())?.payload,
            my: try_load::<SegmenterLstmAutoV1, D>(provider, MY_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            km: try_load::<SegmenterLstmAutoV1, D>(provider, KM_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            lo: try_load::<SegmenterLstmAutoV1, D>(provider, LO_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            th: try_load::<SegmenterLstmAutoV1, D>(provider, TH_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            ja: None,
        })
    }

    pub(crate) fn try_new_dict<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakGraphemeClusterV1>
            + DataProvider<SegmenterDictionaryExtendedV1>
            + DataProvider<SegmenterDictionaryAutoV1>
            + ?Sized,
    {
        Ok(Self {
            grapheme: provider.load(Default::default())?.payload,
            my: try_load::<SegmenterDictionaryExtendedV1, D>(provider, MY_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
            km: try_load::<SegmenterDictionaryExtendedV1, D>(provider, KM_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
            lo: try_load::<SegmenterDictionaryExtendedV1, D>(provider, LO_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
            th: try_load::<SegmenterDictionaryExtendedV1, D>(provider, TH_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
            ja: try_load::<SegmenterDictionaryAutoV1, D>(provider, CJ_DICT)?.map(DataPayload::cast),
        })
    }

    #[cfg(feature = "auto")] // Use by WordSegmenter with "auto" enabled.
    pub(crate) fn try_new_auto<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakGraphemeClusterV1>
            + DataProvider<SegmenterLstmAutoV1>
            + DataProvider<SegmenterDictionaryAutoV1>
            + ?Sized,
    {
        Ok(Self {
            grapheme: provider.load(Default::default())?.payload,
            my: try_load::<SegmenterLstmAutoV1, D>(provider, MY_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            km: try_load::<SegmenterLstmAutoV1, D>(provider, KM_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            lo: try_load::<SegmenterLstmAutoV1, D>(provider, LO_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            th: try_load::<SegmenterLstmAutoV1, D>(provider, TH_LSTM)?
                .map(DataPayload::cast)
                .map(Either::Right),
            ja: try_load::<SegmenterDictionaryAutoV1, D>(provider, CJ_DICT)?.map(DataPayload::cast),
        })
    }

    pub(crate) fn try_new_southeast_asian<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterDictionaryExtendedV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        Ok(Self {
            grapheme: provider.load(Default::default())?.payload,
            my: try_load::<SegmenterDictionaryExtendedV1, _>(provider, MY_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
            km: try_load::<SegmenterDictionaryExtendedV1, _>(provider, KM_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
            lo: try_load::<SegmenterDictionaryExtendedV1, _>(provider, LO_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
            th: try_load::<SegmenterDictionaryExtendedV1, _>(provider, TH_DICT)?
                .map(DataPayload::cast)
                .map(Either::Left),
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

    #[test]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";
        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();

        let lstm = ComplexPayloadsBorrowed::new_lstm();
        let dict = ComplexPayloadsBorrowed::new_dict();

        assert_eq!(
            lstm.complex_language_segment_str(TEST_STR),
            [12, 21, 33, 42]
        );
        assert_eq!(lstm.complex_language_segment_utf16(&utf16), [4, 7, 11, 14]);

        assert_eq!(
            dict.complex_language_segment_str(TEST_STR),
            [12, 21, 33, 42]
        );
        assert_eq!(dict.complex_language_segment_utf16(&utf16), [4, 7, 11, 14]);
    }
}
