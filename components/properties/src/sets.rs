// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The functions in this module return a [`UnicodeSet`] containing
//! the set of characters with a particular Unicode property.
//!
//! The descriptions of most properties are taken from [`TR44`], the documentation for the
//! Unicode Character Database.  Some properties are instead defined in [`TR18`], the
//! documentation for Unicode regular expressions. In particular, Annex C of this document
//! defines properties for POSIX compatibility.
//!
//! [`UnicodeSet`]: icu_uniset::UnicodeSet
//! [`TR44`]: https://www.unicode.org/reports/tr44
//! [`TR18`]: https://www.unicode.org/reports/tr18

use crate::error::PropertiesError;
use crate::provider::*;
use crate::*;
use core::iter::FromIterator;
use core::ops::RangeInclusive;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSet;

/// TODO(#1239): Finalize this API.
pub type UnisetResult = Result<DataPayload<UnicodePropertyV1Marker>, PropertiesError>;

// helper fn
fn get_uniset<D>(provider: &D, resc_key: ResourceKey) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    let resp: DataResponse<UnicodePropertyV1Marker> =
        provider.load_payload(resc_key, &Default::default())?;

    let property_payload: DataPayload<UnicodePropertyV1Marker> = resp.take_payload()?;
    Ok(property_payload)
}

//
// Binary property getter fns
//

/// ASCII characters commonly used for the representation of hexadecimal numbers
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_ascii_hex_digit(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let ascii_hex_digit = &data_struct.inv_list;
///
/// assert!(ascii_hex_digit.contains('3'));
/// assert!(!ascii_hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
/// assert!(ascii_hex_digit.contains('A'));
/// assert!(!ascii_hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
/// ```
pub fn get_ascii_hex_digit<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ASCII_HEX_DIGIT_V1)
}

/// Characters with the Alphabetic or Decimal_Number property
/// This is defined for POSIX compatibility.
pub fn get_alnum<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ALNUM_V1)
}

/// Alphabetic characters
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_alphabetic(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let alphabetic = &data_struct.inv_list;
///
/// assert!(!alphabetic.contains('3'));
/// assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
/// assert!(alphabetic.contains('A'));
/// assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
/// ```
pub fn get_alphabetic<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ALPHABETIC_V1)
}

/// Format control characters which have specific functions in the Unicode Bidirectional
/// Algorithm
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_bidi_control(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let bidi_control = &data_struct.inv_list;
///
/// assert!(bidi_control.contains_u32(0x200F));  // RIGHT-TO-LEFT MARK
/// assert!(!bidi_control.contains('ش'));  // U+0634 ARABIC LETTER SHEEN
/// ```
pub fn get_bidi_control<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::BIDI_CONTROL_V1)
}

/// Characters that are mirrored in bidirectional text
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_bidi_mirrored(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let bidi_mirrored = &data_struct.inv_list;
///
/// assert!(bidi_mirrored.contains('['));
/// assert!(bidi_mirrored.contains(']'));
/// assert!(bidi_mirrored.contains('∑'));  // U+2211 N-ARY SUMMATION
/// assert!(!bidi_mirrored.contains('ཉ'));  // U+0F49 TIBETAN LETTER NYA
/// ```
pub fn get_bidi_mirrored<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::BIDI_MIRRORED_V1)
}

/// Horizontal whitespace characters
pub fn get_blank<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::BLANK_V1)
}

/// Uppercase, lowercase, and titlecase characters
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_cased(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let cased = &data_struct.inv_list;
///
/// assert!(cased.contains('Ꙡ'));  // U+A660 CYRILLIC CAPITAL LETTER REVERSED TSE
/// assert!(!cased.contains('ދ'));  // U+078B THAANA LETTER DHAALU
/// ```
pub fn get_cased<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CASED_V1)
}

/// Characters which are ignored for casing purposes
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_case_ignorable(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let case_ignorable = &data_struct.inv_list;
///
/// assert!(case_ignorable.contains(':'));
/// assert!(!case_ignorable.contains('λ'));  // U+03BB GREEK SMALL LETTER LAMDA
/// ```
pub fn get_case_ignorable<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CASE_IGNORABLE_V1)
}

/// Characters that are excluded from composition
/// See <https://unicode.org/Public/UNIDATA/CompositionExclusions.txt>
pub fn get_full_composition_exclusion<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::FULL_COMPOSITION_EXCLUSION_V1)
}

