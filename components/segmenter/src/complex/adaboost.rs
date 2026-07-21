// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::indices::Utf16Indices;
use crate::provider::{radical::UnihanRadicalsData, AdaboostData};
#[cfg(feature = "unstable")]
use crate::scaffold::PotentiallyIllFormedUtf8;
use crate::scaffold::{RuleBreakType, Utf8, Utf16};
use core::char::REPLACEMENT_CHARACTER;
use core::iter::Peekable;
#[cfg(feature = "unstable")]
use utf8_iter::Utf8CharIndices;

/// A word break iterator using an AdaBoost model.
#[derive(Debug)]
pub(crate) struct AdaboostSegmenterIterator<'data, 's, R: RuleBreakType> {
    segmenter: AdaboostSegmenter<'data>,
    chars: Peekable<R::IterAttr<'s>>,
    len: usize,
    previous: Option<R::CharType>,
    previous_previous: Option<char>,
    current_chunk_len: u32,
}

impl<R: RuleBreakType> Iterator for AdaboostSegmenterIterator<'_, '_, R> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let previous_raw = self.previous?;
            let Some((current_idx, current_raw)) = self.chars.next() else {
                self.previous = None;
                return Some(self.len);
            };

            let previous = scalar_value(previous_raw);
            let current = scalar_value(current_raw);
            let next = self
                .chars
                .peek()
                .map(|(_, ch)| scalar_value(*ch));
            let should_break = self.segmenter.score_x2(
                self.previous_previous,
                previous,
                current,
                next,
                self.current_chunk_len,
            ) > 0;

            self.previous_previous = Some(previous);
            self.previous = Some(current_raw);

            if should_break {
                self.current_chunk_len = 1;
                return Some(current_idx);
            }

            self.current_chunk_len += 1;
        }
    }
}

/// Evaluates an AdaBoost word segmentation model.
#[derive(Debug, Clone, Copy)]
pub(crate) struct AdaboostSegmenter<'data> {
    model: &'data AdaboostData<'data>,
    radicals: &'data UnihanRadicalsData<'data>,
}

impl<'data> AdaboostSegmenter<'data> {
    pub(crate) fn new(
        model: &'data AdaboostData<'data>,
        radicals: &'data UnihanRadicalsData<'data>,
    ) -> Self {
        Self { model, radicals }
    }

    fn score_x2(
        self,
        previous_previous: Option<char>,
        previous: char,
        current: char,
        next: Option<char>,
        current_chunk_len: u32,
    ) -> i64 {
        let mut score = i64::from(self.model.bias_x2) + 2 * 32_i64.pow(current_chunk_len);

        let current_radical = self.radicals.trie.get(current);
        if current_radical != 0 {
            add_weight_x2(
                &mut score,
                self.model
                    .rsrid
                    .get_copied(&(previous, current_radical)),
            );
        }

        let previous_radical = self.radicals.trie.get(previous);
        if previous_radical != 0 {
            add_weight_x2(
                &mut score,
                self.model
                    .lsrid
                    .get_copied(&(previous_radical, current)),
            );
        }

        if previous_radical != 0 && current_radical != 0 {
            add_weight_x2(
                &mut score,
                self.model
                    .rad
                    .get_copied(&(previous_radical, current_radical)),
            );
        }

        add_weight_x2(
            &mut score,
            self.model.bw2.get_copied(&(previous, current)),
        );

        if let Some(previous_previous) = previous_previous {
            add_weight_x2(
                &mut score,
                self.model.uw2.get_copied(&previous_previous),
            );
        }

        add_weight_x2(&mut score, self.model.uw3.get_copied(&previous));
        add_weight_x2(&mut score, self.model.uw4.get_copied(&current));

        if let Some(next) = next {
            add_weight_x2(&mut score, self.model.uw5.get_copied(&next));
        }

        score
    }

    fn segment<'s, R: RuleBreakType>(
        self,
        chars: R::IterAttr<'s>,
        len: usize,
    ) -> AdaboostSegmenterIterator<'data, 's, R> {
        let mut chars = chars.peekable();
        let previous = chars.next().map(|(_, ch)| ch);
        AdaboostSegmenterIterator {
            segmenter: self,
            chars,
            len,
            previous,
            previous_previous: None,
            current_chunk_len: if previous.is_some() { 1 } else { 0 },
        }
    }

    pub(crate) fn segment_str<'s>(
        self,
        input: &'s str,
    ) -> AdaboostSegmenterIterator<'data, 's, Utf8> {
        self.segment(input.char_indices(), input.len())
    }

    #[cfg(feature = "unstable")]
    pub(crate) fn segment_utf8<'s>(
        self,
        input: &'s [u8],
    ) -> AdaboostSegmenterIterator<'data, 's, PotentiallyIllFormedUtf8> {
        self.segment(Utf8CharIndices::new(input), input.len())
    }

    pub(crate) fn segment_utf16<'s>(
        self,
        input: &'s [u16],
    ) -> AdaboostSegmenterIterator<'data, 's, Utf16> {
        self.segment(Utf16Indices::new(input), input.len())
    }
}

