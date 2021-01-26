// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::uniset::UnicodeSet;
use std::borrow::Cow;
use std::convert::TryInto;

//
// resource key structs - the structs used directly by users of data provider
//

pub mod key {
    use icu_provider::resource_key;
    use icu_provider::ResourceKey;

    pub const AHEX_V1: ResourceKey = resource_key!(uniset, "AHex", 1);
    pub const ALNUM_V1: ResourceKey = resource_key!(uniset, "alnum", 1);
    pub const ALPHA_V1: ResourceKey = resource_key!(uniset, "Alpha", 1);
    pub const BIDI_C_V1: ResourceKey = resource_key!(uniset, "Bidi_C", 1);
    pub const BIDI_M_V1: ResourceKey = resource_key!(uniset, "Bidi_M", 1);
    pub const BLANK_V1: ResourceKey = resource_key!(uniset, "blank", 1);
    pub const CASED_V1: ResourceKey = resource_key!(uniset, "Cased", 1);
    pub const CI_V1: ResourceKey = resource_key!(uniset, "CI", 1);
    pub const COMP_EX_V1: ResourceKey = resource_key!(uniset, "Comp_Ex", 1);
    pub const CWCF_V1: ResourceKey = resource_key!(uniset, "CWCF", 1);
    pub const CWCM_V1: ResourceKey = resource_key!(uniset, "CWCM", 1);
    pub const CWKCF_V1: ResourceKey = resource_key!(uniset, "CWKCF", 1);
    pub const CWL_V1: ResourceKey = resource_key!(uniset, "CWL", 1);
    pub const CWT_V1: ResourceKey = resource_key!(uniset, "CWT", 1);
    pub const CWU_V1: ResourceKey = resource_key!(uniset, "CWU", 1);
    pub const DASH_V1: ResourceKey = resource_key!(uniset, "Dash", 1);
    pub const DEP_V1: ResourceKey = resource_key!(uniset, "Dep", 1);
    pub const DI_V1: ResourceKey = resource_key!(uniset, "DI", 1);
    pub const DIA_V1: ResourceKey = resource_key!(uniset, "Dia", 1);
    pub const EBASE_V1: ResourceKey = resource_key!(uniset, "EBase", 1);
    pub const ECOMP_V1: ResourceKey = resource_key!(uniset, "EComp", 1);
    pub const EMOD_V1: ResourceKey = resource_key!(uniset, "EMod", 1);
    pub const EMOJI_V1: ResourceKey = resource_key!(uniset, "Emoji", 1);
    pub const EPRES_V1: ResourceKey = resource_key!(uniset, "EPres", 1);
    pub const EXT_V1: ResourceKey = resource_key!(uniset, "Ext", 1);
    pub const EXT_PICT_V1: ResourceKey = resource_key!(uniset, "ExtPict", 1);
    pub const GRAPH_V1: ResourceKey = resource_key!(uniset, "graph", 1);
    pub const GR_BASE_V1: ResourceKey = resource_key!(uniset, "Gr_Base", 1);
    pub const GR_EXT_V1: ResourceKey = resource_key!(uniset, "Gr_Ext", 1);
    pub const GR_LINK_V1: ResourceKey = resource_key!(uniset, "Gr_Link", 1);
    pub const HEX_V1: ResourceKey = resource_key!(uniset, "Hex", 1);
    pub const HYPHEN_V1: ResourceKey = resource_key!(uniset, "Hyphen", 1);
    pub const IDC_V1: ResourceKey = resource_key!(uniset, "IDC", 1);
    pub const IDEO_V1: ResourceKey = resource_key!(uniset, "Ideo", 1);
    pub const IDS_V1: ResourceKey = resource_key!(uniset, "IDS", 1);
    pub const IDSB_V1: ResourceKey = resource_key!(uniset, "IDSB", 1);
    pub const IDST_V1: ResourceKey = resource_key!(uniset, "IDST", 1);
    pub const JOIN_C_V1: ResourceKey = resource_key!(uniset, "Join_C", 1);
    pub const LOE_V1: ResourceKey = resource_key!(uniset, "LOE", 1);
    pub const LOWER_V1: ResourceKey = resource_key!(uniset, "Lower", 1);
    pub const MATH_V1: ResourceKey = resource_key!(uniset, "Math", 1);
    pub const NCHAR_V1: ResourceKey = resource_key!(uniset, "NChar", 1);
    pub const NFCINERT_V1: ResourceKey = resource_key!(uniset, "nfcinert", 1);
    pub const NFDINERT_V1: ResourceKey = resource_key!(uniset, "nfdinert", 1);
    pub const NFKCINERT_V1: ResourceKey = resource_key!(uniset, "nfkcinert", 1);
    pub const NFKDINERT_V1: ResourceKey = resource_key!(uniset, "nfkdinert", 1);
    pub const PAT_SYN_V1: ResourceKey = resource_key!(uniset, "Pat_Syn", 1);
    pub const PAT_WS_V1: ResourceKey = resource_key!(uniset, "Pat_WS", 1);
    pub const PCM_V1: ResourceKey = resource_key!(uniset, "PCM", 1);
    pub const PRINT_V1: ResourceKey = resource_key!(uniset, "print", 1);
    pub const QMARK_V1: ResourceKey = resource_key!(uniset, "QMark", 1);
    pub const RADICAL_V1: ResourceKey = resource_key!(uniset, "Radical", 1);
    pub const RI_V1: ResourceKey = resource_key!(uniset, "RI", 1);
    pub const SD_V1: ResourceKey = resource_key!(uniset, "SD", 1);
    pub const SEGSTART_V1: ResourceKey = resource_key!(uniset, "segstart", 1);
    pub const SENSITIVE_V1: ResourceKey = resource_key!(uniset, "Sensitive", 1);
    pub const STERM_V1: ResourceKey = resource_key!(uniset, "STerm", 1);
    pub const TERM_V1: ResourceKey = resource_key!(uniset, "Term", 1);
    pub const UIDEO_V1: ResourceKey = resource_key!(uniset, "UIdeo", 1);
    pub const UPPER_V1: ResourceKey = resource_key!(uniset, "Upper", 1);
    pub const VS_V1: ResourceKey = resource_key!(uniset, "VS", 1);
    pub const WSPACE_V1: ResourceKey = resource_key!(uniset, "WSpace", 1);
    pub const XDIGIT_V1: ResourceKey = resource_key!(uniset, "xdigit", 1);
    pub const XIDC_V1: ResourceKey = resource_key!(uniset, "XIDC", 1);
    pub const XIDS_V1: ResourceKey = resource_key!(uniset, "XIDS", 1);
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct UnicodeProperty<'s> {
    pub name: Cow<'s, str>,
    pub inv_list: Vec<u32>,
}

impl Default for UnicodeProperty<'static> {
    /// Default empty nameless property
    fn default() -> UnicodeProperty<'static> {
        UnicodeProperty {
            name: Cow::Borrowed(""),
            inv_list: vec![],
        }
    }
}

impl<'s> UnicodeProperty<'s> {
    pub fn from_uniset(set: &UnicodeSet, name: Cow<'s, str>) -> UnicodeProperty<'s> {
        let inv_list = set.get_inversion_list();
        UnicodeProperty { name, inv_list }
    }
}

impl<'s> TryInto<UnicodeSet> for UnicodeProperty<'s> {
    type Error = crate::UnicodeSetError;
    fn try_into(self) -> Result<UnicodeSet, Self::Error> {
        UnicodeSet::from_inversion_list(self.inv_list)
    }
}