/// Characters whose normalized forms are not stable under case folding
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_changes_when_casefolded(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let changes_when_casefolded = &data_struct.inv_list;
///
/// assert!(changes_when_casefolded.contains('ß'));  // U+00DF LATIN SMALL LETTER SHARP S
/// assert!(!changes_when_casefolded.contains('ᜉ'));  // U+1709 TAGALOG LETTER PA
/// ```
pub fn get_changes_when_casefolded<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_CASEFOLDED_V1)
}

/// Characters which may change when they undergo case mapping
pub fn get_changes_when_casemapped<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_CASEMAPPED_V1)
}

/// Characters which are not identical to their NFKC_Casefold mapping
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_changes_when_nfkc_casefolded(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let changes_when_nfkc_casefolded = &data_struct.inv_list;
///
/// assert!(changes_when_nfkc_casefolded.contains('🄵'));  // U+1F135 SQUARED LATIN CAPITAL LETTER F
/// assert!(!changes_when_nfkc_casefolded.contains('f'));
/// ```
pub fn get_changes_when_nfkc_casefolded<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_NFKC_CASEFOLDED_V1)
}

/// Characters whose normalized forms are not stable under a toLowercase mapping
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_changes_when_lowercased(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let changes_when_lowercased = &data_struct.inv_list;
///
/// assert!(changes_when_lowercased.contains('Ⴔ'));  // U+10B4 GEORGIAN CAPITAL LETTER PHAR
/// assert!(!changes_when_lowercased.contains('ფ'));  // U+10E4 GEORGIAN LETTER PHAR
/// ```
pub fn get_changes_when_lowercased<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_LOWERCASED_V1)
}

/// Characters whose normalized forms are not stable under a toTitlecase mapping
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_changes_when_titlecased(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let changes_when_titlecased = &data_struct.inv_list;
///
/// assert!(changes_when_titlecased.contains('æ'));  // U+00E6 LATIN SMALL LETTER AE
/// assert!(!changes_when_titlecased.contains('Æ'));  // U+00E6 LATIN CAPITAL LETTER AE
/// ```
pub fn get_changes_when_titlecased<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_TITLECASED_V1)
}

/// Characters whose normalized forms are not stable under a toUppercase mapping
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_changes_when_uppercased(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let changes_when_uppercased = &data_struct.inv_list;
///
/// assert!(changes_when_uppercased.contains('ւ'));  // U+0582 ARMENIAN SMALL LETTER YIWN
/// assert!(!changes_when_uppercased.contains('Ւ'));  // U+0552 ARMENIAN CAPITAL LETTER YIWN
/// ```
pub fn get_changes_when_uppercased<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_UPPERCASED_V1)
}

/// Punctuation characters explicitly called out as dashes in the Unicode Standard, plus
/// their compatibility equivalents
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_dash(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let dash = &data_struct.inv_list;
///
/// assert!(dash.contains('⸺'));  // U+2E3A TWO-EM DASH
/// assert!(dash.contains('-'));  // U+002D
/// assert!(!dash.contains('='));  // U+003D
/// ```
pub fn get_dash<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DASH_V1)
}

/// Deprecated characters. No characters will ever be removed from the standard, but the
/// usage of deprecated characters is strongly discouraged.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_deprecated(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let deprecated = &data_struct.inv_list;
///
/// assert!(deprecated.contains('ឣ'));  // U+17A3 KHMER INDEPENDENT VOWEL QAQ
/// assert!(!deprecated.contains('A'));
/// ```
pub fn get_deprecated<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DEPRECATED_V1)
}

/// For programmatic determination of default ignorable code points.  New characters that
/// should be ignored in rendering (unless explicitly supported) will be assigned in these
/// ranges, permitting programs to correctly handle the default rendering of such
/// characters when not otherwise supported.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_default_ignorable_code_point(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let default_ignorable_code_point = &data_struct.inv_list;
///
/// assert!(default_ignorable_code_point.contains_u32(0x180B));  // MONGOLIAN FREE VARIATION SELECTOR ONE
/// assert!(!default_ignorable_code_point.contains('E'));
/// ```
pub fn get_default_ignorable_code_point<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DEFAULT_IGNORABLE_CODE_POINT_V1)
}

