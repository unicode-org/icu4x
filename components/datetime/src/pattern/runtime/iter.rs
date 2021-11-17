// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::{runtime::generic::CombinedPatternIterator, PatternItem};
use zerovec::ZeroVecIter;

pub enum PatternKindItemIterator<'pattern> {
    Plain(ZeroVecIter<'pattern, PatternItem>),
    Combined(CombinedPatternIterator<'pattern>),
}

impl<'pattern> Iterator for PatternKindItemIterator<'pattern> {
    type Item = PatternItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Plain(p) => p.next(),
            Self::Combined(c) => c.next(),
        }
    }
}
