// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]

use icu_segmenter::provider::{Baked, UnihanRadicalsData};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

static MODEL_FOR_TEST: &str = include_str!("../../../../provider/source/data/segmenter/model.json");
static MODEL_FOR_TEST_THAI: &str = include_str!("model_thai.json");

pub(crate) fn get_radical(radicals: &UnihanRadicalsData<'_>, ch: char) -> u8 {
    radicals.trie.get(ch)
}

pub(crate) struct Predictor<'a> {
    pub(crate) model: HashMap<String, HashMap<String, i16>>,
    radicals: &'a UnihanRadicalsData<'a>,
    bias: i32,
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
    pub(crate) fn from_json(json: &str, radicals: &'a UnihanRadicalsData<'a>) -> Self {
        let model: HashMap<String, HashMap<String, i16>> =
            serde_json::from_str(json).unwrap_or_default();
        let bias = -model
            .values()
            .flat_map(|weights| weights.values())
            .map(|&weight| i32::from(weight))
            .sum::<i32>()
            / 2;
        Self {
            model,
            radicals,
            bias,
        }
    }

    pub(crate) fn for_test() -> Self {
        Self::from_json(MODEL_FOR_TEST, Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1)
    }

    pub(crate) fn for_test_thai() -> Self {
        Self::from_json(
            MODEL_FOR_TEST_THAI,
            Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1,
        )
    }

    fn weight(&self, feature: &str, key: &str) -> Option<i16> {
        self.model
            .get(feature)
            .and_then(|weights| weights.get(key))
            .copied()
    }

    fn score(
        &self,
        previous_previous: Option<char>,
        previous: char,
        current: char,
        next: Option<char>,
    ) -> i64 {
        let mut score = i64::from(self.bias);

        let current_radical = get_radical(self.radicals, current);
        if current_radical != 0 {
            add_weight(
                &mut score,
                self.weight("RSRID", &format!("{previous}:{current_radical}")),
            );
        }

        let previous_radical = get_radical(self.radicals, previous);
        if previous_radical != 0 {
            add_weight(
                &mut score,
                self.weight("LSRID", &format!("{previous_radical}:{current}")),
            );
        }

        if previous_radical != 0 && current_radical != 0 {
            add_weight(
                &mut score,
                self.weight("RAD", &format!("{previous_radical}:{current_radical}")),
            );
        }

        add_weight(
            &mut score,
            self.weight("BW2", &format!("{previous}{current}")),
        );

        if let Some(previous_previous) = previous_previous {
            add_weight(
                &mut score,
                self.weight("UW2", &previous_previous.to_string()),
            );
        }
        add_weight(&mut score, self.weight("UW3", &previous.to_string()));
        add_weight(&mut score, self.weight("UW4", &current.to_string()));
        if let Some(next) = next {
            add_weight(&mut score, self.weight("UW5", &next.to_string()));
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
                let score = self.score(
                    i.checked_sub(2).map(|j| chars[j]),
                    chars[i - 1],
                    chars[i],
                    chars.get(i + 1).copied(),
                );
                score
            })
            .collect()
    }

    pub(crate) fn predict_thai(&self, sentence: &str) -> Vec<i16> {
        let chars: Vec<char> = sentence.chars().collect();
        if chars.is_empty() {
            return Vec::new();
        }

        let mut mask = Vec::with_capacity(chars.len());

        for i in 1..chars.len() {
            let c_prev = chars[i - 1];
            let c = chars[i];

            let mut score: i16 = -3755;

            if i > 2
                && let Some(map) = self.model.get("TW1")
            {
                let key: String = chars[i - 3..=i - 1].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i > 1
                && let Some(map) = self.model.get("TW2")
            {
                let key: String = chars[i - 2..=i].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i + 1 < chars.len()
                && let Some(map) = self.model.get("TW3")
            {
                let key: String = chars[i - 1..=i + 1].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i + 2 < chars.len()
                && let Some(map) = self.model.get("TW4")
            {
                let key: String = chars[i..=i + 2].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i > 1
                && let Some(map) = self.model.get("BW1")
            {
                let key: String = chars[i - 2..=i - 1].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if let Some(map) = self.model.get("BW2") {
                let key: String = chars[i - 1..=i].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i + 1 < chars.len()
                && let Some(map) = self.model.get("BW3")
            {
                let key: String = chars[i..=i + 1].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i > 2
                && let Some(map) = self.model.get("UW1")
            {
                let key = chars[i - 3].to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i > 1
                && let Some(map) = self.model.get("UW2")
            {
                let key = chars[i - 2].to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if let Some(map) = self.model.get("UW3") {
                let key = c_prev.to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if let Some(map) = self.model.get("UW4") {
                let key = c.to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i + 1 < chars.len()
                && let Some(map) = self.model.get("UW5")
            {
                let key = chars[i + 1].to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i + 2 < chars.len()
                && let Some(map) = self.model.get("UW6")
            {
                let key = chars[i + 2].to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            mask.push(score);
        }

        mask
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

    pub(crate) fn predict_thai_breakpoints(&self, sentence: &str) -> Vec<usize> {
        let mut breakpoints = vec![0];
        let mut offset = 0;
        for (&score, ch) in self.predict_thai(sentence).iter().zip(sentence.chars()) {
            offset += ch.len_utf8();
            if score > 0 {
                breakpoints.push(offset);
            }
        }
        breakpoints
    }
}

fn add_weight(score: &mut i64, weight: Option<i16>) {
    if let Some(weight) = weight {
        *score += i64::from(weight);
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
fn python_test_output_thai() -> Vec<i16> {
    const PYTHON_OUTPUT: &str = include_str!("python_test_output_thai.txt");
    PYTHON_OUTPUT
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i16>().expect("failed to parse reference float"))
        .collect()
}

#[cfg(test)]
fn empty_predictor(bias: i32) -> Predictor<'static> {
    Predictor {
        model: HashMap::new(),
        radicals: Baked::SINGLETON_SEGMENTER_UNIHAN_RADICAL_V1,
        bias,
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
    let predictor_thai = Predictor::for_test_thai();

    let sentence =
        "根据最新的财报数据显示，该公司的市盈率已经达到了历史最低点，但是其核心竞争力依然保持稳定增长的态势。"
            .to_string();
    let mask = predictor.predict(&sentence);

    let sentence = "ประเทศไทย หรือชื่อทางการว่า ราชอาณาจักรไทย เดิมเรียกว่า สยาม".to_string();
    let mask_thai = predictor_thai.predict_thai(&sentence);

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
