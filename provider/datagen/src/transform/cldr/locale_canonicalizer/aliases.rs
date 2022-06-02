// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::transform::reader::open_reader;
use crate::SourceData;
use icu_locale_canonicalizer::provider::*;
use icu_locid::{language, subtags, LanguageIdentifier};
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use zerovec::{ZeroMap, ZeroSlice};

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(Debug)]
pub struct AliasesProvider {
    source: SourceData,
}

impl From<&SourceData> for AliasesProvider {
    fn from(source: &SourceData) -> Self {
        AliasesProvider {
            source: source.clone(),
        }
    }
}

impl ResourceProvider<AliasesV1Marker> for AliasesProvider {
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<AliasesV1Marker>, DataError> {
        // We treat searching for `und` as a request for all data. Other requests
        // are not currently supported.
        if !req.options.is_empty() {
            return Err(DataErrorKind::ExtraneousResourceOptions.into_error());
        }

        let path = self
            .source
            .get_cldr_paths()?
            .cldr_core()
            .join("supplemental")
            .join("aliases.json");
        let data: cldr_serde::aliases::Resource = serde_json::from_reader(open_reader(&path)?)
            .map_err(|e| DataError::from(e).with_path_context(&path))?;

        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(AliasesV1::from(&data))),
        })
    }
}

icu_provider::make_exportable_provider!(AliasesProvider, [AliasesV1Marker,]);

impl IterableResourceProvider<AliasesV1Marker> for AliasesProvider {
    fn supported_options(&self) -> Result<Vec<ResourceOptions>, DataError> {
        Ok(vec![Default::default()])
    }
}

// The size of the union of all field value sets.
fn union_size(langid: &LanguageIdentifier) -> usize {
    let mut size = langid.variants.len();
    if !langid.language.is_empty() {
        size += 1;
    }
    if langid.script.is_some() {
        size += 1;
    }
    if langid.region.is_some() {
        size += 1;
    }
    size
}

// Sort rules by size of union of field sets and alphabeticaly
// following rules in Preprocessing, step 5 of Appendix C.
fn rules_cmp(a: &LanguageIdentifier, b: &LanguageIdentifier) -> std::cmp::Ordering {
    let size_a = union_size(a);
    let size_b = union_size(b);
    if size_a == size_b {
        a.cmp(b)
    } else {
        size_b.cmp(&size_a)
    }
}

