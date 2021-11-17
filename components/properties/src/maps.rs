// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The functions in this module return a [`CodePointTrie`] representing, for
//! each code point in the entire range of code points, the property values
//! for a particular Unicode property.
//!
//! The descriptions of most properties are taken from [`TR44`], the documentation for the
//! Unicode Character Database.
//!
//! [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
//! [`TR44`]: https://www.unicode.org/reports/tr44

use crate::error::PropertiesError;
use crate::provider::*;
use crate::*;
use icu_codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::prelude::*;

/// TODO(#1239): Finalize this API.
pub type CodePointMapResult<T> =
    Result<DataPayload<UnicodePropertyMapV1Marker<T>>, PropertiesError>;

fn get_cp_map<D, T>(provider: &D, resc_key: ResourceKey) -> CodePointMapResult<T>
where
    D: DataProvider<UnicodePropertyMapV1Marker<T>> + ?Sized,
    T: TrieValue,
{
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: resc_key,
            options: ResourceOptions {
                variant: None,
                langid: None,
            },
        },
    };

    let resp: DataResponse<UnicodePropertyMapV1Marker<T>> = provider.load_payload(&data_req)?;

    let property_payload: DataPayload<UnicodePropertyMapV1Marker<T>> = resp.take_payload()?;
    Ok(property_payload)
}

/// Return a [`CodePointTrie`] for the General_Category Unicode enumerated property. See [`GeneralCategory`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, GeneralSubcategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
///
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// assert_eq!(gc.get('木' as u32), GeneralSubcategory::OtherLetter);  // U+6728
/// assert_eq!(gc.get('🎃' as u32), GeneralSubcategory::OtherSymbol);  // U+1F383 JACK-O-LANTERN
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
pub fn get_general_category<D>(provider: &D) -> CodePointMapResult<GeneralSubcategory>
where
    D: DataProvider<UnicodePropertyMapV1Marker<GeneralSubcategory>> + ?Sized,
{
    get_cp_map(provider, key::GENERAL_CATEGORY_V1)
}

/// Return a [`CodePointTrie`] for the Script Unicode enumerated property. See [`Script`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, Script};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
///
/// let payload =
///     maps::get_script(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let script = &data_struct.code_point_trie;
/// assert_eq!(script.get('木' as u32), Script::Han);  // U+6728
/// assert_eq!(script.get('🎃' as u32), Script::Common);  // U+1F383 JACK-O-LANTERN
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
pub fn get_script<D>(provider: &D) -> CodePointMapResult<Script>
where
    D: DataProvider<UnicodePropertyMapV1Marker<Script>> + ?Sized,
{
    get_cp_map(provider, key::SCRIPT_V1)
}

/// Return a [`CodePointTrie`] for the East_Asian_Width Unicode enumerated
/// property. See [`East_Asian_Width`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, EastAsianWidth};
///
/// let provider = icu_testdata::get_provider();
/// let payload = maps::get_east_asian_width(&provider).expect("The data should be valid!");
/// let eaw = &payload.get().code_point_trie;
///
/// assert_eq!(eaw.get('ｱ' as u32), EastAsianWidth::Halfwidth); // U+FF71: Halfwidth Katakana Letter A
/// assert_eq!(eaw.get('ア' as u32), EastAsianWidth::Wide); //U+30A2: Katakana Letter A
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
pub fn get_east_asian_width<D>(provider: &D) -> CodePointMapResult<EastAsianWidth>
where
    D: DataProvider<UnicodePropertyMapV1Marker<EastAsianWidth>> + ?Sized,
{
    get_cp_map(provider, key::EAST_ASIAN_WIDTH_V1)
}

/// Return a [`CodePointTrie`] for the Line_Break Unicode enumerated
/// property. See [`LineBreak`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, LineBreak};
///
/// let provider = icu_testdata::get_provider();
/// let payload = maps::get_line_break(&provider).expect("The data should be valid!");
/// let lb = &payload.get().code_point_trie;
///
/// assert_eq!(lb.get(')' as u32), LineBreak::CloseParenthesis); // U+0029: Right Parenthesis
/// assert_eq!(lb.get('ぁ' as u32), LineBreak::ConditionalJapaneseStarter); //U+3041: Hiragana Letter Small A
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
pub fn get_line_break<D>(provider: &D) -> CodePointMapResult<LineBreak>
where
    D: DataProvider<UnicodePropertyMapV1Marker<LineBreak>> + ?Sized,
{
    get_cp_map(provider, key::LINE_BREAK_V1)
}

