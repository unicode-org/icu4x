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
//! [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
//! [`TR44`]: https://www.unicode.org/reports/tr44

use crate::error::PropertiesError;
use crate::provider::*;
use crate::*;
use icu_codepointtrie::codepointtrie::TrieValue;
use icu_provider::prelude::*;

type CodePointMapResult<'data, T> =
    Result<DataPayload<'data, UnicodePropertyMapV1Marker<T>>, PropertiesError>;

fn get_cp_map<'data, D, T>(provider: &D, resc_key: ResourceKey) -> CodePointMapResult<'data, T>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<T>> + ?Sized,
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
/// [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
pub fn get_general_category<'data, D>(provider: &D) -> CodePointMapResult<'data, GeneralSubcategory>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<GeneralSubcategory>> + ?Sized,
{
    get_cp_map(provider, key::GENERAL_CATEGORY_V1)
}

/// Return a [`CodePointTrie`] for the Script Unicode enumerated property. See [`Script`].
///
/// [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
pub fn get_script<'data, D>(provider: &D) -> CodePointMapResult<'data, Script>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<Script>> + ?Sized,
{
    get_cp_map(provider, key::SCRIPT_V1)
}

/// Return a [`CodePointTrie`] for the Grapheme_Cluster_Break Unicode enumerated
/// property. See [`GraphemeClusterBreak`].
///
/// # Example
///
/// ```
/// use icu::properties::{maps, GraphemeClusterBreak};
/// use icu_provider_uprops::EnumeratedPropertyCodePointTrieProvider;
///
/// let root_dir = icu_testdata::paths::uprops_toml_root();
/// let provider = EnumeratedPropertyCodePointTrieProvider::new(root_dir);
/// let payload = maps::get_grapheme_cluster_break(&provider).expect("The data should be valid!");
/// let gcb = &payload.get().codepoint_trie;
///
/// assert_eq!(gcb.get('🇦' as u32), GraphemeClusterBreak::RegionalIndicator); // U+1F1E6: Regional Indicator Symbol Letter A
/// assert_eq!(gcb.get('ำ' as u32), GraphemeClusterBreak::SpacingMark); //U+0E33: Thai Character Sara Am
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
pub fn get_grapheme_cluster_break<'data, D>(
    provider: &D,
) -> CodePointMapResult<'data, GraphemeClusterBreak>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<GraphemeClusterBreak>> + ?Sized,
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
/// use icu_provider_uprops::EnumeratedPropertyCodePointTrieProvider;
///
/// let root_dir = icu_testdata::paths::uprops_toml_root();
/// let provider = EnumeratedPropertyCodePointTrieProvider::new(root_dir);
/// let payload = maps::get_word_break(&provider).expect("The data should be valid!");
/// let wb = &payload.get().codepoint_trie;
///
/// assert_eq!(wb.get('.' as u32), WordBreak::MidNumLet); // U+002E: Full Stop
/// assert_eq!(wb.get('，' as u32), WordBreak::MidNum); // U+FF0C: Fullwidth Comma
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
pub fn get_word_break<'data, D>(provider: &D) -> CodePointMapResult<'data, WordBreak>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<WordBreak>> + ?Sized,
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
/// use icu_provider_uprops::EnumeratedPropertyCodePointTrieProvider;
/// let root_dir = icu_testdata::paths::uprops_toml_root();
/// let provider = EnumeratedPropertyCodePointTrieProvider::new(root_dir);
///
/// let payload = maps::get_sentence_break(&provider).expect("The data should be valid!");
/// let sb = &payload.get().codepoint_trie;
///
/// assert_eq!(sb.get('９' as u32), SentenceBreak::Numeric); // U+FF19: Fullwidth Digit Nine
/// assert_eq!(sb.get(',' as u32), SentenceBreak::SContinue); // U+002C: Comma
/// ```
///
/// [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
pub fn get_sentence_break<'data, D>(provider: &D) -> CodePointMapResult<'data, SentenceBreak>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<SentenceBreak>> + ?Sized,
{
    get_cp_map(provider, key::SENTENCE_BREAK_V1)
}