/// Characters that linguistically modify the meaning of another character to which they apply
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_diacritic(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let diacritic = &data_struct.inv_list;
///
/// assert!(diacritic.contains('\u{05B3}'));  // HEBREW POINT HATAF QAMATS
/// assert!(!diacritic.contains('א'));  // U+05D0 HEBREW LETTER ALEF
/// ```
pub fn get_diacritic<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DIACRITIC_V1)
}

/// Characters that can serve as a base for emoji modifiers
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_emoji_modifier_base(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let emoji_modifier_base = &data_struct.inv_list;
///
/// assert!(emoji_modifier_base.contains('✊'));  // U+270A RAISED FIST
/// assert!(!emoji_modifier_base.contains('⛰'));  // U+26F0 MOUNTAIN
/// ```
pub fn get_emoji_modifier_base<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_MODIFIER_BASE_V1)
}

/// Characters used in emoji sequences that normally do not appear on emoji keyboards as
/// separate choices, such as base characters for emoji keycaps
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_emoji_component(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let emoji_component = &data_struct.inv_list;
///
/// assert!(emoji_component.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
/// assert!(emoji_component.contains_u32(0x20E3));  // COMBINING ENCLOSING KEYCAP
/// assert!(emoji_component.contains('7'));
/// assert!(!emoji_component.contains('T'));
/// ```
pub fn get_emoji_component<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_COMPONENT_V1)
}

/// Characters that are emoji modifiers
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_emoji_modifier(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let emoji_modifier = &data_struct.inv_list;
///
/// assert!(emoji_modifier.contains_u32(0x1F3FD));  // EMOJI MODIFIER FITZPATRICK TYPE-4
/// assert!(!emoji_modifier.contains_u32(0x200C));  // ZERO WIDTH NON-JOINER
/// ```
pub fn get_emoji_modifier<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_MODIFIER_V1)
}

/// Characters that are emoji
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_emoji(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let emoji = &data_struct.inv_list;
///
/// assert!(emoji.contains('🔥'));  // U+1F525 FIRE
/// assert!(!emoji.contains('V'));
/// ```
pub fn get_emoji<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_V1)
}

/// Characters that have emoji presentation by default
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_emoji_presentation(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let emoji_presentation = &data_struct.inv_list;
///
/// assert!(emoji_presentation.contains('🦬')); // U+1F9AC BISON
/// assert!(!emoji_presentation.contains('♻'));  // U+267B BLACK UNIVERSAL RECYCLING SYMBOL
/// ```
pub fn get_emoji_presentation<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_PRESENTATION_V1)
}

/// Characters whose principal function is to extend the value of a preceding alphabetic
/// character or to extend the shape of adjacent characters.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_extender(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let extender = &data_struct.inv_list;
///
/// assert!(extender.contains('ヾ'));  // U+30FE KATAKANA VOICED ITERATION MARK
/// assert!(extender.contains('ー'));  // U+30FC KATAKANA-HIRAGANA PROLONGED SOUND MARK
/// assert!(!extender.contains('・'));  // U+30FB KATAKANA MIDDLE DOT
/// ```
pub fn get_extender<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EXTENDER_V1)
}

/// Pictographic symbols, as well as reserved ranges in blocks largely associated with
/// emoji characters
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_extended_pictographic(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let extended_pictographic = &data_struct.inv_list;
///
/// assert!(extended_pictographic.contains('🥳')); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
/// assert!(!extended_pictographic.contains('🇪'));  // U+1F1EA REGIONAL INDICATOR SYMBOL LETTER E
/// ```
pub fn get_extended_pictographic<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EXTENDED_PICTOGRAPHIC_V1)
}

/// Visible characters.
/// This is defined for POSIX compatibility.
pub fn get_graph<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPH_V1)
}

/// Property used together with the definition of Standard Korean Syllable Block to define
/// "Grapheme base". See D58 in Chapter 3, Conformance in the Unicode Standard.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_grapheme_base(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let grapheme_base = &data_struct.inv_list;
///
/// assert!(grapheme_base.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
/// assert!(grapheme_base.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
/// assert!(!grapheme_base.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
/// ```
pub fn get_grapheme_base<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPHEME_BASE_V1)
}

/// Property used to define "Grapheme extender". See D59 in Chapter 3, Conformance in the
/// Unicode Standard.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_grapheme_extend(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let grapheme_extend = &data_struct.inv_list;
///
/// assert!(!grapheme_extend.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
/// assert!(!grapheme_extend.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
/// assert!(grapheme_extend.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
/// ```
pub fn get_grapheme_extend<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPHEME_EXTEND_V1)
}

