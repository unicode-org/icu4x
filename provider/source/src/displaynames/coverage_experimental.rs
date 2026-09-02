// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This file contains experimental, unstable code that uses hard-coded coverage levels
//! generated from a snapshot of CLDR 48 to populate the tiny/light/heavy display names
//! depth tiers. This code should be deleted when a longer-term solution is available.

use crate::cldr_cache::CldrCache;
#[cfg(test)]
use crate::cldr_serde::displaynames::WithAlt;
use crate::cldr_serde::displaynames::{Alt, Menu};
use crate::source::SerdeCache;
use icu_locale_core::LanguageIdentifier;
use icu_locale_core::subtags::{Language, Region, Script, Variant};
use icu_provider::prelude::*;
use litemap::LiteMap;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::HashMap;
use std::sync::OnceLock;
use writeable::Writeable;
use zerotrie::ZeroTrieSimpleAscii;

/// Serde wrapper for `coverageByXPath.json` and locale-specific `coverageByXPath/{locale}.json` files.
/// If this is added to CLDR JSON, this should be moved to `cldr_serde`.
#[derive(Deserialize, Debug)]
pub(super) struct CoverageByXPathResource {
    /// Mapping from locale identifier (or `"root"`) to its corresponding coverage levels object.
    #[serde(rename = "coverageByXPath")]
    pub(super) coverage_by_xpath: BTreeMap<String, CoverageByXPathLevels>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CoverageCategory {
    Language,
    Territory,
    Script,
    Variant,
}

fn parse_cldr_xpath(xpath: &str) -> Option<(CoverageCategory, String)> {
    let prefix_lang = "//ldml/localeDisplayNames/languages/language";
    let prefix_terr = "//ldml/localeDisplayNames/territories/territory";
    let prefix_script = "//ldml/localeDisplayNames/scripts/script";
    let prefix_var = "//ldml/localeDisplayNames/variants/variant";

    let (category, rest) = if let Some(r) = xpath.strip_prefix(prefix_lang) {
        (CoverageCategory::Language, r)
    } else if let Some(r) = xpath.strip_prefix(prefix_terr) {
        (CoverageCategory::Territory, r)
    } else if let Some(r) = xpath.strip_prefix(prefix_script) {
        (CoverageCategory::Script, r)
    } else {
        let r = xpath.strip_prefix(prefix_var)?;
        (CoverageCategory::Variant, r)
    };

    let type_prefix = "[@type=\"";
    let rest = rest.strip_prefix(type_prefix)?;
    let (val, extra_attrs) = rest.split_once("\"]")?;

    let normalized_val = match category {
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
    };

    let key = if extra_attrs.is_empty() {
        normalized_val
    } else {
        format!("{normalized_val}{extra_attrs}")
    };

    Some((category, key))
}

use serde::de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};

/// Single-category trie mapping subtag keys to coverage levels (`CoverageLevelForXPath`).
#[derive(Debug, Default)]
pub(super) struct CoverageCategoryLevels(pub(super) ZeroTrieSimpleAscii<Vec<u8>>);

impl CoverageCategoryLevels {
    pub(super) fn level(
        &self,
        subtag: impl Writeable,
        alt: Option<Alt>,
        menu: Option<Menu>,
    ) -> Option<CoverageLevelForXPath> {
        use core::fmt::Write;
        let mut cursor = self.0.cursor();
        subtag.write_to(&mut cursor).ok()?;

        if cursor.is_empty() {
            return None;
        }

        if let Some(alt) = alt.and_then(Alt::as_str) {
            cursor.write_str("[@alt=\"").ok()?;
            cursor.write_str(alt).ok()?;
            cursor.write_str("\"]").ok()?;
        }

        if let Some(menu) = menu.and_then(Menu::as_str) {
            cursor.write_str("[@menu=\"").ok()?;
            cursor.write_str(menu).ok()?;
            cursor.write_str("\"]").ok()?;
        }

        cursor
            .take_value()
            .and_then(CoverageLevelForXPath::from_usize)
    }
}

/// Coverage level representation for display names categories (`language`, `territory`, `script`, `variant`).
///
/// # Note on File Contents & Scope
/// The raw source JSON files (`coverageByXPath.json` and `coverageByXPath/{locale}.json`) contain `XPaths`
/// for many CLDR elements beyond display names. During Serde deserialization, only supported display name
/// category `XPaths` are parsed into their respective trie maps (`language`, `territory`, `script`, `variant`),
/// and all unrelated `XPaths` are ignored.
#[derive(Debug)]
pub(super) struct CoverageByXPathLevels {
    pub(super) language: CoverageCategoryLevels,
    pub(super) territory: CoverageCategoryLevels,
    pub(super) script: CoverageCategoryLevels,
    pub(super) variant: CoverageCategoryLevels,
}

struct XPathArraySeed<'a> {
    lang: &'a mut LiteMap<Vec<u8>, usize>,
    terr: &'a mut LiteMap<Vec<u8>, usize>,
    script: &'a mut LiteMap<Vec<u8>, usize>,
    var: &'a mut LiteMap<Vec<u8>, usize>,
    level_val: usize,
}

struct XPathArrayVisitor<'a> {
    lang: &'a mut LiteMap<Vec<u8>, usize>,
    terr: &'a mut LiteMap<Vec<u8>, usize>,
    script: &'a mut LiteMap<Vec<u8>, usize>,
    var: &'a mut LiteMap<Vec<u8>, usize>,
    level_val: usize,
}

impl<'de, 'a> DeserializeSeed<'de> for XPathArraySeed<'a> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(XPathArrayVisitor {
            lang: self.lang,
            terr: self.terr,
            script: self.script,
            var: self.var,
            level_val: self.level_val,
        })
    }
}

impl<'de, 'a> Visitor<'de> for XPathArrayVisitor<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a sequence of CLDR XPaths")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(xpath) = seq.next_element::<Cow<'_, str>>()? {
            // Non-ASCII XPaths (e.g. emoji annotations) are not display names subtags
            // and cannot be stored in ZeroTrieSimpleAscii.
            if !xpath.is_ascii() {
                continue;
            }
            if let Some((category, key)) = parse_cldr_xpath(xpath.as_ref()) {
                let map = match category {
                    CoverageCategory::Language => &mut *self.lang,
                    CoverageCategory::Territory => &mut *self.terr,
                    CoverageCategory::Script => &mut *self.script,
                    CoverageCategory::Variant => &mut *self.var,
                };
                map.insert(key.into_bytes(), self.level_val);
            }
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for CoverageByXPathLevels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoverageByXPathLevelsVisitor;

