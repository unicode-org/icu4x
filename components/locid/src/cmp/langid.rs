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
    AfterRegion,
    AfterVariant(usize),
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
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.state == State::Start {
            self.state = State::AfterLanguage;
            return Some(self.langid.language.as_str().as_bytes());
        }
        if self.state == State::AfterLanguage {
            self.state = State::AfterScript;
            if let Some(ref script) = self.langid.script {
                return Some(script.as_str().as_bytes());
            }
        }
        if self.state == State::AfterScript {
            self.state = State::AfterRegion;
            if let Some(ref region) = self.langid.region {
                return Some(region.as_str().as_bytes());
            }
        }
        if self.state == State::AfterRegion {
            self.state = State::AfterVariant(0);
            if let Some(variant) = self.langid.variants.get(0) {
                return Some(variant.as_str().as_bytes());
            }
        }
        if let State::AfterVariant(i) = self.state {
            self.state = State::AfterVariant(i + 1);
            if let Some(variant) = self.langid.variants.get(i + 1) {
                return Some(variant.as_str().as_bytes());
            }
        }
        None
    }
}

pub fn cmp(base: &LanguageIdentifier, other: &[u8]) -> Ordering {
    let base_iter = LanguageIdentifierSubtagIterator::new(base);
    // Note: This does not use get_subtag_iterator because we want to guarantee
    // perfect lexicographic ordering of the strings.
    let other_iter = other.split(|b| *b == b'-');
    base_iter.cmp(other_iter)
}
