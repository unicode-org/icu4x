// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use icu_collator::provider::*;
use icu_locid::extensions::unicode::Value;
use icu_locid::extensions_unicode_key as key;
use icu_locid::subtags_language as language;
use icu_locid::LanguageIdentifier;
use icu_locid::Locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_provider::DataKey;
use std::convert::TryFrom;
use std::str::FromStr;
use writeable::Writeable;
use zerovec::ZeroVec;

mod collator_serde;

/// Collection of all the key for easy reference from the datagen registry.
pub const ALL_KEYS: [DataKey; 6] = [
    CollationDataV1Marker::KEY,
    CollationDiacriticsV1Marker::KEY,
    CollationJamoV1Marker::KEY,
    CollationMetadataV1Marker::KEY,
    CollationReorderingV1Marker::KEY,
    CollationSpecialPrimariesV1Marker::KEY,
];

fn locale_to_file_name(opts: &DataOptions) -> String {
    let mut s = if opts.get_langid() == LanguageIdentifier::UND {
        String::from("root")
    } else {
        opts.get_langid()
            .write_to_string()
            .replace('-', "_")
            .replace("posix", "POSIX")
    };
    if let Some(extension) = &opts.get_unicode_ext(&key!("co")) {
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

        if opts.get_langid().language == language!("zh") {
            s.push_str("_pinyin");
        } else if opts.get_langid().language == language!("sv") {
            s.push_str("_reformed");
        } else {
            s.push_str("_standard");
        }
    }
    s
}

fn file_name_to_locale(file_name: &str) -> Option<Locale> {
    let (language, variant) = file_name.rsplit_once('_').unwrap();
    let langid = if language == "root" {
        LanguageIdentifier::UND
    } else {
        language.parse().ok()?
    };
    let mut locale = Locale::from(langid);
    // See above for the two special cases.
    if !((language == "zh" && variant == "pinyin")
        || (language == "sv" && variant == "reformed")
        || ((language != "zh" && language != "sv") && variant == "standard"))
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

/// A data provider reading from .toml files produced by the ICU4C genrb tool.
pub struct CollationProvider {
    source: SourceData,
}

impl From<&SourceData> for CollationProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

macro_rules! collation_provider {
    ($(($marker:ident, $serde_struct:ident, $suffix:literal, $conversion:expr)),+, $toml_data:ident) => {
        $(
            impl DataProvider<$marker> for CollationProvider {
                fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    let $toml_data: &collator_serde::$serde_struct = self
                        .source
                        .icuexport()?
                        .read_and_parse_toml(
                            &format!(
                                "collation/{}/{}{}.toml",
                                self.source.collation_han_database(),
                                locale_to_file_name(&req.options), $suffix)
                        )
                        .map_err(|e| match e.kind {
                            DataErrorKind::Io(
                                std::io::ErrorKind::NotFound
                            ) => DataErrorKind::MissingDataOptions.with_req(
                                $marker::KEY, &req
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

            impl IterableDataProvider<$marker> for CollationProvider {
                fn supported_options(&self) -> Result<Vec<DataOptions>, DataError> {
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
                        .filter_map(|s|file_name_to_locale(&s))
                        .map(DataOptions::from)
                        .collect()
                    )
                }
            }
        )+
        icu_provider::make_exportable_provider!(CollationProvider, [$($marker),+,]);
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
