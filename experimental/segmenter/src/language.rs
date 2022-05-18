// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::slice::Iter;
use core::str::Chars;

#[derive(PartialEq)]
pub enum Language {
    Burmese,
    Thai,
    Unknown,
}

pub fn get_language(codepoint: u32) -> Language {
    match codepoint {
        0xe01..=0xe7f => Language::Thai,
        0x1000..=0x109f => Language::Burmese,
        0xa9e0..=0xa9ff => Language::Burmese,
        0xaa60..=0xaa7f => Language::Burmese,

        _ => Language::Unknown,
    }
}

/// This struct is an iterator that returns the string per language from the
/// given string.
///
/// Actually supported LSTM model is Thai and Burmese only. If using other
/// code point, it causes panic.
pub struct LanguageIterator<'s> {
    input: Chars<'s>,
    current_ch: Option<char>,
}

impl<'s> LanguageIterator<'s> {
    #[allow(dead_code)]
    pub fn new(input: &'s str) -> Self {
        let mut input = input.chars();
        let current_ch = input.next();
        Self { input, current_ch }
    }
}

impl<'s> Iterator for LanguageIterator<'s> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = "".to_string();

        let lang = get_language(self.current_ch? as u32);
        s.push(self.current_ch.unwrap());
        for c in self.input.by_ref() {
            self.current_ch = Some(c);
            let new_lang = get_language(c as u32);
            if lang != new_lang {
                return Some(s);
            }
            s.push(c);
        }
        self.current_ch = None;
        Some(s)
    }
}

pub struct LanguageIteratorUtf16<'s> {
    input: Iter<'s, u16>,
    current_ch: Option<&'s u16>,
}

impl<'s> LanguageIteratorUtf16<'s> {
    #[allow(dead_code)]
    pub fn new(input: &'s [u16]) -> Self {
        let mut input = input.iter();
        let current_ch = input.next();
        Self { input, current_ch }
    }
}

impl<'s> Iterator for LanguageIteratorUtf16<'s> {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s: Vec<u16> = Vec::new();

        let lang = get_language(*self.current_ch? as u32);
        s.push(*self.current_ch.unwrap());
        for c in self.input.by_ref() {
            self.current_ch = Some(c);
            let new_lang = get_language(*c as u32);
            if lang != new_lang {
                return Some(s);
            }
            s.push(*c);
        }
        self.current_ch = None;
        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thai_only() {
        let s = "ภาษาไทยภาษาไทย";
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let mut iter = LanguageIteratorUtf16::new(&utf16);
        assert_eq!(iter.next(), Some(utf16), "Thai language only with UTF-16");
        let mut iter = LanguageIterator::new(s);
        assert_eq!(
            iter.next(),
            Some(s.to_string()),
            "Thai language only with UTF-8"
        );
        assert_eq!(iter.next(), None, "Iterator for UTF-8 is finished");
    }

    #[test]
    fn test_combine() {
        const TEST_STR_THAI: &str = "ภาษาไทยภาษาไทย";
        const TEST_STR_BURMESE: &str = "ဗမာနွယ်ဘာသာစကားမျာ";
        let mut s = String::from(TEST_STR_THAI);
        s.push_str(TEST_STR_BURMESE);
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let thai_utf16: Vec<u16> = TEST_STR_THAI.encode_utf16().collect();
        let burmese_utf16: Vec<u16> = TEST_STR_BURMESE.encode_utf16().collect();

        let mut iter = LanguageIteratorUtf16::new(&utf16);
        assert_eq!(
            iter.next(),
            Some(thai_utf16),
            "Thai language with UTF-16 at first"
        );
        assert_eq!(
            iter.next(),
            Some(burmese_utf16),
            "Burmese language with UTF-16 at second"
        );
        assert_eq!(iter.next(), None, "Iterator for UTF-16 is finished");

        let mut iter = LanguageIterator::new(&s);
        assert_eq!(
            iter.next(),
            Some(TEST_STR_THAI.to_string()),
            "Thai language with UTF-8 at first"
        );
        assert_eq!(
            iter.next(),
            Some(TEST_STR_BURMESE.to_string()),
            "Burmese language with UTF-8 at second"
        );
        assert_eq!(iter.next(), None, "Iterator for UTF-8 is finished");
    }
}
