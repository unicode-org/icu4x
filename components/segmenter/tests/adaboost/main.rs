// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]

use icu_provider::prelude::*;
#[cfg(test)]
use icu_segmenter::provider::adaboost::AdaboostData;
use icu_segmenter::provider::{
    Baked, UnihanRadicalsData,
    adaboost::{
        SegmenterChineseAutoV1, SegmenterCjAutoV1, SegmenterJapaneseAutoV1,
        SegmenterThaiAutoV1,
    },
};
use std::iter::Peekable;
use std::str::CharIndices;
#[cfg(test)]
use zerovec::ZeroMap;

const CHINESE_ADABOOST: &DataMarkerAttributes =
    DataMarkerAttributes::from_str_or_panic("Chinese_adaboost");
const THAI_ADABOOST: &DataMarkerAttributes =
    DataMarkerAttributes::from_str_or_panic("Thai_adaboost");
const CJ_ADABOOST: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("CJ_adaboost");
const JAPANESE_ADABOOST: &DataMarkerAttributes =
    DataMarkerAttributes::from_str_or_panic("Japanese_adaboost");

pub(crate) fn get_radical(radicals: &UnihanRadicalsData<'_>, ch: char) -> u8 {
    radicals.trie.get(ch)
}

pub(crate) struct Predictor<'a> {
    model: DataPayload<SegmenterChineseAutoV1>,
    radicals: &'a UnihanRadicalsData<'a>,
}

pub(crate) struct ThaiPredictor {
    model: DataPayload<SegmenterThaiAutoV1>,
}

pub(crate) struct CjPredictor<'a> {
    model: DataPayload<SegmenterCjAutoV1>,
    radicals: &'a UnihanRadicalsData<'a>,
}

pub(crate) struct JapanesePredictor {
    model: DataPayload<SegmenterJapaneseAutoV1>,
}

pub(crate) struct AdaboostSegmenterIterator<'predictor, 'data, 's> {
    predictor: &'predictor Predictor<'data>,
    chars: Peekable<CharIndices<'s>>,
    len: usize,
    previous: Option<char>,
    previous_previous: Option<char>,
}

impl Iterator for AdaboostSegmenterIterator<'_, '_, '_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let previous = self.previous?;
            let Some((current_idx, current)) = self.chars.next() else {
                self.previous = None;
                return Some(self.len);
            };

            let next = self.chars.peek().map(|(_, ch)| *ch);
            let should_break =
                self.predictor
                    .score(self.previous_previous, previous, current, next)
                    > 0;

            self.previous_previous = Some(previous);
            self.previous = Some(current);

            if should_break {
                return Some(current_idx);
            }
        }
    }
}

impl<'a> Predictor<'a> {
    pub(crate) fn for_test() -> Self {
        let response: DataResponse<SegmenterChineseAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(CHINESE_ADABOOST),
                ..Default::default()
            })
            .expect("the baked Chinese AdaBoost model should load");
        Self {
            model: response.payload,
            radicals: Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1,
        }
    }

    fn score(
        &self,
        previous_previous: Option<char>,
        previous: char,
        current: char,
        next: Option<char>,
    ) -> i64 {
        let model = self.model.get();
        let mut score = i64::from(model.bias);

        let current_radical = get_radical(self.radicals, current);
        if current_radical != 0 {
            add_weight(
                &mut score,
                model.rsrid.get_copied(&(previous, current_radical)),
            );
        }

        let previous_radical = get_radical(self.radicals, previous);
        if previous_radical != 0 {
            add_weight(
                &mut score,
                model.lsrid.get_copied(&(previous_radical, current)),
            );
        }

        if previous_radical != 0 && current_radical != 0 {
            add_weight(
                &mut score,
                model.rad.get_copied(&(previous_radical, current_radical)),
            );
        }

        add_weight(&mut score, model.bw2.get_copied(&(previous, current)));

        if let Some(previous_previous) = previous_previous {
            add_weight(&mut score, model.uw2.get_copied(&previous_previous));
        }
        add_weight(&mut score, model.uw3.get_copied(&previous));
        add_weight(&mut score, model.uw4.get_copied(&current));
        if let Some(next) = next {
            add_weight(&mut score, model.uw5.get_copied(&next));
        }

        score
    }

    pub(crate) fn segment_str<'predictor, 's>(
        &'predictor self,
        input: &'s str,
    ) -> AdaboostSegmenterIterator<'predictor, 'a, 's> {
        let mut chars = input.char_indices().peekable();
        let previous = chars.next().map(|(_, ch)| ch);
        AdaboostSegmenterIterator {
            predictor: self,
            chars,
            len: input.len(),
            previous,
            previous_previous: None,
        }
    }

    pub(crate) fn predict(&self, input: &str) -> Vec<i64> {
        let chars = input.chars().collect::<Vec<_>>();
        (1..chars.len())
            .map(|i| {
                self.score(
                    i.checked_sub(2).map(|j| chars[j]),
                    chars[i - 1],
                    chars[i],
                    chars.get(i + 1).copied(),
                )
            })
            .collect()
    }

    pub(crate) fn predict_breakpoints(&self, sentence: &str) -> Vec<usize> {
        let mut breakpoints = vec![0];
        let mut offset = 0;
        for (&score, ch) in self.predict(sentence).iter().zip(sentence.chars()) {
            offset += ch.len_utf8();
            if score > 0 {
                breakpoints.push(offset);
            }
        }
        breakpoints
    }
}

