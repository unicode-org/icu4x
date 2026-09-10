// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::ComplexScript;
use utf8_iter::Utf8CharIndices;

// TODO: Use data provider
fn get_complex_script(codepoint: u32) -> ComplexScript {
    // For Thai, Burmese, Lao and Khmer, these are the intersections
    // of lb=SA with the respective Script
    match codepoint {
        0xE01..=0xE3A | 0xE40..=0x0E4E => ComplexScript::Thai,
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
        | 0x0EDC..=0x0EDF => ComplexScript::Lao,
        0x1000..=0x103F
        | 0x1050..=0x108F
        | 0x109A..=0x109F
        | 0xA9E0..=0xA9EF
        | 0xA9FA..=0xA9FE
        | 0xAA60..=0xAA7F => ComplexScript::Myanmar,
        0x1780..=0x17d3 | 0x17d7 | 0x17dc | 0x17dd => ComplexScript::Khmer,
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
        | 0x00031350..=0x00033479 => ComplexScript::ChineseOrJapanese,
        _ => ComplexScript::None,
    }
}

/// This struct is an iterator that returns the string per complex script from the
/// given string.
pub(super) struct ComplexScriptIterator<'s>(pub(super) &'s str);

impl<'s> Iterator for ComplexScriptIterator<'s> {
    type Item = (&'s str, ComplexScript);

    fn next(&mut self) -> Option<Self::Item> {
        let mut indices = self.0.char_indices();
        let lang = get_complex_script(indices.next()?.1 as u32);
        match indices.find(|&(_, ch)| get_complex_script(ch as u32) != lang) {
            Some((i, _)) => {
                let (result, rest) = self.0.split_at(i);
                self.0 = rest;
                Some((result, lang))
            }
            None => Some((core::mem::take(&mut self.0), lang)),
        }
    }
}

pub(super) struct ComplexScriptIteratorUtf8<'s>(pub(super) &'s [u8]);

impl<'s> Iterator for ComplexScriptIteratorUtf8<'s> {
    type Item = (&'s [u8], ComplexScript);

    fn next(&mut self) -> Option<Self::Item> {
        let mut indices = Utf8CharIndices::new(self.0);
        let script = get_complex_script(indices.next()?.1 as u32);
        match indices.find(|&(_, ch)| get_complex_script(ch as u32) != script) {
            Some((i, _)) => {
                let (result, rest) = self.0.split_at(i);
                self.0 = rest;
                Some((result, script))
            }
            None => Some((core::mem::take(&mut self.0), script)),
        }
    }
}

pub(super) struct ComplexScriptIteratorUtf16<'s>(pub(super) &'s [u16]);

impl<'s> Iterator for ComplexScriptIteratorUtf16<'s> {
    type Item = (&'s [u16], ComplexScript);

    fn next(&mut self) -> Option<Self::Item> {
        let lang = get_complex_script(*self.0.first()? as u32);
        match self
            .0
            .iter()
            .position(|&ch| get_complex_script(ch as u32) != lang)
        {
            Some(i) => {
                let (result, rest) = self.0.split_at(i);
                self.0 = rest;
                Some((result, lang))
            }
            None => Some((core::mem::take(&mut self.0), lang)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn test_iter(s: &str, expected: &[(&str, ComplexScript)]) {
        assert_eq!(
            ComplexScriptIterator(s).collect::<Vec<_>>(),
            expected,
            "UTF-8 iteration"
        );

        assert_eq!(
            ComplexScriptIteratorUtf8(s.as_bytes()).collect::<Vec<_>>(),
            expected
                .iter()
                .map(|(s, script)| (s.as_bytes(), *script))
                .collect::<Vec<_>>(),
            "UTF-8 iteration"
        );

        assert_eq!(
            ComplexScriptIteratorUtf16(&s.encode_utf16().collect::<Vec<_>>()).collect::<Vec<_>>(),
            expected
                .iter()
                .copied()
                .map(|(s, script)| (&*s.encode_utf16().collect::<Vec<_>>().leak(), script))
                .collect::<Vec<_>>(),
            "UTF-16 iteration"
        );
    }

    #[test]
    fn test_script_iter() {
        test_iter("ภาษาไทยภาษาไทย", &[("ภาษาไทยภาษาไทย", ComplexScript::Thai)]);

        test_iter(
            "မြန်မာစာမြန်မာစာမြန်မာစာ",
            &[("မြန်မာစာမြန်မာစာမြန်မာစာ", ComplexScript::Myanmar)],
        );

        test_iter(
            "ภาษาไทยภาษาไทยဗမာနွယ်ဘာသာစကားမျာ",
            &[
                ("ภาษาไทยภาษาไทย", ComplexScript::Thai),
                ("ဗမာနွယ်ဘာသာစကားမျာ", ComplexScript::Myanmar),
            ],
        );
    }
}
