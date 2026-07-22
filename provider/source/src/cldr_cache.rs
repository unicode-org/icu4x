// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // features

use crate::CoverageLevel;
use crate::cldr_serde::coverage_by_xpath::CoverageByXPathResource;
use crate::cldr_serde::eras::EraData;
use crate::datetime::DatagenCalendar;
use crate::source::{AbstractFs, SerdeCache};
use icu::locale::LanguageIdentifier;
use icu::locale::LocaleExpander;
use icu::locale::subtags::Language;
#[cfg(feature = "unstable")]
use icu::locale::subtags::Region;
use icu_provider::DataError;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::OnceLock;
use writeable::Writeable;

/// Coverage tiers for display names items derived from CLDR `coverageByXPath` JSON definitions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum CoverageLevelForXPath {
    /// Items with `core` CLDR coverage level.
    Core,
    /// Items with `basic` CLDR coverage level.
    Basic,
    /// Items with `moderate` CLDR coverage level.
    Moderate,
    /// Items with `modern` CLDR coverage level.
    Modern,
    /// Items with `comprehensive` CLDR coverage level, or unlisted items.
    Comprehensive,
}

#[derive(Debug)]
pub(crate) struct CldrCache {
    pub(crate) serde_cache: SerdeCache,
    extended_locale_expander: OnceLock<Result<LocaleExpander, DataError>>,
    #[expect(clippy::type_complexity)]
    pub(crate) calendar_eras: OnceLock<
        Result<
            BTreeMap<DatagenCalendar, (Option<DatagenCalendar>, Vec<(usize, EraData)>)>,
            DataError,
        >,
    >,
    #[cfg(feature = "unstable")]
    // used by transforms/mod.rs
    pub(crate) transforms: OnceLock<
        Result<std::sync::Mutex<icu::experimental::transliterate::RuleCollection>, DataError>,
    >,
    pub(crate) tz_caches: crate::time_zones::Caches,
}

