// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use icu_locid::{
    extensions::unicode::{Key, Value},
    subtags, LanguageIdentifier, Locale,
};
use icu_provider::prelude::*;
use tinystr::{tinystr4, TinyStr4, TinyStr8};

/// Used to track the result of a canonicalization operation that potentially modifies its argument in place.
#[derive(Debug, PartialEq)]
pub enum CanonicalizationResult {
    Modified,
    Unmodified,
}

pub struct LocaleCanonicalizer<'d, 's> {
    aliases: DataPayload<'d, 's, AliasesV1Marker>,
    likely_subtags: DataPayload<'d, 's, LikelySubtagsV1Marker>,
    extension_keys: Vec<Key>,
}

#[inline]
fn uts35_rule_matches(source: &Locale, ruletype: &LanguageIdentifier) -> bool {
    (ruletype.language.is_empty() || ruletype.language == source.id.language)
        && (ruletype.script.is_none() || ruletype.script == source.id.script)
        && (ruletype.region.is_none() || ruletype.region == source.id.region)
        && (ruletype.variants.is_empty()
            || ruletype
                .variants
                .iter()
                .all(|v| source.id.variants.contains(v)))
}

fn uts35_replacement(
    source: &mut Locale,
    ruletype_has_language: bool,
    ruletype_has_script: bool,
    ruletype_has_region: bool,
    ruletype_variants: Option<&subtags::Variants>,
    replacement: &LanguageIdentifier,
) {
    if ruletype_has_language || (source.id.language.is_empty() && !replacement.language.is_empty())
    {
        source.id.language = replacement.language;
    }
    if ruletype_has_script || (source.id.script.is_none() && replacement.script.is_some()) {
        source.id.script = replacement.script;
    }
    if ruletype_has_region || (source.id.region.is_none() && replacement.region.is_some()) {
        source.id.region = replacement.region;
    }
    if let Some(ruletype_variants) = ruletype_variants {
        // The rule matches if the ruletype variants are a subset of the source variants.
        // This means ja-Latn-fonipa-hepburn-heploc matches against the rule for
        // hepburn-heploc and is canonicalized to ja-Latn-alalc97-fonipa
        let mut variants: Vec<subtags::Variant> = source
            .id
            .variants
            .iter()
            .filter(|x| !ruletype_variants.contains(x))
            .copied()
            .collect();
        for variant in replacement.variants.iter() {
            variants.push(*variant);
        }
        variants.sort();
        variants.dedup();
        source.id.variants = subtags::Variants::from_vec_unchecked(variants);
    }
}

#[inline]
fn uts35_check_language_rules(
    locale: &mut Locale,
    alias_data: &DataPayload<AliasesV1Marker>,
) -> CanonicalizationResult {
    let maybe_lang: Option<TinyStr4> = locale.id.language.into();
    if let Some(lang) = maybe_lang {
        let aliases = if lang.len() == 2 {
            &alias_data.get().language_len2
        } else {
            &alias_data.get().language_len3
        };

        if let Ok(index) = aliases.binary_search_by_key(&lang, |alias| alias.0) {
            uts35_replacement(locale, true, false, false, None, &aliases[index].1);
            return CanonicalizationResult::Modified;
        }
    }

    CanonicalizationResult::Unmodified
}

#[inline]
fn update_langid(
    entry: &LanguageIdentifier,
    langid: &mut LanguageIdentifier,
) -> CanonicalizationResult {
    if langid.language.is_empty() {
        langid.language = entry.language;
    }
    langid.script = langid.script.or(entry.script);
    langid.region = langid.region.or(entry.region);
    CanonicalizationResult::Modified
}

macro_rules! maximize_locale {
    ( $langid:ident, $table:expr, $key:expr ) => {{
        if let Ok(index) = $table.binary_search_by_key(&&$key, |(i1, _)| i1) {
            let entry = &$table[index].1;
            return update_langid(entry, $langid);
        }
    }};
    ( $langid:ident, $table:expr, $key1:expr, $key2:expr ) => {{
        if let Ok(index) = $table.binary_search_by_key(&(&$key1, &$key2), |(i1, i2, _)| (i1, i2)) {
            let entry = &$table[index].2;
            return update_langid(entry, $langid);
        }
    }};
}

