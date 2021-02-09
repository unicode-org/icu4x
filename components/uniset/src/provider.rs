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

    pub const ASCII_HEX_DIGIT_V1: ResourceKey = resource_key!(uniset, "AHex", 1);
    pub const ALNUM_V1: ResourceKey = resource_key!(uniset, "alnum", 1);
    pub const ALPHABETIC_V1: ResourceKey = resource_key!(uniset, "Alpha", 1);
    pub const BIDI_CONTROL_V1: ResourceKey = resource_key!(uniset, "Bidi_C", 1);
    pub const BIDI_MIRRORED_V1: ResourceKey = resource_key!(uniset, "Bidi_M", 1);
    pub const BLANK_V1: ResourceKey = resource_key!(uniset, "blank", 1);
    pub const CASED_V1: ResourceKey = resource_key!(uniset, "Cased", 1);
    pub const CASE_IGNORABLE_V1: ResourceKey = resource_key!(uniset, "CI", 1);
    pub const FULL_COMPOSITION_EXCLUSION_V1: ResourceKey = resource_key!(uniset, "Comp_Ex", 1);
    pub const CHANGES_WHEN_CASEFOLDED_V1: ResourceKey = resource_key!(uniset, "CWCF", 1);
    pub const CHANGES_WHEN_CASEMAPPED_V1: ResourceKey = resource_key!(uniset, "CWCM", 1);
    pub const CHANGES_WHEN_NFKC_CASEFOLDED_V1: ResourceKey = resource_key!(uniset, "CWKCF", 1);
    pub const CHANGES_WHEN_LOWERCASED_V1: ResourceKey = resource_key!(uniset, "CWL", 1);
    pub const CHANGES_WHEN_TITLECASED_V1: ResourceKey = resource_key!(uniset, "CWT", 1);
    pub const CHANGES_WHEN_UPPERCASED_V1: ResourceKey = resource_key!(uniset, "CWU", 1);
    pub const DASH_V1: ResourceKey = resource_key!(uniset, "Dash", 1);
    pub const DEPRECATED_V1: ResourceKey = resource_key!(uniset, "Dep", 1);
    pub const DEFAULT_IGNORABLE_CODE_POINT_V1: ResourceKey = resource_key!(uniset, "DI", 1);
    pub const DIACRITIC_V1: ResourceKey = resource_key!(uniset, "Dia", 1);
    pub const EMOJI_MODIFIER_BASE_V1: ResourceKey = resource_key!(uniset, "EBase", 1);
    pub const EMOJI_COMPONENT_V1: ResourceKey = resource_key!(uniset, "EComp", 1);
    pub const EMOJI_MODIFIER_V1: ResourceKey = resource_key!(uniset, "EMod", 1);
    pub const EMOJI_V1: ResourceKey = resource_key!(uniset, "Emoji", 1);
    pub const EMOJI_PRESENTATION_V1: ResourceKey = resource_key!(uniset, "EPres", 1);
    pub const EXTENDER_V1: ResourceKey = resource_key!(uniset, "Ext", 1);
    pub const EXTENDED_PICTOGRAPHIC_V1: ResourceKey = resource_key!(uniset, "ExtPict", 1);
    pub const GRAPH_V1: ResourceKey = resource_key!(uniset, "graph", 1);
    pub const GRAPHEME_BASE_V1: ResourceKey = resource_key!(uniset, "Gr_Base", 1);
    pub const GRAPHEME_EXTEND_V1: ResourceKey = resource_key!(uniset, "Gr_Ext", 1);
    pub const GRAPHEME_LINK_V1: ResourceKey = resource_key!(uniset, "Gr_Link", 1);
    pub const HEX_DIGIT_V1: ResourceKey = resource_key!(uniset, "Hex", 1);
    pub const HYPHEN_V1: ResourceKey = resource_key!(uniset, "Hyphen", 1);
    pub const ID_CONTINUE_V1: ResourceKey = resource_key!(uniset, "IDC", 1);
    pub const IDEOGRAPHIC_V1: ResourceKey = resource_key!(uniset, "Ideo", 1);
    pub const ID_START_V1: ResourceKey = resource_key!(uniset, "IDS", 1);
    pub const IDS_BINARY_OPERATOR_V1: ResourceKey = resource_key!(uniset, "IDSB", 1);
    pub const IDS_TRINARY_OPERATOR_V1: ResourceKey = resource_key!(uniset, "IDST", 1);
    pub const JOIN_CONTROL_V1: ResourceKey = resource_key!(uniset, "Join_C", 1);
    pub const LOGICAL_ORDER_EXCEPTION_V1: ResourceKey = resource_key!(uniset, "LOE", 1);
    pub const LOWERCASE_V1: ResourceKey = resource_key!(uniset, "Lower", 1);
    pub const MATH_V1: ResourceKey = resource_key!(uniset, "Math", 1);
    pub const NONCHARACTER_CODE_POINT_V1: ResourceKey = resource_key!(uniset, "NChar", 1);
    pub const NFC_INERT_V1: ResourceKey = resource_key!(uniset, "nfcinert", 1);
    pub const NFD_INERT_V1: ResourceKey = resource_key!(uniset, "nfdinert", 1);
    pub const NFKC_INERT_V1: ResourceKey = resource_key!(uniset, "nfkcinert", 1);
    pub const NFKD_INERT_V1: ResourceKey = resource_key!(uniset, "nfkdinert", 1);
    pub const PATTERN_SYNTAX_V1: ResourceKey = resource_key!(uniset, "Pat_Syn", 1);
    pub const PATTERN_WHITE_SPACE_V1: ResourceKey = resource_key!(uniset, "Pat_WS", 1);
    pub const PREPENDED_CONCATENATION_MARK_V1: ResourceKey = resource_key!(uniset, "PCM", 1);
    pub const PRINT_V1: ResourceKey = resource_key!(uniset, "print", 1);
    pub const QUOTATION_MARK_V1: ResourceKey = resource_key!(uniset, "QMark", 1);
    pub const RADICAL_V1: ResourceKey = resource_key!(uniset, "Radical", 1);
    pub const REGIONAL_INDICATOR_V1: ResourceKey = resource_key!(uniset, "RI", 1);
    pub const SOFT_DOTTED_V1: ResourceKey = resource_key!(uniset, "SD", 1);
    pub const SEGMENT_STARTER_V1: ResourceKey = resource_key!(uniset, "segstart", 1);
    pub const CASE_SENSITIVE_V1: ResourceKey = resource_key!(uniset, "Sensitive", 1);
    pub const SENTENCE_TERMINAL_V1: ResourceKey = resource_key!(uniset, "STerm", 1);
    pub const TERMINAL_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "Term", 1);
    pub const UNIFIED_IDEOGRAPH_V1: ResourceKey = resource_key!(uniset, "UIdeo", 1);
    pub const UPPERCASE_V1: ResourceKey = resource_key!(uniset, "Upper", 1);
    pub const VARIATION_SELECTOR_V1: ResourceKey = resource_key!(uniset, "VS", 1);
    pub const WHITE_SPACE_V1: ResourceKey = resource_key!(uniset, "WSpace", 1);
    pub const XDIGIT_V1: ResourceKey = resource_key!(uniset, "xdigit", 1);
    pub const XID_CONTINUE_V1: ResourceKey = resource_key!(uniset, "XIDC", 1);
    pub const XID_START_V1: ResourceKey = resource_key!(uniset, "XIDS", 1);
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