/// Deprecated property. Formerly proposed for programmatic determination of grapheme
/// cluster boundaries.
pub fn get_grapheme_link<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPHEME_LINK_V1)
}

/// Characters commonly used for the representation of hexadecimal numbers, plus their
/// compatibility equivalents
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_hex_digit(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let hex_digit = &data_struct.inv_list;
///
/// assert!(hex_digit.contains('0'));
/// assert!(!hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
/// assert!(hex_digit.contains('f'));
/// assert!(hex_digit.contains('ｆ'));  // U+FF46 FULLWIDTH LATIN SMALL LETTER F
/// assert!(hex_digit.contains('Ｆ'));  // U+FF26 FULLWIDTH LATIN CAPITAL LETTER F
/// assert!(!hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
/// ```
pub fn get_hex_digit<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::HEX_DIGIT_V1)
}

/// Deprecated property. Dashes which are used to mark connections between pieces of
/// words, plus the Katakana middle dot.
pub fn get_hyphen<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::HYPHEN_V1)
}

/// Characters that can come after the first character in an identifier. If using NFKC to
/// fold differences between characters, use [`get_xid_continue`] instead.  See
/// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
/// more details.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_id_continue(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let id_continue = &data_struct.inv_list;
///
/// assert!(id_continue.contains('x'));
/// assert!(id_continue.contains('1'));
/// assert!(id_continue.contains('_'));
/// assert!(id_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
/// assert!(!id_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
/// assert!(id_continue.contains_u32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
/// ```
pub fn get_id_continue<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ID_CONTINUE_V1)
}

/// Characters considered to be CJKV (Chinese, Japanese, Korean, and Vietnamese)
/// ideographs, or related siniform ideographs
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_ideographic(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let ideographic = &data_struct.inv_list;
///
/// assert!(ideographic.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
/// assert!(!ideographic.contains('밥'));  // U+BC25 HANGUL SYLLABLE BAB
/// ```
pub fn get_ideographic<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::IDEOGRAPHIC_V1)
}

/// Characters that can begin an identifier. If using NFKC to fold differences between
/// characters, use [`get_xid_start`] instead.  See [`Unicode Standard Annex
/// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_id_start(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let id_start = &data_struct.inv_list;
///
/// assert!(id_start.contains('x'));
/// assert!(!id_start.contains('1'));
/// assert!(!id_start.contains('_'));
/// assert!(id_start.contains('ߝ'));  // U+07DD NKO LETTER FA
/// assert!(!id_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
/// assert!(id_start.contains_u32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
/// ```
pub fn get_id_start<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ID_START_V1)
}

/// Characters used in Ideographic Description Sequences
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_ids_binary_operator(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let ids_binary_operator = &data_struct.inv_list;
///
/// assert!(ids_binary_operator.contains_u32(0x2FF5));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
/// assert!(!ids_binary_operator.contains_u32(0x3006));  // IDEOGRAPHIC CLOSING MARK
/// ```
pub fn get_ids_binary_operator<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::IDS_BINARY_OPERATOR_V1)
}

/// Characters used in Ideographic Description Sequences
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_ids_trinary_operator(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let ids_trinary_operator = &data_struct.inv_list;
///
/// assert!(ids_trinary_operator.contains_u32(0x2FF2));  // IDEOGRAPHIC DESCRIPTION CHARACTER LEFT TO MIDDLE AND RIGHT
/// assert!(ids_trinary_operator.contains_u32(0x2FF3));  // IDEOGRAPHIC DESCRIPTION CHARACTER ABOVE TO MIDDLE AND BELOW
/// assert!(!ids_trinary_operator.contains_u32(0x2FF4));
/// assert!(!ids_trinary_operator.contains_u32(0x2FF5));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
/// assert!(!ids_trinary_operator.contains_u32(0x3006));  // IDEOGRAPHIC CLOSING MARK
/// ```
pub fn get_ids_trinary_operator<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::IDS_TRINARY_OPERATOR_V1)
}