impl<'d, 's> LocaleCanonicalizer<'d, 's> {
    /// A constructor which takes a [`DataProvider`] and creates a [`LocaleCanonicalizer`].
    pub fn new<P>(provider: &P) -> Result<LocaleCanonicalizer<'d, 's>, DataError>
    where
        P: DataProvider<'d, 's, AliasesV1Marker>
            + DataProvider<'d, 's, LikelySubtagsV1Marker>
            + ?Sized,
    {
        // The `rg` region override and `sd` regional subdivision keys may contain
        // language codes that require canonicalization.
        let extension_keys = vec![
            Key::from_tinystr4_unchecked(tinystr4!("rg")),
            Key::from_tinystr4_unchecked(tinystr4!("sd")),
        ];
        let aliases: DataPayload<AliasesV1Marker> = provider
            .load_payload(&DataRequest::from(key::ALIASES_V1))?
            .take_payload()?;

        let likely_subtags: DataPayload<LikelySubtagsV1Marker> = provider
            .load_payload(&DataRequest::from(key::LIKELY_SUBTAGS_V1))?
            .take_payload()?;

        Ok(LocaleCanonicalizer {
            aliases,
            likely_subtags,
            extension_keys,
        })
    }

    /// The canonicalize method potentially updates a passed in locale in place
    /// depending up the results of running the canonicalization algorithm
    /// from http://unicode.org/reports/tr35/#LocaleId_Canonicalization
    ///
    /// Some BCP47 canonicalization data is not part of the CLDR json package. Because
    /// of this, some canonicalizations are not performed, e.g. the canonicalization of
    /// `und-u-ca-islamicc` to `und-u-ca-islamic-civil`. This will be fixed in a future
    /// release once the missing data has been added to the CLDR json data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider)
    ///     .expect("create failed");
    ///
    /// let mut locale : Locale = "ja-Latn-fonipa-hepburn-heploc".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.canonicalize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "ja-Latn-alalc97-fonipa");
    /// ```
    ///
    pub fn canonicalize(&self, locale: &mut Locale) -> CanonicalizationResult {
        let mut result = CanonicalizationResult::Unmodified;

        // This loops until we get a 'fixed point', where applying the rules do not
        // result in any more changes.
        loop {
            let language_aliases = if locale.id.variants.is_empty() {
                &self.aliases.get().language
            } else {
                &self.aliases.get().language_variants
            };

            // This is a linear search due to the ordering imposed by the canonicalization
            // rules, where rules with more variants should be considered first. With the
            // current data in CLDR, we will only do this for locales which have variants,
            // and there are fewer than 20 rules to consider. In benchmarking, the run
            // time of this loop was negligible.
            let mut matched = false;
            for rule in language_aliases.iter() {
                if uts35_rule_matches(locale, &rule.0) {
                    uts35_replacement(
                        locale,
                        !rule.0.language.is_empty(),
                        rule.0.script.is_some(),
                        rule.0.region.is_some(),
                        Some(&rule.0.variants),
                        &rule.1,
                    );
                    result = CanonicalizationResult::Modified;
                    matched = true;
                    break;
                }
            }

            if matched {
                continue;
            }

            if !locale.id.language.is_empty() {
                // If the region is specified, check sgn-region rules first
                if let Some(region) = locale.id.region {
                    if locale.id.language == "sgn" {
                        if let Ok(index) = self
                            .aliases
                            .get()
                            .sgn_region
                            .binary_search_by_key(&region.into(), |alias| alias.0)
                        {
                            uts35_replacement(
                                locale,
                                true,
                                false,
                                true,
                                None,
                                &self.aliases.get().sgn_region[index].1,
                            );
                            result = CanonicalizationResult::Modified;
                            continue;
                        }
                    }
                }

                if uts35_check_language_rules(locale, &self.aliases)
                    == CanonicalizationResult::Modified
                {
                    result = CanonicalizationResult::Modified;
                    continue;
                }
            }

            if let Some(script) = locale.id.script {
                if let Ok(index) = self
                    .aliases
                    .get()
                    .script
                    .binary_search_by_key(&script.into(), |alias| alias.0)
                {
                    if let Ok(replacement) = self.aliases.get().script[index].1.parse() {
                        locale.id.script = Some(replacement);
                        result = CanonicalizationResult::Modified;
                        continue;
                    }
                }
            }

            if let Some(region) = locale.id.region {
                let region_aliases = if region.is_alphabetic() {
                    &self.aliases.get().region_alpha
                } else {
                    &self.aliases.get().region_num
                };

                if let Ok(index) =
                    region_aliases.binary_search_by_key(&region.into(), |alias| alias.0)
                {
                    if let Ok(replacement) = region_aliases[index].1.parse() {
                        locale.id.region = Some(replacement);
                        result = CanonicalizationResult::Modified;
                        continue;
                    }
                }

                if let Ok(index) = self
                    .aliases
                    .get()
                    .complex_region
                    .binary_search_by_key(&region.into(), |alias| alias.0)
                {
                    let rule = &self.aliases.get().complex_region[index];

                    let mut for_likely = LanguageIdentifier {
                        language: locale.id.language,
                        script: locale.id.script,
                        region: None,
                        variants: subtags::Variants::default(),
                    };

                    let replacement =
                        if self.maximize(&mut for_likely) == CanonicalizationResult::Modified {
                            if let Some(likely_region) = for_likely.region {
                                let as_tinystr4: TinyStr4 = likely_region.into();
                                if let Some(region) =
                                    rule.1.iter().find(|region| as_tinystr4 == **region)
                                {
                                    region
                                } else {
                                    &rule.1[0]
                                }
                            } else {
                                &rule.1[0]
                            }
                        } else {
                            &rule.1[0]
                        };
                    if let Ok(replacement) = replacement.parse::<subtags::Region>() {
                        locale.id.region = Some(replacement);
                        result = CanonicalizationResult::Modified;
                        continue;
                    }
                }
            }

            if !locale.id.variants.is_empty() {
                let mut modified = Vec::new();
                let mut unmodified = Vec::new();
                for variant in locale.id.variants.iter() {
                    let variant_as_tinystr: TinyStr8 = (*variant).into();
                    if let Ok(index) = self
                        .aliases
                        .get()
                        .variant
                        .binary_search_by_key(&variant_as_tinystr, |alias| alias.0)
                    {
                        if let Ok(updated) = subtags::Variant::from_bytes(
                            self.aliases.get().variant[index].1.as_bytes(),
                        ) {
                            modified.push(updated);
                        }
                    } else {
                        unmodified.push(variant);
                    }
                }

                if !modified.is_empty() {
                    for variant in unmodified {
                        modified.push(*variant);
                    }
                    modified.sort();
                    modified.dedup();
                    locale.id.variants = subtags::Variants::from_vec_unchecked(modified);
                    result = CanonicalizationResult::Modified;
                    continue;
                }
            }

            // Nothing matched in this iteration, we're done.
            break;
        }

        // Handle Locale extensions in their own loops, because these rules do not interact
        // with each other.
        if let Some(lang) = &locale.extensions.transform.lang {
            let mut tlang: Locale = lang.clone().into();
            let mut matched = false;
            loop {
                if uts35_check_language_rules(&mut tlang, &self.aliases)
                    == CanonicalizationResult::Modified
                {
                    result = CanonicalizationResult::Modified;
                    matched = true;
                    continue;
                }

                break;
            }

            if matched {
                locale.extensions.transform.lang = Some(tlang.id);
            }
        }

        for key in self.extension_keys.iter() {
            if let Some(value) = locale.extensions.unicode.keywords.get_mut(key) {
                if let Ok(value_as_tinystr) = value.to_string().parse::<TinyStr8>() {
                    if let Ok(index) = self
                        .aliases
                        .get()
                        .subdivision
                        .binary_search_by_key(&value_as_tinystr, |alias| alias.0)
                    {
                        if let Ok(modified_value) =
                            Value::from_bytes(&self.aliases.get().subdivision[index].1.as_bytes())
                        {
                            *value = modified_value;
                            result = CanonicalizationResult::Modified;
                        }
                    }
                }
            }
        }

        result
    }

