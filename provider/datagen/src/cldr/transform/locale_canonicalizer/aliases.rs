// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr::cldr_serde;
use crate::cldr::error::Error;
use crate::cldr::reader::open_reader;
use crate::cldr::CldrPaths;
use icu_locale_canonicalizer::provider::*;
use icu_locid::{subtags, LanguageIdentifier};
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use tinystr::TinyAsciiStr;

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(Debug)]
pub struct AliasesProvider {
    path: PathBuf,
}

impl TryFrom<&dyn CldrPaths> for AliasesProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        Ok(Self {
            path: cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("aliases.json"),
        })
    }
}

impl ResourceProvider<AliasesV1Marker> for AliasesProvider {
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<AliasesV1Marker>, DataError> {
        let langid = &req.options.langid;

        let data: cldr_serde::aliases::Resource = serde_json::from_reader(open_reader(&self.path)?)
            .map_err(|e| Error::Json(e, Some(self.path.clone())))?;

        // We treat searching for `und` as a request for all data. Other requests
        // are not currently supported.
        if langid.is_none() {
            let metadata = DataResponseMetadata::default();
            // TODO(#1109): Set metadata.data_langid correctly.
            Ok(DataResponse {
                metadata,
                payload: Some(DataPayload::from_owned(AliasesV1::from(&data))),
            })
        } else {
            Err(DataErrorKind::ExtraneousResourceOptions.with_req(AliasesV1Marker::KEY, req))
        }
    }
}

icu_provider::impl_dyn_provider!(
    AliasesProvider,
    [AliasesV1Marker,],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

impl IterableResourceProvider<AliasesV1Marker> for AliasesProvider {
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
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

impl From<&cldr_serde::aliases::Resource> for AliasesV1 {
    // Step 1. Load the rules from aliases.json
    fn from(other: &cldr_serde::aliases::Resource) -> Self {
        // These all correspond to language aliases in the CLDR data. By storing known
        // special cases in the CLDR data, we can minimize the number of comparisons done
        // for commonly used languages. With the current CLDR data, all aliases end up in
        // a special case, but we retain the catchall language category in case new or
        // customized CLDR data is used.
        let mut language = Vec::new();
        let mut language_variants = Vec::new();
        let mut sgn_region = LiteMap::new();
        let mut language_len2 = LiteMap::new();
        let mut language_len3 = LiteMap::new();

        let mut script = LiteMap::new();

        // There are many more aliases for numeric region codes than for alphabetic,
        // so by storing them separately, we can minimize comparisons for alphabetic codes.
        let mut region_alpha = LiteMap::new();
        let mut region_num = LiteMap::new();

        // Complex regions are cases similar to the Soviet Union, where an old region
        // is replaced by multiple new regions. Determining the new region requires using
        // likely subtags. Many implementations preprocess the complex regions into simple
        // regions as part of data import, but that would introduce a dependency between
        // CDLR providers that we're not currently set up to handle.
        let mut complex_region = LiteMap::new();

        let mut variant = LiteMap::new();
        let mut subdivision = LiteMap::new();

        // Step 2. Capture all languageAlias rules where the type is an invalid languageId
        // into a set of BCP47 LegacyRules. This implementation discards these.
        // Step 3. Discard all rules where the type is an invalid languageId
        for (from, to) in other.supplemental.metadata.alias.language_aliases.iter() {
            if let Ok(langid) = from.parse::<LanguageIdentifier>() {
                if let Ok(replacement) = to.replacement.parse::<LanguageIdentifier>() {
                    // Variants are stored separately to not slow down canonicalization
                    // of locales without variants.
                    if !langid.variants.is_empty() {
                        language_variants.push((langid, replacement));
                        continue;
                    }

                    if !langid.language.is_empty() {
                        let lang: TinyAsciiStr<3> = langid.language.into();
                        if langid.region.is_none() && langid.variants.is_empty() {
                            // Relatively few aliases exist for two character language identifiers,
                            // so we store them separately to not slow down canonicalization of
                            // common identifiers.
                            if lang.len() == 2 {
                                language_len2.insert(lang.resize(), replacement);
                            } else {
                                language_len3.insert(lang, replacement);
                            }
                        } else if let Some(region) = langid.region {
                            // All current language-region aliases are for "sgn", so we store them
                            // separately to not slow down canonicalization of common identifiers.
                            if lang == "sgn" {
                                sgn_region.insert(region.into(), replacement);
                            } else {
                                language.push((langid, replacement));
                            }
                        } else {
                            language.push((langid, replacement));
                        }
                    } else {
                        language.push((langid, replacement));
                    }
                }
            }
        }

        for (from, to) in other.supplemental.metadata.alias.script_aliases.iter() {
            // Don't store data for invalid script codes, we only canonicalize valid locales, so we
            // would never see these anyways.
            if from.parse::<subtags::Script>().is_err() {
                continue;
            }

            script.insert(*from, to.replacement);
        }

        for (from, to) in other.supplemental.metadata.alias.region_aliases.iter() {
            // Don't store data for invalid region codes, we only canonicalize valid locales, so we
            // would never see these anyways.
            if from.parse::<subtags::Region>().is_err() {
                continue;
            }

            if let Ok(replacement) = to.replacement.parse::<TinyAsciiStr<3>>() {
                if from.is_ascii_alphabetic() {
                    region_alpha.insert(from.resize(), replacement);
                } else {
                    region_num.insert(*from, replacement);
                }
            } else {
                complex_region.insert(
                    *from,
                    to.replacement
                        .split(' ')
                        .into_iter()
                        .filter_map(|r| r.parse::<TinyAsciiStr<3>>().ok())
                        .collect(),
                );
            }
        }

        for (from, to) in other.supplemental.metadata.alias.variant_aliases.iter() {
            variant.insert(*from, to.replacement);
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
                subdivision.insert(*from, replacement);
            }
        }

        // 5. Order the set of rules by
        //      1. the size of the union of all field value sets, with largest size first
        //      2. and then alphabetically by field.
        language.sort_unstable_by(|a, b| rules_cmp(&a.0, &b.0));
        language_variants.sort_unstable_by(|a, b| rules_cmp(&a.0, &b.0));

        Self {
            language,
            language_variants,
            sgn_region,
            language_len2,
            language_len3,
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
        "en-GB".parse().unwrap(),
        "CA".parse().unwrap(),
        "und-hepburn-heploc".parse().unwrap(),
        "fr-CA".parse().unwrap(),
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

    let cldr_paths = crate::cldr::cldr_paths::for_test();
    let provider = AliasesProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();
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

    assert_eq!(data.get().script.iter().next().unwrap().0, "Qaai");
    assert_eq!(data.get().script.iter().next().unwrap().1, "Zinh");

    assert_eq!(data.get().region_num.get(&tinystr!(3, "768")).unwrap(), "TG");
}
