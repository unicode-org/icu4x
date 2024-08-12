// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::locale::provider::*;
use icu::locale::{
    subtags::{self, language},
    LanguageIdentifier,
};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use tinystr::TinyAsciiStr;
use zerovec::ZeroSlice;

impl DataProvider<AliasesV2Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<AliasesV2Marker>, DataError> {
        self.check_req::<AliasesV2Marker>(req)?;
        let data: &cldr_serde::aliases::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/aliases.json")?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(AliasesV2::from(data)),
        })
    }
}

impl crate::IterableDataProviderCached<AliasesV2Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

// Sort rules following algorithm in Preprocessing, step 5 of Appendix C:
//   - the size of the union of all field value sets, with largest size first
//   - alphabetically by each field
fn appendix_c_cmp(langid: &LanguageIdentifier) -> impl Ord {
    let mut union_size = langid.variants.len() as i8;
    if !langid.language.is_empty() {
        union_size += 1;
    }
    if langid.script.is_some() {
        union_size += 1;
    }
    if langid.region.is_some() {
        union_size += 1;
    }
    (
        -union_size,
        langid.language,
        langid.script,
        langid.region,
        langid.variants.clone(),
    )
}

impl From<&cldr_serde::aliases::Resource> for AliasesV2<'_> {
    // Step 1. Load the rules from aliases.json
    fn from(other: &cldr_serde::aliases::Resource) -> Self {
        // These all correspond to language aliases in the CLDR data. By storing known
        // special cases in the CLDR data, we can minimize the number of comparisons done
        // for commonly used languages. With the current CLDR data, all aliases end up in
        // a special case, but we retain the catchall language category in case new or
        // customized CLDR data is used.
        let mut language_variants = Vec::new();
        let mut sgn_region = BTreeMap::new();
        let mut language_len2 = BTreeMap::new();
        let mut language_len3 = BTreeMap::new();
        let mut language = Vec::new();

        let mut script = BTreeMap::new();

        // There are many more aliases for numeric region codes than for alphabetic,
        // so by storing them separately, we can minimize comparisons for alphabetic codes.
        let mut region_alpha = BTreeMap::new();
        let mut region_num = BTreeMap::new();

        // Complex regions are cases similar to the Soviet Union, where an old region
        // is replaced by multiple new regions. Determining the new region requires using
        // likely subtags. Many implementations preprocess the complex regions into simple
        // regions as part of data import, but that would introduce a dependency between
        // CDLR providers that we're not currently set up to handle.
        let mut complex_region = BTreeMap::new();

        let mut variant = BTreeMap::new();

        let mut subdivision = BTreeMap::new();

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
                            let lang = langid.language.into_tinystr();
                            if lang.len() == 2 {
                                language_len2.insert(lang.resize(), to.replacement.as_str());
                            } else {
                                language_len3.insert(lang, to.replacement.as_str());
                            }
                        }
                        // sgn-<region> -> <language>
                        (language, None, Some(region), false)
                            if language == language!("sgn")
                                && !replacement.language.is_empty()
                                && replacement.script.is_none()
                                && replacement.region.is_none()
                                && replacement.variants.is_empty() =>
                        {
                            sgn_region.insert(region.into_tinystr(), replacement.language);
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
                script.insert(from, to);
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
                    region_alpha.insert(from.resize(), replacement);
                } else {
                    region_num.insert(from, replacement);
                }
            } else {
                complex_region.insert(
                    from,
                    to.replacement
                        .split(' ')
                        .filter_map(|r| r.parse::<subtags::Region>().ok())
                        .collect::<Box<[_]>>(),
                );
            }
        }

        for (from, to) in other.supplemental.metadata.alias.variant_aliases.iter() {
            if let Ok(to) = to.replacement.parse::<subtags::Variant>() {
                variant.insert(from, to);
            }
        }

        for (from, to) in other.supplemental.metadata.alias.subdivision_aliases.iter() {
            if let Some(replacement) = to.replacement.split(' ').find_map(|r| {
                if r.len() == 2 {
                    // Following http://unicode.org/reports/tr35/#Canonical_Unicode_Locale_Identifiers,
                    // append "zzzz" to make this syntactically correct.
                    let replacement = r.to_string().to_ascii_lowercase() + "zzzz";
                    TinyAsciiStr::<7>::try_from_str(&replacement).ok()
                } else {
                    TinyAsciiStr::<7>::try_from_str(r).ok()
                }
            }) {
                subdivision.insert(from, replacement);
            }
        }

        // 5. Sort the non-special-cased rules
        language_variants.sort_unstable_by_key(|(langid, _)| appendix_c_cmp(langid));
        language.sort_unstable_by_key(|(langid, _)| appendix_c_cmp(langid));

        let language_variants = language_variants
            .iter()
            .map(|(from, to)| {
                LanguageStrStrPair(
                    from.language,
                    from.variants.to_string().into(),
                    to.to_string().into(),
                )
            })
            .collect::<Vec<_>>();
        let language = language
            .iter()
            .map(|(from, to)| StrStrPair(from.to_string().into(), to.to_string().into()))
            .collect::<Vec<_>>();

        Self {
            language_variants: language_variants.as_slice().into(),
            sgn_region: sgn_region
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            language_len2: language_len2
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            language_len3: language_len3
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            language: language.as_slice().into(),

            script: script
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),

            region_alpha: region_alpha
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            region_num: region_num
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            complex_region: complex_region
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), ZeroSlice::from_boxed_slice(v)))
                .collect(),

            variant: variant
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),

            subdivision: subdivision
                .into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
        }
    }
}