        impl<'de> Visitor<'de> for CoverageByXPathLevelsVisitor {
            type Value = CoverageByXPathLevels;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a struct CoverageByXPathLevels")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut map_lang = LiteMap::new_vec();
                let mut map_terr = LiteMap::new_vec();
                let mut map_script = LiteMap::new_vec();
                let mut map_var = LiteMap::new_vec();

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    let level_val = match key.as_ref() {
                        "core" => CoverageLevelForXPath::Core.to_usize(),
                        "basic" => CoverageLevelForXPath::Basic.to_usize(),
                        "moderate" => CoverageLevelForXPath::Moderate.to_usize(),
                        "modern" => CoverageLevelForXPath::Modern.to_usize(),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                            continue;
                        }
                    };
                    map.next_value_seed(XPathArraySeed {
                        lang: &mut map_lang,
                        terr: &mut map_terr,
                        script: &mut map_script,
                        var: &mut map_var,
                        level_val,
                    })?;
                }

                Ok(CoverageByXPathLevels {
                    language: CoverageCategoryLevels(
                        ZeroTrieSimpleAscii::try_from(&map_lang)
                            .map_err(serde::de::Error::custom)?,
                    ),
                    territory: CoverageCategoryLevels(
                        ZeroTrieSimpleAscii::try_from(&map_terr)
                            .map_err(serde::de::Error::custom)?,
                    ),
                    script: CoverageCategoryLevels(
                        ZeroTrieSimpleAscii::try_from(&map_script)
                            .map_err(serde::de::Error::custom)?,
                    ),
                    variant: CoverageCategoryLevels(
                        ZeroTrieSimpleAscii::try_from(&map_var)
                            .map_err(serde::de::Error::custom)?,
                    ),
                })
            }
        }

        deserializer.deserialize_map(CoverageByXPathLevelsVisitor)
    }
}

pub(super) struct CoverageByXPathCache(SerdeCache);

/// Coverage tiers for display names items derived from CLDR `coverageByXPath` JSON definitions.
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
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(val: usize) -> Option<Self> {
        match val {
            0 => Some(Self::Core),
            1 => Some(Self::Basic),
            2 => Some(Self::Moderate),
            3 => Some(Self::Modern),
            4 => Some(Self::Comprehensive),
            _ => None,
        }
    }
}

impl CoverageByXPathCache {
    pub(super) fn get_levels_for_locale<'a>(
        &'a self,
        locale: &DataLocale,
        cldr_cache: &'a CldrCache,
    ) -> Result<Option<&'a CoverageByXPathLevels>, DataError> {
        let locale_path = format!("displaynames/coverageByXPath/{locale}.json");
        if self.0.file_exists(&locale_path)? {
            let resource: &CoverageByXPathResource = self.0.read_and_parse_json(&locale_path)?;
            Ok(resource.coverage_by_xpath.values().next())
        } else {
            if locale.variant.is_some() {
                let mut new_locale = *locale;
                new_locale.variant = None;
                self.get_levels_for_locale(&new_locale, cldr_cache)
            } else if let Some(new_locale) = cldr_cache.add_script_extended(locale)? {
                self.get_levels_for_locale(&new_locale, cldr_cache)
            } else {
                Ok(None)
            }
        }
    }

    pub(super) fn get_root_levels(&self) -> Result<Option<&CoverageByXPathLevels>, DataError> {
        let resource: &CoverageByXPathResource = self
            .0
            .read_and_parse_json("displaynames/coverageByXPath.json")?;
        Ok(resource.coverage_by_xpath.get("root"))
    }

    pub(super) fn coverage_tier_from_levels(
        locale_levels: Option<&CoverageByXPathLevels>,
        root_levels: Option<&CoverageByXPathLevels>,
        get_category: impl Fn(&CoverageByXPathLevels) -> &CoverageCategoryLevels,
        subtag: impl Writeable,
        alt: Option<Alt>,
        menu: Option<Menu>,
    ) -> CoverageLevelForXPath {
        locale_levels
            .and_then(|l| get_category(l).level(&subtag, alt, menu))
            .or_else(|| root_levels.and_then(|l| get_category(l).level(&subtag, alt, menu)))
            .unwrap_or(CoverageLevelForXPath::Comprehensive)
    }

    /// Determines the [`CoverageLevelForXPath`] for a given `locale` and subtag item target.
    ///
    /// Resolution lookup precedence:
    /// 1. Locale-specific override file `displaynames/coverageByXPath/{locale}.json`
    /// 2. Root defaults file `displaynames/coverageByXPath.json`
    /// 3. Defaults to [`CoverageLevelForXPath::Comprehensive`] if unlisted everywhere.
    pub(super) fn coverage_tier(
        &self,
        locale: &DataLocale,
        get_category: impl Fn(&CoverageByXPathLevels) -> &CoverageCategoryLevels,
        subtag: impl Writeable,
        alt: Option<Alt>,
        menu: Option<Menu>,
        cldr_cache: &CldrCache,
    ) -> Result<CoverageLevelForXPath, DataError> {
        let locale_levels = self.get_levels_for_locale(locale, cldr_cache)?;
        let root_levels = self.get_root_levels()?;
        Ok(Self::coverage_tier_from_levels(
            locale_levels,
            root_levels,
            get_category,
            subtag,
            alt,
            menu,
        ))
    }
}

