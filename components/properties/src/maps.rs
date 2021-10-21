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

/// Return a [`CodePointTrie`] for the General_Category Unicode enumerated property.
/// General_Category specifies the most general classification of a code point, usually 
/// determined based on the primary characteristic of the assigned character. For example, is the
/// character a letter, a mark, a number, punctuation, or a symbol, and if so, of what type?
/// /// See <https://www.unicode.org/reports/tr44/> .
/// 
/// [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
pub fn get_general_category<'data, D>(provider: &D) -> CodePointMapResult<'data, GeneralSubcategory>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<GeneralSubcategory>> + ?Sized,
{
    get_cp_map(provider, key::GENERAL_CATEGORY_V1)
}

/// Return a [`CodePointTrie`] for the Script Unicode enumerated property.
/// See <https://www.unicode.org/reports/tr44/> .
///
/// [`CodePointTrie`]: icu_codepointtrie::codepointtrie::CodePointTrie
pub fn get_script<'data, D>(provider: &D) -> CodePointMapResult<'data, Script>
where
    D: DataProvider<'data, UnicodePropertyMapV1Marker<Script>> + ?Sized,
{
    get_cp_map(provider, key::SCRIPT_V1)
}