/// Format control characters which have specific functions for control of cursive joining
/// and ligation
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_join_control(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let join_control = &data_struct.inv_list;
///
/// assert!(join_control.contains_u32(0x200C));  // ZERO WIDTH NON-JOINER
/// assert!(join_control.contains_u32(0x200D));  // ZERO WIDTH JOINER
/// assert!(!join_control.contains_u32(0x200E));
/// ```
pub fn get_join_control<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::JOIN_CONTROL_V1)
}

/// A small number of spacing vowel letters occurring in certain Southeast Asian scripts such as Thai and Lao
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_logical_order_exception(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let logical_order_exception = &data_struct.inv_list;
///
/// assert!(logical_order_exception.contains('ແ'));  // U+0EC1 LAO VOWEL SIGN EI
/// assert!(!logical_order_exception.contains('ະ'));  // U+0EB0 LAO VOWEL SIGN A
/// ```
pub fn get_logical_order_exception<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::LOGICAL_ORDER_EXCEPTION_V1)
}

/// Lowercase characters
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_lowercase(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let lowercase = &data_struct.inv_list;
///
/// assert!(lowercase.contains('a'));
/// assert!(!lowercase.contains('A'));
/// ```
pub fn get_lowercase<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::LOWERCASE_V1)
}

/// Characters used in mathematical notation
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_math(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let math = &data_struct.inv_list;
///
/// assert!(math.contains('='));
/// assert!(math.contains('+'));
/// assert!(!math.contains('-'));
/// assert!(math.contains('−'));  // U+2212 MINUS SIGN
/// assert!(!math.contains('/'));
/// assert!(math.contains('∕'));  // U+2215 DIVISION SLASH
/// ```
pub fn get_math<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::MATH_V1)
}

/// Code points permanently reserved for internal use
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_noncharacter_code_point(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let noncharacter_code_point = &data_struct.inv_list;
///
/// assert!(noncharacter_code_point.contains_u32(0xFDD0));
/// assert!(noncharacter_code_point.contains_u32(0xFFFF));
/// assert!(!noncharacter_code_point.contains_u32(0x10000));
/// ```
pub fn get_noncharacter_code_point<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NONCHARACTER_CODE_POINT_V1)
}

/// Characters that are inert under NFC, i.e., they do not interact with adjacent characters
pub fn get_nfc_inert<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFC_INERT_V1)
}

/// Characters that are inert under NFD, i.e., they do not interact with adjacent characters
pub fn get_nfd_inert<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFD_INERT_V1)
}

/// Characters that are inert under NFKC, i.e., they do not interact with adjacent characters
pub fn get_nfkc_inert<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFKC_INERT_V1)
}

/// Characters that are inert under NFKD, i.e., they do not interact with adjacent characters
pub fn get_nfkd_inert<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFKD_INERT_V1)
}

/// Characters used as syntax in patterns (such as regular expressions). See [`Unicode
/// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
/// details.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_pattern_syntax(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let pattern_syntax = &data_struct.inv_list;
///
/// assert!(pattern_syntax.contains('{'));
/// assert!(pattern_syntax.contains('⇒'));  // U+21D2 RIGHTWARDS DOUBLE ARROW
/// assert!(!pattern_syntax.contains('0'));
/// ```
pub fn get_pattern_syntax<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PATTERN_SYNTAX_V1)
}

/// Characters used as whitespace in patterns (such as regular expressions).  See
/// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
/// more details.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_pattern_white_space(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let pattern_white_space = &data_struct.inv_list;
///
/// assert!(pattern_white_space.contains(' '));
/// assert!(pattern_white_space.contains_u32(0x2029));  // PARAGRAPH SEPARATOR
/// assert!(pattern_white_space.contains_u32(0x000A));  // NEW LINE
/// assert!(!pattern_white_space.contains_u32(0x00A0));  // NO-BREAK SPACE
/// ```
pub fn get_pattern_white_space<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PATTERN_WHITE_SPACE_V1)
}

/// A small class of visible format controls, which precede and then span a sequence of
/// other characters, usually digits.
pub fn get_prepended_concatenation_mark<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PREPENDED_CONCATENATION_MARK_V1)
}

/// Printable characters (visible characters and whitespace).
/// This is defined for POSIX compatibility.
pub fn get_print<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PRINT_V1)
}

/// Punctuation characters that function as quotation marks.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_quotation_mark(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let quotation_mark = &data_struct.inv_list;
///
/// assert!(quotation_mark.contains('\''));
/// assert!(quotation_mark.contains('„'));  // U+201E DOUBLE LOW-9 QUOTATION MARK
/// assert!(!quotation_mark.contains('<'));
/// ```
pub fn get_quotation_mark<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::QUOTATION_MARK_V1)
}