/// Return a [`CodePointTrie`] for the Grapheme_Cluster_Break Unicode enumerated
/// property. See [`GraphemeClusterBreak`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, GraphemeClusterBreak};
///
/// let provider = icu_testdata::get_provider();
/// let payload = maps::get_grapheme_cluster_break(&provider).expect("The data should be valid!");
/// let gcb = &payload.get().code_point_trie;
///
/// assert_eq!(gcb.get('🇦' as u32), GraphemeClusterBreak::RegionalIndicator); // U+1F1E6: Regional Indicator Symbol Letter A
/// assert_eq!(gcb.get('ำ' as u32), GraphemeClusterBreak::SpacingMark); //U+0E33: Thai Character Sara Am
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
pub fn get_grapheme_cluster_break<D>(provider: &D) -> CodePointMapResult<GraphemeClusterBreak>
where
    D: DataProvider<UnicodePropertyMapV1Marker<GraphemeClusterBreak>> + ?Sized,
{
    get_cp_map(provider, key::GRAPHEME_CLUSTER_BREAK_V1)
}

/// Return a [`CodePointTrie`] for the Word_Break Unicode enumerated
/// property. See [`WordBreak`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, WordBreak};
///
/// let provider = icu_testdata::get_provider();
/// let payload = maps::get_word_break(&provider).expect("The data should be valid!");
/// let wb = &payload.get().code_point_trie;
///
/// assert_eq!(wb.get('.' as u32), WordBreak::MidNumLet); // U+002E: Full Stop
/// assert_eq!(wb.get('，' as u32), WordBreak::MidNum); // U+FF0C: Fullwidth Comma
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
pub fn get_word_break<D>(provider: &D) -> CodePointMapResult<WordBreak>
where
    D: DataProvider<UnicodePropertyMapV1Marker<WordBreak>> + ?Sized,
{
    get_cp_map(provider, key::WORD_BREAK_V1)
}

/// Return a [`CodePointTrie`] for the Sentence_Break Unicode enumerated
/// property. See [`SentenceBreak`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, SentenceBreak};
///
/// let provider = icu_testdata::get_provider();
/// let payload = maps::get_sentence_break(&provider).expect("The data should be valid!");
/// let sb = &payload.get().code_point_trie;
///
/// assert_eq!(sb.get('９' as u32), SentenceBreak::Numeric); // U+FF19: Fullwidth Digit Nine
/// assert_eq!(sb.get(',' as u32), SentenceBreak::SContinue); // U+002C: Comma
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
pub fn get_sentence_break<D>(provider: &D) -> CodePointMapResult<SentenceBreak>
where
    D: DataProvider<UnicodePropertyMapV1Marker<SentenceBreak>> + ?Sized,
{
    get_cp_map(provider, key::SENTENCE_BREAK_V1)
}

//
// General_Category Predicate Functions
// 
//   For querying a specific code point for inclusion in the union set of more
//   than one category (General_Category property value).
//




/// Return whether the code point belongs in the multi-value category `Cased_Letter`.
/// This is defined as the union of `GeneralSubcategory::UppercaseLetter`,
/// `GeneralSubcategory::LowercaseLetter`, and `GeneralSubcategory::TitlecaseLetter`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get('A' as u32), GeneralSubcategory::UppercaseLetter);
/// assert!(maps::is_cased_letter(&gc, 'A' as u32));
/// ```
pub fn is_cased_letter(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::CasedLetter.0
}