#[test]
fn test_appendix_c_cmp() {
    let en = icu::locale::langid!("en-GB");
    let ca = icu::locale::langid!("ca");
    let und = "und-hepburn-heploc".parse::<LanguageIdentifier>().unwrap();
    let fr = icu::locale::langid!("fr-CA");

    let mut rules = vec![&en, &ca, &und, &fr];
    rules.sort_unstable_by_key(|&l| appendix_c_cmp(l));

    assert_eq!(rules, &[&en, &fr, &und, &ca]);
}

#[test]
fn test_basic() {
    use icu::locale::subtags::{language, region, script};

    let provider = SourceDataProvider::new_testing();
    let data: DataResponse<AliasesV2Marker> = provider.load(Default::default()).unwrap();

    // We should handle all language rules as special cases, leaving the generic category empty.
    assert!(data.payload.get().language.is_empty());

    // We should have data in all other categories
    assert!(!data.payload.get().language_variants.is_empty());
    assert!(!data.payload.get().sgn_region.is_empty());
    assert!(!data.payload.get().language_len2.is_empty());
    assert!(!data.payload.get().language_len3.is_empty());
    assert!(!data.payload.get().script.is_empty());
    assert!(!data.payload.get().region_alpha.is_empty());
    assert!(!data.payload.get().region_num.is_empty());
    assert!(!data.payload.get().complex_region.is_empty());
    assert!(!data.payload.get().variant.is_empty());
    assert!(!data.payload.get().subdivision.is_empty());

    // Spot check a few expected results. There are more extensive tests in the
    // locale canonicalizer itself.
    assert_eq!(
        data.payload
            .get()
            .language_len2
            .get(&language!("iw").into_tinystr().resize().to_unvalidated())
            .unwrap(),
        "he"
    );

    assert!(data
        .payload
        .get()
        .language_len3
        .get(&language!("iw").into_tinystr().to_unvalidated())
        .is_none());

    assert_eq!(
        data.payload.get().script.iter().next().unwrap(),
        (
            &script!("Qaai").into_tinystr().to_unvalidated(),
            &script!("Zinh")
        )
    );

    assert_eq!(
        data.payload
            .get()
            .region_num
            .get(&region!("768").into_tinystr().to_unvalidated())
            .unwrap(),
        &region!("TG")
    );
}
