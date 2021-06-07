// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::reader::open_reader;
use crate::CldrPaths;
use icu_locale_canonicalizer::provider::*;
use icu_locid::{subtags, LanguageIdentifier};
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::marker::PhantomData;
use tinystr::{TinyStr4, TinyStr8};

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [key::ALIASES_V1];

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(PartialEq, Debug)]
pub struct AliasesProvider<'d> {
    data: cldr_json::Resource,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for AliasesProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let data: cldr_json::Resource = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("aliases.json");
            serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?
        };
        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> TryFrom<&'d str> for AliasesProvider<'d> {
    type Error = serde_json::error::Error;
    /// Attempt to parse a JSON string.
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        let data: cldr_json::Resource = serde_json::from_str(s)?;
        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> KeyedDataProvider for AliasesProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::Aliases || resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d, 's> DataProvider<'d, 's, AliasesV1Marker> for AliasesProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, AliasesV1Marker>, DataError> {
        AliasesProvider::supports_key(&req.resource_path.key)?;
        let langid = &req.resource_path.options.langid;

        // We treat searching for und as a request for all data. Other requests
        // are not currently supported.
        if langid.is_none() {
            Ok(DataResponse {
                metadata: DataResponseMetadata {
                    data_langid: langid.clone(),
                },
                payload: Some(DataPayload::from_owned(AliasesV1::from(&self.data))),
            })
        } else {
            Err(DataError::UnavailableResourceOptions(req.clone()))
        }
    }
}

icu_provider::impl_dyn_provider!(AliasesProvider<'d>, {
    _ => AliasesV1Marker,
}, SERDE_SE, 'd, 's);

