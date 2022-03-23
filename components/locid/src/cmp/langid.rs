// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::LanguageIdentifier;
use core::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum State {
    Start,
    AfterLanguage,
    AfterScript,
    BeforeVariant(usize),
}

pub struct LanguageIdentifierSubtagIterator<'a> {
    langid: &'a LanguageIdentifier,
    state: State,
}

impl<'a> LanguageIdentifierSubtagIterator<'a> {
    pub fn new(langid: &'a LanguageIdentifier) -> Self {
        LanguageIdentifierSubtagIterator {
            langid,
            state: State::Start,
        }
    }
}

impl<'a> Iterator for LanguageIdentifierSubtagIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Start => {
                    self.state = State::AfterLanguage;
                    return Some(self.langid.language.as_str());
                }
                State::AfterLanguage => {
                    self.state = State::AfterScript;
                    if let Some(ref script) = self.langid.script {
                        return Some(script.as_str());
                    }
                }
                State::AfterScript => {
                    self.state = State::BeforeVariant(0);
                    if let Some(ref region) = self.langid.region {
                        return Some(region.as_str());
                    }
                }
                State::BeforeVariant(i) => {
                    if let Some(variant) = self.langid.variants.get(i) {
                        self.state = State::BeforeVariant(i + 1);
                        return Some(variant.as_str());
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}

pub fn cmp(base: &LanguageIdentifier, other: &[u8]) -> Ordering {
    let base_iter = LanguageIdentifierSubtagIterator::new(base);
    // Note: This does not use get_subtag_iterator because we want to guarantee
    // perfect lexicographic ordering of the strings.
    let other_iter = other.split(|b| *b == b'-');
    base_iter.map(str::as_bytes).cmp(other_iter)
}
