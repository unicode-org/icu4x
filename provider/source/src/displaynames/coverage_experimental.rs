// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This file contains code that parses CLDR `cldr-core/pathCoverage/*.txt` files
//! to populate the tiny/light/heavy display names depth tiers.

use crate::cldr_cache::CldrCache;
#[cfg(test)]
#[cfg(feature = "networking")]
use crate::cldr_serde::alt::WithAlt;
use crate::cldr_serde::alt::{Alt, Menu};
use icu_locale_core::LanguageIdentifier;
use icu_locale_core::subtags::{Language, Region, Script, Variant};
use icu_provider::prelude::*;
#[cfg(test)]
#[cfg(feature = "networking")]
use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use writeable::Writeable;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CoverageCategory {
    Language,
    Territory,
    Script,
    Variant,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ChassisKind {
    Base,
    Alt,
    Menu,
}

fn parse_displaynames_chassis(chassis: &str) -> Option<(CoverageCategory, ChassisKind)> {
    let rest = chassis.strip_prefix("//ldml/localeDisplayNames/")?;
    let (category, rest) = if let Some(r) = rest.strip_prefix("languages/language") {
        (CoverageCategory::Language, r)
    } else if let Some(r) = rest.strip_prefix("territories/territory") {
        (CoverageCategory::Territory, r)
    } else if let Some(r) = rest.strip_prefix("scripts/script") {
        (CoverageCategory::Script, r)
    } else {
        let r = rest.strip_prefix("variants/variant")?;
        (CoverageCategory::Variant, r)
    };
    let kind = match rest {
        "[@type]" => ChassisKind::Base,
        "[@type][@alt]" => ChassisKind::Alt,
        "[@type][@menu]" => ChassisKind::Menu,
        _ => return None,
    };
    Some((category, kind))
}

fn normalize_attr0(category: CoverageCategory, val: &str) -> String {
    match category {
        CoverageCategory::Language => {
            if let Ok(lang) = val.parse::<Language>() {
                lang.to_string()
            } else if let Ok(lang_id) = val.parse::<LanguageIdentifier>() {
                lang_id.to_string()
            } else {
                val.replace('_', "-")
            }
        }
        CoverageCategory::Territory => {
            if let Ok(region) = val.parse::<Region>() {
                region.to_string()
            } else {
                val.to_string()
            }
        }
        CoverageCategory::Script => {
            if let Ok(script) = val.parse::<Script>() {
                script.to_string()
            } else {
                val.to_string()
            }
        }
        CoverageCategory::Variant => {
            if let Ok(variant) = val.parse::<Variant>() {
                variant.to_string()
            } else {
                val.to_string()
            }
        }
    }
}

/// Coverage tiers for display names items derived from CLDR `pathCoverage` definitions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum CoverageLevelForXPath {
    /// Items with `core` CLDR coverage level.
    Core = 0,
    /// Items with `basic` CLDR coverage level.
    Basic = 1,
    /// Items with `moderate` CLDR coverage level.
    Moderate = 2,
    /// Items with `modern` CLDR coverage level.
    Modern = 3,
    /// Items with `comprehensive` CLDR coverage level, or unlisted items.
    Comprehensive = 4,
}

impl CoverageLevelForXPath {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "core" => Some(Self::Core),
            "basic" => Some(Self::Basic),
            "moderate" => Some(Self::Moderate),
            "modern" => Some(Self::Modern),
            "comprehensive" => Some(Self::Comprehensive),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct SetMatcher {
    positive: bool,
    values: BTreeSet<String>,
}

impl SetMatcher {
    fn parse(
        raw: &str,
        variables: &BTreeMap<&str, &str>,
        category: Option<CoverageCategory>,
    ) -> Result<Self, DataError> {
        let (positive, rest) = if let Some(stripped) = raw.strip_prefix('!') {
            (false, stripped)
        } else {
            (true, raw)
        };
        let mut values = BTreeSet::new();
        for token in rest.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            if token.starts_with('%') {
                let var_val = variables.get(token).ok_or_else(|| {
                    DataError::custom("Undefined pathCoverage variable").with_display_context(token)
                })?;
                for item in var_val.split(',') {
                    let item = item.trim();
                    if !item.is_empty() {
                        values.insert(match category {
                            Some(cat) => normalize_attr0(cat, item),
                            None => item.to_string(),
                        });
                    }
                }
            } else {
                values.insert(match category {
                    Some(cat) => normalize_attr0(cat, token),
                    None => token.to_string(),
                });
            }
        }
        Ok(Self { positive, values })
    }