/// Characters used in the definition of Ideographic Description Sequences
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_radical(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let radical = &data_struct.inv_list;
///
/// assert!(radical.contains('⺆'));  // U+2E86 CJK RADICAL BOX
/// assert!(!radical.contains('丹'));  // U+F95E CJK COMPATIBILITY IDEOGRAPH-F95E
/// ```
pub fn get_radical<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::RADICAL_V1)
}

/// Regional indicator characters, U+1F1E6..U+1F1FF
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_regional_indicator(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let regional_indicator = &data_struct.inv_list;
///
/// assert!(regional_indicator.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
/// assert!(!regional_indicator.contains('Ⓣ'));  // U+24C9 CIRCLED LATIN CAPITAL LETTER T
/// assert!(!regional_indicator.contains('T'));
/// ```
pub fn get_regional_indicator<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::REGIONAL_INDICATOR_V1)
}

/// Characters with a "soft dot", like i or j. An accent placed on these characters causes
/// the dot to disappear.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_soft_dotted(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let soft_dotted = &data_struct.inv_list;
///
/// assert!(soft_dotted.contains('і'));  //U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
/// assert!(!soft_dotted.contains('ı'));  // U+0131 LATIN SMALL LETTER DOTLESS I
/// ```
pub fn get_soft_dotted<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::SOFT_DOTTED_V1)
}

/// Characters that are starters in terms of Unicode normalization and combining character
/// sequences
pub fn get_segment_starter<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::SEGMENT_STARTER_V1)
}

/// Characters that are either the source of a case mapping or in the target of a case
/// mapping
pub fn get_case_sensitive<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CASE_SENSITIVE_V1)
}

/// Punctuation characters that generally mark the end of sentences
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_sentence_terminal(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let sentence_terminal = &data_struct.inv_list;
///
/// assert!(sentence_terminal.contains('.'));
/// assert!(sentence_terminal.contains('?'));
/// assert!(sentence_terminal.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
/// assert!(!sentence_terminal.contains(','));
/// assert!(!sentence_terminal.contains('¿'));  // U+00BF INVERTED QUESTION MARK
/// ```
pub fn get_sentence_terminal<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::SENTENCE_TERMINAL_V1)
}

/// Punctuation characters that generally mark the end of textual units
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_terminal_punctuation(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let terminal_punctuation = &data_struct.inv_list;
///
/// assert!(terminal_punctuation.contains('.'));
/// assert!(terminal_punctuation.contains('?'));
/// assert!(terminal_punctuation.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
/// assert!(terminal_punctuation.contains(','));
/// assert!(!terminal_punctuation.contains('¿'));  // U+00BF INVERTED QUESTION MARK
/// ```
pub fn get_terminal_punctuation<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::TERMINAL_PUNCTUATION_V1)
}

/// A property which specifies the exact set of Unified CJK Ideographs in the standard
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_unified_ideograph(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let unified_ideograph = &data_struct.inv_list;
///
/// assert!(unified_ideograph.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
/// assert!(unified_ideograph.contains('木'));  // U+6728 CJK UNIFIED IDEOGRAPH-6728
/// assert!(!unified_ideograph.contains('𛅸'));  // U+1B178 NUSHU CHARACTER-1B178
/// ```
pub fn get_unified_ideograph<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::UNIFIED_IDEOGRAPH_V1)
}

/// Uppercase characters
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_uppercase(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let uppercase = &data_struct.inv_list;
///
/// assert!(uppercase.contains('U'));
/// assert!(!uppercase.contains('u'));
/// ```
pub fn get_uppercase<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::UPPERCASE_V1)
}

/// Characters that are Variation Selectors.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_variation_selector(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let variation_selector = &data_struct.inv_list;
///
/// assert!(variation_selector.contains_u32(0x180D));  // MONGOLIAN FREE VARIATION SELECTOR THREE
/// assert!(!variation_selector.contains_u32(0x303E));  // IDEOGRAPHIC VARIATION INDICATOR
/// assert!(variation_selector.contains_u32(0xFE0F));  // VARIATION SELECTOR-16
/// assert!(!variation_selector.contains_u32(0xFE10));  // PRESENTATION FORM FOR VERTICAL COMMA
/// assert!(variation_selector.contains_u32(0xE01EF));  // VARIATION SELECTOR-256
/// ```
pub fn get_variation_selector<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::VARIATION_SELECTOR_V1)
}