    /// The maximize method potentially updates a passed in locale in place
    /// depending up the results of running the 'Add Likely Subtags' algorithm
    /// from https://www.unicode.org/reports/tr35/#Likely_Subtags.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns [`CanonicalizationResult::Modified`]. Otherwise, the method
    /// returns [`CanonicalizationResult::Unmodified`] and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider)
    ///     .expect("create failed");
    ///
    /// let mut locale : Locale = "zh-CN".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "zh-Hans-CN");
    ///
    /// let mut locale : Locale = "zh-Hant-TW".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "zh-Hant-TW");
    /// ```
    pub fn maximize<T: AsMut<LanguageIdentifier>>(&self, mut langid: T) -> CanonicalizationResult {
        let langid = langid.as_mut();
        let data = self.likely_subtags.get();

        if !langid.language.is_empty() && langid.script.is_some() && langid.region.is_some() {
            return CanonicalizationResult::Unmodified;
        }

        if let Some(language) = langid.language.into() {
            if let Some(region) = langid.region {
                maximize_locale!(langid, data.language_region, language, region.into());
            }
            if let Some(script) = langid.script {
                maximize_locale!(langid, data.language_script, language, script.into());
            }
            maximize_locale!(langid, data.language, language);
        } else if let Some(script) = langid.script {
            if let Some(region) = langid.region {
                maximize_locale!(langid, data.script_region, script.into(), region.into());
            }
            maximize_locale!(langid, data.script, script.into());
        } else if let Some(region) = langid.region {
            maximize_locale!(langid, data.region, region.into());
        }
        update_langid(&data.und, langid)
    }