// Note: We statically embed coverage levels for all CLDR locales here so we can support
// locale display name depth slicing before these data are officially published to CLDR JSON.
// TODO: Remove this static cache and read exclusively from CLDR once coverageByXPath is published.
pub(super) fn coverage_cldr_cache() -> &'static CoverageByXPathCache {
    static SINGLETON: OnceLock<CoverageByXPathCache> = OnceLock::new();
    SINGLETON.get_or_init(|| {
        CoverageByXPathCache(SerdeCache::new(crate::source::include_files!(
            "../../data/";
            "displaynames/coverageByXPath.json",
            "displaynames/coverageByXPath/aa-DJ.json",
            "displaynames/coverageByXPath/aa-ER.json",
            "displaynames/coverageByXPath/aa.json",
            "displaynames/coverageByXPath/ab.json",
            "displaynames/coverageByXPath/ady-JO.json",
            "displaynames/coverageByXPath/ady-TR.json",
            "displaynames/coverageByXPath/ady.json",
            "displaynames/coverageByXPath/af-NA.json",
            "displaynames/coverageByXPath/af.json",
            "displaynames/coverageByXPath/agq.json",
            "displaynames/coverageByXPath/ak.json",
            "displaynames/coverageByXPath/am.json",
            "displaynames/coverageByXPath/an.json",
            "displaynames/coverageByXPath/ann.json",
            "displaynames/coverageByXPath/apc.json",
            "displaynames/coverageByXPath/ar-AE.json",
            "displaynames/coverageByXPath/ar-BH.json",
            "displaynames/coverageByXPath/ar-DJ.json",
            "displaynames/coverageByXPath/ar-DZ.json",
            "displaynames/coverageByXPath/ar-EG.json",
            "displaynames/coverageByXPath/ar-EH.json",
            "displaynames/coverageByXPath/ar-ER.json",
            "displaynames/coverageByXPath/ar-IL.json",
            "displaynames/coverageByXPath/ar-IQ.json",
            "displaynames/coverageByXPath/ar-JO.json",
            "displaynames/coverageByXPath/ar-KM.json",
            "displaynames/coverageByXPath/ar-KW.json",
            "displaynames/coverageByXPath/ar-LB.json",
            "displaynames/coverageByXPath/ar-LY.json",
            "displaynames/coverageByXPath/ar-MA.json",
            "displaynames/coverageByXPath/ar-MR.json",
            "displaynames/coverageByXPath/ar-OM.json",
            "displaynames/coverageByXPath/ar-PS.json",
            "displaynames/coverageByXPath/ar-QA.json",
            "displaynames/coverageByXPath/ar-SA.json",
            "displaynames/coverageByXPath/ar-SD.json",
            "displaynames/coverageByXPath/ar-SO.json",
            "displaynames/coverageByXPath/ar-SS.json",
            "displaynames/coverageByXPath/ar-SY.json",
            "displaynames/coverageByXPath/ar-TD.json",
            "displaynames/coverageByXPath/ar-TN.json",
            "displaynames/coverageByXPath/ar-YE.json",
            "displaynames/coverageByXPath/ar.json",
            "displaynames/coverageByXPath/arn.json",
            "displaynames/coverageByXPath/ary.json",
            "displaynames/coverageByXPath/as.json",
            "displaynames/coverageByXPath/asa.json",
            "displaynames/coverageByXPath/ast.json",
            "displaynames/coverageByXPath/az-Arab-IQ.json",
            "displaynames/coverageByXPath/az-Arab-TR.json",
            "displaynames/coverageByXPath/az-Arab.json",
            "displaynames/coverageByXPath/az-Cyrl.json",
            "displaynames/coverageByXPath/az-Latn.json",
            "displaynames/coverageByXPath/az.json",
            "displaynames/coverageByXPath/ba.json",
            "displaynames/coverageByXPath/bal-Arab.json",
            "displaynames/coverageByXPath/bal-Latn.json",
            "displaynames/coverageByXPath/bal.json",
            "displaynames/coverageByXPath/bas.json",
            "displaynames/coverageByXPath/be.json",
            "displaynames/coverageByXPath/bem.json",
            "displaynames/coverageByXPath/bew.json",
            "displaynames/coverageByXPath/bez.json",
            "displaynames/coverageByXPath/bg.json",
            "displaynames/coverageByXPath/bgc.json",
            "displaynames/coverageByXPath/bgn-AE.json",
            "displaynames/coverageByXPath/bgn-AF.json",
            "displaynames/coverageByXPath/bgn-IR.json",
            "displaynames/coverageByXPath/bgn-OM.json",
            "displaynames/coverageByXPath/bgn.json",
            "displaynames/coverageByXPath/bho.json",
            "displaynames/coverageByXPath/blo.json",
            "displaynames/coverageByXPath/blt.json",
            "displaynames/coverageByXPath/bm-Nkoo.json",
            "displaynames/coverageByXPath/bm.json",
            "displaynames/coverageByXPath/bn-IN.json",
            "displaynames/coverageByXPath/bn.json",
            "displaynames/coverageByXPath/bo-IN.json",
            "displaynames/coverageByXPath/bo.json",
            "displaynames/coverageByXPath/bqi.json",
            "displaynames/coverageByXPath/br.json",
            "displaynames/coverageByXPath/brh.json",
            "displaynames/coverageByXPath/brx.json",
            "displaynames/coverageByXPath/bs-Cyrl.json",
            "displaynames/coverageByXPath/bs-Latn.json",
            "displaynames/coverageByXPath/bs.json",
            "displaynames/coverageByXPath/bss.json",
            "displaynames/coverageByXPath/bua.json",
            "displaynames/coverageByXPath/byn.json",
            "displaynames/coverageByXPath/ca-AD.json",
            "displaynames/coverageByXPath/ca-ES-valencia.json",
            "displaynames/coverageByXPath/ca-FR.json",
            "displaynames/coverageByXPath/ca-IT.json",
            "displaynames/coverageByXPath/ca.json",
            "displaynames/coverageByXPath/cad.json",
            "displaynames/coverageByXPath/cch.json",
            "displaynames/coverageByXPath/ccp-IN.json",
            "displaynames/coverageByXPath/ccp.json",
            "displaynames/coverageByXPath/ce.json",
            "displaynames/coverageByXPath/ceb.json",
            "displaynames/coverageByXPath/cgg.json",
            "displaynames/coverageByXPath/cho.json",
            "displaynames/coverageByXPath/chr.json",
            "displaynames/coverageByXPath/cic.json",
            "displaynames/coverageByXPath/ckb-IR.json",
            "displaynames/coverageByXPath/ckb.json",
            "displaynames/coverageByXPath/co.json",
            "displaynames/coverageByXPath/cop.json",
            "displaynames/coverageByXPath/cs.json",
            "displaynames/coverageByXPath/csw.json",
            "displaynames/coverageByXPath/cu.json",
            "displaynames/coverageByXPath/cv.json",
            "displaynames/coverageByXPath/cy.json",
            "displaynames/coverageByXPath/da-GL.json",
            "displaynames/coverageByXPath/da.json",
            "displaynames/coverageByXPath/dav.json",
            "displaynames/coverageByXPath/de-AT.json",
            "displaynames/coverageByXPath/de-BE.json",
            "displaynames/coverageByXPath/de-CH.json",
            "displaynames/coverageByXPath/de-IT.json",
            "displaynames/coverageByXPath/de-LI.json",
            "displaynames/coverageByXPath/de-LU.json",
            "displaynames/coverageByXPath/de.json",
            "displaynames/coverageByXPath/dje.json",
            "displaynames/coverageByXPath/doi.json",
            "displaynames/coverageByXPath/dsb.json",
            "displaynames/coverageByXPath/dua.json",
            "displaynames/coverageByXPath/dv.json",
            "displaynames/coverageByXPath/dyo.json",
            "displaynames/coverageByXPath/dz.json",
            "displaynames/coverageByXPath/ebu.json",
            "displaynames/coverageByXPath/ee-TG.json",
            "displaynames/coverageByXPath/ee.json",
            "displaynames/coverageByXPath/el-CY.json",
            "displaynames/coverageByXPath/el.json",
            "displaynames/coverageByXPath/en-001.json",
            "displaynames/coverageByXPath/en-150.json",
            "displaynames/coverageByXPath/en-AE.json",
            "displaynames/coverageByXPath/en-AG.json",
            "displaynames/coverageByXPath/en-AI.json",
            "displaynames/coverageByXPath/en-AS.json",
            "displaynames/coverageByXPath/en-AT.json",
            "displaynames/coverageByXPath/en-AU.json",
            "displaynames/coverageByXPath/en-BB.json",
            "displaynames/coverageByXPath/en-BE.json",
            "displaynames/coverageByXPath/en-BI.json",
            "displaynames/coverageByXPath/en-BM.json",
            "displaynames/coverageByXPath/en-BS.json",
            "displaynames/coverageByXPath/en-BW.json",
            "displaynames/coverageByXPath/en-BZ.json",
            "displaynames/coverageByXPath/en-CA.json",
            "displaynames/coverageByXPath/en-CC.json",
            "displaynames/coverageByXPath/en-CH.json",
            "displaynames/coverageByXPath/en-CK.json",
            "displaynames/coverageByXPath/en-CM.json",
            "displaynames/coverageByXPath/en-CX.json",
            "displaynames/coverageByXPath/en-CY.json",
            "displaynames/coverageByXPath/en-CZ.json",
            "displaynames/coverageByXPath/en-DE.json",
            "displaynames/coverageByXPath/en-DG.json",
            "displaynames/coverageByXPath/en-DK.json",
            "displaynames/coverageByXPath/en-DM.json",
            "displaynames/coverageByXPath/en-Dsrt.json",
            "displaynames/coverageByXPath/en-EE.json",
            "displaynames/coverageByXPath/en-ER.json",
            "displaynames/coverageByXPath/en-ES.json",
            "displaynames/coverageByXPath/en-FI.json",
            "displaynames/coverageByXPath/en-FJ.json",
            "displaynames/coverageByXPath/en-FK.json",
            "displaynames/coverageByXPath/en-FM.json",
            "displaynames/coverageByXPath/en-FR.json",
            "displaynames/coverageByXPath/en-GB.json",
            "displaynames/coverageByXPath/en-GD.json",
            "displaynames/coverageByXPath/en-GE.json",
            "displaynames/coverageByXPath/en-GG.json",
            "displaynames/coverageByXPath/en-GH.json",
            "displaynames/coverageByXPath/en-GI.json",
            "displaynames/coverageByXPath/en-GM.json",
            "displaynames/coverageByXPath/en-GS.json",
            "displaynames/coverageByXPath/en-GU.json",
            "displaynames/coverageByXPath/en-GY.json",
            "displaynames/coverageByXPath/en-HK.json",
            "displaynames/coverageByXPath/en-HU.json",
            "displaynames/coverageByXPath/en-ID.json",
            "displaynames/coverageByXPath/en-IE.json",
            "displaynames/coverageByXPath/en-IL.json",
            "displaynames/coverageByXPath/en-IM.json",
            "displaynames/coverageByXPath/en-IN.json",
            "displaynames/coverageByXPath/en-IO.json",
            "displaynames/coverageByXPath/en-IT.json",
            "displaynames/coverageByXPath/en-JE.json",
            "displaynames/coverageByXPath/en-JM.json",
            "displaynames/coverageByXPath/en-JP.json",
            "displaynames/coverageByXPath/en-KE.json",
            "displaynames/coverageByXPath/en-KI.json",
            "displaynames/coverageByXPath/en-KN.json",
            "displaynames/coverageByXPath/en-KY.json",
            "displaynames/coverageByXPath/en-LC.json",
            "displaynames/coverageByXPath/en-LR.json",
            "displaynames/coverageByXPath/en-LS.json",
            "displaynames/coverageByXPath/en-LT.json",
            "displaynames/coverageByXPath/en-LV.json",
            "displaynames/coverageByXPath/en-MG.json",
            "displaynames/coverageByXPath/en-MH.json",
            "displaynames/coverageByXPath/en-MO.json",
            "displaynames/coverageByXPath/en-MP.json",
            "displaynames/coverageByXPath/en-MS.json",
            "displaynames/coverageByXPath/en-MT.json",
            "displaynames/coverageByXPath/en-MU.json",
            "displaynames/coverageByXPath/en-MV.json",
            "displaynames/coverageByXPath/en-MW.json",
            "displaynames/coverageByXPath/en-MY.json",
            "displaynames/coverageByXPath/en-NA.json",
            "displaynames/coverageByXPath/en-NF.json",
            "displaynames/coverageByXPath/en-NG.json",
            "displaynames/coverageByXPath/en-NL.json",
            "displaynames/coverageByXPath/en-NO.json",
            "displaynames/coverageByXPath/en-NR.json",
            "displaynames/coverageByXPath/en-NU.json",
            "displaynames/coverageByXPath/en-NZ.json",
            "displaynames/coverageByXPath/en-PG.json",
            "displaynames/coverageByXPath/en-PH.json",
            "displaynames/coverageByXPath/en-PK.json",
            "displaynames/coverageByXPath/en-PL.json",
            "displaynames/coverageByXPath/en-PN.json",
            "displaynames/coverageByXPath/en-PR.json",
            "displaynames/coverageByXPath/en-PT.json",
            "displaynames/coverageByXPath/en-PW.json",
            "displaynames/coverageByXPath/en-RO.json",
            "displaynames/coverageByXPath/en-RW.json",
            "displaynames/coverageByXPath/en-SB.json",
            "displaynames/coverageByXPath/en-SC.json",
            "displaynames/coverageByXPath/en-SD.json",
            "displaynames/coverageByXPath/en-SE.json",
            "displaynames/coverageByXPath/en-SG.json",
            "displaynames/coverageByXPath/en-SH.json",
            "displaynames/coverageByXPath/en-SI.json",
            "displaynames/coverageByXPath/en-SK.json",
            "displaynames/coverageByXPath/en-SL.json",
            "displaynames/coverageByXPath/en-SS.json",
            "displaynames/coverageByXPath/en-SX.json",
            "displaynames/coverageByXPath/en-SZ.json",
            "displaynames/coverageByXPath/en-Shaw.json",
            "displaynames/coverageByXPath/en-TC.json",
            "displaynames/coverageByXPath/en-TK.json",
            "displaynames/coverageByXPath/en-TO.json",
            "displaynames/coverageByXPath/en-TT.json",
            "displaynames/coverageByXPath/en-TV.json",
            "displaynames/coverageByXPath/en-TZ.json",
            "displaynames/coverageByXPath/en-UA.json",
            "displaynames/coverageByXPath/en-UG.json",
            "displaynames/coverageByXPath/en-UM.json",
            "displaynames/coverageByXPath/en-VC.json",
            "displaynames/coverageByXPath/en-VG.json",
            "displaynames/coverageByXPath/en-VI.json",
            "displaynames/coverageByXPath/en-VU.json",
            "displaynames/coverageByXPath/en-WS.json",
            "displaynames/coverageByXPath/en-ZA.json",
            "displaynames/coverageByXPath/en-ZM.json",
            "displaynames/coverageByXPath/en-ZW.json",
            "displaynames/coverageByXPath/en.json",
            "displaynames/coverageByXPath/eo.json",
            "displaynames/coverageByXPath/es-419.json",
            "displaynames/coverageByXPath/es-AR.json",
            "displaynames/coverageByXPath/es-BO.json",
            "displaynames/coverageByXPath/es-BR.json",
            "displaynames/coverageByXPath/es-BZ.json",
            "displaynames/coverageByXPath/es-CL.json",
            "displaynames/coverageByXPath/es-CO.json",
            "displaynames/coverageByXPath/es-CR.json",
            "displaynames/coverageByXPath/es-CU.json",
            "displaynames/coverageByXPath/es-DO.json",
            "displaynames/coverageByXPath/es-EA.json",
            "displaynames/coverageByXPath/es-EC.json",
            "displaynames/coverageByXPath/es-GQ.json",
            "displaynames/coverageByXPath/es-GT.json",
            "displaynames/coverageByXPath/es-HN.json",
            "displaynames/coverageByXPath/es-IC.json",
            "displaynames/coverageByXPath/es-MX.json",
            "displaynames/coverageByXPath/es-NI.json",
            "displaynames/coverageByXPath/es-PA.json",
            "displaynames/coverageByXPath/es-PE.json",
            "displaynames/coverageByXPath/es-PH.json",
            "displaynames/coverageByXPath/es-PR.json",
            "displaynames/coverageByXPath/es-PY.json",
            "displaynames/coverageByXPath/es-SV.json",
            "displaynames/coverageByXPath/es-US.json",
            "displaynames/coverageByXPath/es-UY.json",
            "displaynames/coverageByXPath/es-VE.json",
            "displaynames/coverageByXPath/es.json",
            "displaynames/coverageByXPath/et.json",
            "displaynames/coverageByXPath/eu.json",
            "displaynames/coverageByXPath/ewo.json",
            "displaynames/coverageByXPath/fa-AF.json",
            "displaynames/coverageByXPath/fa.json",
            "displaynames/coverageByXPath/ff-Adlm-BF.json",
            "displaynames/coverageByXPath/ff-Adlm-CM.json",
            "displaynames/coverageByXPath/ff-Adlm-GH.json",
            "displaynames/coverageByXPath/ff-Adlm-GM.json",
            "displaynames/coverageByXPath/ff-Adlm-GW.json",
            "displaynames/coverageByXPath/ff-Adlm-LR.json",
            "displaynames/coverageByXPath/ff-Adlm-MR.json",
            "displaynames/coverageByXPath/ff-Adlm-NE.json",
            "displaynames/coverageByXPath/ff-Adlm-NG.json",
            "displaynames/coverageByXPath/ff-Adlm-SL.json",
            "displaynames/coverageByXPath/ff-Adlm-SN.json",
            "displaynames/coverageByXPath/ff-Adlm.json",
            "displaynames/coverageByXPath/ff-Latn-BF.json",
            "displaynames/coverageByXPath/ff-Latn-CM.json",
            "displaynames/coverageByXPath/ff-Latn-GH.json",
            "displaynames/coverageByXPath/ff-Latn-GM.json",
            "displaynames/coverageByXPath/ff-Latn-GN.json",
            "displaynames/coverageByXPath/ff-Latn-GW.json",
            "displaynames/coverageByXPath/ff-Latn-LR.json",
            "displaynames/coverageByXPath/ff-Latn-MR.json",
            "displaynames/coverageByXPath/ff-Latn-NE.json",
            "displaynames/coverageByXPath/ff-Latn-NG.json",
            "displaynames/coverageByXPath/ff-Latn-SL.json",
            "displaynames/coverageByXPath/ff-Latn.json",
            "displaynames/coverageByXPath/ff.json",
            "displaynames/coverageByXPath/fi.json",
            "displaynames/coverageByXPath/fil.json",
            "displaynames/coverageByXPath/fo-DK.json",
            "displaynames/coverageByXPath/fo.json",
            "displaynames/coverageByXPath/fr-BE.json",
            "displaynames/coverageByXPath/fr-BF.json",
            "displaynames/coverageByXPath/fr-BI.json",
            "displaynames/coverageByXPath/fr-BJ.json",
            "displaynames/coverageByXPath/fr-BL.json",
            "displaynames/coverageByXPath/fr-CA.json",
            "displaynames/coverageByXPath/fr-CD.json",
            "displaynames/coverageByXPath/fr-CF.json",
            "displaynames/coverageByXPath/fr-CG.json",
            "displaynames/coverageByXPath/fr-CH.json",
            "displaynames/coverageByXPath/fr-CI.json",
            "displaynames/coverageByXPath/fr-CM.json",
            "displaynames/coverageByXPath/fr-DJ.json",
            "displaynames/coverageByXPath/fr-DZ.json",
            "displaynames/coverageByXPath/fr-GA.json",
            "displaynames/coverageByXPath/fr-GF.json",
            "displaynames/coverageByXPath/fr-GN.json",
            "displaynames/coverageByXPath/fr-GP.json",
            "displaynames/coverageByXPath/fr-GQ.json",
            "displaynames/coverageByXPath/fr-HT.json",
            "displaynames/coverageByXPath/fr-KM.json",
            "displaynames/coverageByXPath/fr-LU.json",
            "displaynames/coverageByXPath/fr-MA.json",
            "displaynames/coverageByXPath/fr-MC.json",
            "displaynames/coverageByXPath/fr-MF.json",
            "displaynames/coverageByXPath/fr-MG.json",
            "displaynames/coverageByXPath/fr-ML.json",
            "displaynames/coverageByXPath/fr-MQ.json",
            "displaynames/coverageByXPath/fr-MR.json",
            "displaynames/coverageByXPath/fr-MU.json",
            "displaynames/coverageByXPath/fr-NC.json",
            "displaynames/coverageByXPath/fr-NE.json",
            "displaynames/coverageByXPath/fr-PF.json",
            "displaynames/coverageByXPath/fr-PM.json",
            "displaynames/coverageByXPath/fr-RE.json",
            "displaynames/coverageByXPath/fr-RW.json",
            "displaynames/coverageByXPath/fr-SC.json",
            "displaynames/coverageByXPath/fr-SN.json",
            "displaynames/coverageByXPath/fr-SY.json",
            "displaynames/coverageByXPath/fr-TD.json",
            "displaynames/coverageByXPath/fr-TG.json",
            "displaynames/coverageByXPath/fr-TN.json",
            "displaynames/coverageByXPath/fr-VU.json",
            "displaynames/coverageByXPath/fr-WF.json",
            "displaynames/coverageByXPath/fr-YT.json",
            "displaynames/coverageByXPath/fr.json",
            "displaynames/coverageByXPath/frr.json",
            "displaynames/coverageByXPath/fur.json",
            "displaynames/coverageByXPath/fy.json",
            "displaynames/coverageByXPath/ga-GB.json",
            "displaynames/coverageByXPath/ga.json",
            "displaynames/coverageByXPath/gaa.json",
            "displaynames/coverageByXPath/gd.json",
            "displaynames/coverageByXPath/gez-ER.json",
            "displaynames/coverageByXPath/gez.json",
            "displaynames/coverageByXPath/gl.json",
            "displaynames/coverageByXPath/gn.json",
            "displaynames/coverageByXPath/gsw-FR.json",
            "displaynames/coverageByXPath/gsw-LI.json",
            "displaynames/coverageByXPath/gsw.json",
            "displaynames/coverageByXPath/gu.json",
            "displaynames/coverageByXPath/guz.json",
            "displaynames/coverageByXPath/gv.json",
            "displaynames/coverageByXPath/ha-Arab-SD.json",
            "displaynames/coverageByXPath/ha-Arab.json",
            "displaynames/coverageByXPath/ha-GH.json",
            "displaynames/coverageByXPath/ha-NE.json",
            "displaynames/coverageByXPath/ha.json",
            "displaynames/coverageByXPath/haw.json",
            "displaynames/coverageByXPath/he.json",
            "displaynames/coverageByXPath/hi-Latn.json",
            "displaynames/coverageByXPath/hi.json",
            "displaynames/coverageByXPath/hnj-Hmnp.json",
            "displaynames/coverageByXPath/hnj.json",
            "displaynames/coverageByXPath/hr-BA.json",
            "displaynames/coverageByXPath/hr.json",
            "displaynames/coverageByXPath/hrx.json",
            "displaynames/coverageByXPath/hsb.json",
            "displaynames/coverageByXPath/ht.json",
            "displaynames/coverageByXPath/hu.json",
            "displaynames/coverageByXPath/hy.json",
            "displaynames/coverageByXPath/ia.json",
            "displaynames/coverageByXPath/id.json",
            "displaynames/coverageByXPath/ie.json",
            "displaynames/coverageByXPath/ig.json",
            "displaynames/coverageByXPath/ii.json",
            "displaynames/coverageByXPath/io.json",
            "displaynames/coverageByXPath/is.json",
            "displaynames/coverageByXPath/isv-Cyrl.json",
            "displaynames/coverageByXPath/isv-Latn.json",
            "displaynames/coverageByXPath/isv.json",
            "displaynames/coverageByXPath/it-CH.json",
            "displaynames/coverageByXPath/it-SM.json",
            "displaynames/coverageByXPath/it-VA.json",
            "displaynames/coverageByXPath/it.json",
            "displaynames/coverageByXPath/iu-Latn.json",
            "displaynames/coverageByXPath/iu.json",
            "displaynames/coverageByXPath/ja.json",
            "displaynames/coverageByXPath/jbo.json",
            "displaynames/coverageByXPath/jgo.json",
            "displaynames/coverageByXPath/jmc.json",
            "displaynames/coverageByXPath/jv.json",
            "displaynames/coverageByXPath/ka.json",
            "displaynames/coverageByXPath/kaa-Cyrl.json",
            "displaynames/coverageByXPath/kaa-Latn.json",
            "displaynames/coverageByXPath/kaa.json",
            "displaynames/coverageByXPath/kab.json",
            "displaynames/coverageByXPath/kaj.json",
            "displaynames/coverageByXPath/kam.json",
            "displaynames/coverageByXPath/kbd-TR.json",
            "displaynames/coverageByXPath/kbd.json",
            "displaynames/coverageByXPath/kcg.json",
            "displaynames/coverageByXPath/kde.json",
            "displaynames/coverageByXPath/kea.json",
            "displaynames/coverageByXPath/kek.json",
            "displaynames/coverageByXPath/ken.json",
            "displaynames/coverageByXPath/kgp.json",
            "displaynames/coverageByXPath/khq.json",
            "displaynames/coverageByXPath/ki.json",
            "displaynames/coverageByXPath/kk-Arab.json",
            "displaynames/coverageByXPath/kk-Cyrl.json",
            "displaynames/coverageByXPath/kk-KZ.json",
            "displaynames/coverageByXPath/kk.json",
            "displaynames/coverageByXPath/kkj.json",
            "displaynames/coverageByXPath/kl.json",
            "displaynames/coverageByXPath/kln.json",
            "displaynames/coverageByXPath/km.json",
            "displaynames/coverageByXPath/kn.json",
            "displaynames/coverageByXPath/ko-CN.json",
            "displaynames/coverageByXPath/ko-KP.json",
            "displaynames/coverageByXPath/ko.json",
            "displaynames/coverageByXPath/kok-Deva.json",
            "displaynames/coverageByXPath/kok-Latn.json",
            "displaynames/coverageByXPath/kok.json",
            "displaynames/coverageByXPath/kpe-GN.json",
            "displaynames/coverageByXPath/kpe.json",
            "displaynames/coverageByXPath/ks-Arab.json",
            "displaynames/coverageByXPath/ks-Deva.json",
            "displaynames/coverageByXPath/ks.json",
            "displaynames/coverageByXPath/ksb.json",
            "displaynames/coverageByXPath/ksf.json",
            "displaynames/coverageByXPath/ksh.json",
            "displaynames/coverageByXPath/ku-Arab-IR.json",
            "displaynames/coverageByXPath/ku-Arab.json",
            "displaynames/coverageByXPath/ku-Latn-IQ.json",
            "displaynames/coverageByXPath/ku-Latn-SY.json",
            "displaynames/coverageByXPath/ku-Latn.json",
            "displaynames/coverageByXPath/ku-TR.json",
            "displaynames/coverageByXPath/ku.json",
            "displaynames/coverageByXPath/kw.json",
            "displaynames/coverageByXPath/kxv-Deva.json",
            "displaynames/coverageByXPath/kxv-Latn.json",
            "displaynames/coverageByXPath/kxv-Orya.json",
            "displaynames/coverageByXPath/kxv-Telu.json",
            "displaynames/coverageByXPath/kxv.json",
            "displaynames/coverageByXPath/ky.json",
            "displaynames/coverageByXPath/la.json",
            "displaynames/coverageByXPath/lag.json",
            "displaynames/coverageByXPath/lb.json",
            "displaynames/coverageByXPath/lg.json",
            "displaynames/coverageByXPath/lij.json",
            "displaynames/coverageByXPath/lkt.json",
            "displaynames/coverageByXPath/lld.json",
            "displaynames/coverageByXPath/lmo.json",
            "displaynames/coverageByXPath/ln-AO.json",
            "displaynames/coverageByXPath/ln-CF.json",
            "displaynames/coverageByXPath/ln-CG.json",
            "displaynames/coverageByXPath/ln.json",
            "displaynames/coverageByXPath/lo.json",
            "displaynames/coverageByXPath/lrc-IQ.json",
            "displaynames/coverageByXPath/lrc.json",
            "displaynames/coverageByXPath/lt.json",
            "displaynames/coverageByXPath/ltg.json",
            "displaynames/coverageByXPath/lu.json",
            "displaynames/coverageByXPath/luo.json",
            "displaynames/coverageByXPath/luy.json",
            "displaynames/coverageByXPath/lv.json",
            "displaynames/coverageByXPath/lzz.json",
            "displaynames/coverageByXPath/mai.json",
            "displaynames/coverageByXPath/mas-TZ.json",
            "displaynames/coverageByXPath/mas.json",
            "displaynames/coverageByXPath/mdf.json",
            "displaynames/coverageByXPath/mer.json",
            "displaynames/coverageByXPath/mfe.json",
            "displaynames/coverageByXPath/mg.json",
            "displaynames/coverageByXPath/mgh.json",
            "displaynames/coverageByXPath/mgo.json",
            "displaynames/coverageByXPath/mhn.json",
            "displaynames/coverageByXPath/mi.json",
            "displaynames/coverageByXPath/mic.json",
            "displaynames/coverageByXPath/mk.json",
            "displaynames/coverageByXPath/ml.json",
            "displaynames/coverageByXPath/mn-Mong-MN.json",
            "displaynames/coverageByXPath/mn-Mong.json",
            "displaynames/coverageByXPath/mn.json",
            "displaynames/coverageByXPath/mni-Beng.json",
            "displaynames/coverageByXPath/mni-Mtei.json",
            "displaynames/coverageByXPath/mni.json",
            "displaynames/coverageByXPath/moh.json",
            "displaynames/coverageByXPath/mr.json",
            "displaynames/coverageByXPath/mrh-MM.json",
            "displaynames/coverageByXPath/mrh.json",
            "displaynames/coverageByXPath/ms-Arab-BN.json",
            "displaynames/coverageByXPath/ms-Arab.json",
            "displaynames/coverageByXPath/ms-BN.json",
            "displaynames/coverageByXPath/ms-ID.json",
            "displaynames/coverageByXPath/ms-SG.json",
            "displaynames/coverageByXPath/ms.json",
            "displaynames/coverageByXPath/mt.json",
            "displaynames/coverageByXPath/mua.json",
            "displaynames/coverageByXPath/mus.json",
            "displaynames/coverageByXPath/mww-Hmnp.json",
            "displaynames/coverageByXPath/mww.json",
            "displaynames/coverageByXPath/my.json",
            "displaynames/coverageByXPath/myv.json",
            "displaynames/coverageByXPath/mzn.json",
            "displaynames/coverageByXPath/naq.json",
            "displaynames/coverageByXPath/nb-SJ.json",
            "displaynames/coverageByXPath/nb.json",
            "displaynames/coverageByXPath/nd.json",
            "displaynames/coverageByXPath/nds-NL.json",
            "displaynames/coverageByXPath/nds.json",
            "displaynames/coverageByXPath/ne-IN.json",
            "displaynames/coverageByXPath/ne.json",
            "displaynames/coverageByXPath/nl-AW.json",
            "displaynames/coverageByXPath/nl-BE.json",
            "displaynames/coverageByXPath/nl-BQ.json",
            "displaynames/coverageByXPath/nl-CW.json",
            "displaynames/coverageByXPath/nl-SR.json",
            "displaynames/coverageByXPath/nl-SX.json",
            "displaynames/coverageByXPath/nl.json",
            "displaynames/coverageByXPath/nmg.json",
            "displaynames/coverageByXPath/nn.json",
            "displaynames/coverageByXPath/nnh.json",
            "displaynames/coverageByXPath/no.json",
            "displaynames/coverageByXPath/nqo.json",
            "displaynames/coverageByXPath/nr.json",
            "displaynames/coverageByXPath/nso.json",
            "displaynames/coverageByXPath/nus.json",
            "displaynames/coverageByXPath/nv.json",
            "displaynames/coverageByXPath/ny.json",
            "displaynames/coverageByXPath/nyn.json",
            "displaynames/coverageByXPath/oc-ES.json",
            "displaynames/coverageByXPath/oc.json",
            "displaynames/coverageByXPath/oka-US.json",
            "displaynames/coverageByXPath/oka.json",
            "displaynames/coverageByXPath/om-KE.json",
            "displaynames/coverageByXPath/om.json",
            "displaynames/coverageByXPath/or.json",
            "displaynames/coverageByXPath/os-RU.json",
            "displaynames/coverageByXPath/os.json",
            "displaynames/coverageByXPath/osa.json",
            "displaynames/coverageByXPath/pa-Arab.json",
            "displaynames/coverageByXPath/pa-Guru.json",
            "displaynames/coverageByXPath/pa.json",
            "displaynames/coverageByXPath/pap-AW.json",
            "displaynames/coverageByXPath/pap.json",
            "displaynames/coverageByXPath/pcm.json",
            "displaynames/coverageByXPath/pi-Latn.json",
            "displaynames/coverageByXPath/pi.json",
            "displaynames/coverageByXPath/pis.json",
            "displaynames/coverageByXPath/pl.json",
            "displaynames/coverageByXPath/pms.json",
            "displaynames/coverageByXPath/prg.json",
            "displaynames/coverageByXPath/ps-PK.json",
            "displaynames/coverageByXPath/ps.json",
            "displaynames/coverageByXPath/pt-AO.json",
            "displaynames/coverageByXPath/pt-CH.json",
            "displaynames/coverageByXPath/pt-CV.json",
            "displaynames/coverageByXPath/pt-GQ.json",
            "displaynames/coverageByXPath/pt-GW.json",
            "displaynames/coverageByXPath/pt-LU.json",
            "displaynames/coverageByXPath/pt-MO.json",
            "displaynames/coverageByXPath/pt-MZ.json",
            "displaynames/coverageByXPath/pt-PT.json",
            "displaynames/coverageByXPath/pt-ST.json",
            "displaynames/coverageByXPath/pt-TL.json",
            "displaynames/coverageByXPath/pt.json",
            "displaynames/coverageByXPath/qu-BO.json",
            "displaynames/coverageByXPath/qu-EC.json",
            "displaynames/coverageByXPath/qu.json",
            "displaynames/coverageByXPath/quc.json",
            "displaynames/coverageByXPath/raj.json",
            "displaynames/coverageByXPath/rhg-Rohg-BD.json",
            "displaynames/coverageByXPath/rhg-Rohg.json",
            "displaynames/coverageByXPath/rhg.json",
            "displaynames/coverageByXPath/rif.json",
            "displaynames/coverageByXPath/rm.json",
            "displaynames/coverageByXPath/rn.json",
            "displaynames/coverageByXPath/ro-MD.json",
            "displaynames/coverageByXPath/ro.json",
            "displaynames/coverageByXPath/rof.json",
            "displaynames/coverageByXPath/ru-BY.json",
            "displaynames/coverageByXPath/ru-KG.json",
            "displaynames/coverageByXPath/ru-KZ.json",
            "displaynames/coverageByXPath/ru-MD.json",
            "displaynames/coverageByXPath/ru-UA.json",
            "displaynames/coverageByXPath/ru.json",
            "displaynames/coverageByXPath/rw.json",
            "displaynames/coverageByXPath/rwk.json",
            "displaynames/coverageByXPath/sa.json",
            "displaynames/coverageByXPath/sah.json",
            "displaynames/coverageByXPath/saq.json",
            "displaynames/coverageByXPath/sat-Deva.json",
            "displaynames/coverageByXPath/sat-Olck.json",
            "displaynames/coverageByXPath/sat.json",
            "displaynames/coverageByXPath/sbp.json",
            "displaynames/coverageByXPath/sc.json",
            "displaynames/coverageByXPath/scn.json",
            "displaynames/coverageByXPath/sd-Arab.json",
            "displaynames/coverageByXPath/sd-Deva.json",
            "displaynames/coverageByXPath/sd.json",
            "displaynames/coverageByXPath/sdh-IQ.json",
            "displaynames/coverageByXPath/sdh.json",
            "displaynames/coverageByXPath/se-FI.json",
            "displaynames/coverageByXPath/se-SE.json",
            "displaynames/coverageByXPath/se.json",
            "displaynames/coverageByXPath/seh.json",
            "displaynames/coverageByXPath/ses.json",
            "displaynames/coverageByXPath/sg.json",
            "displaynames/coverageByXPath/sgs.json",
            "displaynames/coverageByXPath/shi-Latn.json",
            "displaynames/coverageByXPath/shi-Tfng.json",
            "displaynames/coverageByXPath/shi.json",
            "displaynames/coverageByXPath/shn-TH.json",
            "displaynames/coverageByXPath/shn.json",
            "displaynames/coverageByXPath/si.json",
            "displaynames/coverageByXPath/sid.json",
            "displaynames/coverageByXPath/sk.json",
            "displaynames/coverageByXPath/skr.json",
            "displaynames/coverageByXPath/sl.json",
            "displaynames/coverageByXPath/sma-NO.json",
            "displaynames/coverageByXPath/sma.json",
            "displaynames/coverageByXPath/smj-NO.json",
            "displaynames/coverageByXPath/smj.json",
            "displaynames/coverageByXPath/smn.json",
            "displaynames/coverageByXPath/sms.json",
            "displaynames/coverageByXPath/sn.json",
            "displaynames/coverageByXPath/so-DJ.json",
            "displaynames/coverageByXPath/so-ET.json",
            "displaynames/coverageByXPath/so-KE.json",
            "displaynames/coverageByXPath/so.json",
            "displaynames/coverageByXPath/sq-MK.json",
            "displaynames/coverageByXPath/sq-XK.json",
            "displaynames/coverageByXPath/sq.json",
            "displaynames/coverageByXPath/sr-Cyrl-BA.json",
            "displaynames/coverageByXPath/sr-Cyrl-ME.json",
            "displaynames/coverageByXPath/sr-Cyrl-XK.json",
            "displaynames/coverageByXPath/sr-Cyrl.json",
            "displaynames/coverageByXPath/sr-Latn-BA.json",
            "displaynames/coverageByXPath/sr-Latn-ME.json",
            "displaynames/coverageByXPath/sr-Latn-XK.json",
            "displaynames/coverageByXPath/sr-Latn.json",
            "displaynames/coverageByXPath/sr.json",
            "displaynames/coverageByXPath/ss-SZ.json",
            "displaynames/coverageByXPath/ss.json",
            "displaynames/coverageByXPath/ssy.json",
            "displaynames/coverageByXPath/st-LS.json",
            "displaynames/coverageByXPath/st.json",
            "displaynames/coverageByXPath/su-Latn.json",
            "displaynames/coverageByXPath/su.json",
            "displaynames/coverageByXPath/sus-SL.json",
            "displaynames/coverageByXPath/sus.json",
            "displaynames/coverageByXPath/suz-Deva.json",
            "displaynames/coverageByXPath/suz-Sunu.json",
            "displaynames/coverageByXPath/suz.json",
            "displaynames/coverageByXPath/sv-AX.json",
            "displaynames/coverageByXPath/sv-FI.json",
            "displaynames/coverageByXPath/sv.json",
            "displaynames/coverageByXPath/sw-CD.json",
            "displaynames/coverageByXPath/sw-KE.json",
            "displaynames/coverageByXPath/sw-UG.json",
            "displaynames/coverageByXPath/sw.json",
            "displaynames/coverageByXPath/syr-SY.json",
            "displaynames/coverageByXPath/syr.json",
            "displaynames/coverageByXPath/szl.json",
            "displaynames/coverageByXPath/ta-LK.json",
            "displaynames/coverageByXPath/ta-MY.json",
            "displaynames/coverageByXPath/ta-SG.json",
            "displaynames/coverageByXPath/ta.json",
            "displaynames/coverageByXPath/te.json",
            "displaynames/coverageByXPath/teo-KE.json",
            "displaynames/coverageByXPath/teo.json",
            "displaynames/coverageByXPath/tg.json",
            "displaynames/coverageByXPath/th.json",
            "displaynames/coverageByXPath/ti-ER.json",
            "displaynames/coverageByXPath/ti.json",
            "displaynames/coverageByXPath/tig.json",
            "displaynames/coverageByXPath/tk.json",
            "displaynames/coverageByXPath/tn-BW.json",
            "displaynames/coverageByXPath/tn.json",
            "displaynames/coverageByXPath/to.json",
            "displaynames/coverageByXPath/tok.json",
            "displaynames/coverageByXPath/tpi.json",
            "displaynames/coverageByXPath/tr-CY.json",
            "displaynames/coverageByXPath/tr.json",
            "displaynames/coverageByXPath/trv.json",
            "displaynames/coverageByXPath/trw.json",
            "displaynames/coverageByXPath/ts.json",
            "displaynames/coverageByXPath/tt.json",
            "displaynames/coverageByXPath/twq.json",
            "displaynames/coverageByXPath/tyv.json",
            "displaynames/coverageByXPath/tzm.json",
            "displaynames/coverageByXPath/ug.json",
            "displaynames/coverageByXPath/uk.json",
            "displaynames/coverageByXPath/und.json",
            "displaynames/coverageByXPath/ur-IN.json",
            "displaynames/coverageByXPath/ur.json",
            "displaynames/coverageByXPath/uz-Arab.json",
            "displaynames/coverageByXPath/uz-Cyrl.json",
            "displaynames/coverageByXPath/uz-Latn.json",
            "displaynames/coverageByXPath/uz.json",
            "displaynames/coverageByXPath/vai-Latn.json",
            "displaynames/coverageByXPath/vai-Vaii.json",
            "displaynames/coverageByXPath/vai.json",
            "displaynames/coverageByXPath/ve.json",
            "displaynames/coverageByXPath/vec.json",
            "displaynames/coverageByXPath/vi.json",
            "displaynames/coverageByXPath/vmw.json",
            "displaynames/coverageByXPath/vo.json",
            "displaynames/coverageByXPath/vun.json",
            "displaynames/coverageByXPath/wa.json",
            "displaynames/coverageByXPath/wae.json",
            "displaynames/coverageByXPath/wal.json",
            "displaynames/coverageByXPath/wbp.json",
            "displaynames/coverageByXPath/wo.json",
            "displaynames/coverageByXPath/xdq.json",
            "displaynames/coverageByXPath/xh.json",
            "displaynames/coverageByXPath/xnr.json",
            "displaynames/coverageByXPath/xog.json",
            "displaynames/coverageByXPath/yav.json",
            "displaynames/coverageByXPath/yi.json",
            "displaynames/coverageByXPath/yo-BJ.json",
            "displaynames/coverageByXPath/yo.json",
            "displaynames/coverageByXPath/yrl-CO.json",
            "displaynames/coverageByXPath/yrl-VE.json",
            "displaynames/coverageByXPath/yrl.json",
            "displaynames/coverageByXPath/yue-Hans.json",
            "displaynames/coverageByXPath/yue-Hant-CN.json",
            "displaynames/coverageByXPath/yue-Hant-MO.json",
            "displaynames/coverageByXPath/yue-Hant.json",
            "displaynames/coverageByXPath/yue.json",
            "displaynames/coverageByXPath/za.json",
            "displaynames/coverageByXPath/zgh.json",
            "displaynames/coverageByXPath/zh-Hans-HK.json",
            "displaynames/coverageByXPath/zh-Hans-MO.json",
            "displaynames/coverageByXPath/zh-Hans-MY.json",
            "displaynames/coverageByXPath/zh-Hans-SG.json",
            "displaynames/coverageByXPath/zh-Hans.json",
            "displaynames/coverageByXPath/zh-Hant-HK.json",
            "displaynames/coverageByXPath/zh-Hant-MO.json",
            "displaynames/coverageByXPath/zh-Hant-MY.json",
            "displaynames/coverageByXPath/zh-Hant.json",
            "displaynames/coverageByXPath/zh-Latn.json",
            "displaynames/coverageByXPath/zh.json",
            "displaynames/coverageByXPath/zu.json",
        )))
    })
}

