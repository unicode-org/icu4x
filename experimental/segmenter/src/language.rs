// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_properties::provider::ScriptWithExtensionsPropertyV1;
use icu_properties::script::ScriptWithExtensionsBorrowed;
use icu_properties::Script;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Language {
    Burmese,
    ChineseOrJapanese,
    Khmer,
    Lao,
    Thai,
    Unknown,
}

impl Language {
    fn for_codepoint(script_data: &ScriptWithExtensionsPropertyV1, codepoint: u32) -> Self {
        match ScriptWithExtensionsBorrowed::from_data(script_data).get_script_val(codepoint) {
            Script::Thai => Language::Thai,
            Script::Lao => Language::Lao,
            Script::Myanmar => Language::Burmese,
            Script::Khmer => Language::Khmer,
            Script::Han | Script::Katakana | Script::Hiragana => Language::ChineseOrJapanese,
            _ => Language::Unknown,
        }
    }

    fn matches(&self, script_data: &ScriptWithExtensionsPropertyV1, codepoint: u32) -> bool {
        let map = ScriptWithExtensionsBorrowed::from_data(script_data);
        match self {
            Language::Thai => map.has_script(codepoint, Script::Thai),
            Language::Lao => map.has_script(codepoint, Script::Lao),
            Language::Burmese => map.has_script(codepoint, Script::Myanmar),
            Language::Khmer => map.has_script(codepoint, Script::Khmer),
            Language::ChineseOrJapanese => {
                map.has_script(codepoint, Script::Han)
                    || map.has_script(codepoint, Script::Katakana)
                    || map.has_script(codepoint, Script::Hiragana)
            }
            Language::Unknown => {
                Language::for_codepoint(script_data, codepoint) == Language::Unknown
            }
        }
    }
}

/// This struct is an iterator that returns the string per language from the
/// given string.
pub struct LanguageIterator<'s> {
    rest: &'s str,
    script_data: &'s ScriptWithExtensionsPropertyV1<'s>,
}

impl<'s> LanguageIterator<'s> {
    pub fn new(input: &'s str, script_data: &'s ScriptWithExtensionsPropertyV1<'s>) -> Self {
        Self {
            rest: input,
            script_data,
        }
    }
}

impl<'s> Iterator for LanguageIterator<'s> {
    type Item = (&'s str, Language);

    fn next(&mut self) -> Option<Self::Item> {
        let mut indices = self.rest.char_indices();
        let language = Language::for_codepoint(&self.script_data, indices.next()?.1 as u32);
        match indices.find(|&(_, ch)| language.matches(&self.script_data, ch as u32)) {
            Some((i, _)) => {
                let (result, rest) = self.rest.split_at(i);
                self.rest = rest;
                Some((result, language))
            }
            None => Some((core::mem::take(&mut self.rest), language)),
        }
    }
}

pub struct LanguageIteratorUtf16<'s> {
    rest: &'s [u16],
    script_data: &'s ScriptWithExtensionsPropertyV1<'s>,
}

impl<'s> LanguageIteratorUtf16<'s> {
    pub fn new(input: &'s [u16], script_data: &'s ScriptWithExtensionsPropertyV1<'s>) -> Self {
        Self {
            rest: input,
            script_data,
        }
    }
}

impl<'s> Iterator for LanguageIteratorUtf16<'s> {
    type Item = (&'s [u16], Language);

    fn next(&mut self) -> Option<Self::Item> {
        let language = Language::for_codepoint(&self.script_data, *self.rest.first()? as u32);
        match self
            .rest
            .iter()
            .position(|&ch| language.matches(&self.script_data, ch as u32))
        {
            Some(i) => {
                let (result, rest) = self.rest.split_at(i);
                self.rest = rest;
                Some((result, language))
            }
            None => Some((core::mem::take(&mut self.rest), language)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_properties::provider::ScriptWithExtensionsPropertyV1Marker;
    use icu_provider::prelude::*;

    #[test]
    fn test_thai_only() {
        let s = "ภาษาไทยภาษาไทย";
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let script_data: DataPayload<ScriptWithExtensionsPropertyV1Marker> = icu_testdata::buffer()
            .as_deserializing()
            .load(Default::default())
            .unwrap()
            .take_payload()
            .unwrap();
        let mut iter = LanguageIteratorUtf16::new(&utf16, script_data.get());
        assert_eq!(
            iter.next(),
            Some((utf16.as_slice(), Language::Thai)),
            "Thai language only with UTF-16"
        );
        let mut iter = LanguageIterator::new(s, script_data.get());
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
        let script_data: DataPayload<ScriptWithExtensionsPropertyV1Marker> = icu_testdata::buffer()
            .as_deserializing()
            .load(Default::default())
            .unwrap()
            .take_payload()
            .unwrap();

        let mut iter = LanguageIteratorUtf16::new(&utf16, script_data.get());
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

        let mut iter = LanguageIterator::new(&s, script_data.get());
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
