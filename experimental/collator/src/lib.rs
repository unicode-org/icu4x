// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Various collation-related algorithms and constants in this file are
// adapted from ICU4C and, therefore, are subject to the ICU license as
// described in LICENSE.

#![cfg_attr(not(any(test, feature = "std")), no_std)]

//! `icu_collation` is one of the ICU4X components.
//!
//! This API provides necessary functionality for comparing strings according to language-dependent
//! conventions.
//!
//! # Design notes
//!
//! * The collation element design comes from ICU4C. Some parts of the ICU4C design, notably,
//!   `Tag::BuilderDataTag`, `Tag::LeadSurrogateTag`, `Tag::LatinExpansionTag`, `Tag::U0000Tag`,
//!   and `Tag::HangulTag` are unused.
//!   - `Tag::LatinExpansionTag` might be reallocated to search expansions for archaic jamo
//!     in the future.
//!   - `Tag::HangulTag` might be reallocated to compressed hanja expansions in the future.
//!     See [issue 1315](https://github.com/unicode-org/icu4x/issues/1315).
//! * The key design difference between ICU4C and ICU4X is that ICU4C puts the canonical
//!   closure in the data (larger data) to enable lookup directly by precomposed characters
//!   while ICU4X always omits the canonical closure and always normalizes to NFD on the fly.
//! * Compared to ICU4C, normalization cannot be turned off. There also isn't a separate
//!   "Fast Latin" mode.
//! * The normalization is fused into the collation element lookup algorithm to optimize the
//!   case where an input character decomposes into two BMP characters: a base letter and a
//!   diacritic.
//!   - To optimize away a trie lookup when the combining diacritic doesn't contract,
//!     there is a linear lookup table for the combining diacritics block. Three languages
//!     tailor diacritics: Ewe, Lithuanian, and Vietnamese. Vietnamese and Ewe load an
//!     alternative table. The Lithuanian special cases are hard-coded and activatable by
//!     a metadata bit.
//! * Unfortunately, contractions that contract starters don't fit this model nicely. Therefore,
//!   there's duplicated normalization code for normalizing the lookahead for contractions.
//!   This code can, in principle, do duplicative work, but it shouldn't be excessive with
//!   real-world inputs.
//! * As a result, in terms of code provenance, the algorithms come from ICU4C, except the
//!   normalization part of the code is novel to ICU4X, and the contraction code is custom
//!   to ICU4X despite being informed by ICU4C.
//! * The way input characters are iterated over and resulting collation elements are
//!   buffered is novel to ICU4X.
//! * ICU4C can iterate backwards but ICU4X cannot. ICU4X keeps a buffer of the two most
//!   recent characters for handling prefixes. As of CLDR 40, there were only two kinds
//!   of prefixes: a single starter and a starter followed by a kana voicing mark.
//! * ICU4C sorts unpaired surrogates in their lexical order. ICU4X operates on Unicode
//!   [scalar values](https://unicode.org/glossary/#unicode_scalar_value) (any Unicode
//!   code point except high-surrogate and low-surrogate code points), so unpaired
//!   surrogates sort as REPLACEMENT CHARACTERs. Therefore, all unpaired
//!   surrogates are equal with each other.
//! * Skipping over a bit-identical prefix and then going back over "backward-unsafe"
//!   characters is currently unimplemented but isn't architecturally precluded.
//! * Hangul is handled specially:
//!   - Precomposed syllables are checked for as the first step of processing an
//!     incoming character.
//!   - Individual jamo are lookup up from a linear table instead of a trie. Unlike
//!     in ICU4C, this table covers the whole Unicode block whereas in ICU4C it covers
//!     only modern jamo for use in decomposing the precomposed syllables. The point
//!     is that search collations have a lot of duplicative (across multiple search)
//!     collations data for making archaic jamo searchable by modern jamo.
//!     Unfortunately, the shareable part isn't currently actually shareable, because
//!     the tailored CE32s refer to the expansions table in each collation. To make
//!     them truly shareable, the archaic jamo expansions need to become self-contained
//!     the way Latin mini expansions in ICU4C are self-contained.
//!
//!     One possible alternative to loading a different table for "search" would be
//!     performing the mapping of archaic jamo to the modern approximations as a
//!     special preprocessing step for the incoming characters, which would allow
//!     the lookup of the resulting modern jamo from the normal root jamo table.
//!
//!     "searchjl" is even more problematic than "search", since "searchjl" uses
//!     prefixes matches with jamo, and currently Hangul is assumed not to participate
//!     in prefix or contraction matching.

mod comparison;
mod elements;
pub mod error;
mod options;
pub mod provider;

extern crate alloc;