// Note: We statically embed coverage levels for all CLDR locales here so we can support
// locale display name depth slicing before these data are officially published to CLDR JSON.
// TODO: Remove this static cache and read exclusively from CLDR once coverageByXPath is published.
pub(crate) fn coverage_cldr_cache() -> &'static CldrCache {
    static SINGLETON: OnceLock<CldrCache> = OnceLock::new();
    SINGLETON.get_or_init(|| {
        CldrCache::new(crate::source::include_files!(
            "../data/";
            "cldr-misc-full/coverageByXPath.json",
            "cldr-misc-full/coverageByXPath/aa-DJ.json",
            "cldr-misc-full/coverageByXPath/aa-ER.json",
            "cldr-misc-full/coverageByXPath/aa.json",
            "cldr-misc-full/coverageByXPath/ab.json",
            "cldr-misc-full/coverageByXPath/ady-JO.json",
            "cldr-misc-full/coverageByXPath/ady-TR.json",
            "cldr-misc-full/coverageByXPath/ady.json",
            "cldr-misc-full/coverageByXPath/af-NA.json",
            "cldr-misc-full/coverageByXPath/af.json",
            "cldr-misc-full/coverageByXPath/agq.json",
            "cldr-misc-full/coverageByXPath/ak.json",
            "cldr-misc-full/coverageByXPath/am.json",
            "cldr-misc-full/coverageByXPath/an.json",
            "cldr-misc-full/coverageByXPath/ann.json",
            "cldr-misc-full/coverageByXPath/apc.json",
            "cldr-misc-full/coverageByXPath/ar-AE.json",
            "cldr-misc-full/coverageByXPath/ar-BH.json",
            "cldr-misc-full/coverageByXPath/ar-DJ.json",
            "cldr-misc-full/coverageByXPath/ar-DZ.json",
            "cldr-misc-full/coverageByXPath/ar-EG.json",
            "cldr-misc-full/coverageByXPath/ar-EH.json",
            "cldr-misc-full/coverageByXPath/ar-ER.json",
            "cldr-misc-full/coverageByXPath/ar-IL.json",
            "cldr-misc-full/coverageByXPath/ar-IQ.json",
            "cldr-misc-full/coverageByXPath/ar-JO.json",
            "cldr-misc-full/coverageByXPath/ar-KM.json",
            "cldr-misc-full/coverageByXPath/ar-KW.json",
            "cldr-misc-full/coverageByXPath/ar-LB.json",
            "cldr-misc-full/coverageByXPath/ar-LY.json",
            "cldr-misc-full/coverageByXPath/ar-MA.json",
            "cldr-misc-full/coverageByXPath/ar-MR.json",
            "cldr-misc-full/coverageByXPath/ar-OM.json",
            "cldr-misc-full/coverageByXPath/ar-PS.json",
            "cldr-misc-full/coverageByXPath/ar-QA.json",
            "cldr-misc-full/coverageByXPath/ar-SA.json",
            "cldr-misc-full/coverageByXPath/ar-SD.json",
            "cldr-misc-full/coverageByXPath/ar-SO.json",
            "cldr-misc-full/coverageByXPath/ar-SS.json",
            "cldr-misc-full/coverageByXPath/ar-SY.json",
            "cldr-misc-full/coverageByXPath/ar-TD.json",
            "cldr-misc-full/coverageByXPath/ar-TN.json",
            "cldr-misc-full/coverageByXPath/ar-YE.json",
            "cldr-misc-full/coverageByXPath/ar.json",
            "cldr-misc-full/coverageByXPath/arn.json",
            "cldr-misc-full/coverageByXPath/ary.json",
            "cldr-misc-full/coverageByXPath/as.json",
            "cldr-misc-full/coverageByXPath/asa.json",
            "cldr-misc-full/coverageByXPath/ast.json",
            "cldr-misc-full/coverageByXPath/az-Arab-IQ.json",
            "cldr-misc-full/coverageByXPath/az-Arab-TR.json",
            "cldr-misc-full/coverageByXPath/az-Arab.json",
            "cldr-misc-full/coverageByXPath/az-Cyrl.json",
            "cldr-misc-full/coverageByXPath/az-Latn.json",
            "cldr-misc-full/coverageByXPath/az.json",
            "cldr-misc-full/coverageByXPath/ba.json",
            "cldr-misc-full/coverageByXPath/bal-Arab.json",
            "cldr-misc-full/coverageByXPath/bal-Latn.json",
            "cldr-misc-full/coverageByXPath/bal.json",
            "cldr-misc-full/coverageByXPath/bas.json",
            "cldr-misc-full/coverageByXPath/be.json",
            "cldr-misc-full/coverageByXPath/bem.json",
            "cldr-misc-full/coverageByXPath/bew.json",
            "cldr-misc-full/coverageByXPath/bez.json",
            "cldr-misc-full/coverageByXPath/bg.json",
            "cldr-misc-full/coverageByXPath/bgc.json",
            "cldr-misc-full/coverageByXPath/bgn-AE.json",
            "cldr-misc-full/coverageByXPath/bgn-AF.json",
            "cldr-misc-full/coverageByXPath/bgn-IR.json",
            "cldr-misc-full/coverageByXPath/bgn-OM.json",
            "cldr-misc-full/coverageByXPath/bgn.json",
            "cldr-misc-full/coverageByXPath/bho.json",
            "cldr-misc-full/coverageByXPath/blo.json",
            "cldr-misc-full/coverageByXPath/blt.json",
            "cldr-misc-full/coverageByXPath/bm-Nkoo.json",
            "cldr-misc-full/coverageByXPath/bm.json",
            "cldr-misc-full/coverageByXPath/bn-IN.json",
            "cldr-misc-full/coverageByXPath/bn.json",
            "cldr-misc-full/coverageByXPath/bo-IN.json",
            "cldr-misc-full/coverageByXPath/bo.json",
            "cldr-misc-full/coverageByXPath/bqi.json",
            "cldr-misc-full/coverageByXPath/br.json",
            "cldr-misc-full/coverageByXPath/brh.json",
            "cldr-misc-full/coverageByXPath/brx.json",
            "cldr-misc-full/coverageByXPath/bs-Cyrl.json",
            "cldr-misc-full/coverageByXPath/bs-Latn.json",
            "cldr-misc-full/coverageByXPath/bs.json",
            "cldr-misc-full/coverageByXPath/bss.json",
            "cldr-misc-full/coverageByXPath/bua.json",
            "cldr-misc-full/coverageByXPath/byn.json",
            "cldr-misc-full/coverageByXPath/ca-AD.json",
            "cldr-misc-full/coverageByXPath/ca-ES-valencia.json",
            "cldr-misc-full/coverageByXPath/ca-FR.json",
            "cldr-misc-full/coverageByXPath/ca-IT.json",
            "cldr-misc-full/coverageByXPath/ca.json",
            "cldr-misc-full/coverageByXPath/cad.json",
            "cldr-misc-full/coverageByXPath/cch.json",
            "cldr-misc-full/coverageByXPath/ccp-IN.json",
            "cldr-misc-full/coverageByXPath/ccp.json",
            "cldr-misc-full/coverageByXPath/ce.json",
            "cldr-misc-full/coverageByXPath/ceb.json",
            "cldr-misc-full/coverageByXPath/cgg.json",
            "cldr-misc-full/coverageByXPath/cho.json",
            "cldr-misc-full/coverageByXPath/chr.json",
            "cldr-misc-full/coverageByXPath/cic.json",
            "cldr-misc-full/coverageByXPath/ckb-IR.json",
            "cldr-misc-full/coverageByXPath/ckb.json",
            "cldr-misc-full/coverageByXPath/co.json",
            "cldr-misc-full/coverageByXPath/cop.json",
            "cldr-misc-full/coverageByXPath/cs.json",
            "cldr-misc-full/coverageByXPath/csw.json",
            "cldr-misc-full/coverageByXPath/cu.json",
            "cldr-misc-full/coverageByXPath/cv.json",
            "cldr-misc-full/coverageByXPath/cy.json",
            "cldr-misc-full/coverageByXPath/da-GL.json",
            "cldr-misc-full/coverageByXPath/da.json",
            "cldr-misc-full/coverageByXPath/dav.json",
            "cldr-misc-full/coverageByXPath/de-AT.json",
            "cldr-misc-full/coverageByXPath/de-BE.json",
            "cldr-misc-full/coverageByXPath/de-CH.json",
            "cldr-misc-full/coverageByXPath/de-IT.json",
            "cldr-misc-full/coverageByXPath/de-LI.json",
            "cldr-misc-full/coverageByXPath/de-LU.json",
            "cldr-misc-full/coverageByXPath/de.json",
            "cldr-misc-full/coverageByXPath/dje.json",
            "cldr-misc-full/coverageByXPath/doi.json",
            "cldr-misc-full/coverageByXPath/dsb.json",
            "cldr-misc-full/coverageByXPath/dua.json",
            "cldr-misc-full/coverageByXPath/dv.json",
            "cldr-misc-full/coverageByXPath/dyo.json",
            "cldr-misc-full/coverageByXPath/dz.json",
            "cldr-misc-full/coverageByXPath/ebu.json",
            "cldr-misc-full/coverageByXPath/ee-TG.json",
            "cldr-misc-full/coverageByXPath/ee.json",
            "cldr-misc-full/coverageByXPath/el-CY.json",
            "cldr-misc-full/coverageByXPath/el.json",
            "cldr-misc-full/coverageByXPath/en-001.json",
            "cldr-misc-full/coverageByXPath/en-150.json",
            "cldr-misc-full/coverageByXPath/en-AE.json",
            "cldr-misc-full/coverageByXPath/en-AG.json",
            "cldr-misc-full/coverageByXPath/en-AI.json",
            "cldr-misc-full/coverageByXPath/en-AS.json",
            "cldr-misc-full/coverageByXPath/en-AT.json",
            "cldr-misc-full/coverageByXPath/en-AU.json",
            "cldr-misc-full/coverageByXPath/en-BB.json",
            "cldr-misc-full/coverageByXPath/en-BE.json",
            "cldr-misc-full/coverageByXPath/en-BI.json",
            "cldr-misc-full/coverageByXPath/en-BM.json",
            "cldr-misc-full/coverageByXPath/en-BS.json",
            "cldr-misc-full/coverageByXPath/en-BW.json",
            "cldr-misc-full/coverageByXPath/en-BZ.json",
            "cldr-misc-full/coverageByXPath/en-CA.json",
            "cldr-misc-full/coverageByXPath/en-CC.json",
            "cldr-misc-full/coverageByXPath/en-CH.json",
            "cldr-misc-full/coverageByXPath/en-CK.json",
            "cldr-misc-full/coverageByXPath/en-CM.json",
            "cldr-misc-full/coverageByXPath/en-CX.json",
            "cldr-misc-full/coverageByXPath/en-CY.json",
            "cldr-misc-full/coverageByXPath/en-CZ.json",
            "cldr-misc-full/coverageByXPath/en-DE.json",
            "cldr-misc-full/coverageByXPath/en-DG.json",
            "cldr-misc-full/coverageByXPath/en-DK.json",
            "cldr-misc-full/coverageByXPath/en-DM.json",
            "cldr-misc-full/coverageByXPath/en-Dsrt.json",
            "cldr-misc-full/coverageByXPath/en-EE.json",
            "cldr-misc-full/coverageByXPath/en-ER.json",
            "cldr-misc-full/coverageByXPath/en-ES.json",
            "cldr-misc-full/coverageByXPath/en-FI.json",
            "cldr-misc-full/coverageByXPath/en-FJ.json",
            "cldr-misc-full/coverageByXPath/en-FK.json",
            "cldr-misc-full/coverageByXPath/en-FM.json",
            "cldr-misc-full/coverageByXPath/en-FR.json",
            "cldr-misc-full/coverageByXPath/en-GB.json",
            "cldr-misc-full/coverageByXPath/en-GD.json",
            "cldr-misc-full/coverageByXPath/en-GE.json",
            "cldr-misc-full/coverageByXPath/en-GG.json",
            "cldr-misc-full/coverageByXPath/en-GH.json",
            "cldr-misc-full/coverageByXPath/en-GI.json",
            "cldr-misc-full/coverageByXPath/en-GM.json",
            "cldr-misc-full/coverageByXPath/en-GS.json",
            "cldr-misc-full/coverageByXPath/en-GU.json",
            "cldr-misc-full/coverageByXPath/en-GY.json",
            "cldr-misc-full/coverageByXPath/en-HK.json",
            "cldr-misc-full/coverageByXPath/en-HU.json",
            "cldr-misc-full/coverageByXPath/en-ID.json",
            "cldr-misc-full/coverageByXPath/en-IE.json",
            "cldr-misc-full/coverageByXPath/en-IL.json",
            "cldr-misc-full/coverageByXPath/en-IM.json",
            "cldr-misc-full/coverageByXPath/en-IN.json",
            "cldr-misc-full/coverageByXPath/en-IO.json",
            "cldr-misc-full/coverageByXPath/en-IT.json",
            "cldr-misc-full/coverageByXPath/en-JE.json",
            "cldr-misc-full/coverageByXPath/en-JM.json",
            "cldr-misc-full/coverageByXPath/en-JP.json",
            "cldr-misc-full/coverageByXPath/en-KE.json",
            "cldr-misc-full/coverageByXPath/en-KI.json",
            "cldr-misc-full/coverageByXPath/en-KN.json",
            "cldr-misc-full/coverageByXPath/en-KY.json",
            "cldr-misc-full/coverageByXPath/en-LC.json",
            "cldr-misc-full/coverageByXPath/en-LR.json",
            "cldr-misc-full/coverageByXPath/en-LS.json",
            "cldr-misc-full/coverageByXPath/en-LT.json",
            "cldr-misc-full/coverageByXPath/en-LV.json",
            "cldr-misc-full/coverageByXPath/en-MG.json",
            "cldr-misc-full/coverageByXPath/en-MH.json",
            "cldr-misc-full/coverageByXPath/en-MO.json",
            "cldr-misc-full/coverageByXPath/en-MP.json",
            "cldr-misc-full/coverageByXPath/en-MS.json",
            "cldr-misc-full/coverageByXPath/en-MT.json",
            "cldr-misc-full/coverageByXPath/en-MU.json",
            "cldr-misc-full/coverageByXPath/en-MV.json",
            "cldr-misc-full/coverageByXPath/en-MW.json",
            "cldr-misc-full/coverageByXPath/en-MY.json",
            "cldr-misc-full/coverageByXPath/en-NA.json",
            "cldr-misc-full/coverageByXPath/en-NF.json",
            "cldr-misc-full/coverageByXPath/en-NG.json",
            "cldr-misc-full/coverageByXPath/en-NL.json",
            "cldr-misc-full/coverageByXPath/en-NO.json",
            "cldr-misc-full/coverageByXPath/en-NR.json",
            "cldr-misc-full/coverageByXPath/en-NU.json",
            "cldr-misc-full/coverageByXPath/en-NZ.json",
            "cldr-misc-full/coverageByXPath/en-PG.json",
            "cldr-misc-full/coverageByXPath/en-PH.json",
            "cldr-misc-full/coverageByXPath/en-PK.json",
            "cldr-misc-full/coverageByXPath/en-PL.json",
            "cldr-misc-full/coverageByXPath/en-PN.json",
            "cldr-misc-full/coverageByXPath/en-PR.json",
            "cldr-misc-full/coverageByXPath/en-PT.json",
            "cldr-misc-full/coverageByXPath/en-PW.json",
            "cldr-misc-full/coverageByXPath/en-RO.json",
            "cldr-misc-full/coverageByXPath/en-RW.json",
            "cldr-misc-full/coverageByXPath/en-SB.json",
            "cldr-misc-full/coverageByXPath/en-SC.json",
            "cldr-misc-full/coverageByXPath/en-SD.json",
            "cldr-misc-full/coverageByXPath/en-SE.json",
            "cldr-misc-full/coverageByXPath/en-SG.json",
            "cldr-misc-full/coverageByXPath/en-SH.json",
            "cldr-misc-full/coverageByXPath/en-SI.json",
            "cldr-misc-full/coverageByXPath/en-SK.json",
            "cldr-misc-full/coverageByXPath/en-SL.json",
            "cldr-misc-full/coverageByXPath/en-SS.json",
            "cldr-misc-full/coverageByXPath/en-SX.json",
            "cldr-misc-full/coverageByXPath/en-SZ.json",
            "cldr-misc-full/coverageByXPath/en-Shaw.json",
            "cldr-misc-full/coverageByXPath/en-TC.json",
            "cldr-misc-full/coverageByXPath/en-TK.json",
            "cldr-misc-full/coverageByXPath/en-TO.json",
            "cldr-misc-full/coverageByXPath/en-TT.json",
            "cldr-misc-full/coverageByXPath/en-TV.json",
            "cldr-misc-full/coverageByXPath/en-TZ.json",
            "cldr-misc-full/coverageByXPath/en-UA.json",
            "cldr-misc-full/coverageByXPath/en-UG.json",
            "cldr-misc-full/coverageByXPath/en-UM.json",
            "cldr-misc-full/coverageByXPath/en-VC.json",
            "cldr-misc-full/coverageByXPath/en-VG.json",
            "cldr-misc-full/coverageByXPath/en-VI.json",
            "cldr-misc-full/coverageByXPath/en-VU.json",
            "cldr-misc-full/coverageByXPath/en-WS.json",
            "cldr-misc-full/coverageByXPath/en-ZA.json",
            "cldr-misc-full/coverageByXPath/en-ZM.json",
            "cldr-misc-full/coverageByXPath/en-ZW.json",
            "cldr-misc-full/coverageByXPath/en.json",
            "cldr-misc-full/coverageByXPath/eo.json",
            "cldr-misc-full/coverageByXPath/es-419.json",
            "cldr-misc-full/coverageByXPath/es-AR.json",
            "cldr-misc-full/coverageByXPath/es-BO.json",
            "cldr-misc-full/coverageByXPath/es-BR.json",
            "cldr-misc-full/coverageByXPath/es-BZ.json",
            "cldr-misc-full/coverageByXPath/es-CL.json",
            "cldr-misc-full/coverageByXPath/es-CO.json",
            "cldr-misc-full/coverageByXPath/es-CR.json",
            "cldr-misc-full/coverageByXPath/es-CU.json",
            "cldr-misc-full/coverageByXPath/es-DO.json",
            "cldr-misc-full/coverageByXPath/es-EA.json",
            "cldr-misc-full/coverageByXPath/es-EC.json",
            "cldr-misc-full/coverageByXPath/es-GQ.json",
            "cldr-misc-full/coverageByXPath/es-GT.json",
            "cldr-misc-full/coverageByXPath/es-HN.json",
            "cldr-misc-full/coverageByXPath/es-IC.json",
            "cldr-misc-full/coverageByXPath/es-MX.json",
            "cldr-misc-full/coverageByXPath/es-NI.json",
            "cldr-misc-full/coverageByXPath/es-PA.json",
            "cldr-misc-full/coverageByXPath/es-PE.json",
            "cldr-misc-full/coverageByXPath/es-PH.json",
            "cldr-misc-full/coverageByXPath/es-PR.json",
            "cldr-misc-full/coverageByXPath/es-PY.json",
            "cldr-misc-full/coverageByXPath/es-SV.json",
            "cldr-misc-full/coverageByXPath/es-US.json",
            "cldr-misc-full/coverageByXPath/es-UY.json",
            "cldr-misc-full/coverageByXPath/es-VE.json",
            "cldr-misc-full/coverageByXPath/es.json",
            "cldr-misc-full/coverageByXPath/et.json",
            "cldr-misc-full/coverageByXPath/eu.json",
            "cldr-misc-full/coverageByXPath/ewo.json",
            "cldr-misc-full/coverageByXPath/fa-AF.json",
            "cldr-misc-full/coverageByXPath/fa.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-BF.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-CM.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-GH.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-GM.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-GW.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-LR.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-MR.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-NE.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-NG.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-SL.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm-SN.json",
            "cldr-misc-full/coverageByXPath/ff-Adlm.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-BF.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-CM.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-GH.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-GM.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-GN.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-GW.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-LR.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-MR.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-NE.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-NG.json",
            "cldr-misc-full/coverageByXPath/ff-Latn-SL.json",
            "cldr-misc-full/coverageByXPath/ff-Latn.json",
            "cldr-misc-full/coverageByXPath/ff.json",
            "cldr-misc-full/coverageByXPath/fi.json",
            "cldr-misc-full/coverageByXPath/fil.json",
            "cldr-misc-full/coverageByXPath/fo-DK.json",
            "cldr-misc-full/coverageByXPath/fo.json",
            "cldr-misc-full/coverageByXPath/fr-BE.json",
            "cldr-misc-full/coverageByXPath/fr-BF.json",
            "cldr-misc-full/coverageByXPath/fr-BI.json",
            "cldr-misc-full/coverageByXPath/fr-BJ.json",
            "cldr-misc-full/coverageByXPath/fr-BL.json",
            "cldr-misc-full/coverageByXPath/fr-CA.json",
            "cldr-misc-full/coverageByXPath/fr-CD.json",
            "cldr-misc-full/coverageByXPath/fr-CF.json",
            "cldr-misc-full/coverageByXPath/fr-CG.json",
            "cldr-misc-full/coverageByXPath/fr-CH.json",
            "cldr-misc-full/coverageByXPath/fr-CI.json",
            "cldr-misc-full/coverageByXPath/fr-CM.json",
            "cldr-misc-full/coverageByXPath/fr-DJ.json",
            "cldr-misc-full/coverageByXPath/fr-DZ.json",
            "cldr-misc-full/coverageByXPath/fr-GA.json",
            "cldr-misc-full/coverageByXPath/fr-GF.json",
            "cldr-misc-full/coverageByXPath/fr-GN.json",
            "cldr-misc-full/coverageByXPath/fr-GP.json",
            "cldr-misc-full/coverageByXPath/fr-GQ.json",
            "cldr-misc-full/coverageByXPath/fr-HT.json",
            "cldr-misc-full/coverageByXPath/fr-KM.json",
            "cldr-misc-full/coverageByXPath/fr-LU.json",
            "cldr-misc-full/coverageByXPath/fr-MA.json",
            "cldr-misc-full/coverageByXPath/fr-MC.json",
            "cldr-misc-full/coverageByXPath/fr-MF.json",
            "cldr-misc-full/coverageByXPath/fr-MG.json",
            "cldr-misc-full/coverageByXPath/fr-ML.json",
            "cldr-misc-full/coverageByXPath/fr-MQ.json",
            "cldr-misc-full/coverageByXPath/fr-MR.json",
            "cldr-misc-full/coverageByXPath/fr-MU.json",
            "cldr-misc-full/coverageByXPath/fr-NC.json",
            "cldr-misc-full/coverageByXPath/fr-NE.json",
            "cldr-misc-full/coverageByXPath/fr-PF.json",
            "cldr-misc-full/coverageByXPath/fr-PM.json",
            "cldr-misc-full/coverageByXPath/fr-RE.json",
            "cldr-misc-full/coverageByXPath/fr-RW.json",
            "cldr-misc-full/coverageByXPath/fr-SC.json",
            "cldr-misc-full/coverageByXPath/fr-SN.json",
            "cldr-misc-full/coverageByXPath/fr-SY.json",
            "cldr-misc-full/coverageByXPath/fr-TD.json",
            "cldr-misc-full/coverageByXPath/fr-TG.json",
            "cldr-misc-full/coverageByXPath/fr-TN.json",
            "cldr-misc-full/coverageByXPath/fr-VU.json",
            "cldr-misc-full/coverageByXPath/fr-WF.json",
            "cldr-misc-full/coverageByXPath/fr-YT.json",
            "cldr-misc-full/coverageByXPath/fr.json",
            "cldr-misc-full/coverageByXPath/frr.json",
            "cldr-misc-full/coverageByXPath/fur.json",
            "cldr-misc-full/coverageByXPath/fy.json",
            "cldr-misc-full/coverageByXPath/ga-GB.json",
            "cldr-misc-full/coverageByXPath/ga.json",
            "cldr-misc-full/coverageByXPath/gaa.json",
            "cldr-misc-full/coverageByXPath/gd.json",
            "cldr-misc-full/coverageByXPath/gez-ER.json",
            "cldr-misc-full/coverageByXPath/gez.json",
            "cldr-misc-full/coverageByXPath/gl.json",
            "cldr-misc-full/coverageByXPath/gn.json",
            "cldr-misc-full/coverageByXPath/gsw-FR.json",
            "cldr-misc-full/coverageByXPath/gsw-LI.json",
            "cldr-misc-full/coverageByXPath/gsw.json",
            "cldr-misc-full/coverageByXPath/gu.json",
            "cldr-misc-full/coverageByXPath/guz.json",
            "cldr-misc-full/coverageByXPath/gv.json",
            "cldr-misc-full/coverageByXPath/ha-Arab-SD.json",
            "cldr-misc-full/coverageByXPath/ha-Arab.json",
            "cldr-misc-full/coverageByXPath/ha-GH.json",
            "cldr-misc-full/coverageByXPath/ha-NE.json",
            "cldr-misc-full/coverageByXPath/ha.json",
            "cldr-misc-full/coverageByXPath/haw.json",
            "cldr-misc-full/coverageByXPath/he.json",
            "cldr-misc-full/coverageByXPath/hi-Latn.json",
            "cldr-misc-full/coverageByXPath/hi.json",
            "cldr-misc-full/coverageByXPath/hnj-Hmnp.json",
            "cldr-misc-full/coverageByXPath/hnj.json",
            "cldr-misc-full/coverageByXPath/hr-BA.json",
            "cldr-misc-full/coverageByXPath/hr.json",
            "cldr-misc-full/coverageByXPath/hrx.json",
            "cldr-misc-full/coverageByXPath/hsb.json",
            "cldr-misc-full/coverageByXPath/ht.json",
            "cldr-misc-full/coverageByXPath/hu.json",
            "cldr-misc-full/coverageByXPath/hy.json",
            "cldr-misc-full/coverageByXPath/ia.json",
            "cldr-misc-full/coverageByXPath/id.json",
            "cldr-misc-full/coverageByXPath/ie.json",
            "cldr-misc-full/coverageByXPath/ig.json",
            "cldr-misc-full/coverageByXPath/ii.json",
            "cldr-misc-full/coverageByXPath/io.json",
            "cldr-misc-full/coverageByXPath/is.json",
            "cldr-misc-full/coverageByXPath/isv-Cyrl.json",
            "cldr-misc-full/coverageByXPath/isv-Latn.json",
            "cldr-misc-full/coverageByXPath/isv.json",
            "cldr-misc-full/coverageByXPath/it-CH.json",
            "cldr-misc-full/coverageByXPath/it-SM.json",
            "cldr-misc-full/coverageByXPath/it-VA.json",
            "cldr-misc-full/coverageByXPath/it.json",
            "cldr-misc-full/coverageByXPath/iu-Latn.json",
            "cldr-misc-full/coverageByXPath/iu.json",
            "cldr-misc-full/coverageByXPath/ja.json",
            "cldr-misc-full/coverageByXPath/jbo.json",
            "cldr-misc-full/coverageByXPath/jgo.json",
            "cldr-misc-full/coverageByXPath/jmc.json",
            "cldr-misc-full/coverageByXPath/jv.json",
            "cldr-misc-full/coverageByXPath/ka.json",
            "cldr-misc-full/coverageByXPath/kaa-Cyrl.json",
            "cldr-misc-full/coverageByXPath/kaa-Latn.json",
            "cldr-misc-full/coverageByXPath/kaa.json",
            "cldr-misc-full/coverageByXPath/kab.json",
            "cldr-misc-full/coverageByXPath/kaj.json",
            "cldr-misc-full/coverageByXPath/kam.json",
            "cldr-misc-full/coverageByXPath/kbd-TR.json",
            "cldr-misc-full/coverageByXPath/kbd.json",
            "cldr-misc-full/coverageByXPath/kcg.json",
            "cldr-misc-full/coverageByXPath/kde.json",
            "cldr-misc-full/coverageByXPath/kea.json",
            "cldr-misc-full/coverageByXPath/kek.json",
            "cldr-misc-full/coverageByXPath/ken.json",
            "cldr-misc-full/coverageByXPath/kgp.json",
            "cldr-misc-full/coverageByXPath/khq.json",
            "cldr-misc-full/coverageByXPath/ki.json",
            "cldr-misc-full/coverageByXPath/kk-Arab.json",
            "cldr-misc-full/coverageByXPath/kk-Cyrl.json",
            "cldr-misc-full/coverageByXPath/kk-KZ.json",
            "cldr-misc-full/coverageByXPath/kk.json",
            "cldr-misc-full/coverageByXPath/kkj.json",
            "cldr-misc-full/coverageByXPath/kl.json",
            "cldr-misc-full/coverageByXPath/kln.json",
            "cldr-misc-full/coverageByXPath/km.json",
            "cldr-misc-full/coverageByXPath/kn.json",
            "cldr-misc-full/coverageByXPath/ko-CN.json",
            "cldr-misc-full/coverageByXPath/ko-KP.json",
            "cldr-misc-full/coverageByXPath/ko.json",
            "cldr-misc-full/coverageByXPath/kok-Deva.json",
            "cldr-misc-full/coverageByXPath/kok-Latn.json",
            "cldr-misc-full/coverageByXPath/kok.json",
            "cldr-misc-full/coverageByXPath/kpe-GN.json",
            "cldr-misc-full/coverageByXPath/kpe.json",
            "cldr-misc-full/coverageByXPath/ks-Arab.json",
            "cldr-misc-full/coverageByXPath/ks-Deva.json",
            "cldr-misc-full/coverageByXPath/ks.json",
            "cldr-misc-full/coverageByXPath/ksb.json",
            "cldr-misc-full/coverageByXPath/ksf.json",
            "cldr-misc-full/coverageByXPath/ksh.json",
            "cldr-misc-full/coverageByXPath/ku-Arab-IR.json",
            "cldr-misc-full/coverageByXPath/ku-Arab.json",
            "cldr-misc-full/coverageByXPath/ku-Latn-IQ.json",
            "cldr-misc-full/coverageByXPath/ku-Latn-SY.json",
            "cldr-misc-full/coverageByXPath/ku-Latn.json",
            "cldr-misc-full/coverageByXPath/ku-TR.json",
            "cldr-misc-full/coverageByXPath/ku.json",
            "cldr-misc-full/coverageByXPath/kw.json",
            "cldr-misc-full/coverageByXPath/kxv-Deva.json",
            "cldr-misc-full/coverageByXPath/kxv-Latn.json",
            "cldr-misc-full/coverageByXPath/kxv-Orya.json",
            "cldr-misc-full/coverageByXPath/kxv-Telu.json",
            "cldr-misc-full/coverageByXPath/kxv.json",
            "cldr-misc-full/coverageByXPath/ky.json",
            "cldr-misc-full/coverageByXPath/la.json",
            "cldr-misc-full/coverageByXPath/lag.json",
            "cldr-misc-full/coverageByXPath/lb.json",
            "cldr-misc-full/coverageByXPath/lg.json",
            "cldr-misc-full/coverageByXPath/lij.json",
            "cldr-misc-full/coverageByXPath/lkt.json",
            "cldr-misc-full/coverageByXPath/lld.json",
            "cldr-misc-full/coverageByXPath/lmo.json",
            "cldr-misc-full/coverageByXPath/ln-AO.json",
            "cldr-misc-full/coverageByXPath/ln-CF.json",
            "cldr-misc-full/coverageByXPath/ln-CG.json",
            "cldr-misc-full/coverageByXPath/ln.json",
            "cldr-misc-full/coverageByXPath/lo.json",
            "cldr-misc-full/coverageByXPath/lrc-IQ.json",
            "cldr-misc-full/coverageByXPath/lrc.json",
            "cldr-misc-full/coverageByXPath/lt.json",
            "cldr-misc-full/coverageByXPath/ltg.json",
            "cldr-misc-full/coverageByXPath/lu.json",
            "cldr-misc-full/coverageByXPath/luo.json",
            "cldr-misc-full/coverageByXPath/luy.json",
            "cldr-misc-full/coverageByXPath/lv.json",
            "cldr-misc-full/coverageByXPath/lzz.json",
            "cldr-misc-full/coverageByXPath/mai.json",
            "cldr-misc-full/coverageByXPath/mas-TZ.json",
            "cldr-misc-full/coverageByXPath/mas.json",
            "cldr-misc-full/coverageByXPath/mdf.json",
            "cldr-misc-full/coverageByXPath/mer.json",
            "cldr-misc-full/coverageByXPath/mfe.json",
            "cldr-misc-full/coverageByXPath/mg.json",
            "cldr-misc-full/coverageByXPath/mgh.json",
            "cldr-misc-full/coverageByXPath/mgo.json",
            "cldr-misc-full/coverageByXPath/mhn.json",
            "cldr-misc-full/coverageByXPath/mi.json",
            "cldr-misc-full/coverageByXPath/mic.json",
            "cldr-misc-full/coverageByXPath/mk.json",
            "cldr-misc-full/coverageByXPath/ml.json",
            "cldr-misc-full/coverageByXPath/mn-Mong-MN.json",
            "cldr-misc-full/coverageByXPath/mn-Mong.json",
            "cldr-misc-full/coverageByXPath/mn.json",
            "cldr-misc-full/coverageByXPath/mni-Beng.json",
            "cldr-misc-full/coverageByXPath/mni-Mtei.json",
            "cldr-misc-full/coverageByXPath/mni.json",
            "cldr-misc-full/coverageByXPath/moh.json",
            "cldr-misc-full/coverageByXPath/mr.json",
            "cldr-misc-full/coverageByXPath/mrh-MM.json",
            "cldr-misc-full/coverageByXPath/mrh.json",
            "cldr-misc-full/coverageByXPath/ms-Arab-BN.json",
            "cldr-misc-full/coverageByXPath/ms-Arab.json",
            "cldr-misc-full/coverageByXPath/ms-BN.json",
            "cldr-misc-full/coverageByXPath/ms-ID.json",
            "cldr-misc-full/coverageByXPath/ms-SG.json",
            "cldr-misc-full/coverageByXPath/ms.json",
            "cldr-misc-full/coverageByXPath/mt.json",
            "cldr-misc-full/coverageByXPath/mua.json",
            "cldr-misc-full/coverageByXPath/mus.json",
            "cldr-misc-full/coverageByXPath/mww-Hmnp.json",
            "cldr-misc-full/coverageByXPath/mww.json",
            "cldr-misc-full/coverageByXPath/my.json",
            "cldr-misc-full/coverageByXPath/myv.json",
            "cldr-misc-full/coverageByXPath/mzn.json",
            "cldr-misc-full/coverageByXPath/naq.json",
            "cldr-misc-full/coverageByXPath/nb-SJ.json",
            "cldr-misc-full/coverageByXPath/nb.json",
            "cldr-misc-full/coverageByXPath/nd.json",
            "cldr-misc-full/coverageByXPath/nds-NL.json",
            "cldr-misc-full/coverageByXPath/nds.json",
            "cldr-misc-full/coverageByXPath/ne-IN.json",
            "cldr-misc-full/coverageByXPath/ne.json",
            "cldr-misc-full/coverageByXPath/nl-AW.json",
            "cldr-misc-full/coverageByXPath/nl-BE.json",
            "cldr-misc-full/coverageByXPath/nl-BQ.json",
            "cldr-misc-full/coverageByXPath/nl-CW.json",
            "cldr-misc-full/coverageByXPath/nl-SR.json",
            "cldr-misc-full/coverageByXPath/nl-SX.json",
            "cldr-misc-full/coverageByXPath/nl.json",
            "cldr-misc-full/coverageByXPath/nmg.json",
            "cldr-misc-full/coverageByXPath/nn.json",
            "cldr-misc-full/coverageByXPath/nnh.json",
            "cldr-misc-full/coverageByXPath/no.json",
            "cldr-misc-full/coverageByXPath/nqo.json",
            "cldr-misc-full/coverageByXPath/nr.json",
            "cldr-misc-full/coverageByXPath/nso.json",
            "cldr-misc-full/coverageByXPath/nus.json",
            "cldr-misc-full/coverageByXPath/nv.json",
            "cldr-misc-full/coverageByXPath/ny.json",
            "cldr-misc-full/coverageByXPath/nyn.json",
            "cldr-misc-full/coverageByXPath/oc-ES.json",
            "cldr-misc-full/coverageByXPath/oc.json",
            "cldr-misc-full/coverageByXPath/oka-US.json",
            "cldr-misc-full/coverageByXPath/oka.json",
            "cldr-misc-full/coverageByXPath/om-KE.json",
            "cldr-misc-full/coverageByXPath/om.json",
            "cldr-misc-full/coverageByXPath/or.json",
            "cldr-misc-full/coverageByXPath/os-RU.json",
            "cldr-misc-full/coverageByXPath/os.json",
            "cldr-misc-full/coverageByXPath/osa.json",
            "cldr-misc-full/coverageByXPath/pa-Arab.json",
            "cldr-misc-full/coverageByXPath/pa-Guru.json",
            "cldr-misc-full/coverageByXPath/pa.json",
            "cldr-misc-full/coverageByXPath/pap-AW.json",
            "cldr-misc-full/coverageByXPath/pap.json",
            "cldr-misc-full/coverageByXPath/pcm.json",
            "cldr-misc-full/coverageByXPath/pi-Latn.json",
            "cldr-misc-full/coverageByXPath/pi.json",
            "cldr-misc-full/coverageByXPath/pis.json",
            "cldr-misc-full/coverageByXPath/pl.json",
            "cldr-misc-full/coverageByXPath/pms.json",
            "cldr-misc-full/coverageByXPath/prg.json",
            "cldr-misc-full/coverageByXPath/ps-PK.json",
            "cldr-misc-full/coverageByXPath/ps.json",
            "cldr-misc-full/coverageByXPath/pt-AO.json",
            "cldr-misc-full/coverageByXPath/pt-CH.json",
            "cldr-misc-full/coverageByXPath/pt-CV.json",
            "cldr-misc-full/coverageByXPath/pt-GQ.json",
            "cldr-misc-full/coverageByXPath/pt-GW.json",
            "cldr-misc-full/coverageByXPath/pt-LU.json",
            "cldr-misc-full/coverageByXPath/pt-MO.json",
            "cldr-misc-full/coverageByXPath/pt-MZ.json",
            "cldr-misc-full/coverageByXPath/pt-PT.json",
            "cldr-misc-full/coverageByXPath/pt-ST.json",
            "cldr-misc-full/coverageByXPath/pt-TL.json",
            "cldr-misc-full/coverageByXPath/pt.json",
            "cldr-misc-full/coverageByXPath/qu-BO.json",
            "cldr-misc-full/coverageByXPath/qu-EC.json",
            "cldr-misc-full/coverageByXPath/qu.json",
            "cldr-misc-full/coverageByXPath/quc.json",
            "cldr-misc-full/coverageByXPath/raj.json",
            "cldr-misc-full/coverageByXPath/rhg-Rohg-BD.json",
            "cldr-misc-full/coverageByXPath/rhg-Rohg.json",
            "cldr-misc-full/coverageByXPath/rhg.json",
            "cldr-misc-full/coverageByXPath/rif.json",
            "cldr-misc-full/coverageByXPath/rm.json",
            "cldr-misc-full/coverageByXPath/rn.json",
            "cldr-misc-full/coverageByXPath/ro-MD.json",
            "cldr-misc-full/coverageByXPath/ro.json",
            "cldr-misc-full/coverageByXPath/rof.json",
            "cldr-misc-full/coverageByXPath/ru-BY.json",
            "cldr-misc-full/coverageByXPath/ru-KG.json",
            "cldr-misc-full/coverageByXPath/ru-KZ.json",
            "cldr-misc-full/coverageByXPath/ru-MD.json",
            "cldr-misc-full/coverageByXPath/ru-UA.json",
            "cldr-misc-full/coverageByXPath/ru.json",
            "cldr-misc-full/coverageByXPath/rw.json",
            "cldr-misc-full/coverageByXPath/rwk.json",
            "cldr-misc-full/coverageByXPath/sa.json",
            "cldr-misc-full/coverageByXPath/sah.json",
            "cldr-misc-full/coverageByXPath/saq.json",
            "cldr-misc-full/coverageByXPath/sat-Deva.json",
            "cldr-misc-full/coverageByXPath/sat-Olck.json",
            "cldr-misc-full/coverageByXPath/sat.json",
            "cldr-misc-full/coverageByXPath/sbp.json",
            "cldr-misc-full/coverageByXPath/sc.json",
            "cldr-misc-full/coverageByXPath/scn.json",
            "cldr-misc-full/coverageByXPath/sd-Arab.json",
            "cldr-misc-full/coverageByXPath/sd-Deva.json",
            "cldr-misc-full/coverageByXPath/sd.json",
            "cldr-misc-full/coverageByXPath/sdh-IQ.json",
            "cldr-misc-full/coverageByXPath/sdh.json",
            "cldr-misc-full/coverageByXPath/se-FI.json",
            "cldr-misc-full/coverageByXPath/se-SE.json",
            "cldr-misc-full/coverageByXPath/se.json",
            "cldr-misc-full/coverageByXPath/seh.json",
            "cldr-misc-full/coverageByXPath/ses.json",
            "cldr-misc-full/coverageByXPath/sg.json",
            "cldr-misc-full/coverageByXPath/sgs.json",
            "cldr-misc-full/coverageByXPath/shi-Latn.json",
            "cldr-misc-full/coverageByXPath/shi-Tfng.json",
            "cldr-misc-full/coverageByXPath/shi.json",
            "cldr-misc-full/coverageByXPath/shn-TH.json",
            "cldr-misc-full/coverageByXPath/shn.json",
            "cldr-misc-full/coverageByXPath/si.json",
            "cldr-misc-full/coverageByXPath/sid.json",
            "cldr-misc-full/coverageByXPath/sk.json",
            "cldr-misc-full/coverageByXPath/skr.json",
            "cldr-misc-full/coverageByXPath/sl.json",
            "cldr-misc-full/coverageByXPath/sma-NO.json",
            "cldr-misc-full/coverageByXPath/sma.json",
            "cldr-misc-full/coverageByXPath/smj-NO.json",
            "cldr-misc-full/coverageByXPath/smj.json",
            "cldr-misc-full/coverageByXPath/smn.json",
            "cldr-misc-full/coverageByXPath/sms.json",
            "cldr-misc-full/coverageByXPath/sn.json",
            "cldr-misc-full/coverageByXPath/so-DJ.json",
            "cldr-misc-full/coverageByXPath/so-ET.json",
            "cldr-misc-full/coverageByXPath/so-KE.json",
            "cldr-misc-full/coverageByXPath/so.json",
            "cldr-misc-full/coverageByXPath/sq-MK.json",
            "cldr-misc-full/coverageByXPath/sq-XK.json",
            "cldr-misc-full/coverageByXPath/sq.json",
            "cldr-misc-full/coverageByXPath/sr-Cyrl-BA.json",
            "cldr-misc-full/coverageByXPath/sr-Cyrl-ME.json",
            "cldr-misc-full/coverageByXPath/sr-Cyrl-XK.json",
            "cldr-misc-full/coverageByXPath/sr-Cyrl.json",
            "cldr-misc-full/coverageByXPath/sr-Latn-BA.json",
            "cldr-misc-full/coverageByXPath/sr-Latn-ME.json",
            "cldr-misc-full/coverageByXPath/sr-Latn-XK.json",
            "cldr-misc-full/coverageByXPath/sr-Latn.json",
            "cldr-misc-full/coverageByXPath/sr.json",
            "cldr-misc-full/coverageByXPath/ss-SZ.json",
            "cldr-misc-full/coverageByXPath/ss.json",
            "cldr-misc-full/coverageByXPath/ssy.json",
            "cldr-misc-full/coverageByXPath/st-LS.json",
            "cldr-misc-full/coverageByXPath/st.json",
            "cldr-misc-full/coverageByXPath/su-Latn.json",
            "cldr-misc-full/coverageByXPath/su.json",
            "cldr-misc-full/coverageByXPath/sus-SL.json",
            "cldr-misc-full/coverageByXPath/sus.json",
            "cldr-misc-full/coverageByXPath/suz-Deva.json",
            "cldr-misc-full/coverageByXPath/suz-Sunu.json",
            "cldr-misc-full/coverageByXPath/suz.json",
            "cldr-misc-full/coverageByXPath/sv-AX.json",
            "cldr-misc-full/coverageByXPath/sv-FI.json",
            "cldr-misc-full/coverageByXPath/sv.json",
            "cldr-misc-full/coverageByXPath/sw-CD.json",
            "cldr-misc-full/coverageByXPath/sw-KE.json",
            "cldr-misc-full/coverageByXPath/sw-UG.json",
            "cldr-misc-full/coverageByXPath/sw.json",
            "cldr-misc-full/coverageByXPath/syr-SY.json",
            "cldr-misc-full/coverageByXPath/syr.json",
            "cldr-misc-full/coverageByXPath/szl.json",
            "cldr-misc-full/coverageByXPath/ta-LK.json",
            "cldr-misc-full/coverageByXPath/ta-MY.json",
            "cldr-misc-full/coverageByXPath/ta-SG.json",
            "cldr-misc-full/coverageByXPath/ta.json",
            "cldr-misc-full/coverageByXPath/te.json",
            "cldr-misc-full/coverageByXPath/teo-KE.json",
            "cldr-misc-full/coverageByXPath/teo.json",
            "cldr-misc-full/coverageByXPath/tg.json",
            "cldr-misc-full/coverageByXPath/th.json",
            "cldr-misc-full/coverageByXPath/ti-ER.json",
            "cldr-misc-full/coverageByXPath/ti.json",
            "cldr-misc-full/coverageByXPath/tig.json",
            "cldr-misc-full/coverageByXPath/tk.json",
            "cldr-misc-full/coverageByXPath/tn-BW.json",
            "cldr-misc-full/coverageByXPath/tn.json",
            "cldr-misc-full/coverageByXPath/to.json",
            "cldr-misc-full/coverageByXPath/tok.json",
            "cldr-misc-full/coverageByXPath/tpi.json",
            "cldr-misc-full/coverageByXPath/tr-CY.json",
            "cldr-misc-full/coverageByXPath/tr.json",
            "cldr-misc-full/coverageByXPath/trv.json",
            "cldr-misc-full/coverageByXPath/trw.json",
            "cldr-misc-full/coverageByXPath/ts.json",
            "cldr-misc-full/coverageByXPath/tt.json",
            "cldr-misc-full/coverageByXPath/twq.json",
            "cldr-misc-full/coverageByXPath/tyv.json",
            "cldr-misc-full/coverageByXPath/tzm.json",
            "cldr-misc-full/coverageByXPath/ug.json",
            "cldr-misc-full/coverageByXPath/uk.json",
            "cldr-misc-full/coverageByXPath/und.json",
            "cldr-misc-full/coverageByXPath/ur-IN.json",
            "cldr-misc-full/coverageByXPath/ur.json",
            "cldr-misc-full/coverageByXPath/uz-Arab.json",
            "cldr-misc-full/coverageByXPath/uz-Cyrl.json",
            "cldr-misc-full/coverageByXPath/uz-Latn.json",
            "cldr-misc-full/coverageByXPath/uz.json",
            "cldr-misc-full/coverageByXPath/vai-Latn.json",
            "cldr-misc-full/coverageByXPath/vai-Vaii.json",
            "cldr-misc-full/coverageByXPath/vai.json",
            "cldr-misc-full/coverageByXPath/ve.json",
            "cldr-misc-full/coverageByXPath/vec.json",
            "cldr-misc-full/coverageByXPath/vi.json",
            "cldr-misc-full/coverageByXPath/vmw.json",
            "cldr-misc-full/coverageByXPath/vo.json",
            "cldr-misc-full/coverageByXPath/vun.json",
            "cldr-misc-full/coverageByXPath/wa.json",
            "cldr-misc-full/coverageByXPath/wae.json",
            "cldr-misc-full/coverageByXPath/wal.json",
            "cldr-misc-full/coverageByXPath/wbp.json",
            "cldr-misc-full/coverageByXPath/wo.json",
            "cldr-misc-full/coverageByXPath/xdq.json",
            "cldr-misc-full/coverageByXPath/xh.json",
            "cldr-misc-full/coverageByXPath/xnr.json",
            "cldr-misc-full/coverageByXPath/xog.json",
            "cldr-misc-full/coverageByXPath/yav.json",
            "cldr-misc-full/coverageByXPath/yi.json",
            "cldr-misc-full/coverageByXPath/yo-BJ.json",
            "cldr-misc-full/coverageByXPath/yo.json",
            "cldr-misc-full/coverageByXPath/yrl-CO.json",
            "cldr-misc-full/coverageByXPath/yrl-VE.json",
            "cldr-misc-full/coverageByXPath/yrl.json",
            "cldr-misc-full/coverageByXPath/yue-Hans.json",
            "cldr-misc-full/coverageByXPath/yue-Hant-CN.json",
            "cldr-misc-full/coverageByXPath/yue-Hant-MO.json",
            "cldr-misc-full/coverageByXPath/yue-Hant.json",
            "cldr-misc-full/coverageByXPath/yue.json",
            "cldr-misc-full/coverageByXPath/za.json",
            "cldr-misc-full/coverageByXPath/zgh.json",
            "cldr-misc-full/coverageByXPath/zh-Hans-HK.json",
            "cldr-misc-full/coverageByXPath/zh-Hans-MO.json",
            "cldr-misc-full/coverageByXPath/zh-Hans-MY.json",
            "cldr-misc-full/coverageByXPath/zh-Hans-SG.json",
            "cldr-misc-full/coverageByXPath/zh-Hans.json",
            "cldr-misc-full/coverageByXPath/zh-Hant-HK.json",
            "cldr-misc-full/coverageByXPath/zh-Hant-MO.json",
            "cldr-misc-full/coverageByXPath/zh-Hant-MY.json",
            "cldr-misc-full/coverageByXPath/zh-Hant.json",
            "cldr-misc-full/coverageByXPath/zh-Latn.json",
            "cldr-misc-full/coverageByXPath/zh.json",
            "cldr-misc-full/coverageByXPath/zu.json",
        ))
    })
}