impl<'d> IterableDataProviderCore for AliasesProvider<'d> {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
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

impl From<&cldr_json::Resource> for AliasesV1 {
    // Step 1. Load the rules from aliases.json
    fn from(other: &cldr_json::Resource) -> Self {
        // These all correspond to language aliases in the CLDR data. By storing known
        // special cases in the CLDR data, we can minimize the number of comparisons done
        // for commonly used languages. With the current CLDR data, all aliases end up in
        // a special case, but we retain the catchall language category in case new or
        // customized CLDR data is used.
        let mut language: Vec<(LanguageIdentifier, LanguageIdentifier)> = Vec::new();
        let mut language_variants: Vec<(LanguageIdentifier, LanguageIdentifier)> = Vec::new();
        let mut language_region: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut sgn_region: Vec<(TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut language_len2: Vec<(TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut language_len3: Vec<(TinyStr4, LanguageIdentifier)> = Vec::new();

        let mut script: Vec<(TinyStr4, TinyStr4)> = Vec::new();

        // There are many more aliases for numeric region codes than for alphabetic,
        // so by storing them separately, we can minimize comparisons for alphabetic codes.
        let mut region_alpha: Vec<(TinyStr4, TinyStr4)> = Vec::new();
        let mut region_num: Vec<(TinyStr4, TinyStr4)> = Vec::new();

        // Complex regions are cases similar to the Soviet Union, where an old region
        // is replaced by multiple new regions. Determining the new region requires using
        // likely subtags. Many implementations preprocess the complex regions into simple
        // regions as part of data import, but that would introduce a dependency between
        // CDLR providers that we're not currently set up to handle.
        let mut complex_region: Vec<(TinyStr4, Vec<TinyStr4>)> = Vec::new();

        let mut variant: Vec<(TinyStr8, TinyStr8)> = Vec::new();
        let mut subdivision: Vec<(TinyStr8, TinyStr8)> = Vec::new();

        // Step 2. Capture all languageAlias rules where the type is an invalid languageId
        // into a set of BCP47 LegacyRules. This implementation discards these.
        // Step 3. Discard all rules where the type is an invalid languageId
        for alias in other.supplemental.metadata.alias.language_aliases.iter() {
            if let Ok(langid) = alias.0.parse::<LanguageIdentifier>() {
                if let Ok(replacement) = alias.1.replacement.parse::<LanguageIdentifier>() {
                    // Variants are stored separately to not slow down canonicalization
                    // of locales without variants.
                    if !langid.variants.is_empty() {
                        language_variants.push((langid, replacement));
                        continue;
                    }

                    let maybe_lang: Option<TinyStr4> = langid.language.into();
                    if let Some(lang) = maybe_lang {
                        if langid.region.is_none() && langid.variants.is_empty() {
                            // Relatively few aliases exist for two character language identifiers,
                            // so we store them separately to not slow down canonicalization of
                            // common identifiers.
                            if lang.len() == 2 {
                                language_len2.push((lang, replacement));
                            } else {
                                language_len3.push((lang, replacement));
                            }
                        } else if let Some(region) = langid.region {
                            // All current language-region aliases are for "sgn", so we store them
                            // separately to not slow down canonicalization of common identifiers.
                            if lang == "sgn" {
                                sgn_region.push((region.into(), replacement));
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

        for alias in other.supplemental.metadata.alias.script_aliases.iter() {
            // Don't store data for invalid script codes, we only canonicalize valid locales, so we
            // would never see these anyways.
            if alias.0.parse::<subtags::Script>().is_err() {
                continue;
            }

            script.push((alias.0, alias.1.replacement));
        }

        for alias in other.supplemental.metadata.alias.region_aliases.iter() {
            // Don't store data for invalid region codes, we only canonicalize valid locales, so we
            // would never see these anyways.
            if alias.0.parse::<subtags::Region>().is_err() {
                continue;
            }

            if let Ok(replacement) = alias.1.replacement.parse::<TinyStr4>() {
                if alias.0.is_ascii_alphabetic() {
                    region_alpha.push((alias.0, replacement));
                } else {
                    region_num.push((alias.0, replacement));
                }
            } else {
                complex_region.push((
                    alias.0,
                    alias
                        .1
                        .replacement
                        .split(' ')
                        .into_iter()
                        .filter_map(|r| r.parse::<TinyStr4>().ok())
                        .collect(),
                ));
            }
        }

        for alias in other.supplemental.metadata.alias.variant_aliases.iter() {
            variant.push((alias.0, alias.1.replacement));
        }

        for alias in other.supplemental.metadata.alias.subdivision_aliases.iter() {
            if let Some(replacement) = alias
                .1
                .replacement
                .split(' ')
                .into_iter()
                .filter_map(|r| {
                    if r.len() == 2 {
                        // Following http://unicode.org/reports/tr35/#Canonical_Unicode_Locale_Identifiers,
                        // append "zzzz" to make this syntactically correct.
                        let replacement = r.to_string().to_ascii_lowercase() + "zzzz";
                        TinyStr8::from_bytes(replacement.as_bytes()).ok()
                    } else {
                        TinyStr8::from_bytes(r.as_bytes()).ok()
                    }
                })
                .next()
            {
                subdivision.push((alias.0, replacement));
            }
        }

        // 5. Order the set of rules by
        //      1. the size of the union of all field value sets, with largest size first
        //      2. and then alphabetically by field.
        language.sort_unstable_by(|a, b| rules_cmp(&a.0, &b.0));
        language_variants.sort_unstable_by(|a, b| rules_cmp(&a.0, &b.0));
        language_region.sort_unstable();
        sgn_region.sort_unstable();
        language_len2.sort_unstable();
        language_len3.sort_unstable();
        region_alpha.sort_unstable();
        region_num.sort_unstable();
        complex_region.sort_unstable();
        variant.sort_unstable();
        subdivision.sort_unstable();

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

/// Serde structs for the CLDR JSON aliases file.
pub(self) mod cldr_json {
    use serde::Deserialize;
    use tinystr::{TinyStr4, TinyStr8};

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Replacement<T> {
        #[serde(rename = "_replacement")]
        pub replacement: T,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Alias {
        #[serde(with = "tuple_vec_map", rename = "languageAlias")]
        pub language_aliases: Vec<(String, Replacement<String>)>,
        #[serde(with = "tuple_vec_map", rename = "scriptAlias")]
        pub script_aliases: Vec<(TinyStr4, Replacement<TinyStr4>)>,
        #[serde(with = "tuple_vec_map", rename = "territoryAlias")]
        pub region_aliases: Vec<(TinyStr4, Replacement<String>)>,
        #[serde(with = "tuple_vec_map", rename = "variantAlias")]
        pub variant_aliases: Vec<(TinyStr8, Replacement<TinyStr8>)>,
        #[serde(with = "tuple_vec_map", rename = "subdivisionAlias")]
        pub subdivision_aliases: Vec<(TinyStr8, Replacement<String>)>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Metadata {
        pub alias: Alias,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Supplemental {
        pub metadata: Metadata,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub supplemental: Supplemental,
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

    rules.sort_unstable_by(|a, b| rules_cmp(a, b));
    assert_eq!(rules[0], "und-hepburn-heploc");
    assert_eq!(rules[1], "en-GB");
    assert_eq!(rules[2], "fr-CA");
    assert_eq!(rules[3], "CA");
}

#[test]
fn test_basic() {
    use std::str::FromStr;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = AliasesProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();
    let data: DataPayload<AliasesV1Marker> = provider
        .load_payload(&DataRequest::from(key::ALIASES_V1))
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
    let lang = TinyStr4::from_str("iw").unwrap();
    let res = data
        .get()
        .language_len2
        .binary_search_by_key(&lang, |alias| alias.0);
    assert!(res.is_ok());
    assert_eq!(data.get().language_len2[res.unwrap()].0, "iw");
    assert_eq!(data.get().language_len2[res.unwrap()].1, "he");

    let res = data
        .get()
        .language_len3
        .binary_search_by_key(&lang, |alias| alias.0);
    assert!(res.is_err());

    assert_eq!(data.get().script[0].0, "Qaai");
    assert_eq!(data.get().script[0].1, "Zinh");

    let region = TinyStr4::from_str("768").unwrap();
    let res = data
        .get()
        .region_alpha
        .binary_search_by_key(&region, |alias| alias.0);
    assert!(res.is_err());

    let res = data
        .get()
        .region_num
        .binary_search_by_key(&region, |alias| alias.0);
    assert!(res.is_ok());
    assert_eq!(data.get().region_num[res.unwrap()].0, "768");
    assert_eq!(data.get().region_num[res.unwrap()].1, "TG");
}
