// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Language {
    Burmese,
    ChineseOrJapanese,
    Khmer,
    Lao,
    Thai,
    Unknown,
}

// TODO: Use data provider
pub(crate) fn get_language(codepoint: u32) -> Language {
    // For Thai, Burmese, Lao and Khmer, these are the intersections
    // of lb=SA with the respective Script
    match codepoint {
        0xE01..=0xE3A | 0xE40..=0x0E4E => Language::Thai,
        0x0E81
        | 0x0E82
        | 0x0E84
        | 0x0E86..=0x0E8A
        | 0x0E8C..0x0EA3
        | 0x0EA5
        | 0x0EA7..=0x0EBD
        | 0x0EC0..=0x0EC4
        | 0x0EC6
        | 0x0EC8..=0x0ECE
        | 0x0EDC..=0x0EDF => Language::Lao,
        0x1000..=0x103F
        | 0x1050..=0x108F
        | 0x109A..=0x109F
        | 0xA9E0..=0xA9EF
        | 0xA9FA..=0xA9FE
        | 0xAA60..=0xAA7F => Language::Burmese,
        0x1780..=0x17d3 | 0x17d7 | 0x17dc | 0x17dd => Language::Khmer,
        0x2E80..=0x2E99
        | 0x2E9B..=0x2EF3
        | 0x2F00..=0x2FD5
        | 0x3005
        | 0x3007
        | 0x3021..=0x3029
        | 0x3038..=0x303B
        | 0x3041..=0x3096
        | 0x309D..=0x309F
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xF900..=0xFA6D
        | 0xFA70..=0xFAD9
        | 0x00016FE2
        | 0x00016FE3
        | 0x00016FF0..=0x00016FF6
        | 0x0001B001..=0x0001B11F
        | 0x0001B132
        | 0x0001B150..=0x0001B152
        | 0x0001F200
        | 0x00020000..=0x0002A6DF
        | 0x0002A700..=0x0002B81D
        | 0x0002B820..=0x0002CEAD
        | 0x0002CEB0..=0x0002EBE0
        | 0x0002EBF0..=0x0002EE5D
        | 0x0002F800..=0x0002FA1D
        | 0x00030000..=0x0003134A
        | 0x00031350..=0x00033479 => Language::ChineseOrJapanese,
        _ => Language::Unknown,
    }
}

/// This struct is an iterator that returns the string per language from the
/// given string.
pub(super) struct LanguageIterator<'s> {
    rest: &'s str,
}

impl<'s> LanguageIterator<'s> {
    pub(super) fn new(input: &'s str) -> Self {
        Self { rest: input }
    }
}

impl<'s> Iterator for LanguageIterator<'s> {
    type Item = (&'s str, Language);

    fn next(&mut self) -> Option<Self::Item> {
        let mut indices = self.rest.char_indices();
        let lang = get_language(indices.next()?.1 as u32);
        match indices.find(|&(_, ch)| get_language(ch as u32) != lang) {
            Some((i, _)) => {
                let (result, rest) = self.rest.split_at(i);
                self.rest = rest;
                Some((result, lang))
            }
            None => Some((core::mem::take(&mut self.rest), lang)),
        }
    }
}

pub(super) struct LanguageIteratorUtf16<'s> {
    rest: &'s [u16],
}

impl<'s> LanguageIteratorUtf16<'s> {
    pub(super) fn new(input: &'s [u16]) -> Self {
        Self { rest: input }
    }
}

impl<'s> Iterator for LanguageIteratorUtf16<'s> {
    type Item = (&'s [u16], Language);

    fn next(&mut self) -> Option<Self::Item> {
        let lang = get_language(*self.rest.first()? as u32);
        match self
            .rest
            .iter()
            .position(|&ch| get_language(ch as u32) != lang)
        {
            Some(i) => {
                let (result, rest) = self.rest.split_at(i);
                self.rest = rest;
                Some((result, lang))
            }
            None => Some((core::mem::take(&mut self.rest), lang)),
        }
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
        assert_eq!(
            iter.next(),
            Some((utf16.as_slice(), Language::Thai)),
            "Thai language only with UTF-16"
        );
        let mut iter = LanguageIterator::new(s);
        assert_eq!(
            iter.next(),
            Some((s, Language::Thai)),
            "Thai language only with UTF-8"
        );
        assert_eq!(iter.next(), None, "Iterator for UTF-8 is finished");
    }

    #[test]
    fn test_combine() {
        const TEST_STR_THAI: &str = "ภาษาไทยภาษาไทย";
        const TEST_STR_BURMESE: &str = "ဗမာနွယ်ဘာသာစကားမျာ";
        let s = format!("{TEST_STR_THAI}{TEST_STR_BURMESE}");
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let thai_utf16: Vec<u16> = TEST_STR_THAI.encode_utf16().collect();
        let burmese_utf16: Vec<u16> = TEST_STR_BURMESE.encode_utf16().collect();

        let mut iter = LanguageIteratorUtf16::new(&utf16);
        assert_eq!(
            iter.next(),
            Some((thai_utf16.as_slice(), Language::Thai)),
            "Thai language with UTF-16 at first"
        );
        assert_eq!(
            iter.next(),
            Some((burmese_utf16.as_slice(), Language::Burmese)),
            "Burmese language with UTF-16 at second"
        );
        assert_eq!(iter.next(), None, "Iterator for UTF-16 is finished");

        let mut iter = LanguageIterator::new(&s);
        assert_eq!(
            iter.next(),
            Some((TEST_STR_THAI, Language::Thai)),
            "Thai language with UTF-8 at first"
        );
        assert_eq!(
            iter.next(),
            Some((TEST_STR_BURMESE, Language::Burmese)),
            "Burmese language with UTF-8 at second"
        );
        assert_eq!(iter.next(), None, "Iterator for UTF-8 is finished");
    }
}