impl ThaiPredictor {
    pub(crate) fn for_test() -> Self {
        let response: DataResponse<SegmenterThaiAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(THAI_ADABOOST),
                ..Default::default()
            })
            .expect("the baked Thai AdaBoost model should load");
        Self {
            model: response.payload,
        }
    }

    pub(crate) fn predict(&self, sentence: &str) -> Vec<i32> {
        let model = self.model.get();
        predict_unigram_model(
            sentence,
            model.bias,
            |position, key| match position {
                1 => model.uw1.get_copied(&key),
                2 => model.uw2.get_copied(&key),
                3 => model.uw3.get_copied(&key),
                4 => model.uw4.get_copied(&key),
                5 => model.uw5.get_copied(&key),
                6 => model.uw6.get_copied(&key),
                _ => None,
            },
            |position, key| match position {
                1 => model.bw1.get_copied(&key),
                2 => model.bw2.get_copied(&key),
                3 => model.bw3.get_copied(&key),
                _ => None,
            },
            |position, key| match position {
                1 => model.tw1.get_copied(key),
                2 => model.tw2.get_copied(key),
                3 => model.tw3.get_copied(key),
                4 => model.tw4.get_copied(key),
                _ => None,
            },
        )
    }

    pub(crate) fn predict_breakpoints(&self, sentence: &str) -> Vec<usize> {
        i32_breakpoints(sentence, &self.predict(sentence))
    }
}

impl JapanesePredictor {
    pub(crate) fn for_test() -> Self {
        let response: DataResponse<SegmenterJapaneseAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(JAPANESE_ADABOOST),
                ..Default::default()
            })
            .expect("the baked Japanese AdaBoost model should load");
        Self {
            model: response.payload,
        }
    }

    pub(crate) fn predict(&self, sentence: &str) -> Vec<i32> {
        let model = self.model.get();
        predict_unigram_model(
            sentence,
            model.bias,
            |position, key| match position {
                1 => model.uw1.get_copied(&key),
                2 => model.uw2.get_copied(&key),
                3 => model.uw3.get_copied(&key),
                4 => model.uw4.get_copied(&key),
                5 => model.uw5.get_copied(&key),
                6 => model.uw6.get_copied(&key),
                _ => None,
            },
            |position, key| match position {
                1 => model.bw1.get_copied(&key),
                2 => model.bw2.get_copied(&key),
                3 => model.bw3.get_copied(&key),
                _ => None,
            },
            |position, key| match position {
                1 => model.tw1.get_copied(key),
                2 => model.tw2.get_copied(key),
                3 => model.tw3.get_copied(key),
                4 => model.tw4.get_copied(key),
                _ => None,
            },
        )
    }

    pub(crate) fn predict_breakpoints(&self, sentence: &str) -> Vec<usize> {
        i32_breakpoints(sentence, &self.predict(sentence))
    }
}