    /// This returns a new Locale that is the result of running the
    /// 'Remove Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns [`CanonicalizationResult::Modified`]. Otherwise, the method
    /// returns [`CanonicalizationResult::Unmodified`] and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider)
    ///     .expect("creation failed");
    ///
    /// let mut locale : Locale = "zh-Hans-CN".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "zh");
    ///
    /// let mut locale : Locale = "zh".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "zh");
    /// ```
    pub fn minimize<T: AsMut<LanguageIdentifier>>(&self, mut langid: T) -> CanonicalizationResult {
        let langid = langid.as_mut();

        let mut max = langid.clone();
        self.maximize(&mut max);
        max.variants.clear();
        let mut trial = max.clone();

        trial.script = None;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if langid.script.is_some() || langid.script.is_some() {
                langid.script = None;
                langid.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = None;
        trial.region = max.region;
        self.maximize(&mut trial);
        if trial == max {
            if langid.script.is_some() || langid.region != max.region {
                langid.script = None;
                langid.region = max.region;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = max.script;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if langid.script != max.script || langid.region.is_some() {
                langid.script = max.script;
                langid.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        if langid.script != max.script || langid.region != max.region {
            langid.script = max.script;
            langid.region = max.region;
            CanonicalizationResult::Modified
        } else {
            CanonicalizationResult::Unmodified
        }
    }
}

#[test]
fn test_uts35_rule_matches() {
    assert!(uts35_rule_matches(
        &"ja".parse().unwrap(),
        &LanguageIdentifier::und()
    ));

    assert!(uts35_rule_matches(
        &"und-heploc-hepburn".parse().unwrap(),
        &"und-hepburn".parse().unwrap()
    ));

    assert!(uts35_rule_matches(
        &"ja-heploc-hepburn".parse().unwrap(),
        &"und-hepburn".parse().unwrap()
    ));

    assert!(!uts35_rule_matches(
        &"ja-hepburn".parse().unwrap(),
        &"und-hepburn-heploc".parse().unwrap()
    ));
}

#[test]
fn test_uts35_replacement() {
    let mut locale = "ja-Latn-fonipa-hepburn-heploc".parse().unwrap();
    let ruletype: LanguageIdentifier = "und-hepburn-heploc".parse().unwrap();
    uts35_replacement(
        &mut locale,
        !ruletype.language.is_empty(),
        ruletype.script.is_some(),
        ruletype.region.is_some(),
        Some(&ruletype.variants),
        &"und-alalc97".parse().unwrap(),
    );
    assert_eq!(locale, "ja-Latn-alalc97-fonipa".parse::<Locale>().unwrap());

    let mut locale = "sgn-DD".parse().unwrap();
    let ruletype: LanguageIdentifier = "und-DD".parse().unwrap();
    uts35_replacement(
        &mut locale,
        !ruletype.language.is_empty(),
        ruletype.script.is_some(),
        ruletype.region.is_some(),
        Some(&ruletype.variants),
        &"und-DE".parse().unwrap(),
    );
    assert_eq!(locale, "sgn-DE".parse::<Locale>().unwrap());

    let mut locale = "sgn-DE".parse().unwrap();
    let ruletype: LanguageIdentifier = "sgn-DE".parse().unwrap();
    uts35_replacement(
        &mut locale,
        !ruletype.language.is_empty(),
        ruletype.script.is_some(),
        ruletype.region.is_some(),
        None,
        &"gsg".parse().unwrap(),
    );
    assert_eq!(locale, "gsg".parse::<Locale>().unwrap());
}