/// Spaces, separator characters and other control characters which should be treated by
/// programming languages as "white space" for the purpose of parsing elements
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_white_space(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let white_space = &data_struct.inv_list;
///
/// assert!(white_space.contains(' '));
/// assert!(white_space.contains_u32(0x000A));  // NEW LINE
/// assert!(white_space.contains_u32(0x00A0));  // NO-BREAK SPACE
/// assert!(!white_space.contains_u32(0x200B));  // ZERO WIDTH SPACE
/// ```
pub fn get_white_space<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::WHITE_SPACE_V1)
}

/// Hexadecimal digits
/// This is defined for POSIX compatibility.
pub fn get_xdigit<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::XDIGIT_V1)
}

/// Characters that can begin an identifier.  See [`Unicode Standard Annex
/// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_xid_continue(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let xid_continue = &data_struct.inv_list;
///
/// assert!(xid_continue.contains('x'));
/// assert!(xid_continue.contains('1'));
/// assert!(xid_continue.contains('_'));
/// assert!(xid_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
/// assert!(!xid_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
/// assert!(!xid_continue.contains_u32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
/// ```
pub fn get_xid_continue<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::XID_CONTINUE_V1)
}

/// Characters that can come after the first character in an identifier. See [`Unicode
/// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
/// details.
///
/// # Example
///
/// ```
/// use icu_properties::sets;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     sets::get_xid_start(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let xid_start = &data_struct.inv_list;
///
/// assert!(xid_start.contains('x'));
/// assert!(!xid_start.contains('1'));
/// assert!(!xid_start.contains('_'));
/// assert!(xid_start.contains('ߝ'));  // U+07DD NKO LETTER FA
/// assert!(!xid_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
/// assert!(!xid_start.contains_u32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
/// ```
pub fn get_xid_start<D>(provider: &D) -> UnisetResult
where
    D: DynProvider<UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::XID_START_V1)
}

//
// Enumerated property getter fns
//

