// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

static MODEL_FOR_TEST: &str = include_str!("model.json");

static CODEPOINTS: &[u16] = &[
    20008, 20022, 20031, 20057, 20101, 20108, 20128, 20154, 20799, 20837, 20843, 20866, 20886,
    20907, 20960, 20981, 20992, 21147, 21241, 21269, 21274, 21304, 21313, 21340, 21353, 21378,
    21430, 21448, 21475, 22231, 22303, 22763, 22786, 22794, 22805, 22823, 22899, 23376, 23424,
    23544, 23567, 23586, 23608, 23662, 23665, 24027, 24037, 24049, 24062, 24178, 24186, 24191,
    24308, 24318, 24331, 24339, 24400, 24417, 24435, 24515, 25096, 25142, 25163, 25903, 25908,
    25991, 26007, 26020, 26041, 26080, 26085, 26352, 26376, 26408, 27424, 27490, 27513, 27571,
    27595, 27604, 27611, 27663, 27668, 27700, 28779, 29226, 29238, 29243, 29247, 29255, 29273,
    29275, 29356, 29572, 29577, 29916, 29926, 29976, 29983, 29992, 30000, 30091, 30098, 30326,
    30333, 30382, 30399, 30446, 30683, 30690, 30707, 31034, 31160, 31166, 31348, 31435, 31481,
    31859, 31992, 32566, 32593, 32650, 32701, 32769, 32780, 32786, 32819, 32895, 32905, 33251,
    33258, 33267, 33276, 33292, 33307, 33311, 33390, 33394, 33400, 34381, 34411, 34880, 34892,
    34915, 35198, 35211, 35282, 35328, 35895, 35910, 35925, 35960, 35997, 36196, 36208, 36275,
    36523, 36554, 36763, 36784, 36789, 37009, 37193, 37318, 37324, 37329, 38263, 38272, 38428,
    38582, 38585, 38632, 38737, 38750, 38754, 38761, 38859, 38893, 38899, 38913, 39080, 39131,
    39135, 39318, 39321, 39340, 39592, 39640, 39647, 39717, 39727, 39730, 39740, 39770, 40165,
    40565, 40575, 40613, 40635, 40643, 40653, 40657, 40697, 40701, 40718, 40723, 40736, 40763,
    40778, 40786, 40845, 40860, 40864,
];

pub fn get_radical(ch: char) -> u8 {
    let id = ch as u32;

    if !(19968..=40869).contains(&id) {
        return 0;
    }

    let idx = CODEPOINTS.partition_point(|&b| (b as u32) <= id);
    (idx as u8) + 1
}

pub struct Predictor {
    pub model: HashMap<String, HashMap<String, i16>>,
}

impl Predictor {
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let model: HashMap<String, HashMap<String, i16>> =
            serde_json::from_str(json).unwrap_or_default();
        Ok(Self { model })
    }

    pub(crate) fn for_test() -> Self {
        Self::from_json(MODEL_FOR_TEST).unwrap()
    }

    pub fn predict(&self, sentence: &str) -> Vec<bool> {
        let chars: Vec<char> = sentence.chars().collect();
        if chars.is_empty() {
            return Vec::new();
        }

        let mut mask = Vec::with_capacity(chars.len());

        for i in 1..chars.len() {
            let c_prev = chars[i - 1];
            let c = chars[i];

            let mut score: i16 = 4;

            let rad4 = get_radical(c);
            if rad4 != 0 {
                if let Some(map) = self.model.get("RSRID") {
                    let key = format!("{}:{}", c_prev, rad4);
                    score += map.get(&key).copied().unwrap_or(0);
                }
            }

            let rad3 = get_radical(c_prev);
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

            mask.push(score > 0);
        }
        mask.push(true);

        mask
    }

    pub fn predict_breakpoints(&self, sentence: &str) -> Vec<usize> {
        let mut breakpoints = vec![0];
        let mut offset = 0;
        for (tag, ch) in self.predict(sentence).iter().zip(sentence.chars()) {
            offset += ch.len_utf8();
            if *tag {
                breakpoints.push(offset);
            }
        }
        breakpoints
    }
}

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let predictor = Predictor::for_test();

    let sentence =
        "香港國際機場繼上月停電後，前日再發生冷氣故障，經濟發展及勞工局長葉澍堃形容這是警號"
            .to_string();
    let mask = predictor.predict(&sentence);

    println!("Input: {}", sentence);
    println!("Output: {:?}", mask);
    Ok(())
}