impl CjPredictor<'static> {
    pub(crate) fn for_test() -> Self {
        let response: DataResponse<SegmenterCjAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(CJ_ADABOOST),
                ..Default::default()
            })
            .expect("the baked Chinese/Japanese AdaBoost model should load");
        Self {
            model: response.payload,
            radicals: Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1,
        }
    }

    pub(crate) fn predict(&self, sentence: &str) -> Vec<i32> {
        let chars: Vec<char> = sentence.chars().collect();
        let model = self.model.get();
        let mut mask = Vec::with_capacity(chars.len().saturating_sub(1));

        for i in 1..chars.len() {
            let previous = chars[i - 1];
            let current = chars[i];
            let mut score = model.bias;

            let previous_radical = get_radical(self.radicals, previous);
            let current_radical = get_radical(self.radicals, current);
            if previous_radical != 0 {
                add_i32_weight(
                    &mut score,
                    model.lsrid.get_copied(&(previous_radical, current)),
                );
            }
            if current_radical != 0 {
                add_i32_weight(
                    &mut score,
                    model.rsrid.get_copied(&(previous, current_radical)),
                );
            }
            if previous_radical != 0 && current_radical != 0 {
                add_i32_weight(
                    &mut score,
                    model.rad.get_copied(&(previous_radical, current_radical)),
                );
            }

            if i > 2 {
                let key: String = chars[i - 3..=i - 1].iter().collect();
                add_i32_weight(&mut score, model.tw1.get_copied(&key));
                add_i32_weight(&mut score, model.uw1.get_copied(&chars[i - 3]));
            }
            if i > 1 {
                let key: String = chars[i - 2..=i].iter().collect();
                add_i32_weight(&mut score, model.tw2.get_copied(&key));
                add_i32_weight(&mut score, model.bw1.get_copied(&(chars[i - 2], previous)));
                add_i32_weight(&mut score, model.uw2.get_copied(&chars[i - 2]));
            }
            if i + 1 < chars.len() {
                let key: String = chars[i - 1..=i + 1].iter().collect();
                add_i32_weight(&mut score, model.tw3.get_copied(&key));
                add_i32_weight(&mut score, model.bw3.get_copied(&(current, chars[i + 1])));
                add_i32_weight(&mut score, model.uw5.get_copied(&chars[i + 1]));
            }
            if i + 2 < chars.len() {
                let key: String = chars[i..=i + 2].iter().collect();
                add_i32_weight(&mut score, model.tw4.get_copied(&key));
                add_i32_weight(&mut score, model.uw6.get_copied(&chars[i + 2]));
            }

            add_i32_weight(&mut score, model.bw2.get_copied(&(previous, current)));
            add_i32_weight(&mut score, model.uw3.get_copied(&previous));
            add_i32_weight(&mut score, model.uw4.get_copied(&current));
            mask.push(score);
        }

        mask
    }

    pub(crate) fn predict_breakpoints(&self, sentence: &str) -> Vec<usize> {
        i32_breakpoints(sentence, &self.predict(sentence))
    }
}

fn predict_unigram_model(
    sentence: &str,
    bias: i32,
    unigram: impl Fn(u8, char) -> Option<i16>,
    bigram: impl Fn(u8, (char, char)) -> Option<i16>,
    trigram: impl Fn(u8, &str) -> Option<i16>,
) -> Vec<i32> {
    let chars: Vec<char> = sentence.chars().collect();
    let mut mask = Vec::with_capacity(chars.len().saturating_sub(1));

    for i in 1..chars.len() {
        let previous = chars[i - 1];
        let current = chars[i];
        let mut score = bias;

        if i > 2 {
            let key: String = chars[i - 3..=i - 1].iter().collect();
            add_doubled_weight(&mut score, trigram(1, &key));
            add_doubled_weight(&mut score, unigram(1, chars[i - 3]));
        }
        if i > 1 {
            let key: String = chars[i - 2..=i].iter().collect();
            add_doubled_weight(&mut score, trigram(2, &key));
            add_doubled_weight(&mut score, bigram(1, (chars[i - 2], previous)));
            add_doubled_weight(&mut score, unigram(2, chars[i - 2]));
        }
        if i + 1 < chars.len() {
            let key: String = chars[i - 1..=i + 1].iter().collect();
            add_doubled_weight(&mut score, trigram(3, &key));
            add_doubled_weight(&mut score, bigram(3, (current, chars[i + 1])));
            add_doubled_weight(&mut score, unigram(5, chars[i + 1]));
        }
        if i + 2 < chars.len() {
            let key: String = chars[i..=i + 2].iter().collect();
            add_doubled_weight(&mut score, trigram(4, &key));
            add_doubled_weight(&mut score, unigram(6, chars[i + 2]));
        }

        add_doubled_weight(&mut score, bigram(2, (previous, current)));
        add_doubled_weight(&mut score, unigram(3, previous));
        add_doubled_weight(&mut score, unigram(4, current));
        mask.push(score);
    }

    mask
}

fn i32_breakpoints(sentence: &str, scores: &[i32]) -> Vec<usize> {
    let mut breakpoints = vec![0];
    let mut offset = 0;
    for (&score, ch) in scores.iter().zip(sentence.chars()) {
        offset += ch.len_utf8();
        if score > 0 {
            breakpoints.push(offset);
        }
    }
    breakpoints
}

fn add_weight(score: &mut i64, weight: Option<i16>) {
    if let Some(weight) = weight {
        *score += i64::from(weight);
    }
}

fn add_i32_weight(score: &mut i32, weight: Option<i16>) {
    if let Some(weight) = weight {
        *score += i32::from(weight);
    }
}

fn add_doubled_weight(score: &mut i32, weight: Option<i16>) {
    if let Some(weight) = weight {
        *score += i32::from(weight) * 2;
    }
}