/// Return a [`UnicodeSet`] for a value or a grouping of values of the General_Category property. See [`GeneralCategoryGroup`].
///
/// [`UnicodeSet`]: icu_uniset::UnicodeSet
pub fn get_for_general_category_group<D>(
    provider: &D,
    enum_val: GeneralCategoryGroup,
) -> UnisetResult
where
    D: DynProvider<UnicodePropertyMapV1Marker<GeneralCategory>> + ?Sized,
{
    let gc_map_payload: DataPayload<UnicodePropertyMapV1Marker<GeneralCategory>> =
        maps::get_general_category(provider)?;

    let gc_group_set_payload: DataPayload<UnicodePropertyV1Marker> = gc_map_payload
        .map_project_cloned_with_capture(enum_val, |gc_map_marker, gcg_val_capture, _| {
            let gc = &gc_map_marker.code_point_trie;
            let matching_gc_ranges = gc
                .iter_ranges()
                .filter(move |cpm_range| (1 << cpm_range.value as u32) & gcg_val_capture.0 != 0)
                .map(|cpm_range| {
                    RangeInclusive::new(*cpm_range.range.start(), *cpm_range.range.end())
                });
            let set = UnicodeSet::from_iter(matching_gc_ranges);
            UnicodePropertyV1::from_owned_uniset(set.clone())
        });

    Ok(gc_group_set_payload)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_general_category() {
        use icu::properties::sets;
        use icu::properties::GeneralCategoryGroup;

        let provider = icu_testdata::get_provider();
        let payload = sets::get_for_general_category_group(&provider, GeneralCategoryGroup::Number)
            .expect("The data should be valid");
        let data_struct = payload.get();
        let digits = &data_struct.inv_list;

        assert!(digits.contains('5'));
        assert!(digits.contains('\u{0665}')); // U+0665 ARABIC-INDIC DIGIT FIVE
        assert!(digits.contains('\u{096b}')); // U+0969 DEVANAGARI DIGIT FIVE

        assert!(!digits.contains('A'));
    }

    #[test]
    fn test_script() {
        use icu::properties::maps;
        use icu::properties::Script;

        let provider = icu_testdata::get_provider();
        let payload = maps::get_script(&provider).expect("The data should be valid");
        let data_struct = payload.get();
        let script = &data_struct.code_point_trie;
        let thai = script.get_set_for_value(Script::Thai);

        assert!(thai.contains('\u{0e01}')); // U+0E01 THAI CHARACTER KO KAI
        assert!(thai.contains('\u{0e50}')); // U+0E50 THAI DIGIT ZERO

        assert!(!thai.contains('A'));
        assert!(!thai.contains('\u{0e3f}')); // U+0E50 THAI CURRENCY SYMBOL BAHT
    }

    #[test]
    fn test_gc_groupings() {
        use icu::properties::{maps, sets};
        use icu::properties::{GeneralCategory, GeneralCategoryGroup};
        use icu_uniset::{UnicodeSet, UnicodeSetBuilder};
        use std::convert::TryInto;

        let provider = icu_testdata::get_provider();

        let test_group = |category: GeneralCategoryGroup, subcategories: &[GeneralCategory]| {
            let category_set_payload = sets::get_for_general_category_group(&provider, category)
                .expect("The data should be valid");
            let category_set: UnicodeSet = category_set_payload
                .get()
                .clone()
                .try_into()
                .expect("Valid unicode set");

            let gc_payload =
                maps::get_general_category(&provider).expect("The data should be valid");
            let data_struct = gc_payload.get();
            let gc = &data_struct.code_point_trie;

            let mut builder = UnicodeSetBuilder::new();
            for subcategory in subcategories {
                builder.add_set(&gc.get_set_for_value(*subcategory));
            }
            let combined_set = builder.build();
            println!("{:?} {:?}", category, subcategories);
            assert_eq!(
                category_set.get_inversion_list(),
                combined_set.get_inversion_list()
            );
        };

        test_group(
            GeneralCategoryGroup::Letter,
            &[
                GeneralCategory::UppercaseLetter,
                GeneralCategory::LowercaseLetter,
                GeneralCategory::TitlecaseLetter,
                GeneralCategory::ModifierLetter,
                GeneralCategory::OtherLetter,
            ],
        );
        test_group(
            GeneralCategoryGroup::Other,
            &[
                GeneralCategory::Control,
                GeneralCategory::Format,
                GeneralCategory::Unassigned,
                GeneralCategory::PrivateUse,
                GeneralCategory::Surrogate,
            ],
        );
        test_group(
            GeneralCategoryGroup::Mark,
            &[
                GeneralCategory::SpacingMark,
                GeneralCategory::EnclosingMark,
                GeneralCategory::NonspacingMark,
            ],
        );
        test_group(
            GeneralCategoryGroup::Number,
            &[
                GeneralCategory::DecimalNumber,
                GeneralCategory::LetterNumber,
                GeneralCategory::OtherNumber,
            ],
        );
        test_group(
            GeneralCategoryGroup::Punctuation,
            &[
                GeneralCategory::ConnectorPunctuation,
                GeneralCategory::DashPunctuation,
                GeneralCategory::ClosePunctuation,
                GeneralCategory::FinalPunctuation,
                GeneralCategory::InitialPunctuation,
                GeneralCategory::OtherPunctuation,
                GeneralCategory::OpenPunctuation,
            ],
        );
        test_group(
            GeneralCategoryGroup::Symbol,
            &[
                GeneralCategory::CurrencySymbol,
                GeneralCategory::ModifierSymbol,
                GeneralCategory::MathSymbol,
                GeneralCategory::OtherSymbol,
            ],
        );
        test_group(
            GeneralCategoryGroup::Separator,
            &[
                GeneralCategory::LineSeparator,
                GeneralCategory::ParagraphSeparator,
                GeneralCategory::SpaceSeparator,
            ],
        );
    }

    #[test]
    fn test_gc_surrogate() {
        use icu::properties::maps;
        use icu::properties::GeneralCategory;

        let provider = icu_testdata::get_provider();
        let gc_payload = maps::get_general_category(&provider).expect("The data should be valid");
        let data_struct = gc_payload.get();
        let gc = &data_struct.code_point_trie;
        let surrogates = gc.get_set_for_value(GeneralCategory::Surrogate);

        assert!(surrogates.contains_u32(0xd800));
        assert!(surrogates.contains_u32(0xd900));
        assert!(surrogates.contains_u32(0xdfff));

        assert!(!surrogates.contains('A'));
    }
}
