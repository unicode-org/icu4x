// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_plurals::PluralElements;
use zerovec::VarZeroVec;

use crate::pattern::runtime::{Pattern, PatternULE};

#[derive(Debug)]
pub struct LengthPluralElements<'a> {
    pub long: PluralElements<'a, Pattern<'a>>,
    pub medium: PluralElements<'a, Pattern<'a>>,
    pub short: PluralElements<'a, Pattern<'a>>,
}

#[derive(Debug)]
pub struct PackedSkeletonDataBuilder<'a> {
    pub standard: LengthPluralElements<'a>,
    pub variant0: Option<LengthPluralElements<'a>>,
    pub variant1: Option<LengthPluralElements<'a>>,
}

// #[zerovec::make_varule(TaggedPatternULE)]
pub struct TaggedPattern<'data> {
    pub tag: u8,
    pub pattern: Pattern<'data>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::neo))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PackedSkeletonDataV2<'data> {
    pub header: u32,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, PatternULE>,
}

#[test]
fn test_sizes() {
    use core::mem::size_of;
    assert_eq!(32, size_of::<super::PackedSkeletonDataV1>());
    assert_eq!(32, size_of::<PackedSkeletonDataV2>());
}
