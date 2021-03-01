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

    //
    // Binary properties
    //

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

    //
    // Enumerated properties
    //

    // Note: The ResourceKey subcategory strings are determined from the Rust enum
    // integer representations of the Unicode enumerated property name and the
    // Unicode enumerated property value.
    pub const BIDI_CLASS_ARABIC_LETTER_V1: ResourceKey = resource_key!(uniset, "0=0", 1);
    pub const BIDI_CLASS_ARABIC_NUMBER_V1: ResourceKey = resource_key!(uniset, "0=1", 1);
    pub const BIDI_CLASS_PARAGRAPH_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "0=2", 1);
    pub const BIDI_CLASS_BOUNDARY_NEUTRAL_V1: ResourceKey = resource_key!(uniset, "0=3", 1);
    pub const BIDI_CLASS_COMMON_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "0=4", 1);
    pub const BIDI_CLASS_EUROPEAN_NUMBER_V1: ResourceKey = resource_key!(uniset, "0=5", 1);
    pub const BIDI_CLASS_EUROPEAN_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "0=6", 1);
    pub const BIDI_CLASS_EUROPEAN_TERMINATOR_V1: ResourceKey = resource_key!(uniset, "0=7", 1);
    pub const BIDI_CLASS_FIRST_STRONG_ISOLATE_V1: ResourceKey = resource_key!(uniset, "0=8", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_V1: ResourceKey = resource_key!(uniset, "0=9", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_EMBEDDING_V1: ResourceKey = resource_key!(uniset, "0=10", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_ISOLATE_V1: ResourceKey = resource_key!(uniset, "0=11", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_OVERRIDE_V1: ResourceKey = resource_key!(uniset, "0=12", 1);
    pub const BIDI_CLASS_NONSPACING_MARK_V1: ResourceKey = resource_key!(uniset, "0=13", 1);
    pub const BIDI_CLASS_OTHER_NEUTRAL_V1: ResourceKey = resource_key!(uniset, "0=14", 1);
    pub const BIDI_CLASS_POP_DIRECTIONAL_FORMAT_V1: ResourceKey = resource_key!(uniset, "0=15", 1);
    pub const BIDI_CLASS_POP_DIRECTIONAL_ISOLATE_V1: ResourceKey = resource_key!(uniset, "0=16", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_V1: ResourceKey = resource_key!(uniset, "0=17", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_EMBEDDING_V1: ResourceKey = resource_key!(uniset, "0=18", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_ISOLATE_V1: ResourceKey = resource_key!(uniset, "0=19", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_OVERRIDE_V1: ResourceKey = resource_key!(uniset, "0=20", 1);
    pub const BIDI_CLASS_SEGMENT_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "0=21", 1);
    pub const BIDI_CLASS_WHITE_SPACE_V1: ResourceKey = resource_key!(uniset, "0=22", 1);
    pub const BIDI_PAIRED_BRACKET_TYPE_CLOSE_V1: ResourceKey = resource_key!(uniset, "1=0", 1);
    pub const BIDI_PAIRED_BRACKET_TYPE_NONE_V1: ResourceKey = resource_key!(uniset, "1=1", 1);
    pub const BIDI_PAIRED_BRACKET_TYPE_OPEN_V1: ResourceKey = resource_key!(uniset, "1=2", 1);
    pub const CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1: ResourceKey =
        resource_key!(uniset, "2=0", 1);
    pub const CANONICAL_COMBINING_CLASS_OVERLAY_V1: ResourceKey = resource_key!(uniset, "2=1", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC10_V1: ResourceKey = resource_key!(uniset, "2=10", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC103_V1: ResourceKey = resource_key!(uniset, "2=103", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC107_V1: ResourceKey = resource_key!(uniset, "2=107", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC11_V1: ResourceKey = resource_key!(uniset, "2=11", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC118_V1: ResourceKey = resource_key!(uniset, "2=118", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC12_V1: ResourceKey = resource_key!(uniset, "2=12", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC122_V1: ResourceKey = resource_key!(uniset, "2=122", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC129_V1: ResourceKey = resource_key!(uniset, "2=129", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC13_V1: ResourceKey = resource_key!(uniset, "2=13", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC130_V1: ResourceKey = resource_key!(uniset, "2=130", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC132_V1: ResourceKey = resource_key!(uniset, "2=132", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC133_V1: ResourceKey = resource_key!(uniset, "2=133", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC14_V1: ResourceKey = resource_key!(uniset, "2=14", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC15_V1: ResourceKey = resource_key!(uniset, "2=15", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC16_V1: ResourceKey = resource_key!(uniset, "2=16", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC17_V1: ResourceKey = resource_key!(uniset, "2=17", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC18_V1: ResourceKey = resource_key!(uniset, "2=18", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC19_V1: ResourceKey = resource_key!(uniset, "2=19", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC20_V1: ResourceKey = resource_key!(uniset, "2=20", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "2=200", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1: ResourceKey =
        resource_key!(uniset, "2=202", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC21_V1: ResourceKey = resource_key!(uniset, "2=21", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "2=214", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "2=216", 1);
    pub const CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "2=218", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC22_V1: ResourceKey = resource_key!(uniset, "2=22", 1);
    pub const CANONICAL_COMBINING_CLASS_BELOW_V1: ResourceKey = resource_key!(uniset, "2=220", 1);
    pub const CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "2=222", 1);
    pub const CANONICAL_COMBINING_CLASS_LEFT_V1: ResourceKey = resource_key!(uniset, "2=224", 1);
    pub const CANONICAL_COMBINING_CLASS_RIGHT_V1: ResourceKey = resource_key!(uniset, "2=226", 1);
    pub const CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1: ResourceKey =
        resource_key!(uniset, "2=228", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC23_V1: ResourceKey = resource_key!(uniset, "2=23", 1);
    pub const CANONICAL_COMBINING_CLASS_ABOVE_V1: ResourceKey = resource_key!(uniset, "2=230", 1);
    pub const CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "2=232", 1);
    pub const CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1: ResourceKey =
        resource_key!(uniset, "2=233", 1);
    pub const CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "2=234", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC24_V1: ResourceKey = resource_key!(uniset, "2=324", 1);
    pub const CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1: ResourceKey =
        resource_key!(uniset, "2=240", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC25_V1: ResourceKey = resource_key!(uniset, "2=25", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC26_V1: ResourceKey = resource_key!(uniset, "2=26", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC27_V1: ResourceKey = resource_key!(uniset, "2=27", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC28_V1: ResourceKey = resource_key!(uniset, "2=28", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC29_V1: ResourceKey = resource_key!(uniset, "2=29", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC30_V1: ResourceKey = resource_key!(uniset, "2=30", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC31_V1: ResourceKey = resource_key!(uniset, "2=31", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC32_V1: ResourceKey = resource_key!(uniset, "2=32", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC33_V1: ResourceKey = resource_key!(uniset, "2=33", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC34_V1: ResourceKey = resource_key!(uniset, "2=34", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC35_V1: ResourceKey = resource_key!(uniset, "2=35", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC36_V1: ResourceKey = resource_key!(uniset, "2=36", 1);
    pub const CANONICAL_COMBINING_CLASS_HAN_READING_V1: ResourceKey =
        resource_key!(uniset, "2=6", 1);
    pub const CANONICAL_COMBINING_CLASS_NUKTA_V1: ResourceKey = resource_key!(uniset, "2=7", 1);
    pub const CANONICAL_COMBINING_CLASS_KANA_VOICING_V1: ResourceKey =
        resource_key!(uniset, "2=8", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC84_V1: ResourceKey = resource_key!(uniset, "2=84", 1);
    pub const CANONICAL_COMBINING_CLASS_VIRAMA_V1: ResourceKey = resource_key!(uniset, "2=9", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC91_V1: ResourceKey = resource_key!(uniset, "2=91", 1);
    pub const DECOMPOSITION_TYPE_CAN_V1: ResourceKey = resource_key!(uniset, "3=0", 1);
    pub const DECOMPOSITION_TYPE_COM_V1: ResourceKey = resource_key!(uniset, "3=1", 1);
    pub const DECOMPOSITION_TYPE_ENC_V1: ResourceKey = resource_key!(uniset, "3=2", 1);
    pub const DECOMPOSITION_TYPE_FIN_V1: ResourceKey = resource_key!(uniset, "3=3", 1);
    pub const DECOMPOSITION_TYPE_FONT_V1: ResourceKey = resource_key!(uniset, "3=4", 1);
    pub const DECOMPOSITION_TYPE_FRA_V1: ResourceKey = resource_key!(uniset, "3=5", 1);
    pub const DECOMPOSITION_TYPE_INIT_V1: ResourceKey = resource_key!(uniset, "3=6", 1);
    pub const DECOMPOSITION_TYPE_ISO_V1: ResourceKey = resource_key!(uniset, "3=7", 1);
    pub const DECOMPOSITION_TYPE_MED_V1: ResourceKey = resource_key!(uniset, "3=8", 1);
    pub const DECOMPOSITION_TYPE_NAR_V1: ResourceKey = resource_key!(uniset, "3=9", 1);
    pub const DECOMPOSITION_TYPE_NB_V1: ResourceKey = resource_key!(uniset, "3=10", 1);
    pub const DECOMPOSITION_TYPE_NONE_V1: ResourceKey = resource_key!(uniset, "3=11", 1);
    pub const DECOMPOSITION_TYPE_SML_V1: ResourceKey = resource_key!(uniset, "3=12", 1);
    pub const DECOMPOSITION_TYPE_SQR_V1: ResourceKey = resource_key!(uniset, "3=13", 1);
    pub const DECOMPOSITION_TYPE_SUB_V1: ResourceKey = resource_key!(uniset, "3=14", 1);
    pub const DECOMPOSITION_TYPE_SUP_V1: ResourceKey = resource_key!(uniset, "3=15", 1);
    pub const DECOMPOSITION_TYPE_VERT_V1: ResourceKey = resource_key!(uniset, "3=16", 1);
    pub const DECOMPOSITION_TYPE_WIDE_V1: ResourceKey = resource_key!(uniset, "3=17", 1);
    pub const EAST_ASIAN_WIDTH_AMBIGUOUS_V1: ResourceKey = resource_key!(uniset, "4=0", 1);
    pub const EAST_ASIAN_WIDTH_FULLWIDTH_V1: ResourceKey = resource_key!(uniset, "4=1", 1);
    pub const EAST_ASIAN_WIDTH_HALFWIDTH_V1: ResourceKey = resource_key!(uniset, "4=2", 1);
    pub const EAST_ASIAN_WIDTH_NEUTRAL_V1: ResourceKey = resource_key!(uniset, "4=3", 1);
    pub const EAST_ASIAN_WIDTH_NARROW_V1: ResourceKey = resource_key!(uniset, "4=4", 1);
    pub const EAST_ASIAN_WIDTH_WIDE_V1: ResourceKey = resource_key!(uniset, "4=5", 1);
    pub const GENERAL_CATEGORY_OTHER_V1: ResourceKey = resource_key!(uniset, "5=0", 1);
    pub const GENERAL_CATEGORY_CNTRL_V1: ResourceKey = resource_key!(uniset, "5=1", 1);
    pub const GENERAL_CATEGORY_FORMAT_V1: ResourceKey = resource_key!(uniset, "5=2", 1);
    pub const GENERAL_CATEGORY_UNASSIGNED_V1: ResourceKey = resource_key!(uniset, "5=3", 1);
    pub const GENERAL_CATEGORY_PRIVATE_USE_V1: ResourceKey = resource_key!(uniset, "5=4", 1);
    pub const GENERAL_CATEGORY_SURROGATE_V1: ResourceKey = resource_key!(uniset, "5=5", 1);
    pub const GENERAL_CATEGORY_LETTER_V1: ResourceKey = resource_key!(uniset, "5=6", 1);
    pub const GENERAL_CATEGORY_CASED_LETTER_V1: ResourceKey = resource_key!(uniset, "5=7", 1);
    pub const GENERAL_CATEGORY_LOWERCASE_LETTER_V1: ResourceKey = resource_key!(uniset, "5=8", 1);
    pub const GENERAL_CATEGORY_MODIFIER_LETTER_V1: ResourceKey = resource_key!(uniset, "5=9", 1);
    pub const GENERAL_CATEGORY_OTHER_LETTER_V1: ResourceKey = resource_key!(uniset, "5=10", 1);
    pub const GENERAL_CATEGORY_TITLECASE_LETTER_V1: ResourceKey = resource_key!(uniset, "5=11", 1);
    pub const GENERAL_CATEGORY_UPPERCASE_LETTER_V1: ResourceKey = resource_key!(uniset, "5=12", 1);
    pub const GENERAL_CATEGORY_COMBINING_MARK_V1: ResourceKey = resource_key!(uniset, "5=13", 1);
    pub const GENERAL_CATEGORY_SPACING_MARK_V1: ResourceKey = resource_key!(uniset, "5=14", 1);
    pub const GENERAL_CATEGORY_ENCLOSING_MARK_V1: ResourceKey = resource_key!(uniset, "5=15", 1);
    pub const GENERAL_CATEGORY_NONSPACING_MARK_V1: ResourceKey = resource_key!(uniset, "5=16", 1);
    pub const GENERAL_CATEGORY_NUMBER_V1: ResourceKey = resource_key!(uniset, "5=17", 1);
    pub const GENERAL_CATEGORY_DIGIT_V1: ResourceKey = resource_key!(uniset, "5=18", 1);
    pub const GENERAL_CATEGORY_LETTER_NUMBER_V1: ResourceKey = resource_key!(uniset, "5=19", 1);
    pub const GENERAL_CATEGORY_OTHER_NUMBER_V1: ResourceKey = resource_key!(uniset, "5=20", 1);
    pub const GENERAL_CATEGORY_PUNCT_V1: ResourceKey = resource_key!(uniset, "5=21", 1);
    pub const GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1: ResourceKey =
        resource_key!(uniset, "5=22", 1);
    pub const GENERAL_CATEGORY_DASH_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "5=23", 1);
    pub const GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "5=24", 1);
    pub const GENERAL_CATEGORY_FINAL_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "5=25", 1);
    pub const GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1: ResourceKey =
        resource_key!(uniset, "5=26", 1);
    pub const GENERAL_CATEGORY_OTHER_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "5=27", 1);
    pub const GENERAL_CATEGORY_OPEN_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "5=28", 1);
    pub const GENERAL_CATEGORY_SYMBOL_V1: ResourceKey = resource_key!(uniset, "5=29", 1);
    pub const GENERAL_CATEGORY_CURRENCY_SYMBOL_V1: ResourceKey = resource_key!(uniset, "5=30", 1);
    pub const GENERAL_CATEGORY_MODIFIER_SYMBOL_V1: ResourceKey = resource_key!(uniset, "5=31", 1);
    pub const GENERAL_CATEGORY_MATH_SYMBOL_V1: ResourceKey = resource_key!(uniset, "5=32", 1);
    pub const GENERAL_CATEGORY_OTHER_SYMBOL_V1: ResourceKey = resource_key!(uniset, "5=33", 1);
    pub const GENERAL_CATEGORY_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "5=34", 1);
    pub const GENERAL_CATEGORY_LINE_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "5=35", 1);
    pub const GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1: ResourceKey =
        resource_key!(uniset, "5=36", 1);
    pub const GENERAL_CATEGORY_SPACE_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "5=37", 1);
    pub const GRAPHEME_CLUSTER_BREAK_CONTROL_V1: ResourceKey = resource_key!(uniset, "6=0", 1);
    pub const GRAPHEME_CLUSTER_BREAK_CR_V1: ResourceKey = resource_key!(uniset, "6=1", 1);
    pub const GRAPHEME_CLUSTER_BREAK_E_BASE_V1: ResourceKey = resource_key!(uniset, "6=2", 1);
    pub const GRAPHEME_CLUSTER_BREAK_E_BASE_GAZ_V1: ResourceKey = resource_key!(uniset, "6=3", 1);
    pub const GRAPHEME_CLUSTER_BREAK_E_MODIFIER_V1: ResourceKey = resource_key!(uniset, "6=4", 1);
    pub const GRAPHEME_CLUSTER_BREAK_EXTEND_V1: ResourceKey = resource_key!(uniset, "6=5", 1);
    pub const GRAPHEME_CLUSTER_BREAK_GLUE_AFTER_ZWJ_V1: ResourceKey =
        resource_key!(uniset, "6=6", 1);
    pub const GRAPHEME_CLUSTER_BREAK_L_V1: ResourceKey = resource_key!(uniset, "6=7", 1);
    pub const GRAPHEME_CLUSTER_BREAK_LF_V1: ResourceKey = resource_key!(uniset, "6=8", 1);
    pub const GRAPHEME_CLUSTER_BREAK_LV_V1: ResourceKey = resource_key!(uniset, "6=9", 1);
    pub const GRAPHEME_CLUSTER_BREAK_LVT_V1: ResourceKey = resource_key!(uniset, "6=10", 1);
    pub const GRAPHEME_CLUSTER_BREAK_PREPEND_V1: ResourceKey = resource_key!(uniset, "6=11", 1);
    pub const GRAPHEME_CLUSTER_BREAK_REGIONAL_INDICATOR_V1: ResourceKey =
        resource_key!(uniset, "6=12", 1);
    pub const GRAPHEME_CLUSTER_BREAK_SPACINGMARK_V1: ResourceKey = resource_key!(uniset, "6=13", 1);
    pub const GRAPHEME_CLUSTER_BREAK_T_V1: ResourceKey = resource_key!(uniset, "6=14", 1);
    pub const GRAPHEME_CLUSTER_BREAK_V_V1: ResourceKey = resource_key!(uniset, "6=15", 1);
    pub const GRAPHEME_CLUSTER_BREAK_OTHER_V1: ResourceKey = resource_key!(uniset, "6=16", 1);
    pub const GRAPHEME_CLUSTER_BREAK_ZWJ_V1: ResourceKey = resource_key!(uniset, "6=17", 1);
    pub const HANGUL_SYLLABLE_TYPE_LEADING_JAMO_V1: ResourceKey = resource_key!(uniset, "7=0", 1);
    pub const HANGUL_SYLLABLE_TYPE_LV_SYLLABLE_V1: ResourceKey = resource_key!(uniset, "7=1", 1);
    pub const HANGUL_SYLLABLE_TYPE_LVT_SYLLABLE_V1: ResourceKey = resource_key!(uniset, "7=2", 1);
    pub const HANGUL_SYLLABLE_TYPE_NOT_APPLICABLE_V1: ResourceKey = resource_key!(uniset, "7=3", 1);
    pub const HANGUL_SYLLABLE_TYPE_TRAILING_JAMO_V1: ResourceKey = resource_key!(uniset, "7=4", 1);
    pub const HANGUL_SYLLABLE_TYPE_VOWEL_JAMO_V1: ResourceKey = resource_key!(uniset, "7=5", 1);
    pub const INDIC_POSITIONAL_CATEGORY_BOTTOM_V1: ResourceKey = resource_key!(uniset, "8=0", 1);
    pub const INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_LEFT_V1: ResourceKey =
        resource_key!(uniset, "8=1", 1);
    pub const INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "8=2", 1);
    pub const INDIC_POSITIONAL_CATEGORY_LEFT_V1: ResourceKey = resource_key!(uniset, "8=3", 1);
    pub const INDIC_POSITIONAL_CATEGORY_LEFT_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "8=4", 1);
    pub const INDIC_POSITIONAL_CATEGORY_NA_V1: ResourceKey = resource_key!(uniset, "8=5", 1);
    pub const INDIC_POSITIONAL_CATEGORY_OVERSTRUCK_V1: ResourceKey =
        resource_key!(uniset, "8=6", 1);
    pub const INDIC_POSITIONAL_CATEGORY_RIGHT_V1: ResourceKey = resource_key!(uniset, "8=7", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_V1: ResourceKey = resource_key!(uniset, "8=8", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_V1: ResourceKey =
        resource_key!(uniset, "8=9", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_LEFT_V1: ResourceKey =
        resource_key!(uniset, "8=10", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "8=11", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_V1: ResourceKey =
        resource_key!(uniset, "8=12", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "8=13", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "8=14", 1);
    pub const INDIC_POSITIONAL_CATEGORY_VISUAL_ORDER_LEFT_V1: ResourceKey =
        resource_key!(uniset, "8=15", 1);
    pub const INDIC_SYLLABIC_CATEGORY_AVAGRAHA_V1: ResourceKey = resource_key!(uniset, "9=0", 1);
    pub const INDIC_SYLLABIC_CATEGORY_BINDU_V1: ResourceKey = resource_key!(uniset, "9=1", 1);
    pub const INDIC_SYLLABIC_CATEGORY_BRAHMI_JOINING_NUMBER_V1: ResourceKey =
        resource_key!(uniset, "9=2", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CANTILLATION_MARK_V1: ResourceKey =
        resource_key!(uniset, "9=3", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_V1: ResourceKey = resource_key!(uniset, "9=4", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_DEAD_V1: ResourceKey =
        resource_key!(uniset, "9=5", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_FINAL_V1: ResourceKey =
        resource_key!(uniset, "9=6", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_HEAD_LETTER_V1: ResourceKey =
        resource_key!(uniset, "9=7", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_INITIAL_POSTFIXED_V1: ResourceKey =
        resource_key!(uniset, "9=8", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_KILLER_V1: ResourceKey =
        resource_key!(uniset, "9=9", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_MEDIAL_V1: ResourceKey =
        resource_key!(uniset, "9=10", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_PLACEHOLDER_V1: ResourceKey =
        resource_key!(uniset, "9=11", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_PRECEDING_REPHA_V1: ResourceKey =
        resource_key!(uniset, "9=12", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_PREFIXED_V1: ResourceKey =
        resource_key!(uniset, "9=13", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_SUBJOINED_V1: ResourceKey =
        resource_key!(uniset, "9=14", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_SUCCEEDING_REPHA_V1: ResourceKey =
        resource_key!(uniset, "9=15", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_WITH_STACKER_V1: ResourceKey =
        resource_key!(uniset, "9=16", 1);
    pub const INDIC_SYLLABIC_CATEGORY_GEMINATION_MARK_V1: ResourceKey =
        resource_key!(uniset, "9=17", 1);
    pub const INDIC_SYLLABIC_CATEGORY_INVISIBLE_STACKER_V1: ResourceKey =
        resource_key!(uniset, "9=18", 1);
    pub const INDIC_SYLLABIC_CATEGORY_JOINER_V1: ResourceKey = resource_key!(uniset, "9=19", 1);
    pub const INDIC_SYLLABIC_CATEGORY_MODIFYING_LETTER_V1: ResourceKey =
        resource_key!(uniset, "9=20", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NON_JOINER_V1: ResourceKey = resource_key!(uniset, "9=21", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NUKTA_V1: ResourceKey = resource_key!(uniset, "9=22", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NUMBER_V1: ResourceKey = resource_key!(uniset, "9=23", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NUMBER_JOINER_V1: ResourceKey =
        resource_key!(uniset, "9=24", 1);
    pub const INDIC_SYLLABIC_CATEGORY_OTHER_V1: ResourceKey = resource_key!(uniset, "9=25", 1);
    pub const INDIC_SYLLABIC_CATEGORY_PURE_KILLER_V1: ResourceKey =
        resource_key!(uniset, "9=26", 1);
    pub const INDIC_SYLLABIC_CATEGORY_REGISTER_SHIFTER_V1: ResourceKey =
        resource_key!(uniset, "9=27", 1);
    pub const INDIC_SYLLABIC_CATEGORY_SYLLABLE_MODIFIER_V1: ResourceKey =
        resource_key!(uniset, "9=28", 1);
    pub const INDIC_SYLLABIC_CATEGORY_TONE_LETTER_V1: ResourceKey =
        resource_key!(uniset, "9=29", 1);
    pub const INDIC_SYLLABIC_CATEGORY_TONE_MARK_V1: ResourceKey = resource_key!(uniset, "9=30", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VIRAMA_V1: ResourceKey = resource_key!(uniset, "9=31", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VISARGA_V1: ResourceKey = resource_key!(uniset, "9=32", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VOWEL_V1: ResourceKey = resource_key!(uniset, "9=33", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VOWEL_DEPENDENT_V1: ResourceKey =
        resource_key!(uniset, "9=34", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VOWEL_INDEPENDENT_V1: ResourceKey =
        resource_key!(uniset, "9=35", 1);
    pub const JOINING_GROUP_AFRICAN_FEH_V1: ResourceKey = resource_key!(uniset, "10=0", 1);
    pub const JOINING_GROUP_AFRICAN_NOON_V1: ResourceKey = resource_key!(uniset, "10=1", 1);
    pub const JOINING_GROUP_AFRICAN_QAF_V1: ResourceKey = resource_key!(uniset, "10=2", 1);
    pub const JOINING_GROUP_AIN_V1: ResourceKey = resource_key!(uniset, "10=3", 1);
    pub const JOINING_GROUP_ALAPH_V1: ResourceKey = resource_key!(uniset, "10=4", 1);
    pub const JOINING_GROUP_ALEF_V1: ResourceKey = resource_key!(uniset, "10=5", 1);
    pub const JOINING_GROUP_BEH_V1: ResourceKey = resource_key!(uniset, "10=6", 1);
    pub const JOINING_GROUP_BETH_V1: ResourceKey = resource_key!(uniset, "10=7", 1);
    pub const JOINING_GROUP_BURUSHASKI_YEH_BARREE_V1: ResourceKey =
        resource_key!(uniset, "10=8", 1);
    pub const JOINING_GROUP_DAL_V1: ResourceKey = resource_key!(uniset, "10=9", 1);
    pub const JOINING_GROUP_DALATH_RISH_V1: ResourceKey = resource_key!(uniset, "10=10", 1);
    pub const JOINING_GROUP_E_V1: ResourceKey = resource_key!(uniset, "10=11", 1);
    pub const JOINING_GROUP_FARSI_YEH_V1: ResourceKey = resource_key!(uniset, "10=12", 1);
    pub const JOINING_GROUP_FE_V1: ResourceKey = resource_key!(uniset, "10=13", 1);
    pub const JOINING_GROUP_FEH_V1: ResourceKey = resource_key!(uniset, "10=14", 1);
    pub const JOINING_GROUP_FINAL_SEMKATH_V1: ResourceKey = resource_key!(uniset, "10=15", 1);
    pub const JOINING_GROUP_GAF_V1: ResourceKey = resource_key!(uniset, "10=16", 1);
    pub const JOINING_GROUP_GAMAL_V1: ResourceKey = resource_key!(uniset, "10=17", 1);
    pub const JOINING_GROUP_HAH_V1: ResourceKey = resource_key!(uniset, "10=18", 1);
    pub const JOINING_GROUP_HANIFI_ROHINGYA_KINNA_YA_V1: ResourceKey =
        resource_key!(uniset, "10=19", 1);
    pub const JOINING_GROUP_HANIFI_ROHINGYA_PA_V1: ResourceKey = resource_key!(uniset, "10=20", 1);
    pub const JOINING_GROUP_HE_V1: ResourceKey = resource_key!(uniset, "10=21", 1);
    pub const JOINING_GROUP_HEH_V1: ResourceKey = resource_key!(uniset, "10=22", 1);
    pub const JOINING_GROUP_HEH_GOAL_V1: ResourceKey = resource_key!(uniset, "10=23", 1);
    pub const JOINING_GROUP_HETH_V1: ResourceKey = resource_key!(uniset, "10=24", 1);
    pub const JOINING_GROUP_KAF_V1: ResourceKey = resource_key!(uniset, "10=25", 1);
    pub const JOINING_GROUP_KAPH_V1: ResourceKey = resource_key!(uniset, "10=26", 1);
    pub const JOINING_GROUP_KHAPH_V1: ResourceKey = resource_key!(uniset, "10=27", 1);
    pub const JOINING_GROUP_KNOTTED_HEH_V1: ResourceKey = resource_key!(uniset, "10=28", 1);
    pub const JOINING_GROUP_LAM_V1: ResourceKey = resource_key!(uniset, "10=29", 1);
    pub const JOINING_GROUP_LAMADH_V1: ResourceKey = resource_key!(uniset, "10=30", 1);
    pub const JOINING_GROUP_MALAYALAM_BHA_V1: ResourceKey = resource_key!(uniset, "10=31", 1);
    pub const JOINING_GROUP_MALAYALAM_JA_V1: ResourceKey = resource_key!(uniset, "10=32", 1);
    pub const JOINING_GROUP_MALAYALAM_LLA_V1: ResourceKey = resource_key!(uniset, "10=33", 1);
    pub const JOINING_GROUP_MALAYALAM_LLLA_V1: ResourceKey = resource_key!(uniset, "10=34", 1);
    pub const JOINING_GROUP_MALAYALAM_NGA_V1: ResourceKey = resource_key!(uniset, "10=35", 1);
    pub const JOINING_GROUP_MALAYALAM_NNA_V1: ResourceKey = resource_key!(uniset, "10=36", 1);
    pub const JOINING_GROUP_MALAYALAM_NNNA_V1: ResourceKey = resource_key!(uniset, "10=37", 1);
    pub const JOINING_GROUP_MALAYALAM_NYA_V1: ResourceKey = resource_key!(uniset, "10=38", 1);
    pub const JOINING_GROUP_MALAYALAM_RA_V1: ResourceKey = resource_key!(uniset, "10=39", 1);
    pub const JOINING_GROUP_MALAYALAM_SSA_V1: ResourceKey = resource_key!(uniset, "10=40", 1);
    pub const JOINING_GROUP_MALAYALAM_TTA_V1: ResourceKey = resource_key!(uniset, "10=41", 1);
    pub const JOINING_GROUP_MANICHAEAN_ALEPH_V1: ResourceKey = resource_key!(uniset, "10=42", 1);
    pub const JOINING_GROUP_MANICHAEAN_AYIN_V1: ResourceKey = resource_key!(uniset, "10=43", 1);
    pub const JOINING_GROUP_MANICHAEAN_BETH_V1: ResourceKey = resource_key!(uniset, "10=44", 1);
    pub const JOINING_GROUP_MANICHAEAN_DALETH_V1: ResourceKey = resource_key!(uniset, "10=45", 1);
    pub const JOINING_GROUP_MANICHAEAN_DHAMEDH_V1: ResourceKey = resource_key!(uniset, "10=46", 1);
    pub const JOINING_GROUP_MANICHAEAN_FIVE_V1: ResourceKey = resource_key!(uniset, "10=47", 1);
    pub const JOINING_GROUP_MANICHAEAN_GIMEL_V1: ResourceKey = resource_key!(uniset, "10=48", 1);
    pub const JOINING_GROUP_MANICHAEAN_HETH_V1: ResourceKey = resource_key!(uniset, "10=49", 1);
    pub const JOINING_GROUP_MANICHAEAN_HUNDRED_V1: ResourceKey = resource_key!(uniset, "10=50", 1);
    pub const JOINING_GROUP_MANICHAEAN_KAPH_V1: ResourceKey = resource_key!(uniset, "10=51", 1);
    pub const JOINING_GROUP_MANICHAEAN_LAMEDH_V1: ResourceKey = resource_key!(uniset, "10=52", 1);
    pub const JOINING_GROUP_MANICHAEAN_MEM_V1: ResourceKey = resource_key!(uniset, "10=53", 1);
    pub const JOINING_GROUP_MANICHAEAN_NUN_V1: ResourceKey = resource_key!(uniset, "10=54", 1);
    pub const JOINING_GROUP_MANICHAEAN_ONE_V1: ResourceKey = resource_key!(uniset, "10=55", 1);
    pub const JOINING_GROUP_MANICHAEAN_PE_V1: ResourceKey = resource_key!(uniset, "10=56", 1);
    pub const JOINING_GROUP_MANICHAEAN_QOPH_V1: ResourceKey = resource_key!(uniset, "10=57", 1);
    pub const JOINING_GROUP_MANICHAEAN_RESH_V1: ResourceKey = resource_key!(uniset, "10=58", 1);
    pub const JOINING_GROUP_MANICHAEAN_SADHE_V1: ResourceKey = resource_key!(uniset, "10=59", 1);
    pub const JOINING_GROUP_MANICHAEAN_SAMEKH_V1: ResourceKey = resource_key!(uniset, "10=60", 1);
    pub const JOINING_GROUP_MANICHAEAN_TAW_V1: ResourceKey = resource_key!(uniset, "10=61", 1);
    pub const JOINING_GROUP_MANICHAEAN_TEN_V1: ResourceKey = resource_key!(uniset, "10=62", 1);
    pub const JOINING_GROUP_MANICHAEAN_TETH_V1: ResourceKey = resource_key!(uniset, "10=63", 1);
    pub const JOINING_GROUP_MANICHAEAN_THAMEDH_V1: ResourceKey = resource_key!(uniset, "10=64", 1);
    pub const JOINING_GROUP_MANICHAEAN_TWENTY_V1: ResourceKey = resource_key!(uniset, "10=65", 1);
    pub const JOINING_GROUP_MANICHAEAN_WAW_V1: ResourceKey = resource_key!(uniset, "10=66", 1);
    pub const JOINING_GROUP_MANICHAEAN_YODH_V1: ResourceKey = resource_key!(uniset, "10=67", 1);
    pub const JOINING_GROUP_MANICHAEAN_ZAYIN_V1: ResourceKey = resource_key!(uniset, "10=68", 1);
    pub const JOINING_GROUP_MEEM_V1: ResourceKey = resource_key!(uniset, "10=69", 1);
    pub const JOINING_GROUP_MIM_V1: ResourceKey = resource_key!(uniset, "10=70", 1);
    pub const JOINING_GROUP_NO_JOINING_GROUP_V1: ResourceKey = resource_key!(uniset, "10=71", 1);
    pub const JOINING_GROUP_NOON_V1: ResourceKey = resource_key!(uniset, "10=72", 1);
    pub const JOINING_GROUP_NUN_V1: ResourceKey = resource_key!(uniset, "10=73", 1);
    pub const JOINING_GROUP_NYA_V1: ResourceKey = resource_key!(uniset, "10=74", 1);
    pub const JOINING_GROUP_PE_V1: ResourceKey = resource_key!(uniset, "10=75", 1);
    pub const JOINING_GROUP_QAF_V1: ResourceKey = resource_key!(uniset, "10=76", 1);
    pub const JOINING_GROUP_QAPH_V1: ResourceKey = resource_key!(uniset, "10=77", 1);
    pub const JOINING_GROUP_REH_V1: ResourceKey = resource_key!(uniset, "10=78", 1);
    pub const JOINING_GROUP_REVERSED_PE_V1: ResourceKey = resource_key!(uniset, "10=79", 1);
    pub const JOINING_GROUP_ROHINGYA_YEH_V1: ResourceKey = resource_key!(uniset, "10=80", 1);
    pub const JOINING_GROUP_SAD_V1: ResourceKey = resource_key!(uniset, "10=81", 1);
    pub const JOINING_GROUP_SADHE_V1: ResourceKey = resource_key!(uniset, "10=82", 1);
    pub const JOINING_GROUP_SEEN_V1: ResourceKey = resource_key!(uniset, "10=83", 1);
    pub const JOINING_GROUP_SEMKATH_V1: ResourceKey = resource_key!(uniset, "10=84", 1);
    pub const JOINING_GROUP_SHIN_V1: ResourceKey = resource_key!(uniset, "10=85", 1);
    pub const JOINING_GROUP_STRAIGHT_WAW_V1: ResourceKey = resource_key!(uniset, "10=86", 1);
    pub const JOINING_GROUP_SWASH_KAF_V1: ResourceKey = resource_key!(uniset, "10=87", 1);
    pub const JOINING_GROUP_SYRIAC_WAW_V1: ResourceKey = resource_key!(uniset, "10=88", 1);
    pub const JOINING_GROUP_TAH_V1: ResourceKey = resource_key!(uniset, "10=89", 1);
    pub const JOINING_GROUP_TAW_V1: ResourceKey = resource_key!(uniset, "10=90", 1);
    pub const JOINING_GROUP_TEH_MARBUTA_V1: ResourceKey = resource_key!(uniset, "10=91", 1);
    pub const JOINING_GROUP_HAMZA_ON_HEH_GOAL_V1: ResourceKey = resource_key!(uniset, "10=92", 1);
    pub const JOINING_GROUP_TETH_V1: ResourceKey = resource_key!(uniset, "10=93", 1);
    pub const JOINING_GROUP_WAW_V1: ResourceKey = resource_key!(uniset, "10=94", 1);
    pub const JOINING_GROUP_YEH_V1: ResourceKey = resource_key!(uniset, "10=95", 1);
    pub const JOINING_GROUP_YEH_BARREE_V1: ResourceKey = resource_key!(uniset, "10=96", 1);
    pub const JOINING_GROUP_YEH_WITH_TAIL_V1: ResourceKey = resource_key!(uniset, "10=97", 1);
    pub const JOINING_GROUP_YUDH_V1: ResourceKey = resource_key!(uniset, "10=98", 1);
    pub const JOINING_GROUP_YUDH_HE_V1: ResourceKey = resource_key!(uniset, "10=99", 1);
    pub const JOINING_GROUP_ZAIN_V1: ResourceKey = resource_key!(uniset, "10=100", 1);
    pub const JOINING_GROUP_ZHAIN_V1: ResourceKey = resource_key!(uniset, "10=101", 1);
    pub const JOINING_TYPE_JOIN_CAUSING_V1: ResourceKey = resource_key!(uniset, "11=0", 1);
    pub const JOINING_TYPE_DUAL_JOINING_V1: ResourceKey = resource_key!(uniset, "11=1", 1);
    pub const JOINING_TYPE_LEFT_JOINING_V1: ResourceKey = resource_key!(uniset, "11=2", 1);
    pub const JOINING_TYPE_RIGHT_JOINING_V1: ResourceKey = resource_key!(uniset, "11=3", 1);
    pub const JOINING_TYPE_TRANSPARENT_V1: ResourceKey = resource_key!(uniset, "11=4", 1);
    pub const JOINING_TYPE_NON_JOINING_V1: ResourceKey = resource_key!(uniset, "11=5", 1);
    pub const LINE_BREAK_AMBIGUOUS_V1: ResourceKey = resource_key!(uniset, "12=0", 1);
    pub const LINE_BREAK_ALPHABETIC_V1: ResourceKey = resource_key!(uniset, "12=1", 1);
    pub const LINE_BREAK_BREAK_BOTH_V1: ResourceKey = resource_key!(uniset, "12=2", 1);
    pub const LINE_BREAK_BREAK_AFTER_V1: ResourceKey = resource_key!(uniset, "12=3", 1);
    pub const LINE_BREAK_BREAK_BEFORE_V1: ResourceKey = resource_key!(uniset, "12=4", 1);
    pub const LINE_BREAK_MANDATORY_BREAK_V1: ResourceKey = resource_key!(uniset, "12=5", 1);
    pub const LINE_BREAK_CONTINGENT_BREAK_V1: ResourceKey = resource_key!(uniset, "12=6", 1);
    pub const LINE_BREAK_CONDITIONAL_JAPANESE_STARTER_V1: ResourceKey =
        resource_key!(uniset, "12=7", 1);
    pub const LINE_BREAK_CLOSE_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "12=8", 1);
    pub const LINE_BREAK_COMBINING_MARK_V1: ResourceKey = resource_key!(uniset, "12=9", 1);
    pub const LINE_BREAK_CLOSE_PARENTHESIS_V1: ResourceKey = resource_key!(uniset, "12=10", 1);
    pub const LINE_BREAK_CARRIAGE_RETURN_V1: ResourceKey = resource_key!(uniset, "12=11", 1);
    pub const LINE_BREAK_E_BASE_V1: ResourceKey = resource_key!(uniset, "12=12", 1);
    pub const LINE_BREAK_E_MODIFIER_V1: ResourceKey = resource_key!(uniset, "12=13", 1);
    pub const LINE_BREAK_EXCLAMATION_V1: ResourceKey = resource_key!(uniset, "12=14", 1);
    pub const LINE_BREAK_GLUE_V1: ResourceKey = resource_key!(uniset, "12=15", 1);
    pub const LINE_BREAK_H2_V1: ResourceKey = resource_key!(uniset, "12=16", 1);
    pub const LINE_BREAK_H3_V1: ResourceKey = resource_key!(uniset, "12=17", 1);
    pub const LINE_BREAK_HEBREW_LETTER_V1: ResourceKey = resource_key!(uniset, "12=18", 1);
    pub const LINE_BREAK_HYPHEN_V1: ResourceKey = resource_key!(uniset, "12=19", 1);
    pub const LINE_BREAK_IDEOGRAPHIC_V1: ResourceKey = resource_key!(uniset, "12=20", 1);
    pub const LINE_BREAK_INSEPERABLE_V1: ResourceKey = resource_key!(uniset, "12=21", 1);
    pub const LINE_BREAK_INFIX_NUMERIC_V1: ResourceKey = resource_key!(uniset, "12=22", 1);
    pub const LINE_BREAK_JL_V1: ResourceKey = resource_key!(uniset, "12=23", 1);
    pub const LINE_BREAK_JT_V1: ResourceKey = resource_key!(uniset, "12=24", 1);
    pub const LINE_BREAK_JV_V1: ResourceKey = resource_key!(uniset, "12=25", 1);
    pub const LINE_BREAK_LINE_FEED_V1: ResourceKey = resource_key!(uniset, "12=26", 1);
    pub const LINE_BREAK_NEXT_LINE_V1: ResourceKey = resource_key!(uniset, "12=27", 1);
    pub const LINE_BREAK_NONSTARTER_V1: ResourceKey = resource_key!(uniset, "12=28", 1);
    pub const LINE_BREAK_NUMERIC_V1: ResourceKey = resource_key!(uniset, "12=29", 1);
    pub const LINE_BREAK_OPEN_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "12=30", 1);
    pub const LINE_BREAK_POSTFIX_NUMERIC_V1: ResourceKey = resource_key!(uniset, "12=31", 1);
    pub const LINE_BREAK_PREFIX_NUMERIC_V1: ResourceKey = resource_key!(uniset, "12=32", 1);
    pub const LINE_BREAK_QUOTATION_V1: ResourceKey = resource_key!(uniset, "12=33", 1);
    pub const LINE_BREAK_REGIONAL_INDICATOR_V1: ResourceKey = resource_key!(uniset, "12=34", 1);
    pub const LINE_BREAK_COMPLEX_CONTEXT_V1: ResourceKey = resource_key!(uniset, "12=35", 1);
    pub const LINE_BREAK_SURROGATE_V1: ResourceKey = resource_key!(uniset, "12=36", 1);
    pub const LINE_BREAK_SPACE_V1: ResourceKey = resource_key!(uniset, "12=37", 1);
    pub const LINE_BREAK_BREAK_SYMBOLS_V1: ResourceKey = resource_key!(uniset, "12=38", 1);
    pub const LINE_BREAK_WORD_JOINER_V1: ResourceKey = resource_key!(uniset, "12=39", 1);
    pub const LINE_BREAK_UNKNOWN_V1: ResourceKey = resource_key!(uniset, "12=40", 1);
    pub const LINE_BREAK_ZWSPACE_V1: ResourceKey = resource_key!(uniset, "12=41", 1);
    pub const LINE_BREAK_ZWJ_V1: ResourceKey = resource_key!(uniset, "12=42", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1: ResourceKey =
        resource_key!(uniset, "13=0", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_OVERLAY_V1: ResourceKey =
        resource_key!(uniset, "13=1", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC10_V1: ResourceKey =
        resource_key!(uniset, "13=10", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC103_V1: ResourceKey =
        resource_key!(uniset, "13=103", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC107_V1: ResourceKey =
        resource_key!(uniset, "13=107", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC11_V1: ResourceKey =
        resource_key!(uniset, "13=11", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC118_V1: ResourceKey =
        resource_key!(uniset, "13=118", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC12_V1: ResourceKey =
        resource_key!(uniset, "13=12", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC122_V1: ResourceKey =
        resource_key!(uniset, "13=122", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC129_V1: ResourceKey =
        resource_key!(uniset, "13=129", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC13_V1: ResourceKey =
        resource_key!(uniset, "13=13", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC130_V1: ResourceKey =
        resource_key!(uniset, "13=130", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC132_V1: ResourceKey =
        resource_key!(uniset, "13=132", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC133_V1: ResourceKey =
        resource_key!(uniset, "13=133", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC14_V1: ResourceKey =
        resource_key!(uniset, "13=14", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC15_V1: ResourceKey =
        resource_key!(uniset, "13=15", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC16_V1: ResourceKey =
        resource_key!(uniset, "13=16", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC17_V1: ResourceKey =
        resource_key!(uniset, "13=17", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC18_V1: ResourceKey =
        resource_key!(uniset, "13=18", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC19_V1: ResourceKey =
        resource_key!(uniset, "13=19", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC20_V1: ResourceKey =
        resource_key!(uniset, "13=20", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "13=200", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1: ResourceKey =
        resource_key!(uniset, "13=202", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC21_V1: ResourceKey =
        resource_key!(uniset, "13=21", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "13=214", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "13=216", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "13=218", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC22_V1: ResourceKey =
        resource_key!(uniset, "13=22", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_BELOW_V1: ResourceKey =
        resource_key!(uniset, "13=220", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "13=222", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_LEFT_V1: ResourceKey =
        resource_key!(uniset, "13=224", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "13=226", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1: ResourceKey =
        resource_key!(uniset, "13=228", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC23_V1: ResourceKey =
        resource_key!(uniset, "13=23", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "13=230", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "13=232", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1: ResourceKey =
        resource_key!(uniset, "13=233", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "13=234", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC24_V1: ResourceKey =
        resource_key!(uniset, "13=24", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1: ResourceKey =
        resource_key!(uniset, "13=240", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC25_V1: ResourceKey =
        resource_key!(uniset, "13=25", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC26_V1: ResourceKey =
        resource_key!(uniset, "13=26", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC27_V1: ResourceKey =
        resource_key!(uniset, "13=27", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC28_V1: ResourceKey =
        resource_key!(uniset, "13=28", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC29_V1: ResourceKey =
        resource_key!(uniset, "13=29", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC30_V1: ResourceKey =
        resource_key!(uniset, "13=30", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC31_V1: ResourceKey =
        resource_key!(uniset, "13=31", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC32_V1: ResourceKey =
        resource_key!(uniset, "13=32", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC33_V1: ResourceKey =
        resource_key!(uniset, "13=33", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC34_V1: ResourceKey =
        resource_key!(uniset, "13=34", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC35_V1: ResourceKey =
        resource_key!(uniset, "13=35", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC36_V1: ResourceKey =
        resource_key!(uniset, "13=36", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_HAN_READING_V1: ResourceKey =
        resource_key!(uniset, "13=6", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_NUKTA_V1: ResourceKey =
        resource_key!(uniset, "13=7", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1: ResourceKey =
        resource_key!(uniset, "13=8", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC84_V1: ResourceKey =
        resource_key!(uniset, "13=84", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_VIRAMA_V1: ResourceKey =
        resource_key!(uniset, "13=9", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC91_V1: ResourceKey =
        resource_key!(uniset, "13=91", 1);
    pub const NFC_QUICK_CHECK_MAYBE_V1: ResourceKey = resource_key!(uniset, "14=0", 1);
    pub const NFC_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "14=1", 1);
    pub const NFC_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "14=2", 1);
    pub const NFD_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "15=0", 1);
    pub const NFD_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "15=1", 1);
    pub const NFKC_QUICK_CHECK_MAYBE_V1: ResourceKey = resource_key!(uniset, "16=0", 1);
    pub const NFKC_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "16=1", 1);
    pub const NFKC_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "16=2", 1);
    pub const NFKD_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "17=0", 1);
    pub const NFKD_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "17=1", 1);
    pub const NUMERIC_TYPE_DECIMAL_V1: ResourceKey = resource_key!(uniset, "18=0", 1);
    pub const NUMERIC_TYPE_DIGIT_V1: ResourceKey = resource_key!(uniset, "18=1", 1);
    pub const NUMERIC_TYPE_NONE_V1: ResourceKey = resource_key!(uniset, "18=2", 1);
    pub const NUMERIC_TYPE_NUMERIC_V1: ResourceKey = resource_key!(uniset, "18=3", 1);
    pub const SENTENCE_BREAK_ATERM_V1: ResourceKey = resource_key!(uniset, "19=0", 1);
    pub const SENTENCE_BREAK_CLOSE_V1: ResourceKey = resource_key!(uniset, "19=1", 1);
    pub const SENTENCE_BREAK_CR_V1: ResourceKey = resource_key!(uniset, "19=2", 1);
    pub const SENTENCE_BREAK_EXTEND_V1: ResourceKey = resource_key!(uniset, "19=3", 1);
    pub const SENTENCE_BREAK_FORMAT_V1: ResourceKey = resource_key!(uniset, "19=4", 1);
    pub const SENTENCE_BREAK_OLETTER_V1: ResourceKey = resource_key!(uniset, "19=5", 1);
    pub const SENTENCE_BREAK_LF_V1: ResourceKey = resource_key!(uniset, "19=6", 1);
    pub const SENTENCE_BREAK_LOWER_V1: ResourceKey = resource_key!(uniset, "19=7", 1);
    pub const SENTENCE_BREAK_NUMERIC_V1: ResourceKey = resource_key!(uniset, "19=8", 1);
    pub const SENTENCE_BREAK_SCONTINUE_V1: ResourceKey = resource_key!(uniset, "19=9", 1);
    pub const SENTENCE_BREAK_SEP_V1: ResourceKey = resource_key!(uniset, "19=10", 1);
    pub const SENTENCE_BREAK_SP_V1: ResourceKey = resource_key!(uniset, "19=11", 1);
    pub const SENTENCE_BREAK_STERM_V1: ResourceKey = resource_key!(uniset, "19=12", 1);
    pub const SENTENCE_BREAK_UPPER_V1: ResourceKey = resource_key!(uniset, "19=13", 1);
    pub const SENTENCE_BREAK_OTHER_V1: ResourceKey = resource_key!(uniset, "19=14", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1: ResourceKey =
        resource_key!(uniset, "20=0", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_OVERLAY_V1: ResourceKey =
        resource_key!(uniset, "20=1", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC10_V1: ResourceKey =
        resource_key!(uniset, "20=10", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC103_V1: ResourceKey =
        resource_key!(uniset, "20=103", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC107_V1: ResourceKey =
        resource_key!(uniset, "20=107", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC11_V1: ResourceKey =
        resource_key!(uniset, "20=11", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC118_V1: ResourceKey =
        resource_key!(uniset, "20=118", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC12_V1: ResourceKey =
        resource_key!(uniset, "20=12", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC122_V1: ResourceKey =
        resource_key!(uniset, "20=122", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC129_V1: ResourceKey =
        resource_key!(uniset, "20=129", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC13_V1: ResourceKey =
        resource_key!(uniset, "20=13", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC130_V1: ResourceKey =
        resource_key!(uniset, "20=130", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC132_V1: ResourceKey =
        resource_key!(uniset, "20=132", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC133_V1: ResourceKey =
        resource_key!(uniset, "20=133", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC14_V1: ResourceKey =
        resource_key!(uniset, "20=14", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC15_V1: ResourceKey =
        resource_key!(uniset, "20=15", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC16_V1: ResourceKey =
        resource_key!(uniset, "20=16", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC17_V1: ResourceKey =
        resource_key!(uniset, "20=17", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC18_V1: ResourceKey =
        resource_key!(uniset, "20=18", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC19_V1: ResourceKey =
        resource_key!(uniset, "20=19", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC20_V1: ResourceKey =
        resource_key!(uniset, "20=20", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "20=200", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1: ResourceKey =
        resource_key!(uniset, "20=202", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC21_V1: ResourceKey =
        resource_key!(uniset, "20=21", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "20=214", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "20=216", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "20=218", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC22_V1: ResourceKey =
        resource_key!(uniset, "20=22", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_BELOW_V1: ResourceKey =
        resource_key!(uniset, "20=220", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "20=222", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_LEFT_V1: ResourceKey =
        resource_key!(uniset, "20=224", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "20=226", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1: ResourceKey =
        resource_key!(uniset, "20=228", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC23_V1: ResourceKey =
        resource_key!(uniset, "20=23", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "20=230", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "20=232", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1: ResourceKey =
        resource_key!(uniset, "20=233", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "20=234", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC24_V1: ResourceKey =
        resource_key!(uniset, "20=24", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1: ResourceKey =
        resource_key!(uniset, "20=240", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC25_V1: ResourceKey =
        resource_key!(uniset, "20=25", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC26_V1: ResourceKey =
        resource_key!(uniset, "20=26", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC27_V1: ResourceKey =
        resource_key!(uniset, "20=27", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC28_V1: ResourceKey =
        resource_key!(uniset, "20=28", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC29_V1: ResourceKey =
        resource_key!(uniset, "20=29", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC30_V1: ResourceKey =
        resource_key!(uniset, "20=30", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC31_V1: ResourceKey =
        resource_key!(uniset, "20=31", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC32_V1: ResourceKey =
        resource_key!(uniset, "20=32", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC33_V1: ResourceKey =
        resource_key!(uniset, "20=33", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC34_V1: ResourceKey =
        resource_key!(uniset, "20=34", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC35_V1: ResourceKey =
        resource_key!(uniset, "20=35", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC36_V1: ResourceKey =
        resource_key!(uniset, "20=36", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_HAN_READING_V1: ResourceKey =
        resource_key!(uniset, "20=6", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_NUKTA_V1: ResourceKey =
        resource_key!(uniset, "20=7", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1: ResourceKey =
        resource_key!(uniset, "20=8", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC84_V1: ResourceKey =
        resource_key!(uniset, "20=84", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_VIRAMA_V1: ResourceKey =
        resource_key!(uniset, "20=9", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC91_V1: ResourceKey =
        resource_key!(uniset, "20=91", 1);
    pub const VERTICAL_ORIENTATION_ROTATED_V1: ResourceKey = resource_key!(uniset, "21=0", 1);
    pub const VERTICAL_ORIENTATION_TRANSFORMED_ROTATED_V1: ResourceKey =
        resource_key!(uniset, "21=1", 1);
    pub const VERTICAL_ORIENTATION_TRANSFORMED_UPRIGHT_V1: ResourceKey =
        resource_key!(uniset, "21=2", 1);
    pub const VERTICAL_ORIENTATION_UPRIGHT_V1: ResourceKey = resource_key!(uniset, "21=3", 1);
    pub const WORD_BREAK_CR_V1: ResourceKey = resource_key!(uniset, "22=0", 1);
    pub const WORD_BREAK_DOUBLE_QUOTE_V1: ResourceKey = resource_key!(uniset, "22=1", 1);
    pub const WORD_BREAK_E_BASE_V1: ResourceKey = resource_key!(uniset, "22=2", 1);
    pub const WORD_BREAK_E_BASE_GAZ_V1: ResourceKey = resource_key!(uniset, "22=3", 1);
    pub const WORD_BREAK_E_MODIFIER_V1: ResourceKey = resource_key!(uniset, "22=4", 1);
    pub const WORD_BREAK_EXTENDNUMLET_V1: ResourceKey = resource_key!(uniset, "22=5", 1);
    pub const WORD_BREAK_EXTEND_V1: ResourceKey = resource_key!(uniset, "22=6", 1);
    pub const WORD_BREAK_FORMAT_V1: ResourceKey = resource_key!(uniset, "22=7", 1);
    pub const WORD_BREAK_GLUE_AFTER_ZWJ_V1: ResourceKey = resource_key!(uniset, "22=8", 1);
    pub const WORD_BREAK_HEBREW_LETTER_V1: ResourceKey = resource_key!(uniset, "22=9", 1);
    pub const WORD_BREAK_KATAKANA_V1: ResourceKey = resource_key!(uniset, "22=10", 1);
    pub const WORD_BREAK_ALETTER_V1: ResourceKey = resource_key!(uniset, "22=11", 1);
    pub const WORD_BREAK_LF_V1: ResourceKey = resource_key!(uniset, "22=12", 1);
    pub const WORD_BREAK_MIDNUMLET_V1: ResourceKey = resource_key!(uniset, "22=13", 1);
    pub const WORD_BREAK_MIDLETTER_V1: ResourceKey = resource_key!(uniset, "22=14", 1);
    pub const WORD_BREAK_MIDNUM_V1: ResourceKey = resource_key!(uniset, "22=15", 1);
    pub const WORD_BREAK_NEWLINE_V1: ResourceKey = resource_key!(uniset, "22=16", 1);
    pub const WORD_BREAK_NUMERIC_V1: ResourceKey = resource_key!(uniset, "22=17", 1);
    pub const WORD_BREAK_REGIONAL_INDICATOR_V1: ResourceKey = resource_key!(uniset, "22=18", 1);
    pub const WORD_BREAK_SINGLE_QUOTE_V1: ResourceKey = resource_key!(uniset, "22=19", 1);
    pub const WORD_BREAK_WSEGSPACE_V1: ResourceKey = resource_key!(uniset, "22=20", 1);
    pub const WORD_BREAK_OTHER_V1: ResourceKey = resource_key!(uniset, "22=21", 1);
    pub const WORD_BREAK_ZWJ_V1: ResourceKey = resource_key!(uniset, "22=22", 1);
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "testing", derive(Hash, Eq))]
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