/// Return whether the code point belongs in the multi-value category `Letter`.
/// This is defined as the union of `GeneralCategory::UppercaseLetter`,
/// `GeneralSubcategory::LowercaseLetter`, `GeneralSubcategory::TitlecaseLetter`,
/// `GeneralSubcategory::ModifierLetter`, and `GeneralSubcategory::OtherLetter`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get('𝔅' as u32), GeneralSubcategory::UppercaseLetter);  // U+1D505 MATHEMATICAL FRAKTUR CAPITAL B
/// assert!(maps::is_letter(&gc, '𝔅' as u32));
/// ```
pub fn is_letter(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::Letter.0
}

/// Return whether the code point belongs in the multi-value category `Mark`.
/// This is defined as the union of `GeneralSubcategory::NonspacingMark`,
/// `GeneralSubcategory::SpacingMark`, and `GeneralSubcategory::EnclosingMark`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get(0x0301), GeneralSubcategory::NonspacingMark);  // U+0301 COMBINING ACUTE ACCENT
/// assert!(maps::is_mark(&gc, 0x0301));
/// ```
pub fn is_mark(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::Mark.0
}

/// Return whether the code point belongs in the multi-value category `Number`.
/// This is defined as the union of `GeneralSubcategory::DecimalNumber`,
/// `GeneralSubcategory::LetterNumber`, and `GeneralSubcategory::OtherNumber`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get('0' as u32), GeneralSubcategory::DecimalNumber);
/// assert!(maps::is_number(&gc, '0' as u32));
/// ```
pub fn is_number(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::Number.0
}

/// Return whether the code point belongs in the multi-value category `Punctuation`.
/// This is defined as the union of `GeneralSubcategory::ConnectorPunctuation`,
/// `GeneralSubcategory::DashPunctuation`, `GeneralSubcategory::OpenPunctuation`,
/// `GeneralSubcategory::ClosePunctuation`, `GeneralSubcategory::InitialPunctuation`,
/// `GeneralSubcategory::FinalPunctuation`, and `GeneralSubcategory::OtherPunctuation`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get('(' as u32), GeneralSubcategory::OpenPunctuation);
/// assert!(maps::is_punctuation(&gc, '(' as u32));
/// ```
pub fn is_punctuation(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::Punctuation.0
}

/// Return whether the code point belongs in the multi-value category `Symbol`.
/// This is defined as the union of `GeneralSubcategory::MathSymbol`,
/// `GeneralSubcategory::CurrencySymbol`, `GeneralSubcategory::ModifiedSymbol`, and
/// `GeneralSubcategory::OtherSymbol`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get('✓' as u32), GeneralSubcategory::OtherSymbol);
/// assert!(maps::is_symbol(&gc, '✓' as u32));
/// ```
pub fn is_symbol(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::Symbol.0
}

/// Return whether the code point belongs in the multi-value category `Separator`.
/// This is defined as the union of `GeneralSubcategory::SpaceSeparator`,
/// `GeneralSubcategory::LineSeparator`, and `GeneralSubcategory::ParagraphSeparator`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get(' ' as u32), GeneralSubcategory::SpaceSeprator);
/// assert!(maps::is_separator(&gc, ' ' as u32));
/// ```
pub fn is_separator(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::Separator.0
}

/// Return whether the code point belongs in the multi-value category `Other`.
/// This is defined as the union of `GeneralSubcategory::Control`,
/// `GeneralSubcategory::Format`, `GeneralSubcategory::Surrogate`.
/// `GeneralSubcategory::PrivateUse`, and `GeneralSubcategory::Unassigned`.
/// 
/// ```
/// use icu::properties::{maps, GeneralSubcategory, GeneralCategory};
/// use icu_codepointtrie::CodePointTrie;
///
/// let provider = icu_testdata::get_provider();
/// let payload =
///     maps::get_general_category(&provider)
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let gc = &data_struct.code_point_trie;
/// 
/// assert_eq!(gc.get(0xE007F), GeneralSubcategory::Format);  // U+E007F CANCEL TAG
/// assert!(maps::is_other(&gc, 0xE007F));
/// ```
pub fn is_other(gc_map: &CodePointTrie<GeneralSubcategory>, code_point: u32) -> bool {
    let gc_val = gc_map.get(code_point);
    0 != (gc_val as u32) & GeneralCategory::Other.0
}

