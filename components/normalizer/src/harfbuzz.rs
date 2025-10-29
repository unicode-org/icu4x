// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::properties::{
    CanonicalCombiningClassMap, CanonicalCombiningClassMapBorrowed, CanonicalComposition,
    CanonicalCompositionBorrowed, CanonicalDecomposition, CanonicalDecompositionBorrowed,
    Decomposed,
};
use harfbuzz_traits::{CombiningClassFunc, ComposeFunc, DecomposeFunc};

impl ComposeFunc for CanonicalCompositionBorrowed<'_> {
    fn compose(&self, a: char, b: char) -> Option<char> {
        CanonicalCompositionBorrowed::compose(*self, a, b)
    }
}

impl ComposeFunc for CanonicalComposition {
    fn compose(&self, a: char, b: char) -> Option<char> {
        ComposeFunc::compose(&self.as_borrowed(), a, b)
    }
}

impl ComposeFunc for &'_ CanonicalComposition {
    fn compose(&self, a: char, b: char) -> Option<char> {
        ComposeFunc::compose(&self.as_borrowed(), a, b)
    }
}

impl DecomposeFunc for CanonicalDecompositionBorrowed<'_> {
    fn decompose(&self, ab: char) -> Option<(char, char)> {
        match CanonicalDecompositionBorrowed::decompose(self, ab) {
            Decomposed::Default => None,
            Decomposed::Expansion(first, second) => Some((first, second)),
            Decomposed::Singleton(single) => Some((single, '\0')),
        }
    }
}

impl DecomposeFunc for CanonicalDecomposition {
    fn decompose(&self, ab: char) -> Option<(char, char)> {
        DecomposeFunc::decompose(&self.as_borrowed(), ab)
    }
}

impl DecomposeFunc for &'_ CanonicalDecomposition {
    fn decompose(&self, ab: char) -> Option<(char, char)> {
        DecomposeFunc::decompose(&self.as_borrowed(), ab)
    }
}

impl CombiningClassFunc for CanonicalCombiningClassMapBorrowed<'_> {
    fn combining_class(&self, ch: char) -> u8 {
        self.get_u8(ch)
    }
}

impl CombiningClassFunc for CanonicalCombiningClassMap {
    fn combining_class(&self, ch: char) -> u8 {
        CombiningClassFunc::combining_class(&self.as_borrowed(), ch)
    }
}

impl CombiningClassFunc for &'_ CanonicalCombiningClassMap {
    fn combining_class(&self, ch: char) -> u8 {
        CombiningClassFunc::combining_class(&self.as_borrowed(), ch)
    }
}