#[inline]
fn scalar_value<T: Into<u32>>(value: T) -> char {
    char::from_u32(value.into()).unwrap_or(REPLACEMENT_CHARACTER)
}

#[inline]
fn add_weight_x2(score: &mut i64, weight: Option<i16>) {
    if let Some(weight) = weight {
        *score += i64::from(weight) * 2;
    }
}

#[cfg(all(test, feature = "compiled_data"))]
mod tests {
    use super::*;
    use crate::provider::Baked;
    use zerovec::ZeroMap;

    fn empty_model(bias_x2: i32) -> AdaboostData<'static> {
        AdaboostData {
            bias_x2,
            uw2: ZeroMap::new(),
            uw3: ZeroMap::new(),
            uw4: ZeroMap::new(),
            uw5: ZeroMap::new(),
            bw2: ZeroMap::new(),
            rad: ZeroMap::new(),
            lsrid: ZeroMap::new(),
            rsrid: ZeroMap::new(),
        }
    }

    fn make_segmenter(model: &AdaboostData<'static>) -> AdaboostSegmenter<'_> {
        AdaboostSegmenter::new(model, Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1)
    }

    #[test]
    fn exact_score_x2() {
        let radicals = Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1;
        let left_radical = radicals.trie.get('中');
        let right_radical = radicals.trie.get('國');
        assert_ne!(left_radical, 0);
        assert_ne!(right_radical, 0);

        let model = AdaboostData {
            bias_x2: -36,
            uw2: [('甲', 1)].into_iter().collect(),
            uw3: [('中', 2)].into_iter().collect(),
            uw4: [('國', 3)].into_iter().collect(),
            uw5: [('乙', 4)].into_iter().collect(),
            bw2: [(('中', '國'), 5)].into_iter().collect(),
            rad: [((left_radical, right_radical), 6)]
                .into_iter()
                .collect(),
            lsrid: [((left_radical, '國'), 7)].into_iter().collect(),
            rsrid: [(('中', right_radical), 8)].into_iter().collect(),
        };

        assert_eq!(
            AdaboostSegmenter::new(&model, radicals).score_x2(
                Some('甲'),
                '中',
                '國',
                Some('乙'),
                2,
            ),
            -36 + 2 * 32_i64.pow(2) + 2 * (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8)
        );
    }

    #[test]
    fn streaming_boundaries_and_terminal() {
        let model = empty_model(-100);
        let segmenter = make_segmenter(&model);
        assert_eq!(segmenter.segment_str("").collect::<Vec<_>>(), []);
        assert_eq!(segmenter.segment_str("甲").collect::<Vec<_>>(), [3]);

        let input = "甲乙丙丁戊";
        assert_eq!(
            segmenter.segment_str(input).collect::<Vec<_>>(),
            [6, 12, 15]
        );
        assert_eq!(
            segmenter.segment_utf8(input.as_bytes()).collect::<Vec<_>>(),
            [6, 12, 15]
        );
        assert_eq!(
            segmenter
                .segment_utf16(&input.encode_utf16().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [2, 4, 5]
        );

        let zero_score = empty_model(-64);
        assert_eq!(
            make_segmenter(&zero_score)
                .segment_str("AB")
                .collect::<Vec<_>>(),
            [2]
        );
    }

    #[test]
    fn unicode_and_missing_radicals() {
        let model = empty_model(-36);
        let segmenter = make_segmenter(&model);

        assert_eq!(segmenter.segment_str("中国").collect::<Vec<_>>(), [3, 6]);
        assert_eq!(segmenter.segment_str("中國").collect::<Vec<_>>(), [3, 6]);

        let supplementary = "𠀀甲";
        assert_eq!(
            segmenter.segment_utf8(supplementary.as_bytes()).collect::<Vec<_>>(),
            [4, 7]
        );
        assert_eq!(
            segmenter
                .segment_utf16(&supplementary.encode_utf16().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [2, 3]
        );

        assert_eq!(
            Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1
                .trie
                .get('A'),
            0
        );
        assert_eq!(segmenter.segment_str("AB").collect::<Vec<_>>(), [1, 2]);
    }
}