impl CldrCache {
    pub(crate) fn new(root: AbstractFs) -> Self {
        CldrCache {
            serde_cache: SerdeCache::new(root),
            extended_locale_expander: Default::default(),
            calendar_eras: Default::default(),
            #[cfg(feature = "unstable")]
            transforms: Default::default(),
            tz_caches: Default::default(),
        }
    }

    pub(crate) fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-core")
    }

    pub(crate) fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-numbers-full/main")
    }

    pub(crate) fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-misc-full/main")
    }

    pub(crate) fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-bcp47/bcp47")
    }

    pub(crate) fn personnames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-person-names-full/main")
    }

    pub(crate) fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-localenames-full/main")
    }

    pub(crate) fn units(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-units-full/main")
    }

    pub(crate) fn segments(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-segments-full/segments")
    }

    pub(crate) fn dates(&self, cal: Option<DatagenCalendar>) -> CldrDirLang<'_> {
        CldrDirLang(
            self,
            match cal {
                Some(DatagenCalendar::Buddhist) => "cldr-cal-buddhist-full/main",
                Some(DatagenCalendar::Chinese) => "cldr-cal-chinese-full/main",
                Some(DatagenCalendar::Coptic) => "cldr-cal-coptic-full/main",
                Some(DatagenCalendar::Dangi) => "cldr-cal-dangi-full/main",
                Some(DatagenCalendar::Ethiopic) => "cldr-cal-ethiopic-full/main",
                Some(DatagenCalendar::Hebrew) => "cldr-cal-hebrew-full/main",
                Some(DatagenCalendar::Indian) => "cldr-cal-indian-full/main",
                Some(DatagenCalendar::Hijri) => "cldr-cal-islamic-full/main",
                Some(DatagenCalendar::Japanese) => "cldr-cal-japanese-full/main",
                Some(DatagenCalendar::Persian) => "cldr-cal-persian-full/main",
                Some(DatagenCalendar::Roc) => "cldr-cal-roc-full/main",
                Some(DatagenCalendar::Gregorian) | None => "cldr-dates-full/main",
            },
        )
    }

    pub(crate) fn locales(
        &self,
        levels: impl IntoIterator<Item = CoverageLevel>,
    ) -> Result<Vec<DataLocale>, DataError> {
        let levels = levels.into_iter().collect::<HashSet<_>>();
        let mut locales: Vec<DataLocale> = self
            .serde_cache
            .read_and_parse_json::<crate::cldr_serde::coverage_levels::Resource>(
                "cldr-core/coverageLevels.json",
            )?
            .coverage_levels
            .iter()
            .filter_map(|(locale, c)| levels.contains(c).then_some(locale))
            .cloned()
            .map(Into::into)
            // `und` needs to be part of every set
            .chain([Default::default()])
            .collect();
        locales.sort_by(|a, b| {
            let b = b.write_to_string();
            a.strict_cmp(b.as_bytes())
        });
        Ok(locales)
    }

    pub(crate) fn extended_locale_expander(&self) -> Result<&LocaleExpander, DataError> {
        use super::locale::likely_subtags::*;
        self.extended_locale_expander
            .get_or_init(|| {
                LocaleExpander::try_new_extended_unstable(
                    &LikelySubtagsResources::try_from_cldr_cache(self)?,
                )
                .map_err(|e| {
                    DataError::custom("creating LocaleExpander in CldrCache")
                        .with_display_context(&e)
                })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// CLDR sometimes stores locales with default scripts.
    /// Add in the likely script here to make that data reachable.
    fn add_script_extended(&self, locale: &DataLocale) -> Result<Option<DataLocale>, DataError> {
        if locale.language.is_unknown() || locale.script.is_some() {
            return Ok(None);
        }
        let mut new_langid =
            LanguageIdentifier::from((locale.language, locale.script, locale.region));
        self.extended_locale_expander()?.maximize(&mut new_langid);
        debug_assert!(
            new_langid.script.is_some(),
            "Script not found for: {new_langid:?}"
        );
        if locale.region.is_none() {
            new_langid.region = None;
        }
        Ok(Some(new_langid.into()))
    }

    /// ICU4X does not store locales with their script
    /// if the script is the default for the language.
    /// Perform that normalization mapping here.
    fn remove_script_extended(&self, locale: &DataLocale) -> Result<Option<DataLocale>, DataError> {
        if locale.language.is_unknown() || locale.script.is_none() {
            return Ok(None);
        }
        let mut langid = LanguageIdentifier::from((locale.language, locale.script, locale.region));
        self.extended_locale_expander()?.minimize(&mut langid);
        if langid.script.is_some() || (locale.region.is_none() && langid.region.is_some()) {
            // Wasn't able to minimize the script, or had to add a region
            return Ok(None);
        }
        // Restore the region
        langid.region = locale.region;
        Ok(Some(langid.into()))
    }

    /// Extracts the region from a [`DataLocale`].
    ///
    /// If the locale already has a region, it is returned.  
    /// Otherwise, the likely region is inferred from the language.
    ///
    /// # Example
    ///  - "en-US" -> "US"
    ///  - "en" -> "US"
    ///  - "ar" -> "EG"
    ///  - "und" -> "001"
    #[cfg(feature = "unstable")]
    pub(crate) fn extract_or_infer_region(&self, locale: &DataLocale) -> Result<Region, DataError> {
        if let Some(region) = locale.region {
            return Ok(region);
        }

        let mut lang_id = LanguageIdentifier::from((locale.language, locale.script, locale.region));
        let _ = self.extended_locale_expander()?.maximize(&mut lang_id);
        Ok(lang_id
            .region
            .unwrap_or(icu::locale::subtags::region!("001")))
    }

    /// Computes the script-based locale group for a given locale.
    ///
    /// This finds the most likely language for the locale's script, then minimizes it
    /// (keeping the script if it's not the default for that language).
    ///
    /// Example:
    /// - "en-US" -> "en-Latn-US" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "es-US" ->  "es-Latn-US" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "fr-FR" -> "fr-Latn-FR" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "ar-SA" -> "ar-Arab-SA" -> "und-Arab" -> "ar-Arab-EG" -> "ar"
    /// - "bm-Nkoo" -> "bm-Nkoo-ML" -> "und-Nkoo" -> "man-Nkoo-GN" -> "man-Nkoo"
    /// - "nqo" -> "nqo-Nkoo-GN" -> "und-Nkoo" -> "man-Nkoo-GN" -> "man-Nkoo"
    /// - "und-Latn" -> "en-Latn-US" -> "en"
    /// - "und-Arab" -> "ar-Arab-EG" -> "ar"
    /// - "und-US" -> "en-Latn-US" -> "en"
    /// - "und" -> "und"
    pub(crate) fn script_based_locale_group(
        &self,
        locale: &DataLocale,
    ) -> Result<DataLocale, DataError> {
        let mut group = LanguageIdentifier::from((locale.language, locale.script, locale.region));

        // 1. Maximizes the input locale to get full language/script/region
        //    (e.g. "es-US" -> "es-Latn-US")
        self.extended_locale_expander()?.maximize(&mut group);

        // 2. Strips language and region, keeping only script
        //    (e.g. "es-Latn-US" -> "und-Latn")
        group.language = Language::UNKNOWN;
        group.region = Default::default();

        // 3. Maximizes again to find the most likely language for that script
        //    (e.g. "und-Latn" -> "en-Latn-US")
        //    (e.g. "und-Nkoo" -> "man-Nkoo-GN")
        self.extended_locale_expander()?.maximize(&mut group);

        // 4. Minimizes the locale, keeping the script if it's not the default for the language
        //    (e.g. "en-Latn-US" -> "en")
        //    (e.g. "man-Nkoo-GN" -> "man-Nkoo")
        self.extended_locale_expander()?
            .minimize_favor_script(&mut group);
        Ok(group.into())
    }

    /// Determines the [`CoverageTier`] for a given `locale` and `xpath` target.
    ///
    /// Resolution lookup precedence:
    /// 1. Locale-specific override file `cldr-misc-full/coverageByXPath/{locale}.json`
    /// 2. Root defaults file `cldr-misc-full/coverageByXPath.json`
    /// 3. Defaults to [`CoverageTier::Extended`] if unlisted everywhere.
    pub(crate) fn coverage_tier(
        &self,
        locale: &DataLocale,
        xpath: impl Writeable,
    ) -> Result<CoverageLevelForXPath, DataError> {
        let dir = self.coverage_by_xpath();
        let locale_file = format!("{locale}.json");
        if dir.file_exists(&locale_file)? {
            let resource: &CoverageByXPathResource = dir.read_and_parse(&locale_file)?;
            let levels = resource.coverage_by_xpath.values().next();
            if let Some(level) = levels.and_then(|l| l.level_for_xpath(&xpath)) {
                return Ok(level);
            }
        } else {
            log::warn!("Coverage data file not found for locale {locale}");
        }

        let misc_dir = CldrDirNoLang(self, "cldr-misc-full");
        if misc_dir.file_exists("coverageByXPath.json")? {
            let resource: &CoverageByXPathResource =
                misc_dir.read_and_parse("coverageByXPath.json")?;
            if let Some(level) = resource
                .coverage_by_xpath
                .get("root")
                .and_then(|l| l.level_for_xpath(&xpath))
            {
                return Ok(level);
            }
        }

        // Not found: default to Comprehensive
        Ok(CoverageLevelForXPath::Comprehensive)
    }

    pub(crate) fn coverage_by_xpath(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-misc-full/coverageByXPath")
    }
}

pub(crate) struct CldrDirNoLang<'a>(&'a CldrCache, &'static str);

impl<'a> CldrDirNoLang<'a> {
    pub(crate) fn read_and_parse<S>(&self, file_name: &str) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.0
            .serde_cache
            .read_and_parse_json(&format!("{}/{}", self.1, file_name))
    }

    pub(crate) fn file_exists(&self, file_name: &str) -> Result<bool, DataError> {
        self.0
            .serde_cache
            .file_exists(&format!("{}/{}", self.1, file_name))
    }
}

pub(crate) struct CldrDirLang<'a>(&'a CldrCache, &'static str);

impl<'a> CldrDirLang<'a> {
    pub(crate) fn read_and_parse<S>(
        &self,
        locale: &DataLocale,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        let path = format!("{}/{locale}/{file_name}", self.1);
        if self.0.serde_cache.file_exists(&path)? {
            self.0.serde_cache.read_and_parse_json(&path)
        } else if let Some(new_locale) = self.0.add_script_extended(locale)? {
            self.read_and_parse(&new_locale, file_name)
        } else {
            Err(DataErrorKind::Io(std::io::ErrorKind::NotFound)
                .into_error()
                .with_display_context(&path))
        }
    }

    pub(crate) fn list_locales(&self) -> Result<impl Iterator<Item = DataLocale> + '_, DataError> {
        Ok(self
            .0
            .serde_cache
            .list(self.1)?
            .map(|path| -> Result<DataLocale, DataError> {
                let locale = DataLocale::from_str(&path).unwrap();
                Ok(self.0.remove_script_extended(&locale)?.unwrap_or(locale))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter())
    }

    pub(crate) fn file_exists(
        &self,
        lang: &DataLocale,
        file_name: &str,
    ) -> Result<bool, DataError> {
        let path = format!("{}/{lang}/{file_name}", self.1);
        if self.0.serde_cache.file_exists(&path)? {
            Ok(true)
        } else if let Some(new_locale) = self.0.add_script_extended(lang)? {
            self.file_exists(&new_locale, file_name)
        } else {
            Ok(false)
        }
    }
}

#[test]
fn test_script_based_locale_group() {
    use crate::SourceDataProvider;

    let provider = SourceDataProvider::new_testing();
    let cldr = provider.cldr().unwrap();

    // Test cases from the documentation
    // "en-US" -> "en"
    let en_us = DataLocale::from_str("en-US").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&en_us).unwrap().to_string(),
        "en"
    );

    // "es-US" -> "en" (Spanish uses Latin script, English is most common Latin-script language)
    let es_us = DataLocale::from_str("es-US").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&es_us).unwrap().to_string(),
        "en"
    );

    // "fr-FR" -> "en"
    let fr_fr = DataLocale::from_str("fr-FR").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&fr_fr).unwrap().to_string(),
        "en"
    );

    // "ar-SA" -> "ar" (Arabic uses Arabic script)
    let ar_sa = DataLocale::from_str("ar-SA").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&ar_sa).unwrap().to_string(),
        "ar"
    );

    // "nqo" -> "man-Nkoo" (N'Ko language uses N'Ko script, most likely language for N'Ko is Mandingo,
    // but N'Ko is not Mandingo's default script so it's kept)
    let nqo = DataLocale::from_str("nqo").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&nqo).unwrap().to_string(),
        "man-Nkoo"
    );

    // "bm-Nkoo" -> "man-Nkoo" (Bambara in N'Ko script -> Mandingo is most likely for N'Ko script,
    // but N'Ko is not Mandingo's default script so it's kept)
    let bm_nkoo = DataLocale::from_str("bm-Nkoo").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&bm_nkoo)
            .unwrap()
            .to_string(),
        "man-Nkoo"
    );

    // "man" -> "en" (Mandingo's default script is Latin, Latin's most likely language is English)
    let man = DataLocale::from_str("man").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&man).unwrap().to_string(),
        "en"
    );
}