pub use comparison::Collator;
pub use options::AlternateHandling;
pub use options::CaseFirst;
pub use options::CollatorOptions;
pub use options::MaxVariable;
pub use options::Strength;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use super::*;
    use atoi::FromRadix16;
    use icu_locid::{langid, Locale};

    type StackString = arraystring::ArrayString<arraystring::typenum::U32>;

    /// Parse a string of space-separated hexadecimal code points (ending in end of input or semicolon)
    fn parse_hex(mut hexes: &[u8]) -> Option<StackString> {
        let mut buf = StackString::new();
        loop {
            let (scalar, mut offset) = u32::from_radix_16(hexes);
            if let Some(c) = core::char::from_u32(scalar) {
                buf.try_push(c).unwrap();
            } else {
                return None;
            }
            if offset == hexes.len() {
                return Some(buf);
            }
            match hexes[offset] {
                // '\r' is for git on Windows touching the line ends
                b';' | b'\r' => {
                    return Some(buf);
                }
                b' ' => {
                    offset += 1;
                }
                _ => {
                    panic!("Bad format: Garbage");
                }
            }
            hexes = &hexes[offset..];
        }
    }

    #[test]
    fn test_parse_hex() {
        assert_eq!(
            &parse_hex(b"1F926 1F3FC 200D 2642 FE0F").unwrap(),
            "\u{1F926}\u{1F3FC}\u{200D}\u{2642}\u{FE0F}"
        );
        assert_eq!(
            &parse_hex(b"1F926 1F3FC 200D 2642 FE0F; whatever").unwrap(),
            "\u{1F926}\u{1F3FC}\u{200D}\u{2642}\u{FE0F}"
        );
    }

    fn check_expectations(
        collator: &Collator,
        left: &[&str],
        right: &[&str],
        expectations: &[Ordering],
    ) {
        let mut left_iter = left.iter();
        let mut right_iter = right.iter();
        let mut expect_iter = expectations.iter();
        while let (Some(left_str), Some(right_str), Some(expectation)) =
            (left_iter.next(), right_iter.next(), expect_iter.next())
        {
            assert_eq!(collator.compare(left_str, right_str), *expectation);
        }
    }

    #[test]
    fn test_basic() {
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        assert_eq!(collator.compare("ac", "äb"), Ordering::Greater);
    }

    #[test]
    fn test_implicit_unihan() {
        // Adapted from `CollationTest::TestImplicits()` in collationtest.cpp of ICU4C.
        // The radical-stroke order of the characters tested agrees with their code point
        // order.
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        assert_eq!(collator.compare("\u{4E00}", "\u{4E00}"), Ordering::Equal);
        assert_eq!(collator.compare("\u{4E00}", "\u{4E01}"), Ordering::Less);
        assert_eq!(collator.compare("\u{4E01}", "\u{4E00}"), Ordering::Greater);

        assert_eq!(collator.compare("\u{4E18}", "\u{4E42}"), Ordering::Less);
        assert_eq!(collator.compare("\u{4E94}", "\u{50F6}"), Ordering::Less);
    }

    #[test]
    fn test_currency() {
        // Adapted from `CollationCurrencyTest::currencyTest` in currcoll.cpp of ICU4C.
        // All the currency symbols, in collation order.
        let currencies = "\u{00A4}\u{00A2}\u{FFE0}\u{0024}\u{FF04}\u{FE69}\u{00A3}\u{FFE1}\u{00A5}\u{FFE5}\u{09F2}\u{09F3}\u{0E3F}\u{17DB}\u{20A0}\u{20A1}\u{20A2}\u{20A3}\u{20A4}\u{20A5}\u{20A6}\u{20A7}\u{20A9}\u{FFE6}\u{20AA}\u{20AB}\u{20AC}\u{20AD}\u{20AE}\u{20AF}";
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        // Iterating as chars and re-encoding due to
        // https://github.com/rust-lang/rust/issues/83871 being nightly-only. :-(
        let mut lower_buf = [0u8; 4];
        let mut higher_buf = [0u8; 4];
        let mut chars = currencies.chars();
        while let Some(lower) = chars.next() {
            let tail = chars.clone();
            for higher in tail {
                let lower_str = lower.encode_utf8(&mut lower_buf);
                let higher_str = higher.encode_utf8(&mut higher_buf);
                assert_eq!(collator.compare(lower_str, higher_str), Ordering::Less);
            }
        }
    }

    #[test]
    fn test_de() {
        // Adapted from `CollationGermanTest` in decoll.cpp of ICU4C.
        let left = [
            "\u{47}\u{72}\u{00F6}\u{00DF}\u{65}",
            "\u{61}\u{62}\u{63}",
            "\u{54}\u{00F6}\u{6e}\u{65}",
            "\u{54}\u{00F6}\u{6e}\u{65}",
            "\u{54}\u{00F6}\u{6e}\u{65}",
            "\u{61}\u{0308}\u{62}\u{63}",
            "\u{00E4}\u{62}\u{63}",
            "\u{00E4}\u{62}\u{63}",
            "\u{53}\u{74}\u{72}\u{61}\u{00DF}\u{65}",
            "\u{65}\u{66}\u{67}",
            "\u{00E4}\u{62}\u{63}",
            "\u{53}\u{74}\u{72}\u{61}\u{00DF}\u{65}",
        ];

        let right = [
            "\u{47}\u{72}\u{6f}\u{73}\u{73}\u{69}\u{73}\u{74}",
            "\u{61}\u{0308}\u{62}\u{63}",
            "\u{54}\u{6f}\u{6e}",
            "\u{54}\u{6f}\u{64}",
            "\u{54}\u{6f}\u{66}\u{75}",
            "\u{41}\u{0308}\u{62}\u{63}",
            "\u{61}\u{0308}\u{62}\u{63}",
            "\u{61}\u{65}\u{62}\u{63}",
            "\u{53}\u{74}\u{72}\u{61}\u{73}\u{73}\u{65}",
            "\u{65}\u{66}\u{67}",
            "\u{61}\u{65}\u{62}\u{63}",
            "\u{53}\u{74}\u{72}\u{61}\u{73}\u{73}\u{65}",
        ];

        let expect_primary = [
            Ordering::Less,
            Ordering::Equal,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Equal,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Equal,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Equal,
        ];

        let expect_tertiary = [
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Greater,
        ];

        //        let locale: Locale = langid!("de").into();
        let locale: Locale = Locale::default(); // German uses the root collation

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Primary));

        let data_provider = icu_testdata::get_provider();
        {
            let collator: Collator =
                Collator::try_new(locale.clone(), &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expect_primary);
        }

        options.set_strength(Some(Strength::Tertiary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expect_tertiary);
        }
    }

    #[test]
    fn test_en() {
        // Adapted from encoll.cpp in ICU4C
        let left = [
            "\u{0061}\u{0062}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{002D}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{0020}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{002D}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0048}\u{0065}\u{006C}\u{006C}\u{006F}",
            "\u{0041}\u{0042}\u{0043}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{002D}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{002D}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0070}\u{00EA}\u{0063}\u{0068}\u{0065}",                                            
            "\u{0070}\u{00E9}\u{0063}\u{0068}\u{00E9}",
            "\u{00C4}\u{0042}\u{0308}\u{0043}\u{0308}",
            "\u{0061}\u{0308}\u{0062}\u{0063}",
            "\u{0070}\u{00E9}\u{0063}\u{0068}\u{0065}\u{0072}",
            "\u{0072}\u{006F}\u{006C}\u{0065}\u{0073}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0041}",
            "\u{0041}",
            "\u{0061}\u{0062}",                                                                
            "\u{0074}\u{0063}\u{006F}\u{006D}\u{0070}\u{0061}\u{0072}\u{0065}\u{0070}\u{006C}\u{0061}\u{0069}\u{006E}",
            "\u{0061}\u{0062}", 
            "\u{0061}\u{0023}\u{0062}",
            "\u{0061}\u{0023}\u{0062}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0041}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{0061}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{0061}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{00E6}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{00E4}\u{0062}\u{0063}\u{0064}\u{0061}",                                            
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0063}\u{0048}\u{0063}",
            "\u{0061}\u{0308}\u{0062}\u{0063}",
            "\u{0074}\u{0068}\u{0069}\u{0302}\u{0073}",
            "\u{0070}\u{00EA}\u{0063}\u{0068}\u{0065}",
            "\u{0061}\u{0062}\u{0063}",                                                         
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{00E6}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{00E6}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0070}\u{00E9}\u{0063}\u{0068}\u{00E9}"
        ]; // 49

        let right = [
            "\u{0061}\u{0062}\u{0063}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{002D}\u{0062}\u{0069}\u{0072}\u{0064}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}",
            "\u{0068}\u{0065}\u{006C}\u{006C}\u{006F}",
            "\u{0041}\u{0042}\u{0043}",
            "\u{0041}\u{0042}\u{0043}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{0062}\u{0069}\u{0072}\u{0064}\u{0073}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{0062}\u{0069}\u{0072}\u{0064}\u{0073}",
            "\u{0062}\u{006C}\u{0061}\u{0063}\u{006B}\u{0062}\u{0069}\u{0072}\u{0064}",                             
            "\u{0070}\u{00E9}\u{0063}\u{0068}\u{00E9}",
            "\u{0070}\u{00E9}\u{0063}\u{0068}\u{0065}\u{0072}",
            "\u{00C4}\u{0042}\u{0308}\u{0043}\u{0308}",
            "\u{0041}\u{0308}\u{0062}\u{0063}",
            "\u{0070}\u{00E9}\u{0063}\u{0068}\u{0065}",
            "\u{0072}\u{006F}\u{0302}\u{006C}\u{0065}",
            "\u{0041}\u{00E1}\u{0063}\u{0064}",
            "\u{0041}\u{00E1}\u{0063}\u{0064}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0054}\u{0043}\u{006F}\u{006D}\u{0070}\u{0061}\u{0072}\u{0065}\u{0050}\u{006C}\u{0061}\u{0069}\u{006E}",
            "\u{0061}\u{0042}\u{0063}",
            "\u{0061}\u{0023}\u{0042}",
            "\u{0061}\u{0026}\u{0062}",
            "\u{0061}\u{0023}\u{0063}",
            "\u{0061}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{00C4}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{00E4}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{00C4}\u{0062}\u{0063}\u{0064}\u{0061}",
            "\u{00C4}\u{0062}\u{0063}\u{0064}\u{0061}",                                             
            "\u{0061}\u{0062}\u{0023}\u{0063}",
            "\u{0061}\u{0062}\u{0063}",
            "\u{0061}\u{0062}\u{003D}\u{0063}",
            "\u{0061}\u{0062}\u{0064}",
            "\u{00E4}\u{0062}\u{0063}",
            "\u{0061}\u{0043}\u{0048}\u{0063}",
            "\u{00E4}\u{0062}\u{0063}",
            "\u{0074}\u{0068}\u{00EE}\u{0073}",
            "\u{0070}\u{00E9}\u{0063}\u{0068}\u{00E9}",
            "\u{0061}\u{0042}\u{0043}",                                                          
            "\u{0061}\u{0062}\u{0064}",
            "\u{00E4}\u{0062}\u{0063}",
            "\u{0061}\u{00C6}\u{0063}",
            "\u{0061}\u{0042}\u{0064}",
            "\u{00E4}\u{0062}\u{0063}",
            "\u{0061}\u{00C6}\u{0063}",
            "\u{0061}\u{0042}\u{0064}",
            "\u{00E4}\u{0062}\u{0063}",          
            "\u{0070}\u{00EA}\u{0063}\u{0068}\u{0065}"
        ]; // 49

        let expectations = [
            Ordering::Less,
            Ordering::Less, /*Ordering::Greater,*/
            Ordering::Less,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less, /*Ordering::Greater,*/
            /* 10 */
            Ordering::Greater,
            Ordering::Less,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less, /* 20 */
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Greater,
            /* Test Tertiary  > 26 */
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Less, /* 30 */
            Ordering::Greater,
            Ordering::Equal,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            /* test identical > 36 */
            Ordering::Equal,
            Ordering::Equal,
            /* test primary > 38 */
            Ordering::Equal,
            Ordering::Equal, /* 40 */
            Ordering::Less,
            Ordering::Equal,
            Ordering::Equal,
            /* test secondary > 43 */
            Ordering::Less,
            Ordering::Less,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less, // 49
        ];

        //        let locale: Locale = langid!("en").into();
        let locale: Locale = Locale::default(); // English uses the root collation
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));

        {
            let collator: Collator =
                Collator::try_new(locale.clone(), &data_provider, options).unwrap();
            check_expectations(&collator, &left[..38], &right[..38], &expectations[..38]);
        }

        options.set_strength(Some(Strength::Primary));

        {
            let collator: Collator =
                Collator::try_new(locale.clone(), &data_provider, options).unwrap();
            check_expectations(
                &collator,
                &left[38..43],
                &right[38..43],
                &expectations[38..43],
            );
        }

        options.set_strength(Some(Strength::Secondary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left[43..], &right[43..], &expectations[43..]);
        }
    }

    #[test]
    fn test_en_bugs() {
        // Adapted from encoll.cpp in ICU4C
        let bugs = [
            "\u{61}",
            "\u{41}",
            "\u{65}",
            "\u{45}",
            "\u{00e9}",
            "\u{00e8}",
            "\u{00ea}",
            "\u{00eb}",
            "\u{65}\u{61}",
            "\u{78}",
        ];
        //        let locale: Locale = langid!("en").into();
        let locale: Locale = Locale::default(); // English uses the root collation
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            let mut outer = bugs.iter();
            while let Some(left) = outer.next() {
                let inner = outer.clone();
                for right in inner {
                    assert_eq!(collator.compare(left, right), Ordering::Less);
                }
            }
        }
    }

    #[test]
    fn test_ja_tertiary() {
        // Adapted from `CollationKanaTest::TestTertiary` in jacoll.cpp in ICU4C
        let left = [
            "\u{FF9E}",
            "\u{3042}",
            "\u{30A2}",
            "\u{3042}\u{3042}",
            "\u{30A2}\u{30FC}",
            "\u{30A2}\u{30FC}\u{30C8}",
        ];
        let right = [
            "\u{FF9F}",
            "\u{30A2}",
            "\u{3042}\u{3042}",
            "\u{30A2}\u{30FC}",
            "\u{30A2}\u{30FC}\u{30C8}",
            "\u{3042}\u{3042}\u{3068}",
        ];
        let expectations = [
            Ordering::Less,
            Ordering::Equal, // Katakanas and Hiraganas are equal on tertiary level
            Ordering::Less,
            Ordering::Greater, // Prolonged sound mark sorts BEFORE equivalent vowel
            Ordering::Less,
            Ordering::Less, // Prolonged sound mark sorts BEFORE equivalent vowel
        ];
        let locale: Locale = langid!("ja").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));
        options.set_case_level(Some(true));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    #[test]
    fn test_ja_base() {
        // Adapted from `CollationKanaTest::TestBase` in jacoll.cpp of ICU4C.
        let cases = [
            "\u{30AB}",
            "\u{30AB}\u{30AD}",
            "\u{30AD}",
            "\u{30AD}\u{30AD}",
        ];

        let locale: Locale = langid!("ja").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Primary));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        let mut case_iter = cases.iter();
        while let Some(lower) = case_iter.next() {
            let tail = case_iter.clone();
            for higher in tail {
                assert_eq!(collator.compare(lower, higher), Ordering::Less);
            }
        }
    }

    #[test]
    fn test_ja_plain_dakuten_handakuten() {
        // Adapted from `CollationKanaTest::TestPlainDakutenHandakuten` in jacoll.cpp of ICU4C.
        let cases = [
            "\u{30CF}\u{30AB}",
            "\u{30D0}\u{30AB}",
            "\u{30CF}\u{30AD}",
            "\u{30D0}\u{30AD}",
        ];

        let locale: Locale = langid!("ja").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Secondary));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        let mut case_iter = cases.iter();
        while let Some(lower) = case_iter.next() {
            let tail = case_iter.clone();
            for higher in tail {
                assert_eq!(collator.compare(lower, higher), Ordering::Less);
            }
        }
    }

    #[test]
    fn test_ja_small_large() {
        // Adapted from `CollationKanaTest::TestSmallLarge` in jacoll.cpp of ICU4C.
        let cases = [
            "\u{30C3}\u{30CF}",
            "\u{30C4}\u{30CF}",
            "\u{30C3}\u{30D0}",
            "\u{30C4}\u{30D0}",
        ];

        let locale: Locale = langid!("ja").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));
        options.set_case_level(Some(true));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        let mut case_iter = cases.iter();
        while let Some(lower) = case_iter.next() {
            let tail = case_iter.clone();
            for higher in tail {
                assert_eq!(collator.compare(lower, higher), Ordering::Less);
            }
        }
    }

    #[test]
    fn test_ja_hiragana_katakana() {
        // Adapted from `CollationKanaTest::TestKatakanaHiragana` in jacoll.cpp of ICU4C.
        let cases = [
            "\u{3042}\u{30C3}",
            "\u{30A2}\u{30C3}",
            "\u{3042}\u{30C4}",
            "\u{30A2}\u{30C4}",
        ];

        let locale: Locale = langid!("ja").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));
        options.set_case_level(Some(true));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        let mut case_iter = cases.iter();
        while let Some(lower) = case_iter.next() {
            let tail = case_iter.clone();
            for higher in tail {
                assert_eq!(collator.compare(lower, higher), Ordering::Less);
            }
        }
    }

    #[test]
    fn test_ja_hiragana_katakana_utf16() {
        // Adapted from `CollationKanaTest::TestKatakanaHiragana` in jacoll.cpp of ICU4C.
        let cases = [
            &[0x3042u16, 0x30C3u16],
            &[0x30A2u16, 0x30C3u16],
            &[0x3042u16, 0x30C4u16],
            &[0x30A2u16, 0x30C4u16],
        ];

        let locale: Locale = langid!("ja").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));
        options.set_case_level(Some(true));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        let mut case_iter = cases.iter();
        while let Some(lower) = case_iter.next() {
            let tail = case_iter.clone();
            for higher in tail {
                assert_eq!(
                    collator.compare_utf16(&lower[..], &higher[..]),
                    Ordering::Less
                );
            }
        }
    }

    #[test]
    fn test_ja_chooon_kigoo() {
        // Adapted from `CollationKanaTest::TestChooonKigoo` in jacoll.cpp of ICU4C.
        let cases = [
            "\u{30AB}\u{30FC}\u{3042}",
            "\u{30AB}\u{30FC}\u{30A2}",
            "\u{30AB}\u{30A4}\u{3042}",
            "\u{30AB}\u{30A4}\u{30A2}",
            "\u{30AD}\u{30FC}\u{3042}", /* Prolonged sound mark sorts BEFORE equivalent vowel (ICU 2.0)*/
            "\u{30AD}\u{30FC}\u{30A2}", /* Prolonged sound mark sorts BEFORE equivalent vowel (ICU 2.0)*/
            "\u{30AD}\u{30A4}\u{3042}",
            "\u{30AD}\u{30A4}\u{30A2}",
        ];

        let locale: Locale = langid!("ja").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));
        options.set_case_level(Some(true));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        let mut case_iter = cases.iter();
        while let Some(lower) = case_iter.next() {
            let tail = case_iter.clone();
            for higher in tail {
                assert_eq!(collator.compare(lower, higher), Ordering::Less);
            }
        }
    }

    // #[test]
    // fn test_fi() {
    //     // Adapted from ficoll.cpp in ICU4C
    //     // Testing that w and v behave as in the root collation is for checking
    //     // that the sorting collation doesn't exhibit the behavior of the search
    //     // collation, which (somewhat questionably) treats w and v as primary-equal.
    //     let left = [
    //         "wat",
    //         "vat",
    //         "a\u{FC}beck",
    //         "L\u{E5}vi",
    //         // ICU4C has a duplicate of the first case below.
    //         // The duplicate is omitted here.
    //         // Instead, the subsequent tests are added for ICU4X.
    //         "\u{E4}",
    //         "a\u{0308}",
    //     ];
    //     let right = [
    //         "vat",
    //         "way",
    //         "axbeck",
    //         "L\u{E4}we",
    //         // ICU4C has a duplicate of the first case below.
    //         // The duplicate is omitted here.
    //         // Instead, the subsequent tests are added for ICU4X.
    //         "o",
    //         "\u{E4}",
    //     ];
    //     let expectations = [
    //         Ordering::Greater,
    //         Ordering::Less,
    //         Ordering::Greater,
    //         Ordering::Less,
    //         Ordering::Greater,
    //         Ordering::Equal,
    //     ];
    //     let locale: Locale = langid!("fi").into();
    //     let data_provider = icu_testdata::get_provider();

    //     let mut options = CollatorOptions::new();
    //     options.set_strength(Some(Strength::Tertiary));

    //     {
    //         let collator: Collator =
    //             Collator::try_new(locale.clone(), &data_provider, options).unwrap();
    //         check_expectations(&collator, &left, &right, &expectations);
    //     }

    //     options.set_strength(Some(Strength::Primary));

    //     {
    //         let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
    //         check_expectations(&collator, &left, &right, &expectations);
    //     }
    // }

    // #[test]
    // fn test_sv() {
    //     // This is the same as test_fi. The purpose of this copy is to test that
    //     // Swedish defaults to "reformed", which behaves like Finnish "standard",
    //     // and not to "standard", which behaves like Finnash "traditional".

    //     // Adapted from ficoll.cpp in ICU4C
    //     // Testing that w and v behave as in the root collation is for checking
    //     // that the sorting collation doesn't exhibit the behavior of the search
    //     // collation, which (somewhat questionably) treats w and v as primary-equal.
    //     let left = [
    //         "wat",
    //         "vat",
    //         "a\u{FC}beck",
    //         "L\u{E5}vi",
    //         // ICU4C has a duplicate of the first case below.
    //         // The duplicate is omitted here.
    //         // Instead, the subsequent tests are added for ICU4X.
    //         "\u{E4}",
    //         "a\u{0308}",
    //     ];
    //     let right = [
    //         "vat",
    //         "way",
    //         "axbeck",
    //         "L\u{E4}we",
    //         // ICU4C has a duplicate of the first case below.
    //         // The duplicate is omitted here.
    //         // Instead, the subsequent tests are added for ICU4X.
    //         "o",
    //         "\u{E4}",
    //     ];
    //     let expectations = [
    //         Ordering::Greater,
    //         Ordering::Less,
    //         Ordering::Greater,
    //         Ordering::Less,
    //         Ordering::Greater,
    //         Ordering::Equal,
    //     ];
    //     let locale: Locale = langid!("sv").into();
    //     let data_provider = icu_testdata::get_provider();

    //     let mut options = CollatorOptions::new();
    //     options.set_strength(Some(Strength::Tertiary));

    //     {
    //         let collator: Collator =
    //             Collator::try_new(locale.clone(), &data_provider, options).unwrap();
    //         check_expectations(&collator, &left, &right, &expectations);
    //     }

    //     options.set_strength(Some(Strength::Primary));

    //     {
    //         let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
    //         check_expectations(&collator, &left, &right, &expectations);
    //     }
    // }

    // // TODO: This test should eventually test fallback
    // // TODO: Test Swedish and Chinese also, since they have unusual
    // // variant defaults. (But are currently not part of the test data.)
    // #[test]
    // fn test_region_fallback() {
    //     // There's no explicit fi-FI data.
    //     let locale: Locale = "fi-u-co-standard".parse().unwrap();

    //     // let locale: Locale = langid!("fi-FI").into();

    //     let data_provider = icu_testdata::get_provider();

    //     let collator: Collator =
    //         Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //     assert_eq!(collator.compare("ä", "z"), Ordering::Greater);
    // }

    #[test]
    fn test_reordering() {
        let locale: Locale = langid!("bn").into();
        let data_provider = icu_testdata::get_provider();

        {
            let collator: Collator =
                Collator::try_new(Locale::default(), &data_provider, CollatorOptions::new())
                    .unwrap();
            assert_eq!(collator.compare("অ", "a"), Ordering::Greater);
            assert_eq!(collator.compare("ऄ", "a"), Ordering::Greater);
            assert_eq!(collator.compare("অ", "ऄ"), Ordering::Greater);
        }

        {
            let collator: Collator =
                Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
            assert_eq!(collator.compare("অ", "a"), Ordering::Less);
            assert_eq!(collator.compare("ऄ", "a"), Ordering::Less);
            assert_eq!(collator.compare("অ", "ऄ"), Ordering::Less);
        }
    }

    // #[test]
    // fn test_zh() {
    //     let data_provider = icu_testdata::get_provider();

    //     // Note: ㄅ is Bopomofo.

    //     {
    //         let collator: Collator =
    //             Collator::try_new(Locale::default(), &data_provider, CollatorOptions::new())
    //                 .unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Greater);
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Greater);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Greater);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Less);
    //     }
    //     {
    //         let locale: Locale = langid!("zh").into(); // Defaults to -u-co-pinyin
    //         let collator: Collator =
    //             Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    //     }
    //     {
    //         let locale: Locale = "zh-u-co-pinyin".parse().unwrap();
    //         let collator: Collator =
    //             Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    //     }
    //     {
    //         let locale: Locale = "zh-u-co-gb2312".parse().unwrap();
    //         let collator: Collator =
    //             Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
    //         // In GB2312 proper, Bopomofo comes before Han, but the
    //         // collation leaves Bopomofo unreordered, so it comes after.
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    //     }
    //     {
    //         let locale: Locale = "zh-u-co-stroke".parse().unwrap();
    //         let collator: Collator =
    //             Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Less);
    //     }
    //     {
    //         let locale: Locale = "zh-u-co-zhuyin".parse().unwrap();
    //         let collator: Collator =
    //             Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Greater);
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    //     }
    //     {
    //         let locale: Locale = "zh-u-co-unihan".parse().unwrap();
    //         let collator: Collator =
    //             Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Less);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Greater);
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Less);
    //     }
    //     {
    //         let locale: Locale = "zh-u-co-big5han".parse().unwrap();
    //         let collator: Collator =
    //             Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
    //         assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
    //         assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
    //         assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
    //         assert_eq!(collator.compare("不", "把"), Ordering::Less);
    //     }
    //     // TODO: Test script and region aliases
    // }

    // TODO: frcoll requires support for fr-CA

    // TODO: Write a test for Bangla

    #[test]
    fn test_es_tertiary() {
        // Adapted from `CollationSpanishTest::TestTertiary` in escoll.cpp in ICU4C
        let left = [
            "\u{61}\u{6c}\u{69}\u{61}\u{73}",
            "\u{45}\u{6c}\u{6c}\u{69}\u{6f}\u{74}",
            "\u{48}\u{65}\u{6c}\u{6c}\u{6f}",
            "\u{61}\u{63}\u{48}\u{63}",
            "\u{61}\u{63}\u{63}",
        ];
        let right = [
            "\u{61}\u{6c}\u{6c}\u{69}\u{61}\u{73}",
            "\u{45}\u{6d}\u{69}\u{6f}\u{74}",
            "\u{68}\u{65}\u{6c}\u{6c}\u{4f}",
            "\u{61}\u{43}\u{48}\u{63}",
            "\u{61}\u{43}\u{48}\u{63}",
        ];
        let expectations = [
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Less,
        ];
        let locale: Locale = langid!("es").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    #[test]
    fn test_es_primary() {
        // Adapted from `CollationSpanishTest::TestPrimary` in escoll.cpp in ICU4C
        let left = [
            "\u{61}\u{6c}\u{69}\u{61}\u{73}",
            "\u{61}\u{63}\u{48}\u{63}",
            "\u{61}\u{63}\u{63}",
            "\u{48}\u{65}\u{6c}\u{6c}\u{6f}",
        ];
        let right = [
            "\u{61}\u{6c}\u{6c}\u{69}\u{61}\u{73}",
            "\u{61}\u{43}\u{48}\u{63}",
            "\u{61}\u{43}\u{48}\u{63}",
            "\u{68}\u{65}\u{6c}\u{6c}\u{4f}",
        ];
        let expectations = [
            Ordering::Less,
            Ordering::Equal,
            Ordering::Less,
            Ordering::Equal,
        ];
        let locale: Locale = langid!("es").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Primary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    #[test]
    fn test_el_secondary() {
        // Adapted from `CollationRegressionTest::Test4095316` in regcoll.cpp of ICU4C.
        let locale: Locale = Locale::default(); // Greek uses the root collation
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Secondary));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        assert_eq!(collator.compare("\u{03D4}", "\u{03AB}"), Ordering::Equal);
    }

    #[test]
    fn test_th_dictionary() {
        // Adapted from `CollationThaiTest::TestDictionary` of thcoll.cpp in ICU4C.
        let dict = include_str!("../testdata/riwords.txt")
            .strip_prefix('\u{FEFF}')
            .unwrap();
        let locale: Locale = langid!("th").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));

        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        let mut lines = dict.lines();
        let mut prev = loop {
            if let Some(line) = lines.next() {
                if line.starts_with('#') {
                    continue;
                }
                break line;
            } else {
                panic!("Malformed dictionary");
            }
        };

        for line in lines {
            assert_eq!(collator.compare(prev, line), Ordering::Less);
            prev = line;
        }
    }

    #[test]
    fn test_th_corner_cases() {
        // Adapted from `CollationThaiTest::TestCornerCases` in thcoll.cpp in ICU4C
        let left = [
            // Shorter words precede longer
            "\u{0E01}",
            // Tone marks are considered after letters (i.e. are primary ignorable)
            "\u{0E01}\u{0E32}",
            // ditto for other over-marks
            "\u{0E01}\u{0E32}",
            // commonly used mark-in-context order.
            // In effect, marks are sorted after each syllable.
            "\u{0E01}\u{0E32}\u{0E01}\u{0E49}\u{0E32}",
            // Hyphens and other punctuation follow whitespace but come before letters
            //            "\u{0E01}\u{0E32}",
            "\u{0E01}\u{0E32}-",
            // Doubler follows an identical word without the doubler
            // "\u{0E01}\u{0E32}",
            "\u{0E01}\u{0E32}\u{0E46}",
            // \u{0E45} after either \u{0E24} or \{u0E26} is treated as a single
            // combining character, similar to "c < ch" in traditional spanish.
            "\u{0E24}\u{0E29}\u{0E35}",
            "\u{0E26}\u{0E29}\u{0E35}",
            // Vowels reorder, should compare \u{0E2D} and \u{0E34}
            // XXX Should the middle code point differ between the two strings?
            "\u{0E40}\u{0E01}\u{0E2D}",
            // Tones are compared after the rest of the word (e.g. primary ignorable)
            "\u{0E01}\u{0E32}\u{0E01}\u{0E48}\u{0E32}",
            // Periods are ignored entirely
            "\u{0E01}.\u{0E01}.",
        ];
        let right = [
            "\u{0E01}\u{0E01}",
            "\u{0E01}\u{0E49}\u{0E32}",
            "\u{0E01}\u{0E32}\u{0E4C}",
            "\u{0E01}\u{0E48}\u{0E32}\u{0E01}\u{0E49}\u{0E32}",
            //            "\u{0E01}\u{0E32}-",
            "\u{0E01}\u{0E32}\u{0E01}\u{0E32}",
            // "\u{0E01}\u{0E32}\u{0E46}",
            "\u{0E01}\u{0E32}\u{0E01}\u{0E32}",
            "\u{0E24}\u{0E45}\u{0E29}\u{0E35}",
            "\u{0E26}\u{0E45}\u{0E29}\u{0E35}",
            "\u{0E40}\u{0E01}\u{0E34}",
            "\u{0E01}\u{0E49}\u{0E32}\u{0E01}\u{0E32}",
            "\u{0E01}\u{0E32}",
        ];
        let expectations = [
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            //            Ordering::Equal,
            Ordering::Less,
            // Ordering::Equal,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
        ];
        let locale: Locale = langid!("th").into();
        let data_provider = icu_testdata::get_provider();
        {
            // XXX TODO: Check why the commented-out cases fail
            let collator: Collator =
                Collator::try_new(locale, &data_provider, CollatorOptions::new()).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    #[test]
    fn test_th_reordering() {
        // Adapted from `CollationThaiTest::TestReordering` in thcoll.cpp in ICU4C
        let left = [
            // composition
            "\u{0E41}c\u{0301}",
            // supplementaries
            // XXX Why does this fail?
            // "\u{0E41}\u{1D7CE}",
            // supplementary composition decomps to supplementary
            "\u{0E41}\u{1D15F}",
            // supplementary composition decomps to BMP
            "\u{0E41}\u{2F802}", // omit bacward iteration tests
                                 // contraction bug
                                 // "\u{0E24}\u{0E41}",
                                 // TODO: Support contracting starters, then add more here
        ];
        let right = [
            "\u{0E41}\u{0107}",
            // "\u{0E41}\u{1D7CF}",
            "\u{0E41}\u{1D158}\u{1D165}",
            "\u{0E41}\u{4E41}", // "\u{0E41}\u{0E24}",
        ];
        let expectations = [
            Ordering::Equal,
            // Ordering::Less,
            Ordering::Equal,
            Ordering::Equal,
            // Ordering::Equal,
        ];
        let locale: Locale = langid!("th").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Secondary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    #[test]
    fn test_tr_tertiary() {
        // Adapted from `CollationTurkishTest::TestTertiary` in trcoll.cpp in ICU4C
        let left = [
            "\u{73}\u{0327}",
            "\u{76}\u{00E4}\u{74}",
            "\u{6f}\u{6c}\u{64}",
            "\u{00FC}\u{6f}\u{69}\u{64}",
            "\u{68}\u{011E}\u{61}\u{6c}\u{74}",
            "\u{73}\u{74}\u{72}\u{65}\u{73}\u{015E}",
            "\u{76}\u{6f}\u{0131}\u{64}",
            "\u{69}\u{64}\u{65}\u{61}",
        ];
        let right = [
            "\u{75}\u{0308}",
            "\u{76}\u{62}\u{74}",
            "\u{00D6}\u{61}\u{79}",
            "\u{76}\u{6f}\u{69}\u{64}",
            "\u{68}\u{61}\u{6c}\u{74}",
            "\u{015E}\u{74}\u{72}\u{65}\u{015E}\u{73}",
            "\u{76}\u{6f}\u{69}\u{64}",
            "\u{49}\u{64}\u{65}\u{61}",
        ];
        let expectations = [
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
        ];
        let locale: Locale = langid!("tr").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    #[test]
    fn test_tr_primary() {
        // Adapted from `CollationTurkishTest::TestPrimary` in trcoll.cpp in ICU4C
        let left = [
            "\u{00FC}\u{6f}\u{69}\u{64}",
            "\u{76}\u{6f}\u{0131}\u{64}",
            "\u{69}\u{64}\u{65}\u{61}",
        ];
        let right = [
            "\u{76}\u{6f}\u{69}\u{64}",
            "\u{76}\u{6f}\u{69}\u{64}",
            "\u{49}\u{64}\u{65}\u{61}",
        ];
        let expectations = [Ordering::Less, Ordering::Less, Ordering::Greater];
        let locale: Locale = langid!("tr").into();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    // #[test]
    // fn test_lt_tertiary() {
    //     let left = [
    //         "a\u{0307}\u{0300}a",
    //         "a\u{0307}\u{0301}a",
    //         "a\u{0307}\u{0302}a",
    //         "a\u{0307}\u{0303}a",
    //         "ž",
    //     ];
    //     let right = [
    //         "a\u{0300}a",
    //         "a\u{0301}a",
    //         "a\u{0302}a",
    //         "a\u{0303}a",
    //         "z\u{033F}",
    //     ];
    //     let expectations = [
    //         Ordering::Equal,
    //         Ordering::Equal,
    //         Ordering::Greater,
    //         Ordering::Equal,
    //         Ordering::Greater,
    //     ];
    //     let locale: Locale = langid!("lt").into();
    //     let data_provider = icu_testdata::get_provider();

    //     let mut options = CollatorOptions::new();
    //     options.set_strength(Some(Strength::Tertiary));

    //     {
    //         let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
    //         check_expectations(&collator, &left, &right, &expectations);
    //     }
    // }

    // #[test]
    // fn test_lt_primary() {
    //     let left = ["ž"];
    //     let right = ["z"];
    //     let expectations = [Ordering::Greater];
    //     let locale: Locale = langid!("lt").into();
    //     let data_provider = icu_testdata::get_provider();

    //     let mut options = CollatorOptions::new();
    //     options.set_strength(Some(Strength::Primary));

    //     {
    //         let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
    //         check_expectations(&collator, &left, &right, &expectations);
    //     }
    // }

    #[test]
    fn test_basics() {
        // Adapted from `CollationAPITest::TestProperty` in apicoll.cpp in ICU4C
        let left = ["ab", "ab", "blackbird", "black bird", "Hello", "", "abä"];
        let right = ["abc", "AB", "black-bird", "black-bird", "hello", "", "abß"];
        let expectations = [
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Equal,
        ];
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Tertiary));

        {
            let collator: Collator =
                Collator::try_new(Locale::default(), &data_provider, options).unwrap();
            check_expectations(&collator, &left, &right, &expectations);
        }
    }

    #[test]
    fn test_numeric_off() {
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_numeric(Some(false));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        assert_eq!(collator.compare("a10b", "a2b"), Ordering::Less);
    }

    #[test]
    fn test_numeric_on() {
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_numeric(Some(true));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        assert_eq!(collator.compare("a10b", "a2b"), Ordering::Greater);
    }

    #[test]
    fn test_numeric_long() {
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_numeric(Some(true));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        let mut left = String::new();
        let mut right = String::new();
        // We'll make left larger than right numerically. However, first, let's use
        // a leading zero to make left look less than right non-numerically.
        left.push('0');
        left.push('1');
        right.push('1');
        // Now, let's make sure we end up with segments longer than
        // 254 digits
        for _ in 0..256 {
            left.push('2');
            right.push('2');
        }
        // Now the difference:
        left.push('4');
        right.push('3');
        // Again making segments long
        for _ in 0..256 {
            left.push('5');
            right.push('5');
        }
        // And some trailing zeros
        for _ in 0..7 {
            left.push('0');
            right.push('0');
        }
        assert_eq!(collator.compare(&left, &right), Ordering::Greater);
    }

    #[test]
    fn test_numeric_after() {
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_numeric(Some(true));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        assert_eq!(collator.compare("0001000b", "1000a"), Ordering::Greater);
    }

    #[test]
    fn test_unpaired_surrogates() {
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        assert_eq!(
            collator.compare_utf16(&[0xD801u16], &[0xD802u16]),
            Ordering::Equal
        );
    }

    #[test]
    fn test_backward_second_level() {
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Secondary));

        {
            let collator: Collator =
                Collator::try_new(Locale::default(), &data_provider, options).unwrap();

            let cases = ["cote", "coté", "côte", "côté"];
            let mut case_iter = cases.iter();
            while let Some(lower) = case_iter.next() {
                let tail = case_iter.clone();
                for higher in tail {
                    assert_eq!(collator.compare(lower, higher), Ordering::Less);
                }
            }
        }

        options.set_backward_second_level(Some(true));

        {
            let collator: Collator =
                Collator::try_new(Locale::default(), &data_provider, options).unwrap();

            {
                let cases = ["cote", "côte", "coté", "côté"];
                let mut case_iter = cases.iter();
                while let Some(lower) = case_iter.next() {
                    let tail = case_iter.clone();
                    for higher in tail {
                        assert_eq!(collator.compare(lower, higher), Ordering::Less);
                    }
                }
            }
            {
                let cases = ["cote\u{FFFE}coté", "côte\u{FFFE}cote"];
                let mut case_iter = cases.iter();
                while let Some(lower) = case_iter.next() {
                    let tail = case_iter.clone();
                    for higher in tail {
                        assert_eq!(collator.compare(lower, higher), Ordering::Less);
                    }
                }
            }
        }
    }

    #[test]
    fn test_cantillation() {
        let locale: Locale = Locale::default();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));

        {
            let collator: Collator =
                Collator::try_new(locale.clone(), &data_provider, options).unwrap();
            assert_eq!(
                collator.compare(
                    "\u{05D3}\u{05D7}\u{05D9}\u{05AD}",
                    "\u{05D3}\u{05D7}\u{05D9}"
                ),
                Ordering::Equal
            );
        }

        options.set_strength(Some(Strength::Identical));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            assert_eq!(
                collator.compare(
                    "\u{05D3}\u{05D7}\u{05D9}\u{05AD}",
                    "\u{05D3}\u{05D7}\u{05D9}"
                ),
                Ordering::Greater
            );
        }
    }

    #[test]
    fn test_cantillation_utf8() {
        let locale: Locale = Locale::default();
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));

        {
            let collator: Collator =
                Collator::try_new(locale.clone(), &data_provider, options).unwrap();
            assert_eq!(
                collator.compare_utf8(
                    "\u{05D3}\u{05D7}\u{05D9}\u{05AD}".as_bytes(),
                    "\u{05D3}\u{05D7}\u{05D9}".as_bytes()
                ),
                Ordering::Equal
            );
        }

        options.set_strength(Some(Strength::Identical));

        {
            let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
            assert_eq!(
                collator.compare(
                    "\u{05D3}\u{05D7}\u{05D9}\u{05AD}",
                    "\u{05D3}\u{05D7}\u{05D9}"
                ),
                Ordering::Greater
            );
        }
    }

    #[test]
    fn test_conformance_shifted() {
        // Adapted from `UCAConformanceTest::TestTableShifted` of ucaconf.cpp in ICU4C.
        let bugs = [];
        let dict = include_bytes!("../testdata/CollationTest_CLDR_SHIFTED.txt");
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));
        options.set_alternate_handling(Some(AlternateHandling::Shifted));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        let mut lines = dict.split(|b| b == &b'\n');
        let mut prev = loop {
            if let Some(line) = lines.next() {
                if line.is_empty() {
                    continue;
                }
                if line.starts_with(&[b'#']) {
                    continue;
                }
                if let Some(parsed) = parse_hex(line) {
                    break parsed;
                }
            } else {
                panic!("Malformed dictionary");
            }
        };

        for line in lines {
            if line.is_empty() {
                continue;
            }
            if let Some(parsed) = parse_hex(line) {
                if !bugs.contains(&parsed.as_str())
                    && collator.compare(&prev, &parsed) == Ordering::Greater
                {
                    assert_eq!(&prev[..], &parsed[..]);
                }
                prev = parsed;
            }
        }
    }

    #[test]
    fn test_conformance_non_ignorable() {
        // Adapted from `UCAConformanceTest::TestTableNonIgnorable` of ucaconf.cpp in ICU4C.
        let bugs = [];
        let dict = include_bytes!("../testdata/CollationTest_CLDR_NON_IGNORABLE.txt");
        let data_provider = icu_testdata::get_provider();

        let mut options = CollatorOptions::new();
        options.set_strength(Some(Strength::Quaternary));
        options.set_alternate_handling(Some(AlternateHandling::NonIgnorable));

        let collator: Collator =
            Collator::try_new(Locale::default(), &data_provider, options).unwrap();
        let mut lines = dict.split(|b| b == &b'\n');
        let mut prev = loop {
            if let Some(line) = lines.next() {
                if line.is_empty() {
                    continue;
                }
                if line.starts_with(&[b'#']) {
                    continue;
                }
                if let Some(parsed) = parse_hex(line) {
                    break parsed;
                }
            } else {
                panic!("Malformed dictionary");
            }
        };

        for line in lines {
            if line.is_empty() {
                continue;
            }
            if let Some(parsed) = parse_hex(line) {
                if !bugs.contains(&parsed.as_str())
                    && collator.compare(&prev, &parsed) == Ordering::Greater
                {
                    assert_eq!(&prev[..], &parsed[..]);
                }
                prev = parsed;
            }
        }
    }
}

// TODO: Test languages that map to the root.
// The languages that map to root without script reordering are:
// ca (at least for now)
// de
// en
// fr
// ga
// id
// it
// lb
// ms
// nl
// pt
// sw
// xh
// zu
//
// A bit unclear: ff in Latin script?
//
// These are root plus script reordering:
// am
// be
// bg
// chr
// el
// ka
// lo
// mn
// ne
// ru

// TODO: Test imports. These aren't aliases but should get deduplicated
// in the provider:
// bo: dz-u-co-standard (draft: unconfirmed)
// bs: hr
// bs-Cyrl: sr
// fa-AF: ps
// sr-Latn: hr

// TODO: Test that nn and nb are aliases for no

// TODO: Consider testing ff-Adlm for supplementary-plane tailoring, including contractions

// TODO: Test Tibetan

// TODO: Test de-AT-u-co-phonebk vs de-DE-u-co-phonebk

// TODO: Test da defaulting to [caseFirst upper]
// TODO: Test fr-CA defaulting to backward second level