#[cfg(test)]
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
pub(super) fn for_each_cldr_key_and_tier<Resource, T>(
    cldr: &CldrCache,
    file_name: &str,
    ignored_alts: &[Alt],
    get_category: impl Fn(&CoverageByXPathLevels) -> &CoverageCategoryLevels,
    mut extract_keys: impl FnMut(&Resource) -> &HashMap<WithAlt<T>, String>,
    mut callback: impl FnMut(&DataLocale, &WithAlt<T>, CoverageLevelForXPath),
) where
    Resource: serde::de::DeserializeOwned + Send + Sync + 'static,
    T: Writeable,
{
    let coverage_cldr = coverage_cldr_cache();
    let displaynames_dir = cldr.displaynames();
    let mut locales = displaynames_dir.list_locales().unwrap().collect::<Vec<_>>();
    locales.sort_by(|a, b| a.total_cmp(b));
    for locale in locales {
        if let Ok(res) = displaynames_dir.read_and_parse::<Resource>(&locale, file_name) {
            let mut keys = extract_keys(res).keys().collect::<Vec<_>>();
            keys.sort_by_cached_key(|k| (k.subtag.write_to_string().to_string(), k.alt, k.menu));
            for key in keys {
                if let Some(alt) = key.alt
                    && ignored_alts.contains(&alt)
                {
                    continue;
                }
                let tier = coverage_cldr
                    .coverage_tier(&locale, &get_category, &key.subtag, key.alt, key.menu, cldr)
                    .unwrap();
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
    let coverage_cldr = coverage_cldr_cache();

    let en = DataLocale::from_str("en").unwrap();
    assert_eq!(
        coverage_cldr
            .coverage_tier(&en, |l| &l.language, "en", None, None, cldr)
            .unwrap(),
        CoverageLevelForXPath::Basic
    );

    assert_eq!(
        coverage_cldr
            .coverage_tier(&en, |l| &l.language, "unlisted_test_code", None, None, cldr)
            .unwrap(),
        CoverageLevelForXPath::Comprehensive
    );
}

#[test]
fn test_all_filesystem_locales_in_coverage_cldr_cache() {
    use crate::source::AbstractFs;
    let data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let fs_cldr = SerdeCache::new(AbstractFs::new(&data_dir).unwrap());
    let coverage_cldr = coverage_cldr_cache();

    let files = fs_cldr.list("displaynames/coverageByXPath").unwrap();

    for file_name in files {
        let full_path = format!("displaynames/coverageByXPath/{file_name}");
        assert!(
            coverage_cldr.0.file_exists(&full_path).unwrap(),
            "Missing file in coverage_cldr_cache: {full_path}"
        );
    }
}
