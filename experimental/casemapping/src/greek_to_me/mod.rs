// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains internal data handling tools for the special-cased Greek uppercasing
//! code. The Greek uppercasing algorithm is code-driven, not user-data-driven, however the code
//! relies on a CodePointTrie generated based on some Unicode rules.
//!
//! We try to keep most of the Greek-specific logic in here, though the actual logic to remove
//! accents is in full_helper() as it must integrate with the control flow.
//!
//! This is public and doc(hidden) so that it can be accessed from the gen_greek_to_me test file,
//! and should not be used otherwise.

mod data;

pub(crate) use data::GREEK_DATA_TRIE;

use icu_collections::codepointtrie::TrieValue;
use zerovec::ule::AsULE;

/// The per-precomposed letter data stored in the hardcoded trie from `mod data`
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "datagen", derive(databake::Bake), databake(path = self))] // the `super` makes this bakeable within the same crate
pub struct GreekPrecomposedLetterData {
    /// Whether it has an accent
    pub accented: bool,
    /// Whether it has a dialytika
    pub dialytika: bool,
    /// Whether it contains an ypogegrammeni
    pub ypogegrammeni: bool,
    /// The uppercase Greek letter that this maps to (with accents removed)
    /// expressed as an offset from U+0391 GREEK CAPITAL LETTER ALPHA
    pub uppercase: Option<char>,
}

const GREEK_BLOCK_START: u32 = 0x370;

/// Format:
///
/// 0..7: uppercase_delta
/// 8: uppercase_delta is set
/// 9: ypogegrammeni
/// 10: dialytika
/// 11: accented
/// rest: unused, but unvalidated as this is not user data
impl AsULE for GreekPrecomposedLetterData {
    type ULE = <u16 as AsULE>::ULE;
    fn to_unaligned(self) -> Self::ULE {
        let sixteen = u16::from(self);

        sixteen.to_unaligned()
    }

    fn from_unaligned(ule: Self::ULE) -> Self {
        u16::from_unaligned(ule).into()
    }
}

impl TrieValue for GreekPrecomposedLetterData {
    type TryFromU32Error = <u16 as TrieValue>::TryFromU32Error;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from_u32(i).map(From::from)
    }
}

impl GreekPrecomposedLetterData {
    /// Data on the diacritics present in this character
    pub(crate) fn diacritics(self) -> GreekDiacritics {
        GreekDiacritics {
            accented: self.accented,
            dialytika: self.dialytika,
            precomposed_ypogegrammeni: self.ypogegrammeni,
            // This is a single character, there is no way for it
            // to have a combining ypogegrammeni just yet
            combining_ypogegrammeni: false,
        }
    }
    /// The base greek letter in this character, uppercased (if any)
    pub(crate) fn greek_base_uppercase(self) -> Option<char> {
        self.uppercase
    }

    /// Is this a greek letter?
    pub(crate) fn is_greek_letter(self) -> bool {
        self.uppercase.is_some()
    }
}

impl From<u16> for GreekPrecomposedLetterData {
    fn from(sixteen: u16) -> Self {
        let mut this = Self::default();
        if sixteen & (1 << 8) != 0 {
            let int = u32::from(sixteen & 0xFF);
            let ch = int + GREEK_BLOCK_START;
            let ch = char::try_from(ch);
            debug_assert!(ch.is_ok());
            this.uppercase = ch.ok();
        } else {
            debug_assert!((sixteen & 0xFF00) == 0)
        }
        this.ypogegrammeni = sixteen & (1 << 9) != 0;
        this.dialytika = sixteen & (1 << 10) != 0;
        this.accented = sixteen & (1 << 11) != 0;
        debug_assert!(
            sixteen & (0xFFFF - (1 << 12) + 1) == 0,
            "Bits 12..16 should not be set in GreekPrecomposedLetterData"
        );

        this
    }
}

impl From<GreekPrecomposedLetterData> for u16 {
    fn from(this: GreekPrecomposedLetterData) -> Self {
        let mut sixteen = 0;
        if let Some(uppercase) = this.uppercase {
            let delta = u32::from(uppercase) - GREEK_BLOCK_START;
            debug_assert!(delta <= 0xFF);
            sixteen = u16::try_from(delta).unwrap_or(0);

            sixteen |= 1 << 8;
        }
        if this.ypogegrammeni {
            sixteen |= 1 << 9;
        }
        if this.dialytika {
            sixteen |= 1 << 10;
        }
        if this.accented {
            sixteen |= 1 << 11;
        }
        sixteen
    }
}

