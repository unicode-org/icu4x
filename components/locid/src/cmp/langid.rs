// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::LanguageIdentifier;
use core::cmp::Ordering;
use tinystr::TinyAsciiStr;

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
    type Item = TinyAsciiStr<8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state == State::Start {
            self.state = State::AfterLanguage;
            return Some(TinyAsciiStr::from(self.langid.language).resize());
        }
        if self.state == State::AfterLanguage {
            self.state = State::AfterScript;
            if let Some(script) = self.langid.script {
                return Some(TinyAsciiStr::from(script).resize());
            }
        }
        if self.state == State::AfterScript {
            self.state = State::AfterRegion;
            if let Some(region) = self.langid.region {
                return Some(TinyAsciiStr::from(region).resize());
            }
        }
        if self.state == State::AfterRegion {
            self.state = State::AfterVariant(0);
            if let Some(variant) = self.langid.variants.get(0) {
                return Some(TinyAsciiStr::from(*variant).resize());
            }
        }
        if let State::AfterVariant(i) = self.state {
            // if i >= self.langid.variants.len() {
            //     return None;
            // }
            self.state = State::AfterVariant(i + 1);
            if let Some(variant) = self.langid.variants.get(i + 1) {
                return Some(TinyAsciiStr::from(*variant).resize());
            }
        }
        return None;
    }
}

// The following function is adapted from the Rust standard library:
// https://github.com/rust-lang/rust/blob/master/library/core/src/iter/traits/iterator.rs
//
// It is used under the terms of the Apache 2.0 license. Details:
// https://github.com/rust-lang/rust/blob/master/COPYRIGHT
//
// TODO: Remove this when iter_order_by is stable.
// https://github.com/rust-lang/rust/issues/64295
fn cmp_by<B, I, F>(mut base: B, other: I, mut cmp: F) -> Ordering
where
    B: Iterator,
    I: IntoIterator,
    F: FnMut(B::Item, I::Item) -> Ordering,
{
    let mut other = other.into_iter();

    loop {
        let x = match base.next() {
            None => {
                if other.next().is_none() {
                    return Ordering::Equal;
                } else {
                    return Ordering::Less;
                }
            }
            Some(val) => val,
        };

        let y = match other.next() {
            None => return Ordering::Greater,
            Some(val) => val,
        };

        match cmp(x, y) {
            Ordering::Equal => (),
            non_eq => return non_eq,
        }
    }
}

pub fn cmp(base: &LanguageIdentifier, other: &[u8]) -> Ordering {
    let base_iter = LanguageIdentifierSubtagIterator::new(base);
    // Note: This does not use get_subtag_iterator because we want to guarantee
    // perfect lexicographic ordering of the strings.
    let other_iter = other.split(|b| *b == b'-');
    cmp_by(base_iter, other_iter, |base_tinystr, other_u8_slice| {
        base_tinystr.as_bytes().cmp(other_u8_slice)
    })
}
