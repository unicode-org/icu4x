// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]

use icu_segmenter::provider::{Baked, radical::UnihanRadicalsData};
use std::collections::HashMap;

static MODEL_FOR_TEST: &str = include_str!("model.json");
static MODEL_FOR_TEST_THAI: &str = include_str!("model_thai.json");

pub(crate) fn get_radical(radicals: &UnihanRadicalsData<'_>, ch: char) -> u8 {
    radicals.trie.get(ch)
}

pub(crate) struct Predictor<'a> {
    pub(crate) model: HashMap<String, HashMap<String, i16>>,
    radicals: &'a UnihanRadicalsData<'a>,
}

impl<'a> Predictor<'a> {
    pub(crate) fn from_json(json: &str, radicals: &'a UnihanRadicalsData<'a>) -> Self {
        let model: HashMap<String, HashMap<String, i16>> =
            serde_json::from_str(json).unwrap_or_default();
        Self { model, radicals }
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

    pub(crate) fn predict(&self, sentence: &str) -> Vec<i16> {
        let chars: Vec<char> = sentence.chars().collect();
        if chars.is_empty() {
            return Vec::new();
        }

        let mut mask = Vec::with_capacity(chars.len());

        for i in 1..chars.len() {
            let c_prev = chars[i - 1];
            let c = chars[i];

            let mut score: i16 = 4;

            let rad4 = get_radical(self.radicals, c);
            if rad4 != 0 {
                if let Some(map) = self.model.get("RSRID") {
                    let key = format!("{}:{}", c_prev, rad4);
                    score += map.get(&key).copied().unwrap_or(0);
                }
            }

            let rad3 = get_radical(self.radicals, c_prev);
            if rad3 != 0 {
                if let Some(map) = self.model.get("LSRID") {
                    let key = format!("{}:{}", rad3, c);
                    score += map.get(&key).copied().unwrap_or(0);
                }
            }

            if rad3 != 0 && rad4 != 0 {
                if let Some(map) = self.model.get("RAD") {
                    let key = format!("{}:{}", rad3, rad4);
                    score += map.get(&key).copied().unwrap_or(0);
                }
            }

            if let Some(map) = self.model.get("BW2") {
                let key: String = chars[i - 1..=i].iter().collect();
                score += map.get(&key).copied().unwrap_or(0);
            }

            if i > 1 {
                if let Some(map) = self.model.get("UW2") {
                    let key = chars[i - 2].to_string();
                    score += map.get(&key).copied().unwrap_or(0);
                }
            }

            if let Some(map) = self.model.get("UW3") {
                let key = c_prev.to_string();
                score += map.get(&key).copied().unwrap_or(0);
            }

            if let Some(map) = self.model.get("UW4") {
                let key = c.to_string();
                score += map.get(&key).copied().unwrap_or(0);
            }

            if i + 1 < chars.len() {
                if let Some(map) = self.model.get("UW5") {
                    let key = chars[i + 1].to_string();
                    score += map.get(&key).copied().unwrap_or(0);
                }
            }

            mask.push(score);
        }

        mask
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

            if i > 2 {
                if let Some(map) = self.model.get("TW1") {
                    let key: String = chars[i - 3..=i - 1].iter().collect();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if i > 1 {
                if let Some(map) = self.model.get("TW2") {
                    let key: String = chars[i - 2..=i].iter().collect();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if i + 1 < chars.len() {
                if let Some(map) = self.model.get("TW3") {
                    let key: String = chars[i - 1..=i + 1].iter().collect();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if i + 2 < chars.len() {
                if let Some(map) = self.model.get("TW4") {
                    let key: String = chars[i..=i + 2].iter().collect();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if i > 1 {
                if let Some(map) = self.model.get("BW1") {
                    let key: String = chars[i - 2..=i - 1].iter().collect();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if let Some(map) = self.model.get("BW2") {
                let key: String = chars[i - 1..=i].iter().collect();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i + 1 < chars.len() {
                if let Some(map) = self.model.get("BW3") {
                    let key: String = chars[i..=i + 1].iter().collect();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if i > 2 {
                if let Some(map) = self.model.get("UW1") {
                    let key = chars[i - 3].to_string();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if i > 1 {
                if let Some(map) = self.model.get("UW2") {
                    let key = chars[i - 2].to_string();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if let Some(map) = self.model.get("UW3") {
                let key = c_prev.to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if let Some(map) = self.model.get("UW4") {
                let key = c.to_string();
                score += map.get(&key).copied().unwrap_or(0) << 1;
            }

            if i + 1 < chars.len() {
                if let Some(map) = self.model.get("UW5") {
                    let key = chars[i + 1].to_string();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
            }

            if i + 2 < chars.len() {
                if let Some(map) = self.model.get("UW6") {
                    let key = chars[i + 2].to_string();
                    score += map.get(&key).copied().unwrap_or(0) << 1;
                }
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

#[cfg(test)]
fn python_test_output() -> Vec<i16> {
    const PYTHON_OUTPUT: &str = include_str!("python_test_output.txt");
    PYTHON_OUTPUT
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i16>().expect("failed to parse reference float"))
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

#[test]
fn main() {
    let predictor = Predictor::for_test();

    let sentence =
        "根据最新的财报数据显示，该公司的市盈率已经达到了历史最低点，但是其核心竞争力依然保持稳定增长的态势。"
            .to_string();
    let mask = predictor.predict(&sentence);

    println!("Input: {}", sentence);
    println!("Output: {:?}", mask);
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