impl From<GreekPrecomposedLetterData> for u32 {
    fn from(this: GreekPrecomposedLetterData) -> Self {
        u16::from(this).into()
    }
}

/// General diacritic information for a character or string
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct GreekDiacritics {
    /// Whether it has an accent
    pub accented: bool,
    /// Whether it has a dialytika
    pub dialytika: bool,
    /// Whether it contains an ypogegrammeni (precomposed)
    pub precomposed_ypogegrammeni: bool,
    /// Whether it contains an ypogegrammeni (combining)
    pub combining_ypogegrammeni: bool,
}

pub const DIALYTIKA: char = '\u{0308}';
pub const DIALYTIKA_TONOS: char = '\u{0344}';
pub const YPOGEGRAMMENI: char = '\u{0345}';

#[macro_export]
#[doc(hidden)]
macro_rules! accent_marks {
    () => {
        // Accent marks that map to a tonos when found with a lone eta
        // https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%5Cu0300+%5Cu0301+%5Cu0342+%5Cu0302+%5Cu0303+%5Cu0311%5D&g=&i=
        '\u{0300}' | '\u{0301}' | '\u{0342}' | '\u{0302}' | '\u{0303}' | '\u{0311}'
    };
    (extra) => {
        // Other combining marks that are expected to be used with Greek
        // https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%5Cu0304+%5Cu0306+%5Cu0313+%5Cu0314+%5Cu0343%5D&g=&i=
        '\u{0304}' | '\u{0306}' | '\u{0313}' | '\u{0314}' | '\u{0343}'
    };
}

/// All Greek accent marks that need to be removed during uppercasing
///
/// Call with `accent_marks!(extra)` to get extra accent marks that are expected
/// to be used with Greek but do not need to be removed
///
/// This is a macro to make it easy to keep the lists of accents in sync
pub use crate::diacritics;

impl GreekDiacritics {
    /// Whilst forwards-iterating from an existing character,
    /// consume all further greek diacritics and store their existence into this struct
    pub(crate) fn consume_greek_diacritics(&mut self, context_after: &str) {
        for c in context_after.chars() {
            match c {
                accent_marks!() => self.accented = true,
                DIALYTIKA_TONOS => {
                    self.dialytika = true;
                    self.accented = true;
                }
                DIALYTIKA => self.dialytika = true,
                YPOGEGRAMMENI => self.combining_ypogegrammeni = true,
                // Ignore other accent marks that are expected to co-occur with Greek
                accent_marks!(extra) => (),
                _ => break,
            }
        }
    }
}

/// Given the context before a character, check if it is preceded by a Greek letter
pub(crate) fn preceded_by_greek_letter(context_before: &str) -> bool {
    for c in context_before.chars().rev() {
        match c {
            accent_marks!()
            | accent_marks!(extra)
            | DIALYTIKA
            | DIALYTIKA_TONOS
            | YPOGEGRAMMENI => continue,
            _ => return GREEK_DATA_TRIE.get(c).is_greek_letter(),
        }
    }
    return false;
}

/// Given the context before a character, check if is preceded by a Greek accented vowel
/// that does not have a dialytika
pub(crate) fn preceded_by_greek_accented_vowel_with_no_dialytika(context_before: &str) -> bool {
    let mut accented = false;
    for c in context_before.chars().rev() {
        match c {
            accent_marks!() => accented = true,
            DIALYTIKA | DIALYTIKA_TONOS => return false,
            accent_marks!(extra) => continue,
            _ => {
                let data = GREEK_DATA_TRIE.get(c);
                if let Some(uppercase) = data.greek_base_uppercase() {
                    let is_vowel = matches!(uppercase, 'Α' | 'Ε' | 'Η' | 'Ι' | 'Ο' | 'Υ' | 'Ω');
                    let base_diacritics = data.diacritics();
                    return is_vowel
                        && (accented || base_diacritics.accented)
                        && !base_diacritics.dialytika;
                } else {
                    // Not a greek letter
                    return false;
                }
            }
        }
    }
    return false;
}

/// Is the character a diacritic expected to be used with greek (except ypogegrammeni)
pub(crate) fn is_greek_diacritic_except_ypogegrammeni(c: char) -> bool {
    matches!(
        c,
        accent_marks!() | accent_marks!(extra) | DIALYTIKA | DIALYTIKA_TONOS
    )
}
