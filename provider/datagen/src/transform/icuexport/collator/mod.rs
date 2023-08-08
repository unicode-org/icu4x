// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use icu_collator::provider::*;
use icu_collections::codepointtrie::CodePointTrie;
use icu_locid::extensions::unicode::key;
use icu_locid::extensions::unicode::Value;
use icu_locid::subtags::language;
use icu_locid::LanguageIdentifier;
use icu_locid::Locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;
use writeable::Writeable;
use zerovec::ZeroVec;

mod collator_serde;

// Collations removed by default from ICU4X data, plus all starting with "search".
static DEFAULT_REMOVED_COLLATIONS: &[&str] = &["big5han", "gb2312"];

/// Backward compatibility for https://unicode-org.atlassian.net/browse/CLDR-15603
fn has_legacy_swedish_variants(source: &crate::SourceData) -> bool {
    source
        .icuexport()
        .and_then(|i| {
            i.file_exists(&format!(
                "collation/{}/sv_reformed_meta.toml",
                source.collation_han_database,
            ))
        })
        .unwrap_or(false)
}

fn locale_to_file_name(locale: &DataLocale, has_legacy_swedish_variants: bool) -> String {
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
    } else if locale.get_langid().language == language!("zh") {
        // "zh" uses "_pinyin" as the default
        s.push_str("_pinyin");
    } else if has_legacy_swedish_variants && locale.get_langid().language == language!("sv") {
        // "sv" used to use "_reformed" as the default
        // TODO(#2856): Remove when dropping pre-42 support in 2.0
        s.push_str("_reformed");
    } else {
        // Everyting else uses "_standard"
        s.push_str("_standard");
    }
    s
}

fn file_name_to_locale(file_name: &str, has_legacy_swedish_variants: bool) -> Option<Locale> {
    let (language, variant) = file_name.rsplit_once('_').unwrap();
    let langid = if language == "root" {
        LanguageIdentifier::UND
    } else {
        language.parse().ok()?
    };
    let mut locale = Locale::from(langid);

    // See above for the two special cases.
    if language == "zh" {
        if variant == "pinyin" {
            return Some(locale);
        }
    } else if has_legacy_swedish_variants && language == "sv" {
        // TODO(#2856): Remove when dropping pre-42 support in 2.0
        if variant == "reformed" {
            return Some(locale);
        }
    } else if variant == "standard" {
        return Some(locale);
    }

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
    Some(locale)
}

pub(crate) fn filter_data_locales(
    locales: HashSet<DataLocale>,
    collations: &HashSet<String>,
) -> HashSet<DataLocale> {
    locales
        .into_iter()
        .filter(|locale| {
            locale
                .get_unicode_ext(&key!("co"))
                .and_then(|co| co.as_single_subtag().copied())
                .map(|collation| {
                    if collations.contains(collation.as_str()) {
                        true
                    } else if collation.starts_with("search") {
                        // Note: literal "search" and "searchjl" are handled above
                        collations.contains("search*")
                    } else {
                        !DEFAULT_REMOVED_COLLATIONS.contains(&collation.as_str())
                    }
                })
                .unwrap_or(true)
        })
        .collect()
}

macro_rules! collation_provider {
    ($(($marker:ident, $serde_struct:ident, $suffix:literal, $conversion:expr)),+, $toml_data:ident) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let $toml_data: &collator_serde::$serde_struct = self
                        .source
                        .icuexport()?
                        .read_and_parse_toml(&format!(
                            "collation/{}/{}{}.toml",
                            self.source.collation_han_database,
                            locale_to_file_name(&req.locale, has_legacy_swedish_variants(&self.source)),
                            $suffix
                        ))
                        .map_err(|e| match e.kind {
                            DataErrorKind::Io(std::io::ErrorKind::NotFound) => {
                                DataErrorKind::MissingLocale.with_req($marker::KEY, req)
                            }
                            _ => e,
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
                    if <$marker>::KEY.metadata().singleton {
                        return Ok(vec![Default::default()])
                    }
                    Ok(self
                        .source
                        .icuexport()?
                        .list(&format!(
                            "collation/{}",
                            self.source.collation_han_database
                        ))?
                        .filter_map(|mut file_name| {
                            file_name.truncate(file_name.len() - ".toml".len());
                            file_name.ends_with($suffix).then(|| {
                                file_name.truncate(file_name.len() - $suffix.len());
                                file_name
                            })
                        })
                        .filter_map(|s| file_name_to_locale(&s, has_legacy_swedish_variants(&self.source)))
                        .map(DataLocale::from)
                        .collect())
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

#[test]
fn test_collation_filtering() {
    use crate::options;
    use icu_locid::langid;
    use std::collections::BTreeSet;

    #[derive(Debug)]
    struct TestCase<'a> {
        include_collations: &'a [&'a str],
        language: LanguageIdentifier,
        expected: &'a [&'a str],
    }
    let cases = [
        TestCase {
            include_collations: &[],
            language: langid!("zh"),
            expected: &["zh", "zh-u-co-stroke", "zh-u-co-unihan", "zh-u-co-zhuyin"],
        },
        TestCase {
            include_collations: &["gb2312"],
            language: langid!("zh"),
            expected: &[
                "zh",
                "zh-u-co-gb2312",
                "zh-u-co-stroke",
                "zh-u-co-unihan",
                "zh-u-co-zhuyin",
            ],
        },
        TestCase {
            include_collations: &["big5han"],
            language: langid!("zh"),
            expected: &[
                "zh",
                "zh-u-co-big5han",
                "zh-u-co-stroke",
                "zh-u-co-unihan",
                "zh-u-co-zhuyin",
            ],
        },
        TestCase {
            include_collations: &["gb2312", "search*"],
            language: langid!("zh"),
            expected: &[
                "zh",
                "zh-u-co-gb2312",
                "zh-u-co-stroke",
                "zh-u-co-unihan",
                "zh-u-co-zhuyin",
            ],
        },
        TestCase {
            include_collations: &[],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["search"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-search", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["searchjl"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-searchjl", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["search", "searchjl"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-search", "ko-u-co-searchjl", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["search*", "big5han"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-search", "ko-u-co-searchjl", "ko-u-co-unihan"],
        },
    ];
    for cas in cases {
        let provider = crate::DatagenProvider::for_test();
        let mut options = options::Options::default();
        options.collations = cas
            .include_collations
            .iter()
            .copied()
            .map(String::from)
            .collect();
        options.locales =
            crate::options::LocaleInclude::Explicit([cas.language.clone()].into_iter().collect());

        let resolved_locales = provider
            .select_locales_for_key(CollationDataV1Marker::KEY, &options, None)
            .unwrap()
            .into_iter()
            .map(|l| l.to_string())
            .collect::<BTreeSet<_>>();
        let expected_locales = cas
            .expected
            .iter()
            .copied()
            .map(String::from)
            .collect::<BTreeSet<_>>();
        assert_eq!(resolved_locales, expected_locales, "{cas:?}");
    }
}