    fn matches(&self, val: &str) -> bool {
        let contains = self.values.contains(val);
        if self.positive { contains } else { !contains }
    }
}

#[derive(Debug)]
struct AttrMatcher {
    attr0: Option<SetMatcher>,
    attr1: Option<SetMatcher>,
    level: CoverageLevelForXPath,
}

impl AttrMatcher {
    fn matches(&self, attr0_val: &str, attr1_val: Option<&str>) -> bool {
        if let Some(m0) = &self.attr0
            && !m0.matches(attr0_val)
        {
            return false;
        }
        if let Some(m1) = &self.attr1
            && !m1.matches(attr1_val.unwrap_or(""))
        {
            return false;
        }
        true
    }
}

#[derive(Debug)]
pub(super) struct ChassisRule {
    matchers: Vec<AttrMatcher>,
    else_level: CoverageLevelForXPath,
}

impl ChassisRule {
    fn evaluate(&self, attr0_val: &str, attr1_val: Option<&str>) -> CoverageLevelForXPath {
        for matcher in &self.matchers {
            if matcher.matches(attr0_val, attr1_val) {
                return matcher.level;
            }
        }
        self.else_level
    }
}

/// Coverage rules for a single display names category (`language`, `territory`, `script`, or `variant`).
#[derive(Debug, Default)]
pub(super) struct CategoryRules {
    base: Option<ChassisRule>,
    alt: Option<ChassisRule>,
    menu: Option<ChassisRule>,
}

impl CategoryRules {
    fn rule_for(&self, alt: Option<Alt>, menu: Option<Menu>) -> Option<&ChassisRule> {
        if alt.is_some() {
            self.alt.as_ref()
        } else if menu.is_some() {
            self.menu.as_ref()
        } else {
            self.base.as_ref()
        }
    }
}

/// Parsed representation of a `cldr-core/pathCoverage/{locale}.txt` file for display names categories.
#[derive(Debug, Default)]
pub(super) struct PathCoverageFile {
    pub(super) import: Option<String>,
    pub(super) language: CategoryRules,
    pub(super) territory: CategoryRules,
    pub(super) script: CategoryRules,
    pub(super) variant: CategoryRules,
}

fn locale_to_stem(locale: &DataLocale) -> String {
    if locale.is_unknown() {
        return "root".to_string();
    }
    let mut s = locale.language.to_string();
    if let Some(script) = locale.script {
        s.push('_');
        s.push_str(script.as_str());
    }
    if let Some(region) = locale.region {
        s.push('_');
        s.push_str(region.as_str());
    }
    if let Some(variant) = locale.variant {
        s.push('_');
        s.push_str(&variant.as_str().to_ascii_uppercase());
    }
    s
}

fn file_exists_for_stem(cldr_cache: &CldrCache, stem: &str) -> Result<bool, DataError> {
    cldr_cache
        .serde_cache
        .file_exists(&format!("cldr-core/pathCoverage/{stem}.txt"))
}

fn try_locale_or_extended(
    cldr_cache: &CldrCache,
    locale: &DataLocale,
) -> Result<Option<String>, DataError> {
    let stem = locale_to_stem(locale);
    if file_exists_for_stem(cldr_cache, &stem)? {
        return Ok(Some(stem));
    }
    if let Some(extended) = cldr_cache.add_script_extended(locale)? {
        let stem = locale_to_stem(&extended);
        if file_exists_for_stem(cldr_cache, &stem)? {
            return Ok(Some(stem));
        }
    }
    Ok(None)
}

fn resolve_initial_stem(cldr_cache: &CldrCache, locale: &DataLocale) -> Result<String, DataError> {
    if locale.is_unknown() {
        return Ok("root".to_string());
    }
    if let Some(stem) = try_locale_or_extended(cldr_cache, locale)? {
        return Ok(stem);
    }
    if locale.variant.is_some() {
        let mut without_var = *locale;
        without_var.variant = None;
        if let Some(stem) = try_locale_or_extended(cldr_cache, &without_var)? {
            return Ok(stem);
        }
    }
    let lang_stem = locale.language.as_str();
    if file_exists_for_stem(cldr_cache, lang_stem)? {
        return Ok(lang_stem.to_string());
    }
    Ok("root".to_string())
}

