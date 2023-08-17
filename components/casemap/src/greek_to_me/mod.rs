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
#[cfg_attr(feature = "datagen", derive(databake::Bake), databake(path = self))] // the `self` makes this bakeable if it's already in scope
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
            ypogegrammeni: self.ypogegrammeni,
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

/// General diacritic information about a character or combining character sequence.
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct GreekDiacritics {
    /// Whether it has an accent.
    pub accented: bool,
    /// Whether it has a dialytika.
    pub dialytika: bool,
    /// Whether it has a ypogegrammeni.
    pub ypogegrammeni: bool,
}

/// General diacritic information about a combining character sequence,
/// identifying the source of the diacritics.
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct GreekCombiningCharacterSequenceDiacritics {
    // Diacritics precomposed on the base.
    pub precomposed: GreekDiacritics,
    // Combining diacritics.
    pub combining: GreekDiacritics,
}

pub const TONOS: char = '\u{0301}';
pub const DIALYTIKA: char = '\u{0308}';
pub const DIALYTIKA_TONOS: char = '\u{0344}';
pub const YPOGEGRAMMENI: char = '\u{0345}';

#[macro_export]
#[doc(hidden)]
macro_rules! diacritics {
    // Accents.
    // These are mostly removed when uppercasing, but their presence may require
    // adding a διαλυτικά to a following vowel.
    (ACCENTS) => {
        // https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%5Cu0300+%5Cu0301+%5Cu0342+%5Cu0302+%5Cu0303+%5Cu0311%5D&g=&i=
        '\u{0300}' // Polytonic βαρεία (varia), grave accent.
        | $crate::greek_to_me::TONOS // Polytonic οξεία (oxia) unified with monotonic τόνος (tonos), acute accent.
        | '\u{0342}' // Polytonic περισπωμένη (perispomeni), often translated to circumflex.
        | '\u{0302}' // Circumflex accent, sometimes a lookalike of the περισπωμένη.
        | '\u{0303}' // Tilde, sometimes a lookalike of the περισπωμένη.
        | '\u{0311}' // Inverted breve, sometimes a lookalike of the περισπωμένη.
    };
    // Breathings and length marks.
    // These expected to occur in Greek combining sequences, and are removed when uppercasing.
    // This removal has no other effect.
    (BREATHING_AND_LENGTH) => {
        // https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%5Cu0304+%5Cu0306+%5Cu0313+%5Cu0314+%5Cu0343%5D&g=&i=
        '\u{0304}'  // Macron, marking long vowels.
        | '\u{0306}'  // Breve, marking short vowels.
        | '\u{0313}'  // Comma above, smooth breathing or κορωνίς marking crasis.
        | '\u{0314}'  // Reversed comma above, rough breathing.
        | '\u{0343}'  // κορωνίς (koronis), canonically decomposes to comma above.
    };
    // All diacritics containing a dialytika
    (DIALYTIKA_ALL) => { $crate::greek_to_me::DIALYTIKA | $crate::greek_to_me::DIALYTIKA_TONOS };
    (DIALYTIKA) => { $crate::greek_to_me::DIALYTIKA };
    (DIALYTIKA_TONOS) => { $crate::greek_to_me::DIALYTIKA_TONOS };
    (YPOGEGRAMMENI) => { $crate::greek_to_me::YPOGEGRAMMENI };
    ($($i:ident)|+) => { $(diacritics!($i))|+};
}

/// Macro that generates match arms for various diacritic groupings.
///
/// Groupings supported:
///
/// - ACCENTS
/// - BREATHING_AND_LENGTH
/// - DIALYTIKA, DIALYTIKA_TONOS, and DIALITYKA_ALL
/// - YPOGEGRAMMENI
///
/// This is a macro to make it easy to keep the lists of accents in sync.
pub use crate::diacritics;

impl GreekDiacritics {
    /// Whilst forwards-iterating from an existing character,
    /// consume all further greek diacritics and store their existence into this struct.
    pub(crate) fn consume_greek_diacritics(&mut self, context_after: &str) {
        for c in context_after.chars() {
            match c {
                diacritics!(ACCENTS) => self.accented = true,
                DIALYTIKA_TONOS => {
                    self.dialytika = true;
                    self.accented = true;
                }
                DIALYTIKA => self.dialytika = true,
                YPOGEGRAMMENI => self.ypogegrammeni = true,
                // Ignore other accent marks that are expected to co-occur with Greek.
                diacritics!(BREATHING_AND_LENGTH) => (),
                _ => break,
            }
        }
    }
}

/// Given the context before a character, check if it is preceded by a Greek letter.
pub(crate) fn preceded_by_greek_letter(context_before: &str) -> bool {
    for c in context_before.chars().rev() {
        match c {
            diacritics!(ACCENTS | BREATHING_AND_LENGTH | DIALYTIKA_ALL | YPOGEGRAMMENI) => continue,
            _ => return GREEK_DATA_TRIE.get(c).is_greek_letter(),
        }
    }
    false
}

/// Returns diacritic information for the combining character sequence preceding the current character
/// if it that preceding combining character sequence is a greek vowel.
pub(crate) fn preceding_greek_vowel_diacritics(
    context_before: &str,
) -> Option<GreekCombiningCharacterSequenceDiacritics> {
    let mut combining: GreekDiacritics = Default::default();
    for c in context_before.chars().rev() {
        match c {
            diacritics!(ACCENTS) => combining.accented = true,
            diacritics!(DIALYTIKA_TONOS) => {
                combining.dialytika = true;
                combining.accented = true;
            }
            diacritics!(DIALYTIKA) => combining.dialytika = true,
            diacritics!(BREATHING_AND_LENGTH) => continue,
            _ => {
                let data = GREEK_DATA_TRIE.get(c);
                if let Some(uppercase) = data.greek_base_uppercase() {
                    let is_vowel = matches!(uppercase, 'Α' | 'Ε' | 'Η' | 'Ι' | 'Ο' | 'Υ' | 'Ω');
                    if !is_vowel {
                        return None;
                    }
                    let base_diacritics = data.diacritics();
                    return Some(GreekCombiningCharacterSequenceDiacritics {
                        precomposed: base_diacritics,
                        combining,
                    });
                } else {
                    // Not a greek letter.
                    return None;
                }
            }
        }
    }
    None
}

/// Is the character a diacritic expected to be used with greek (except ypogegrammeni).
pub(crate) fn is_greek_diacritic_except_ypogegrammeni(c: char) -> bool {
    matches!(
        c,
        diacritics!(ACCENTS | BREATHING_AND_LENGTH | DIALYTIKA_ALL)
    )
}