#[test]
fn test_coverage_tier() {
    use crate::SourceDataProvider;
    let provider = SourceDataProvider::new_testing();
    let _cldr = provider.cldr().unwrap();
    let coverage_cldr = coverage_cldr_cache();

    let en = DataLocale::from_str("en").unwrap();
    // Minimal tier XPath (basic language display name in root defaults)
    let xpath_minimal = "//ldml/localeDisplayNames/languages/language[@type=\"en\"]";
    assert_eq!(
        coverage_cldr.coverage_tier(&en, xpath_minimal).unwrap(),
        CoverageLevelForXPath::Basic
    );

    // Default/unlisted XPath falls back to Comprehensive tier
    let xpath_unlisted =
        "//ldml/localeDisplayNames/languages/language[@type=\"unlisted_test_code\"]";
    assert_eq!(
        coverage_cldr.coverage_tier(&en, xpath_unlisted).unwrap(),
        CoverageLevelForXPath::Comprehensive
    );
}

#[test]
fn test_all_filesystem_locales_in_coverage_cldr_cache() {
    let data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let fs_cldr = CldrCache::new(AbstractFs::new(&data_dir).unwrap());
    let coverage_cldr = coverage_cldr_cache();

    let files = fs_cldr
        .serde_cache
        .list("cldr-misc-full/coverageByXPath")
        .unwrap();

    for file_name in files {
        let full_path = format!("cldr-misc-full/coverageByXPath/{file_name}");
        assert!(
            coverage_cldr.serde_cache.file_exists(&full_path).unwrap(),
            "Missing file in coverage_cldr_cache: {full_path}"
        );
    }
}