impl PathCoverageFile {
    pub(super) fn parse(bytes: Box<[u8]>) -> Result<Self, DataError> {
        let text = core::str::from_utf8(&bytes)
            .map_err(|e| DataError::custom("pathCoverage UTF-8").with_display_context(&e))?;
        Self::parse_str(text)
    }

    fn parse_str(text: &str) -> Result<Self, DataError> {
        let mut file = Self::default();
        let mut variables: BTreeMap<&str, &str> = BTreeMap::new();
        let mut current_target: Option<(CoverageCategory, ChassisKind)> = None;
        let mut current_matchers: Vec<AttrMatcher> = Vec::new();
        let mut pending_attr0: Option<SetMatcher> = None;
        let mut pending_attr1: Option<SetMatcher> = None;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(
                    DataError::custom("Malformed pathCoverage line").with_display_context(line)
                );
            };
            let key = key.trim();
            let value = value.trim();

            if key == "import" {
                file.import = Some(value.to_string());
            } else if key.starts_with('%') {
                variables.insert(key, value);
            } else if key == "path" {
                current_target = parse_displaynames_chassis(value);
                current_matchers.clear();
                pending_attr0 = None;
                pending_attr1 = None;
            } else if let Some((category, kind)) = current_target {
                match key {
                    "attr0" => {
                        pending_attr0 = Some(SetMatcher::parse(value, &variables, Some(category))?);
                    }
                    "attr1" => {
                        pending_attr1 = Some(SetMatcher::parse(value, &variables, None)?);
                    }
                    "level" => {
                        let level = CoverageLevelForXPath::parse(value).ok_or_else(|| {
                            DataError::custom("Unknown pathCoverage level")
                                .with_display_context(value)
                        })?;
                        current_matchers.push(AttrMatcher {
                            attr0: pending_attr0.take(),
                            attr1: pending_attr1.take(),
                            level,
                        });
                    }
                    "elseLevel" => {
                        let else_level = CoverageLevelForXPath::parse(value).ok_or_else(|| {
                            DataError::custom("Unknown pathCoverage elseLevel")
                                .with_display_context(value)
                        })?;
                        let rule = ChassisRule {
                            matchers: core::mem::take(&mut current_matchers),
                            else_level,
                        };
                        let cat_rules = match category {
                            CoverageCategory::Language => &mut file.language,
                            CoverageCategory::Territory => &mut file.territory,
                            CoverageCategory::Script => &mut file.script,
                            CoverageCategory::Variant => &mut file.variant,
                        };
                        match kind {
                            ChassisKind::Base => cat_rules.base = Some(rule),
                            ChassisKind::Alt => cat_rules.alt = Some(rule),
                            ChassisKind::Menu => cat_rules.menu = Some(rule),
                        }
                        current_target = None;
                    }
                    _ => {}
                }
            }
        }

        Ok(file)
    }
}

fn read_stem<'a>(cldr_cache: &'a CldrCache, stem: &str) -> Result<&'a PathCoverageFile, DataError> {
    cldr_cache.serde_cache.read_and_parse(
        &format!("cldr-core/pathCoverage/{stem}.txt"),
        PathCoverageFile::parse,
    )
}

/// The inheritance chain of [`PathCoverageFile`]s for a locale, used to evaluate coverage tiers.
#[derive(Debug)]
pub(super) struct PathCoverageChain<'a>(Vec<&'a PathCoverageFile>);

impl<'a> PathCoverageChain<'a> {
    /// Resolves and loads the fallback chain of [`PathCoverageFile`]s for `locale`.
    pub(super) fn for_locale(
        cldr_cache: &'a CldrCache,
        locale: &DataLocale,
    ) -> Result<Self, DataError> {
        let mut chain = Vec::with_capacity(3);
        let mut current_stem = Some(resolve_initial_stem(cldr_cache, locale)?);
        while let Some(stem) = current_stem.take() {
            let file = read_stem(cldr_cache, &stem)?;
            chain.push(file);
            if let Some(import_stem) = &file.import {
                if file_exists_for_stem(cldr_cache, import_stem)? {
                    current_stem = Some(import_stem.clone());
                } else if stem != "root" {
                    current_stem = Some("root".to_string());
                }
            }
        }
        Ok(Self(chain))
    }

