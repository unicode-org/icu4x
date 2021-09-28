// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::language::*;

use icu_segmenter_lstm::lstm::Lstm;
use std::char::decode_utf16;
use std::str::Chars;

// TODO:
// json file is big, So I should use anoher binary format like npy.
// But provided npy uses tensorflow dtype.
const THAI_MODEL: &[u8; 373466] = include_bytes!(
    "../../segmenter_lstm/tests/testdata/Thai_codepoints_exclusive_model4_heavy/weights.json"
);
const BURMESE_MODEL: &[u8; 475209] =
    include_bytes!("../data/Burmese_codepoints_exclusive_model4_heavy/weights.json");

lazy_static! {
    static ref THAI_LSTM: Lstm = {
        let lstm_data = serde_json::from_slice(THAI_MODEL).expect("JSON syntax error");
        Lstm::try_new(lstm_data).unwrap()
    };
    static ref BURMESE_LSTM: Lstm = {
        let lstm_data = serde_json::from_slice(BURMESE_MODEL).expect("JSON syntax error");
        Lstm::try_new(lstm_data).unwrap()
    };
}

// LSTM model depends on language, So we have to switch models per language.
fn get_best_lstm_model(codepoint: u32) -> &'static Lstm {
    let lang = get_language(codepoint);
    match lang {
        Language::Thai => &*THAI_LSTM,
        Language::Burmese => &*BURMESE_LSTM,
        _ => panic!("Unsupported"),
    }
}

/// This struct is an iterator that returns the string per language from the
/// given string.
///
/// Actually supported LSTM model is Thai and Burmese only. If using other
/// code point, it causes panic.
struct LanguageIterator<'a> {
    input: Chars<'a>,
    last: Option<char>,
}

impl<'a> LanguageIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut input = input.chars();
        let last = input.next();
        Self { input, last }
    }
}

impl<'a> Iterator for LanguageIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = "".to_string();

        let lang = get_language(self.last? as u32);
        s.push(self.last.unwrap());
        loop {
            let c = self.input.next();
            if c.is_none() {
                self.last = None;
                break;
            }
            self.last = c;
            let new_lang = get_language(c.unwrap() as u32);
            if lang != new_lang {
                break;
            }
            s.push(c.unwrap());
        }
        Some(s)
    }
}

// A word break iterator using LSTM model. Input string have to be same language.

struct LstmSegmenterIterator {
    input: String,
    bies_str: String,
    pos: usize,
    pos_utf8: usize,
}

impl Iterator for LstmSegmenterIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            self.pos_utf8 += self.input.chars().nth(self.pos).unwrap().len_utf8();
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos_utf8);
            }
        }
    }
}

impl LstmSegmenterIterator {
    #[cfg(test)]
    pub fn new(lstm: &Lstm, input: &str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            input: input.to_string(),
            bies_str: lstm_output,
            pos: 0,
            pos_utf8: 0,
        }
    }
}

struct LstmSegmenterIteratorUtf16 {
    bies_str: String,
    pos: usize,
}

impl Iterator for LstmSegmenterIteratorUtf16 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            // This ch is always in bitmap.
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos);
            }
        }
    }
}

impl LstmSegmenterIteratorUtf16 {
    pub fn new(lstm: &Lstm, input: &str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            bies_str: lstm_output,
            pos: 0,
        }
    }
}

#[cfg(test)]
pub fn get_line_break_utf8(input: &str) -> Option<Vec<usize>> {
    let mut result: Vec<usize> = Vec::new();
    let mut lang_iter = LanguageIterator::new(input);
    let mut offset = 0;
    loop {
        let str_per_lang = lang_iter.next();
        if str_per_lang.is_none() {
            break;
        }
        if offset != 0 {
            result.push(offset);
        }

        let str_per_lang = str_per_lang.unwrap();
        let lstm = get_best_lstm_model(str_per_lang.chars().next().unwrap() as u32);
        let lstm_iter = LstmSegmenterIterator::new(lstm, &str_per_lang);
        let mut r: Vec<usize> = lstm_iter.map(|n| offset + n).collect();
        result.append(&mut r);
        offset += str_per_lang.len();
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}

pub fn get_line_break_utf16(input: &[u16]) -> Option<Vec<usize>> {
    let s: String = decode_utf16(input.iter().cloned())
        .map(|r| r.unwrap())
        .collect();
    let mut result: Vec<usize> = Vec::new();
    let mut offset = 0;
    for str_per_lang in LanguageIterator::new(&s) {
        if offset != 0 {
            // language break
            result.push(offset);
        }

        let lstm = get_best_lstm_model(str_per_lang.chars().next().unwrap() as u32);
        let lstm_iter = LstmSegmenterIteratorUtf16::new(lstm, &str_per_lang);
        let mut r: Vec<usize> = lstm_iter.map(|n| offset + n).collect();
        result.append(&mut r);
        offset += str_per_lang.chars().fold(0, |n, c| n + c.len_utf16());
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::lstm::get_line_break_utf16;
    use crate::lstm::get_line_break_utf8;

    #[test]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";

        let breaks = get_line_break_utf8(TEST_STR);
        assert_eq!(breaks.unwrap(), [12, 21, 33], "Thai test");
    }

    #[test]
    fn thai_word_break_utf16() {
        let text: [u16; 14] = [
            0x0e20, 0x0e32, 0x0e29, 0x0e32, 0x0e44, 0x0e17, 0x0e22, 0x0e20, 0x0e32, 0x0e29, 0x0e32,
            0x0e44, 0x0e17, 0x0e22,
        ];
        let breaks = get_line_break_utf16(&text);
        assert_eq!(breaks.unwrap(), [4, 7, 11], "Thai test");

        let text: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        let breaks = get_line_break_utf16(&text);
        assert_eq!(breaks, None, "Thai test");
    }

    #[test]
    fn burmese_word_break() {
        // "Burmese Language" in Burmese
        const TEST_STR: &str = "မြန်မာဘာသာစကား";

        let breaks = get_line_break_utf8(TEST_STR);
        // LSTM model breaks more characters, but it is better to return [30].
        assert_eq!(breaks.unwrap(), [12, 18, 30], "Burmese test");
    }

    #[test]
    fn burmese_word_break_utf16() {
        // "Burmese Language" in Burmese
        let text: [u16; 14] = [
            0x1019, 0x103c, 0x1014, 0x103a, 0x1019, 0x102c, 0x1018, 0x102c, 0x101e, 0x102c, 0x1005,
            0x1000, 0x102c, 0x1038,
        ];
        let breaks = get_line_break_utf16(&text);
        // LSTM model breaks more characters, but it is better to return [10].
        assert_eq!(breaks.unwrap(), [4, 6, 10], "Burmese utf-16 test");
    }

    #[test]
    fn combined_word_break() {
        const TEST_STR_THAI: &str = "ภาษาไทยภาษาไทย";
        const TEST_STR_BURMESE: &str = "ဗမာနွယ်ဘာသာစကားမျာ";
        let mut sample = String::from(TEST_STR_THAI);
        sample.push_str(TEST_STR_BURMESE);

        let breaks = get_line_break_utf8(&sample);
        assert_eq!(
            breaks.unwrap(),
            [12, 21, 33, 42, 51, 63, 75, 87],
            "Combined test"
        );
    }
}
