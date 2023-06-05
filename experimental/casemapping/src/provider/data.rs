// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// The case of a Unicode character
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CaseType {
    // Not a cased letter
    None = 0,
    // Lowercase letter
    Lower = 1,
    // Uppercase letter
    Upper = 2,
    // Titlecase letter
    Title = 3,
}

impl CaseType {
    pub(crate) const CASE_MASK: u16 = 0x3;

    // The casetype is stored in the codepoint trie as two bits.
    // After masking them to get a value between 0 and 3, this
    // function converts to CaseType.
    #[inline]
    pub(crate) fn from_masked_bits(b: u16) -> Self {
        debug_assert!(b & Self::CASE_MASK == b);
        match b {
            0 => CaseType::None,
            1 => CaseType::Lower,
            2 => CaseType::Upper,
            _ => CaseType::Title,
        }
    }
}

// The dot type of a Unicode character. This indicates how dotted
// letters (like `i` and `j`) combine with accents placed above the
// letter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DotType {
    // Normal characters with combining class 0
    NoDot = 0,
    // Soft-dotted characters with combining class 0
    SoftDotted = 1,
    // "Above" accents with combining class 230
    Above = 2,
    // Other accent characters
    OtherAccent = 3,
}

impl DotType {
    pub(crate) const DOT_MASK: u16 = 0x3;

    // The dot type is stored in either the codepoint trie or the
    // exception table as two bits.  After shifting and masking them
    // to get a value between 0 and 3, this function converts to
    // DotType.
    #[inline]
    pub(crate) fn from_masked_bits(b: u16) -> Self {
        debug_assert!(b & Self::DOT_MASK == b);
        match b {
            0 => DotType::NoDot,
            1 => DotType::SoftDotted,
            2 => DotType::Above,
            _ => DotType::OtherAccent,
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum MappingKind {
    Lower = 0,
    Fold = 1,
    Upper = 2,
    Title = 3,
}