    /// Evaluates the [`CoverageLevelForXPath`] for a subtag target within this chain.
    pub(super) fn coverage_tier(
        &self,
        get_category: impl Fn(&PathCoverageFile) -> &CategoryRules,
        subtag: impl Writeable,
        alt: Option<Alt>,
        menu: Option<Menu>,
    ) -> CoverageLevelForXPath {
        let subtag_str = subtag.write_to_string();
        if subtag_str.is_empty() {
            return CoverageLevelForXPath::Comprehensive;
        }
        let attr1_val = if let Some(alt) = alt {
            Some(alt.as_str().unwrap_or(""))
        } else if let Some(menu) = menu {
            Some(menu.as_str().unwrap_or(""))
        } else {
            None
        };
        for file in &self.0 {
            if let Some(rule) = get_category(file).rule_for(alt, menu) {
                return rule.evaluate(&subtag_str, attr1_val);
            }
        }
        CoverageLevelForXPath::Comprehensive
    }
}

#[cfg(test)]
#[cfg(feature = "networking")]
pub(super) trait CheckAltCoverage {
    fn contains_key<T>(key: &WithAlt<T>, tier: CoverageLevelForXPath) -> bool;
}

/// Test helper that iterates over all display name entries across all locales in CLDR in deterministic order.
///
/// For each entry found in `file_name` (e.g., `"languages.json"`), this function:
/// 1. Extracts the map of subtag keys (`WithAlt<T>`) via `extract_keys`.
/// 2. Looks up the coverage tier (`CoverageLevelForXPath`) for that entry in the given locale using `get_category`.
/// 3. Invokes `callback(locale, key, tier)`.
#[cfg(test)]
#[cfg(feature = "networking")]
pub(super) fn for_each_cldr_key_and_tier<Resource, T>(
    cldr: &CldrCache,
    file_name: &str,
    ignored_alts: &[Alt],
    get_category: impl Fn(&PathCoverageFile) -> &CategoryRules,
    mut extract_keys: impl FnMut(&Resource) -> &HashMap<WithAlt<T>, String>,
    mut callback: impl FnMut(&DataLocale, &WithAlt<T>, CoverageLevelForXPath),
) where
    Resource: serde::de::DeserializeOwned + Send + Sync + 'static,
    T: Writeable,
{
    let displaynames_dir = cldr.displaynames();
    let mut locales = displaynames_dir.list_locales().unwrap().collect::<Vec<_>>();
    locales.sort_by(|a, b| a.total_cmp(b));
    for locale in locales {
        if let Ok(res) = displaynames_dir.read_and_parse::<Resource>(&locale, file_name) {
            let coverage_chain = PathCoverageChain::for_locale(cldr, &locale).unwrap();
            let mut keys = extract_keys(res).keys().collect::<Vec<_>>();
            keys.sort_by_cached_key(|k| (k.t.write_to_string().to_string(), k.alt, k.menu));
            for key in keys {
                if let Some(alt) = key.alt
                    && ignored_alts.contains(&alt)
                {
                    continue;
                }
                let tier = coverage_chain.coverage_tier(&get_category, &key.t, key.alt, key.menu);
                callback(&locale, key, tier);
            }
        }
    }
}

#[test]
fn test_coverage_tier() {
    use crate::SourceDataProvider;
    use std::str::FromStr;
    let provider = SourceDataProvider::new_testing();
    let cldr = provider.cldr().unwrap();

    let en = DataLocale::from_str("en").unwrap();
    assert_eq!(
        PathCoverageChain::for_locale(cldr, &en)
            .unwrap()
            .coverage_tier(|l| &l.language, "en", None, None),
        CoverageLevelForXPath::Basic
    );

    assert_eq!(
        PathCoverageChain::for_locale(cldr, &en)
            .unwrap()
            .coverage_tier(|l| &l.language, "unlisted_test_code", None, None),
        CoverageLevelForXPath::Comprehensive
    );

    // Check inheritance and non-comprehensive elseLevel:
    // en-001 inherits territory[@type][@alt] from root.txt where GB/US short is basic
    // and !GB,US short is moderate.
    let en_001 = DataLocale::from_str("en-001").unwrap();
    assert_eq!(
        PathCoverageChain::for_locale(cldr, &en_001)
            .unwrap()
            .coverage_tier(|l| &l.territory, "GB", Some(Alt::Short), None),
        CoverageLevelForXPath::Basic
    );
    assert_eq!(
        PathCoverageChain::for_locale(cldr, &en_001)
            .unwrap()
            .coverage_tier(|l| &l.territory, "BA", Some(Alt::Short), None),
        CoverageLevelForXPath::Moderate
    );
}