#[cfg(test)]
fn python_test_output() -> Vec<i64> {
    const PYTHON_OUTPUT: &str = include_str!("python_test_output.txt");
    PYTHON_OUTPUT
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().expect("failed to parse reference float"))
        .collect()
}

#[cfg(test)]
fn python_test_output_thai() -> Vec<i32> {
    const PYTHON_OUTPUT: &str = include_str!("python_test_output_thai.txt");
    PYTHON_OUTPUT
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().expect("failed to parse reference float"))
        .collect()
}

#[cfg(test)]
fn empty_predictor(bias: i32) -> Predictor<'static> {
    Predictor {
        model: DataPayload::from_owned(AdaboostData {
            bias,
            uw2: ZeroMap::new(),
            uw3: ZeroMap::new(),
            uw4: ZeroMap::new(),
            uw5: ZeroMap::new(),
            bw2: ZeroMap::new(),
            rad: ZeroMap::new(),
            lsrid: ZeroMap::new(),
            rsrid: ZeroMap::new(),
        }),
        radicals: Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1,
    }
}

#[test]
fn exact_scores_match_python() {
    let predictor = Predictor::for_test();
    assert_eq!(
        predictor.predict("根据最新的财报数据显示"),
        [-1346, 3484, -565, 3642, 3702, -1509, -9, -285, 2373, -1364]
    );
}

#[test]
fn new_model_scores_match_reference() {
    let sentence = "これはひらがなです";
    let cj = CjPredictor::for_test();
    let japanese = JapanesePredictor::for_test();

    assert_eq!(
        cj.predict(sentence),
        [-5251, -3066, 2899, -5681, -5058, -263, -4561, -3241]
    );
    assert_eq!(
        japanese.predict(sentence),
        [-14783, -11637, 8897, -17055, -14447, -499, -11029, -6673]
    );
    assert_eq!(cj.predict_breakpoints(sentence), [0, 9]);
    assert_eq!(japanese.predict_breakpoints(sentence), [0, 9]);
}

#[test]
fn streaming_boundaries_and_terminal() {
    let predictor = empty_predictor(-1);
    assert!(predictor.segment_str("").collect::<Vec<_>>().is_empty());
    assert_eq!(predictor.segment_str("甲").collect::<Vec<_>>(), [3]);
    assert_eq!(predictor.segment_str("甲乙丙").collect::<Vec<_>>(), [9]);

    let positive_score = empty_predictor(1);
    assert_eq!(
        positive_score.segment_str("甲乙丙").collect::<Vec<_>>(),
        [3, 6, 9]
    );

    let zero_score = empty_predictor(0);
    assert_eq!(zero_score.segment_str("AB").collect::<Vec<_>>(), [2]);
}

#[test]
fn unicode_byte_offsets_and_missing_radicals() {
    let predictor = empty_predictor(1);
    assert_eq!(predictor.segment_str("中国").collect::<Vec<_>>(), [3, 6]);
    assert_eq!(predictor.segment_str("中國").collect::<Vec<_>>(), [3, 6]);
    assert_eq!(predictor.segment_str("𠀀甲").collect::<Vec<_>>(), [4, 7]);
    assert_eq!(predictor.segment_str("AB").collect::<Vec<_>>(), [1, 2]);
    assert_eq!(
        Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1.trie.get('A'),
        0
    );
}

#[test]
fn rust_matches_python_probs() {
    let python = python_test_output();
    let python_thai = python_test_output_thai();
    let predictor = Predictor::for_test();
    let predictor_thai = ThaiPredictor::for_test();

    let sentence =
        "根据最新的财报数据显示，该公司的市盈率已经达到了历史最低点，但是其核心竞争力依然保持稳定增长的态势。"
            .to_string();
    let mask = predictor.predict(&sentence);

    let sentence = "ประเทศไทย หรือชื่อทางการว่า ราชอาณาจักรไทย เดิมเรียกว่า สยาม".to_string();
    let mask_thai = predictor_thai.predict(&sentence);

    assert_eq!(mask.len(), python.len());
    assert_eq!(mask_thai.len(), python_thai.len());

    let tol = 0;
    for (i, (&got, &expected)) in mask.iter().zip(python.iter()).enumerate() {
        let diff = (got - expected).abs();
        assert!(
            diff <= tol,
            "mismatch at index {i}: got={got:}, expected={expected:}, diff={diff:}"
        );
    }

    let tol = 0;
    for (i, (&got, &expected)) in mask_thai.iter().zip(python_thai.iter()).enumerate() {
        let diff = (got - expected).abs();
        assert!(
            diff <= tol,
            "mismatch at index {i}: got={got:}, expected={expected:}, diff={diff:}"
        );
    }
}
