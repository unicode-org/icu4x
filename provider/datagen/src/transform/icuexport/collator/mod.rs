// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use icu_collator::provider::*;
use icu_collections::codepointtrie::CodePointTrie;
use icu_locid::extensions::unicode::Value;
use icu_locid::extensions_unicode_key as key;
use icu_locid::subtags_language as language;
use icu_locid::LanguageIdentifier;
use icu_locid::Locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::str::FromStr;
use writeable::Writeable;
use zerovec::ZeroVec;

mod collator_serde;

#[cfg(test)]
mod test;

/// Forward compatibility with https://unicode-org.atlassian.net/browse/CLDR-15603
/// See https://github.com/unicode-org/cldr/commit/aca740fb9c59efa1f1717bee682d98bded5d0428
/// and https://github.com/unicode-org/cldr/commit/5b1423acc49c6b539e0cfbc69ae38c9cf044b1ca
fn reformed_swedish_exists(
    icuexport: &crate::SerdeCache,
    collation_han_database: crate::CollationHanDatabase,
) -> bool {
    icuexport
        .read_and_parse_toml::<collator_serde::CollationMetadata>(&format!(
            "collation/{}/sv_reformed_meta.toml",
            collation_han_database
        ))
        .is_ok()
}

fn locale_to_file_name(
    icuexport: &crate::SerdeCache,
    collation_han_database: crate::CollationHanDatabase,
    locale: &DataLocale,
) -> String {
    let mut s = if locale.get_langid() == LanguageIdentifier::UND {
        "root".to_owned()
    } else {
        locale
            .get_langid()
            .write_to_string()
            .replace('-', "_")
            .replace("posix", "POSIX")
    };
    if let Some(extension) = &locale.get_unicode_ext(&key!("co")) {
        s.push('_');
        s.push_str(match extension.to_string().as_str() {
            "trad" => "traditional",
            "phonebk" => "phonebook",
            "dict" => "dictionary",
            "gb2312" => "gb2312han",
            extension => extension,
        });
    } else {
        // "standard" is the default for all but two languages: sv and zh.
        // Since there are only two special cases, hard-coding them
        // here for now instead of making the defaulting fancy and data driven.
        // The Swedish naming seems ad hoc from
        // https://unicode-org.atlassian.net/browse/CLDR-679 .

        if locale.get_langid().language == language!("zh") {
            s.push_str("_pinyin");
        } else if locale.get_langid().language == language!("sv")
            && reformed_swedish_exists(icuexport, collation_han_database)
        {
            s.push_str("_reformed");
        } else {
            s.push_str("_standard");
        }
    }
    s
}

fn file_name_to_locale(
    icuexport: &crate::SerdeCache,
    collation_han_database: crate::CollationHanDatabase,
    file_name: &str,
) -> Option<Locale> {
    let (language, variant) = file_name.rsplit_once('_').unwrap();
    let langid = if language == "root" {
        LanguageIdentifier::UND
    } else {
        language.parse().ok()?
    };
    let mut locale = Locale::from(langid);
    // See above for the two special cases.
    let reformed_exists = reformed_swedish_exists(icuexport, collation_han_database);
    if !((language == "zh" && variant == "pinyin")
        || (language == "sv" && reformed_exists && variant == "reformed")
        || ((language != "zh" && !(language == "sv" && reformed_exists)) && variant == "standard"))
    {
        let shortened = match variant {
            "traditional" => "trad",
            "phonebook" => "phonebk",
            "dictionary" => "dict",
            "gb2312han" => "gb2312",
            _ => variant,
        };
        locale.extensions.unicode.keywords.set(
            key!("co"),
            Value::from_str(shortened).expect("valid extension subtag"),
        );
    };
    Some(locale)
}

macro_rules! collation_provider {
    ($(($marker:ident, $serde_struct:ident, $suffix:literal, $conversion:expr)),+, $toml_data:ident) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    let $toml_data: &collator_serde::$serde_struct = self
                        .source
                        .icuexport()?
                        .read_and_parse_toml(
                            &format!(
                                "collation/{}/{}{}.toml",
                                self.source.collation_han_database(),
                                locale_to_file_name(self
                                    .source
                                    // `unwrap` OK due to the `?` earlier
                                    .icuexport().unwrap(), self.source.collation_han_database(), &req.locale), $suffix)
                        )
                        .map_err(|e| match e.kind {
                            DataErrorKind::Io(
                                std::io::ErrorKind::NotFound
                            ) => DataErrorKind::MissingLocale.with_req(
                                $marker::KEY, req
                            ),
                            _ => e
                        })?;

                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        // The struct conversion is macro-based instead of
                        // using a method on the Serde struct, because the
                        // method approach caused lifetime issues that I
                        // didn't know how to solve.
                        payload: Some(DataPayload::from_owned($conversion)),
                    })
                }
            }

            impl IterableDataProvider<$marker> for crate::DatagenProvider {
                fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                    Ok(self
                        .source
                        .icuexport()?
                        .list(&format!("collation/{}", self.source.collation_han_database()))?
                        .filter_map(|entry|
                            entry
                                .file_stem()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned()
                                .strip_suffix($suffix)
                                .map(ToString::to_string)
                        )
                        .filter_map(|s|file_name_to_locale(self
                            .source
                            // `unwrap` OK due to the `?` earlier
                            .icuexport().unwrap(), self.source.collation_han_database(), &s))
                        .map(DataLocale::from)
                        .collect()
                    )
                }
            }
        )+
    };
}

collation_provider!(
    (
        CollationDataV1Marker,
        CollationData,
        "_data",
        icu_collator::provider::CollationDataV1 {
            trie: CodePointTrie::<u32>::try_from(&toml_data.trie)
                .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?,
            contexts: ZeroVec::alloc_from_slice(&toml_data.contexts),
            ce32s: ZeroVec::alloc_from_slice(&toml_data.ce32s),
            ces: toml_data.ces.iter().map(|i| *i as u64).collect(),
        }
    ),
    (
        CollationDiacriticsV1Marker,
        CollationDiacritics,
        "_dia",
        icu_collator::provider::CollationDiacriticsV1 {
            secondaries: ZeroVec::alloc_from_slice(&toml_data.secondaries),
        }
    ),
    (
        CollationJamoV1Marker,
        CollationJamo,
        "_jamo",
        icu_collator::provider::CollationJamoV1 {
            ce32s: ZeroVec::alloc_from_slice(&toml_data.ce32s),
        }
    ),
    (
        CollationMetadataV1Marker,
        CollationMetadata,
        "_meta",
        icu_collator::provider::CollationMetadataV1 {
            bits: toml_data.bits,
        }
    ),
    (
        CollationReorderingV1Marker,
        CollationReordering,
        "_reord",
        icu_collator::provider::CollationReorderingV1 {
            min_high_no_reorder: toml_data.min_high_no_reorder,
            reorder_table: ZeroVec::alloc_from_slice(&toml_data.reorder_table),
            reorder_ranges: ZeroVec::alloc_from_slice(&toml_data.reorder_ranges),
        }
    ),
    (
        CollationSpecialPrimariesV1Marker,
        CollationSpecialPrimaries,
        "_prim",
        icu_collator::provider::CollationSpecialPrimariesV1 {
            last_primaries: ZeroVec::alloc_from_slice(&toml_data.last_primaries),
            numeric_primary: toml_data.numeric_primary,
        }
    ),
    toml_data
);