impl From<&cldr_serde::aliases::Resource> for AliasesV1<'_> {
    // Step 1. Load the rules from aliases.json
    fn from(other: &cldr_serde::aliases::Resource) -> Self {
        // These all correspond to language aliases in the CLDR data. By storing known
        // special cases in the CLDR data, we can minimize the number of comparisons done
        // for commonly used languages. With the current CLDR data, all aliases end up in
        // a special case, but we retain the catchall language category in case new or
        // customized CLDR data is used.
        let mut language_variants = Vec::new();
        let mut sgn_region = ZeroMap::new();
        let mut language_len2 = ZeroMap::new();
        let mut language_len3 = ZeroMap::new();
        let mut language = Vec::new();

        let mut script = ZeroMap::new();

        // There are many more aliases for numeric region codes than for alphabetic,
        // so by storing them separately, we can minimize comparisons for alphabetic codes.
        let mut region_alpha = ZeroMap::new();
        let mut region_num = ZeroMap::new();

        // Complex regions are cases similar to the Soviet Union, where an old region
        // is replaced by multiple new regions. Determining the new region requires using
        // likely subtags. Many implementations preprocess the complex regions into simple
        // regions as part of data import, but that would introduce a dependency between
        // CDLR providers that we're not currently set up to handle.
        let mut complex_region = ZeroMap::new();

        let mut variant = ZeroMap::new();

        let mut subdivision = ZeroMap::new();

        // Step 2. Capture all languageAlias rules where the type is an invalid languageId
        // into a set of BCP47 LegacyRules. This implementation discards these.
        // Step 3. Discard all rules where the type is an invalid languageId
        for (from, to) in other.supplemental.metadata.alias.language_aliases.iter() {
            if let Ok(langid) = from.parse::<LanguageIdentifier>() {
                if let Ok(replacement) = to.replacement.parse::<LanguageIdentifier>() {
                    match (
                        langid.language,
                        langid.script,
                        langid.region,
                        !langid.variants.is_empty(),
                    ) {
                        // Anything that has a variant needs to be parsed at runtime, so we isolate
                        // these in their own map.
                        (_, None, None, true) => language_variants.push((langid, replacement)),
                        // <language> -> <language identifier>
                        (lang, None, None, false) if !lang.is_empty() => {
                            // Relatively few aliases exist for two character language identifiers,
                            // so we store them separately to not slow down canonicalization of
                            // common identifiers.
                            let lang: TinyAsciiStr<3> = langid.language.into();
                            if lang.len() == 2 {
                                language_len2.insert(&lang.resize(), to.replacement.as_str());
                            } else {
                                language_len3.insert(&lang, to.replacement.as_str());
                            }
                        }
                        // sgn-<region> -> <language>
                        (language, None, Some(region), false)
                            if language == language!("sgn")
                                && !replacement.language.is_empty()
                                && replacement == replacement.language.as_str() =>
                        {
                            sgn_region.insert(&region.into(), &replacement.language);
                        }
                        _ => language.push((langid, replacement)),
                    }
                }
            }
        }

        if !language.is_empty() {
            panic!("Aliases contain a non-special-cased rule. Remove this check if that is intended behaviour.")
        }

        for (from, to) in other.supplemental.metadata.alias.script_aliases.iter() {
            // Don't store data for invalid script codes, we only canonicalize valid locales, so we
            // would never see these anyways.
            if from.parse::<subtags::Script>().is_err() {
                continue;
            }

            if let Ok(to) = to.replacement.parse::<subtags::Script>() {
                script.insert(from, &to);
            }
        }

        for (from, to) in other.supplemental.metadata.alias.region_aliases.iter() {
            // Don't store data for invalid region codes, we only canonicalize valid locales, so we
            // would never see these anyways.
            if from.parse::<subtags::Region>().is_err() {
                continue;
            }

            if let Ok(replacement) = to.replacement.parse::<subtags::Region>() {
                if from.is_ascii_alphabetic() {
                    region_alpha.insert(&from.resize(), &replacement);
                } else {
                    region_num.insert(from, &replacement);
                }
            } else {
                complex_region.insert(
                    from,
                    ZeroSlice::from_boxed_slice(
                        to.replacement
                            .split(' ')
                            .into_iter()
                            .filter_map(|r| r.parse::<subtags::Region>().ok())
                            .collect::<Box<[_]>>(),
                    )
                    .as_ref(),
                );
            }
        }

        for (from, to) in other.supplemental.metadata.alias.variant_aliases.iter() {
            if let Ok(to) = to.replacement.parse::<subtags::Variant>() {
                variant.insert(from, &to);
            }
        }

        for (from, to) in other.supplemental.metadata.alias.subdivision_aliases.iter() {
            if let Some(replacement) = to
                .replacement
                .split(' ')
                .into_iter()
                .filter_map(|r| {
                    if r.len() == 2 {
                        // Following http://unicode.org/reports/tr35/#Canonical_Unicode_Locale_Identifiers,
                        // append "zzzz" to make this syntactically correct.
                        let replacement = r.to_string().to_ascii_lowercase() + "zzzz";
                        <TinyAsciiStr<7>>::from_bytes(replacement.as_bytes()).ok()
                    } else {
                        <TinyAsciiStr<7>>::from_bytes(r.as_bytes()).ok()
                    }
                })
                .next()
            {
                subdivision.insert(from, &replacement);
            }
        }

        // 5. Order the set of rules by
        //      1. the size of the union of all field value sets, with largest size first
        //      2. and then alphabetically by field.
        language_variants.sort_unstable_by(|a, b| rules_cmp(&a.0, &b.0));
        language.sort_unstable_by(|a, b| rules_cmp(&a.0, &b.0));

        let language_variants = zerovec::VarZeroVec::Owned(
            zerovec::vecs::VarZeroVecOwned::try_from_elements(
                &language_variants
                    .into_iter()
                    .map(|(from, to)| StrStrPair(from.to_string().into(), to.to_string().into()))
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        );

        let language = zerovec::VarZeroVec::Owned(
            zerovec::vecs::VarZeroVecOwned::try_from_elements(
                &language
                    .into_iter()
                    .map(|(from, to)| StrStrPair(from.to_string().into(), to.to_string().into()))
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        );

        Self {
            language_variants,
            sgn_region,
            language_len2,
            language_len3,
            language,

            script,

            region_alpha,
            region_num,
            complex_region,

            variant,

            subdivision,
        }
    }
}

#[test]
fn test_rules_cmp() {
    let mut rules: Vec<LanguageIdentifier> = vec![
        icu_locid::langid!("en-GB"),
        icu_locid::langid!("CA"),
        "und-hepburn-heploc".parse().unwrap(),
        icu_locid::langid!("fr-CA"),
    ];

    assert_eq!(union_size(&rules[0]), 2);
    assert_eq!(union_size(&rules[1]), 1);
    assert_eq!(union_size(&rules[2]), 2);
    assert_eq!(union_size(&rules[3]), 2);

    rules.sort_unstable_by(rules_cmp);
    assert_eq!(rules[0], "en-GB");
    assert_eq!(rules[1], "fr-CA");
    assert_eq!(rules[2], "und-hepburn-heploc");
    assert_eq!(rules[3], "CA");
}

#[test]
fn test_basic() {
    use tinystr::tinystr;

    let provider = AliasesProvider::from(&SourceData::for_test());
    let data: DataPayload<AliasesV1Marker> = provider
        .load_resource(&DataRequest::default())
        .unwrap()
        .take_payload()
        .unwrap();

    // We should handle all language rules as special cases, leaving the generic category empty.
    assert!(data.get().language.is_empty());

    // We should have data in all other categories
    assert!(!data.get().language_variants.is_empty());
    assert!(!data.get().sgn_region.is_empty());
    assert!(!data.get().language_len2.is_empty());
    assert!(!data.get().language_len3.is_empty());
    assert!(!data.get().script.is_empty());
    assert!(!data.get().region_alpha.is_empty());
    assert!(!data.get().region_num.is_empty());
    assert!(!data.get().complex_region.is_empty());
    assert!(!data.get().variant.is_empty());
    assert!(!data.get().subdivision.is_empty());

    // Spot check a few expected results. There are more extensive tests in the
    // locale canonicalizer itself.
    assert_eq!(
        data.get().language_len2.get(&tinystr!(2, "iw")).unwrap(),
        "he"
    );

    assert!(data.get().language_len3.get(&tinystr!(3, "iw")).is_none());

    assert_eq!(
        data.get().script.iter().next().unwrap(),
        (&tinystr!(4, "Qaai"), &icu_locid::script!("Zinh"))
    );

    assert_eq!(
        data.get().region_num.get(&tinystr!(3, "768")).unwrap(),
        &icu_locid::region!("TG")
    );
}
