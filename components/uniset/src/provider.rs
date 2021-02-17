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

    pub const BIDI_CLASS_ARABIC_LETTER_V1: ResourceKey = resource_key!(uniset, "bc=AL", 1);
    pub const BIDI_CLASS_ARABIC_NUMBER_V1: ResourceKey = resource_key!(uniset, "bc=AN", 1);
    pub const BIDI_CLASS_PARAGRAPH_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "bc=B", 1);
    pub const BIDI_CLASS_BOUNDARY_NEUTRAL_V1: ResourceKey = resource_key!(uniset, "bc=BN", 1);
    pub const BIDI_CLASS_COMMON_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "bc=CS", 1);
    pub const BIDI_CLASS_EUROPEAN_NUMBER_V1: ResourceKey = resource_key!(uniset, "bc=EN", 1);
    pub const BIDI_CLASS_EUROPEAN_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "bc=ES", 1);
    pub const BIDI_CLASS_EUROPEAN_TERMINATOR_V1: ResourceKey = resource_key!(uniset, "bc=ET", 1);
    pub const BIDI_CLASS_FIRST_STRONG_ISOLATE_V1: ResourceKey = resource_key!(uniset, "bc=FSI", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_V1: ResourceKey = resource_key!(uniset, "bc=L", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_EMBEDDING_V1: ResourceKey =
        resource_key!(uniset, "bc=LRE", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_ISOLATE_V1: ResourceKey = resource_key!(uniset, "bc=LRI", 1);
    pub const BIDI_CLASS_LEFT_TO_RIGHT_OVERRIDE_V1: ResourceKey =
        resource_key!(uniset, "bc=LRO", 1);
    pub const BIDI_CLASS_NONSPACING_MARK_V1: ResourceKey = resource_key!(uniset, "bc=NSM", 1);
    pub const BIDI_CLASS_OTHER_NEUTRAL_V1: ResourceKey = resource_key!(uniset, "bc=ON", 1);
    pub const BIDI_CLASS_POP_DIRECTIONAL_FORMAT_V1: ResourceKey =
        resource_key!(uniset, "bc=PDF", 1);
    pub const BIDI_CLASS_POP_DIRECTIONAL_ISOLATE_V1: ResourceKey =
        resource_key!(uniset, "bc=PDI", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_V1: ResourceKey = resource_key!(uniset, "bc=R", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_EMBEDDING_V1: ResourceKey =
        resource_key!(uniset, "bc=RLE", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_ISOLATE_V1: ResourceKey = resource_key!(uniset, "bc=RLI", 1);
    pub const BIDI_CLASS_RIGHT_TO_LEFT_OVERRIDE_V1: ResourceKey =
        resource_key!(uniset, "bc=RLO", 1);
    pub const BIDI_CLASS_SEGMENT_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "bc=S", 1);
    pub const BIDI_CLASS_WHITE_SPACE_V1: ResourceKey = resource_key!(uniset, "bc=WS", 1);
    pub const BIDI_PAIRED_BRACKET_TYPE_CLOSE_V1: ResourceKey = resource_key!(uniset, "bpt=c", 1);
    pub const BIDI_PAIRED_BRACKET_TYPE_NONE_V1: ResourceKey = resource_key!(uniset, "bpt=n", 1);
    pub const BIDI_PAIRED_BRACKET_TYPE_OPEN_V1: ResourceKey = resource_key!(uniset, "bpt=o", 1);
    pub const CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1: ResourceKey =
        resource_key!(uniset, "ccc=0", 1);
    pub const CANONICAL_COMBINING_CLASS_OVERLAY_V1: ResourceKey = resource_key!(uniset, "ccc=1", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC10_V1: ResourceKey = resource_key!(uniset, "ccc=10", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC103_V1: ResourceKey =
        resource_key!(uniset, "ccc=103", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC107_V1: ResourceKey =
        resource_key!(uniset, "ccc=107", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC11_V1: ResourceKey = resource_key!(uniset, "ccc=11", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC118_V1: ResourceKey =
        resource_key!(uniset, "ccc=118", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC12_V1: ResourceKey = resource_key!(uniset, "ccc=12", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC122_V1: ResourceKey =
        resource_key!(uniset, "ccc=122", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC129_V1: ResourceKey =
        resource_key!(uniset, "ccc=129", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC13_V1: ResourceKey = resource_key!(uniset, "ccc=13", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC130_V1: ResourceKey =
        resource_key!(uniset, "ccc=130", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC132_V1: ResourceKey =
        resource_key!(uniset, "ccc=132", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC133_V1: ResourceKey =
        resource_key!(uniset, "ccc=133", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC14_V1: ResourceKey = resource_key!(uniset, "ccc=14", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC15_V1: ResourceKey = resource_key!(uniset, "ccc=15", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC16_V1: ResourceKey = resource_key!(uniset, "ccc=16", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC17_V1: ResourceKey = resource_key!(uniset, "ccc=17", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC18_V1: ResourceKey = resource_key!(uniset, "ccc=18", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC19_V1: ResourceKey = resource_key!(uniset, "ccc=19", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC20_V1: ResourceKey = resource_key!(uniset, "ccc=20", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "ccc=200", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1: ResourceKey =
        resource_key!(uniset, "ccc=202", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC21_V1: ResourceKey = resource_key!(uniset, "ccc=21", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "ccc=214", 1);
    pub const CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "ccc=216", 1);
    pub const CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "ccc=218", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC22_V1: ResourceKey = resource_key!(uniset, "ccc=22", 1);
    pub const CANONICAL_COMBINING_CLASS_BELOW_V1: ResourceKey = resource_key!(uniset, "ccc=220", 1);
    pub const CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "ccc=222", 1);
    pub const CANONICAL_COMBINING_CLASS_LEFT_V1: ResourceKey = resource_key!(uniset, "ccc=224", 1);
    pub const CANONICAL_COMBINING_CLASS_RIGHT_V1: ResourceKey = resource_key!(uniset, "ccc=226", 1);
    pub const CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1: ResourceKey =
        resource_key!(uniset, "ccc=228", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC23_V1: ResourceKey = resource_key!(uniset, "ccc=23", 1);
    pub const CANONICAL_COMBINING_CLASS_ABOVE_V1: ResourceKey = resource_key!(uniset, "ccc=230", 1);
    pub const CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "ccc=232", 1);
    pub const CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1: ResourceKey =
        resource_key!(uniset, "ccc=233", 1);
    pub const CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "ccc=234", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC24_V1: ResourceKey = resource_key!(uniset, "ccc=24", 1);
    pub const CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1: ResourceKey =
        resource_key!(uniset, "ccc=240", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC25_V1: ResourceKey = resource_key!(uniset, "ccc=25", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC26_V1: ResourceKey = resource_key!(uniset, "ccc=26", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC27_V1: ResourceKey = resource_key!(uniset, "ccc=27", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC28_V1: ResourceKey = resource_key!(uniset, "ccc=28", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC29_V1: ResourceKey = resource_key!(uniset, "ccc=29", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC30_V1: ResourceKey = resource_key!(uniset, "ccc=30", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC31_V1: ResourceKey = resource_key!(uniset, "ccc=31", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC32_V1: ResourceKey = resource_key!(uniset, "ccc=32", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC33_V1: ResourceKey = resource_key!(uniset, "ccc=33", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC34_V1: ResourceKey = resource_key!(uniset, "ccc=34", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC35_V1: ResourceKey = resource_key!(uniset, "ccc=35", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC36_V1: ResourceKey = resource_key!(uniset, "ccc=36", 1);
    pub const CANONICAL_COMBINING_CLASS_HAN_READING_V1: ResourceKey =
        resource_key!(uniset, "ccc=6", 1);
    pub const CANONICAL_COMBINING_CLASS_NUKTA_V1: ResourceKey = resource_key!(uniset, "ccc=7", 1);
    pub const CANONICAL_COMBINING_CLASS_KANA_VOICING_V1: ResourceKey =
        resource_key!(uniset, "ccc=8", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC84_V1: ResourceKey = resource_key!(uniset, "ccc=84", 1);
    pub const CANONICAL_COMBINING_CLASS_VIRAMA_V1: ResourceKey = resource_key!(uniset, "ccc=9", 1);
    pub const CANONICAL_COMBINING_CLASS_CCC91_V1: ResourceKey = resource_key!(uniset, "ccc=91", 1);
    pub const DECOMPOSITION_TYPE_CAN_V1: ResourceKey = resource_key!(uniset, "dt=Can", 1);
    pub const DECOMPOSITION_TYPE_COM_V1: ResourceKey = resource_key!(uniset, "dt=Com", 1);
    pub const DECOMPOSITION_TYPE_ENC_V1: ResourceKey = resource_key!(uniset, "dt=Enc", 1);
    pub const DECOMPOSITION_TYPE_FIN_V1: ResourceKey = resource_key!(uniset, "dt=Fin", 1);
    pub const DECOMPOSITION_TYPE_FONT_V1: ResourceKey = resource_key!(uniset, "dt=Font", 1);
    pub const DECOMPOSITION_TYPE_FRA_V1: ResourceKey = resource_key!(uniset, "dt=Fra", 1);
    pub const DECOMPOSITION_TYPE_INIT_V1: ResourceKey = resource_key!(uniset, "dt=Init", 1);
    pub const DECOMPOSITION_TYPE_ISO_V1: ResourceKey = resource_key!(uniset, "dt=Iso", 1);
    pub const DECOMPOSITION_TYPE_MED_V1: ResourceKey = resource_key!(uniset, "dt=Med", 1);
    pub const DECOMPOSITION_TYPE_NAR_V1: ResourceKey = resource_key!(uniset, "dt=Nar", 1);
    pub const DECOMPOSITION_TYPE_NB_V1: ResourceKey = resource_key!(uniset, "dt=Nb", 1);
    pub const DECOMPOSITION_TYPE_NONE_V1: ResourceKey = resource_key!(uniset, "dt=None", 1);
    pub const DECOMPOSITION_TYPE_SML_V1: ResourceKey = resource_key!(uniset, "dt=Sml", 1);
    pub const DECOMPOSITION_TYPE_SQR_V1: ResourceKey = resource_key!(uniset, "dt=Sqr", 1);
    pub const DECOMPOSITION_TYPE_SUB_V1: ResourceKey = resource_key!(uniset, "dt=Sub", 1);
    pub const DECOMPOSITION_TYPE_SUP_V1: ResourceKey = resource_key!(uniset, "dt=Sup", 1);
    pub const DECOMPOSITION_TYPE_VERT_V1: ResourceKey = resource_key!(uniset, "dt=Vert", 1);
    pub const DECOMPOSITION_TYPE_WIDE_V1: ResourceKey = resource_key!(uniset, "dt=Wide", 1);
    pub const EAST_ASIAN_WIDTH_AMBIGUOUS_V1: ResourceKey = resource_key!(uniset, "ea=A", 1);
    pub const EAST_ASIAN_WIDTH_FULLWIDTH_V1: ResourceKey = resource_key!(uniset, "ea=F", 1);
    pub const EAST_ASIAN_WIDTH_HALFWIDTH_V1: ResourceKey = resource_key!(uniset, "ea=H", 1);
    pub const EAST_ASIAN_WIDTH_NEUTRAL_V1: ResourceKey = resource_key!(uniset, "ea=N", 1);
    pub const EAST_ASIAN_WIDTH_NARROW_V1: ResourceKey = resource_key!(uniset, "ea=Na", 1);
    pub const EAST_ASIAN_WIDTH_WIDE_V1: ResourceKey = resource_key!(uniset, "ea=W", 1);
    pub const GENERAL_CATEGORY_OTHER_V1: ResourceKey = resource_key!(uniset, "gc=C", 1);
    pub const GENERAL_CATEGORY_CNTRL_V1: ResourceKey = resource_key!(uniset, "gc=Cc", 1);
    pub const GENERAL_CATEGORY_FORMAT_V1: ResourceKey = resource_key!(uniset, "gc=Cf", 1);
    pub const GENERAL_CATEGORY_UNASSIGNED_V1: ResourceKey = resource_key!(uniset, "gc=Cn", 1);
    pub const GENERAL_CATEGORY_PRIVATE_USE_V1: ResourceKey = resource_key!(uniset, "gc=Co", 1);
    pub const GENERAL_CATEGORY_SURROGATE_V1: ResourceKey = resource_key!(uniset, "gc=Cs", 1);
    pub const GENERAL_CATEGORY_LETTER_V1: ResourceKey = resource_key!(uniset, "gc=L", 1);
    pub const GENERAL_CATEGORY_CASED_LETTER_V1: ResourceKey = resource_key!(uniset, "gc=LC", 1);
    pub const GENERAL_CATEGORY_LOWERCASE_LETTER_V1: ResourceKey = resource_key!(uniset, "gc=Ll", 1);
    pub const GENERAL_CATEGORY_MODIFIER_LETTER_V1: ResourceKey = resource_key!(uniset, "gc=Lm", 1);
    pub const GENERAL_CATEGORY_OTHER_LETTER_V1: ResourceKey = resource_key!(uniset, "gc=Lo", 1);
    pub const GENERAL_CATEGORY_TITLECASE_LETTER_V1: ResourceKey = resource_key!(uniset, "gc=Lt", 1);
    pub const GENERAL_CATEGORY_UPPERCASE_LETTER_V1: ResourceKey = resource_key!(uniset, "gc=Lu", 1);
    pub const GENERAL_CATEGORY_COMBINING_MARK_V1: ResourceKey = resource_key!(uniset, "gc=M", 1);
    pub const GENERAL_CATEGORY_SPACING_MARK_V1: ResourceKey = resource_key!(uniset, "gc=Mc", 1);
    pub const GENERAL_CATEGORY_ENCLOSING_MARK_V1: ResourceKey = resource_key!(uniset, "gc=Me", 1);
    pub const GENERAL_CATEGORY_NONSPACING_MARK_V1: ResourceKey = resource_key!(uniset, "gc=Mn", 1);
    pub const GENERAL_CATEGORY_NUMBER_V1: ResourceKey = resource_key!(uniset, "gc=N", 1);
    pub const GENERAL_CATEGORY_DIGIT_V1: ResourceKey = resource_key!(uniset, "gc=Nd", 1);
    pub const GENERAL_CATEGORY_LETTER_NUMBER_V1: ResourceKey = resource_key!(uniset, "gc=Nl", 1);
    pub const GENERAL_CATEGORY_OTHER_NUMBER_V1: ResourceKey = resource_key!(uniset, "gc=No", 1);
    pub const GENERAL_CATEGORY_PUNCT_V1: ResourceKey = resource_key!(uniset, "gc=P", 1);
    pub const GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1: ResourceKey =
        resource_key!(uniset, "gc=Pc", 1);
    pub const GENERAL_CATEGORY_DASH_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "gc=Pd", 1);
    pub const GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1: ResourceKey =
        resource_key!(uniset, "gc=Pe", 1);
    pub const GENERAL_CATEGORY_FINAL_PUNCTUATION_V1: ResourceKey =
        resource_key!(uniset, "gc=Pf", 1);
    pub const GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1: ResourceKey =
        resource_key!(uniset, "gc=Pi", 1);
    pub const GENERAL_CATEGORY_OTHER_PUNCTUATION_V1: ResourceKey =
        resource_key!(uniset, "gc=Po", 1);
    pub const GENERAL_CATEGORY_OPEN_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "gc=Ps", 1);
    pub const GENERAL_CATEGORY_SYMBOL_V1: ResourceKey = resource_key!(uniset, "gc=S", 1);
    pub const GENERAL_CATEGORY_CURRENCY_SYMBOL_V1: ResourceKey = resource_key!(uniset, "gc=Sc", 1);
    pub const GENERAL_CATEGORY_MODIFIER_SYMBOL_V1: ResourceKey = resource_key!(uniset, "gc=Sk", 1);
    pub const GENERAL_CATEGORY_MATH_SYMBOL_V1: ResourceKey = resource_key!(uniset, "gc=Sm", 1);
    pub const GENERAL_CATEGORY_OTHER_SYMBOL_V1: ResourceKey = resource_key!(uniset, "gc=So", 1);
    pub const GENERAL_CATEGORY_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "gc=Z", 1);
    pub const GENERAL_CATEGORY_LINE_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "gc=Zl", 1);
    pub const GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1: ResourceKey =
        resource_key!(uniset, "gc=Zp", 1);
    pub const GENERAL_CATEGORY_SPACE_SEPARATOR_V1: ResourceKey = resource_key!(uniset, "gc=Zs", 1);
    pub const GRAPHEME_CLUSTER_BREAK_CONTROL_V1: ResourceKey = resource_key!(uniset, "GCB=CN", 1);
    pub const GRAPHEME_CLUSTER_BREAK_CR_V1: ResourceKey = resource_key!(uniset, "GCB=CR", 1);
    pub const GRAPHEME_CLUSTER_BREAK_E_BASE_V1: ResourceKey = resource_key!(uniset, "GCB=EB", 1);
    pub const GRAPHEME_CLUSTER_BREAK_E_BASE_GAZ_V1: ResourceKey =
        resource_key!(uniset, "GCB=EBG", 1);
    pub const GRAPHEME_CLUSTER_BREAK_E_MODIFIER_V1: ResourceKey =
        resource_key!(uniset, "GCB=EM", 1);
    pub const GRAPHEME_CLUSTER_BREAK_EXTEND_V1: ResourceKey = resource_key!(uniset, "GCB=EX", 1);
    pub const GRAPHEME_CLUSTER_BREAK_GLUE_AFTER_ZWJ_V1: ResourceKey =
        resource_key!(uniset, "GCB=GAZ", 1);
    pub const GRAPHEME_CLUSTER_BREAK_L_V1: ResourceKey = resource_key!(uniset, "GCB=L", 1);
    pub const GRAPHEME_CLUSTER_BREAK_LF_V1: ResourceKey = resource_key!(uniset, "GCB=LF", 1);
    pub const GRAPHEME_CLUSTER_BREAK_LV_V1: ResourceKey = resource_key!(uniset, "GCB=LV", 1);
    pub const GRAPHEME_CLUSTER_BREAK_LVT_V1: ResourceKey = resource_key!(uniset, "GCB=LVT", 1);
    pub const GRAPHEME_CLUSTER_BREAK_PREPEND_V1: ResourceKey = resource_key!(uniset, "GCB=PP", 1);
    pub const GRAPHEME_CLUSTER_BREAK_REGIONAL_INDICATOR_V1: ResourceKey =
        resource_key!(uniset, "GCB=RI", 1);
    pub const GRAPHEME_CLUSTER_BREAK_SPACINGMARK_V1: ResourceKey =
        resource_key!(uniset, "GCB=SM", 1);
    pub const GRAPHEME_CLUSTER_BREAK_T_V1: ResourceKey = resource_key!(uniset, "GCB=T", 1);
    pub const GRAPHEME_CLUSTER_BREAK_V_V1: ResourceKey = resource_key!(uniset, "GCB=V", 1);
    pub const GRAPHEME_CLUSTER_BREAK_OTHER_V1: ResourceKey = resource_key!(uniset, "GCB=XX", 1);
    pub const GRAPHEME_CLUSTER_BREAK_ZWJ_V1: ResourceKey = resource_key!(uniset, "GCB=ZWJ", 1);
    pub const HANGUL_SYLLABLE_TYPE_LEADING_JAMO_V1: ResourceKey = resource_key!(uniset, "hst=L", 1);
    pub const HANGUL_SYLLABLE_TYPE_LV_SYLLABLE_V1: ResourceKey = resource_key!(uniset, "hst=LV", 1);
    pub const HANGUL_SYLLABLE_TYPE_LVT_SYLLABLE_V1: ResourceKey =
        resource_key!(uniset, "hst=LVT", 1);
    pub const HANGUL_SYLLABLE_TYPE_NOT_APPLICABLE_V1: ResourceKey =
        resource_key!(uniset, "hst=NA", 1);
    pub const HANGUL_SYLLABLE_TYPE_TRAILING_JAMO_V1: ResourceKey =
        resource_key!(uniset, "hst=T", 1);
    pub const HANGUL_SYLLABLE_TYPE_VOWEL_JAMO_V1: ResourceKey = resource_key!(uniset, "hst=V", 1);
    pub const INDIC_POSITIONAL_CATEGORY_BOTTOM_V1: ResourceKey =
        resource_key!(uniset, "InPC=Bottom", 1);
    pub const INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_LEFT_V1: ResourceKey =
        resource_key!(uniset, "InPC=BottomLeft", 1);
    pub const INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "InPC=BottomRight", 1);
    pub const INDIC_POSITIONAL_CATEGORY_LEFT_V1: ResourceKey =
        resource_key!(uniset, "InPC=Left", 1);
    pub const INDIC_POSITIONAL_CATEGORY_LEFT_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "InPC=LeftRight", 1);
    pub const INDIC_POSITIONAL_CATEGORY_NA_V1: ResourceKey = resource_key!(uniset, "InPC=NA", 1);
    pub const INDIC_POSITIONAL_CATEGORY_OVERSTRUCK_V1: ResourceKey =
        resource_key!(uniset, "InPC=Overstruck", 1);
    pub const INDIC_POSITIONAL_CATEGORY_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "InPC=Right", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_V1: ResourceKey = resource_key!(uniset, "InPC=Top", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_V1: ResourceKey =
        resource_key!(uniset, "InPC=TopBottom", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_LEFT_V1: ResourceKey =
        resource_key!(uniset, "InPC=TopBotLeft", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "InPC=TopBotRight", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_V1: ResourceKey =
        resource_key!(uniset, "InPC=TopLeft", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "InPC=TopLR", 1);
    pub const INDIC_POSITIONAL_CATEGORY_TOP_AND_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "InPC=TopRight", 1);
    pub const INDIC_POSITIONAL_CATEGORY_VISUAL_ORDER_LEFT_V1: ResourceKey =
        resource_key!(uniset, "InPC=VisOrdLeft", 1);
    pub const INDIC_SYLLABIC_CATEGORY_AVAGRAHA_V1: ResourceKey =
        resource_key!(uniset, "InSC=Avagraha", 1);
    pub const INDIC_SYLLABIC_CATEGORY_BINDU_V1: ResourceKey =
        resource_key!(uniset, "InSC=Bindu", 1);
    pub const INDIC_SYLLABIC_CATEGORY_BRAHMI_JOINING_NUMBER_V1: ResourceKey =
        resource_key!(uniset, "InSC=BJoinNum", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CANTILLATION_MARK_V1: ResourceKey =
        resource_key!(uniset, "InSC=CantillMark", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_V1: ResourceKey =
        resource_key!(uniset, "InSC=Consonant", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_DEAD_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsDead", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_FINAL_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsFinal", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_HEAD_LETTER_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsHeadLet", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_INITIAL_POSTFIXED_V1: ResourceKey =
        resource_key!(uniset, "InSC=CInitPost", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_KILLER_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsKiller", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_MEDIAL_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsMedial", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_PLACEHOLDER_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsPholder", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_PRECEDING_REPHA_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsPreReph", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_PREFIXED_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsPre", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_SUBJOINED_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsSubjoin", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_SUCCEEDING_REPHA_V1: ResourceKey =
        resource_key!(uniset, "InSC=CSuccRepha", 1);
    pub const INDIC_SYLLABIC_CATEGORY_CONSONANT_WITH_STACKER_V1: ResourceKey =
        resource_key!(uniset, "InSC=ConsStacker", 1);
    pub const INDIC_SYLLABIC_CATEGORY_GEMINATION_MARK_V1: ResourceKey =
        resource_key!(uniset, "InSC=GeminMark", 1);
    pub const INDIC_SYLLABIC_CATEGORY_INVISIBLE_STACKER_V1: ResourceKey =
        resource_key!(uniset, "InSC=InvisStack", 1);
    pub const INDIC_SYLLABIC_CATEGORY_JOINER_V1: ResourceKey =
        resource_key!(uniset, "InSC=Joiner", 1);
    pub const INDIC_SYLLABIC_CATEGORY_MODIFYING_LETTER_V1: ResourceKey =
        resource_key!(uniset, "InSC=ModLetter", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NON_JOINER_V1: ResourceKey =
        resource_key!(uniset, "InSC=Non_Joiner", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NUKTA_V1: ResourceKey =
        resource_key!(uniset, "InSC=Nukta", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NUMBER_V1: ResourceKey =
        resource_key!(uniset, "InSC=Number", 1);
    pub const INDIC_SYLLABIC_CATEGORY_NUMBER_JOINER_V1: ResourceKey =
        resource_key!(uniset, "InSC=NumJoiner", 1);
    pub const INDIC_SYLLABIC_CATEGORY_OTHER_V1: ResourceKey =
        resource_key!(uniset, "InSC=Other", 1);
    pub const INDIC_SYLLABIC_CATEGORY_PURE_KILLER_V1: ResourceKey =
        resource_key!(uniset, "InSC=Pure_Killer", 1);
    pub const INDIC_SYLLABIC_CATEGORY_REGISTER_SHIFTER_V1: ResourceKey =
        resource_key!(uniset, "InSC=RegistShift", 1);
    pub const INDIC_SYLLABIC_CATEGORY_SYLLABLE_MODIFIER_V1: ResourceKey =
        resource_key!(uniset, "InSC=SyllMod", 1);
    pub const INDIC_SYLLABIC_CATEGORY_TONE_LETTER_V1: ResourceKey =
        resource_key!(uniset, "InSC=Tone_Letter", 1);
    pub const INDIC_SYLLABIC_CATEGORY_TONE_MARK_V1: ResourceKey =
        resource_key!(uniset, "InSC=Tone_Mark", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VIRAMA_V1: ResourceKey =
        resource_key!(uniset, "InSC=Virama", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VISARGA_V1: ResourceKey =
        resource_key!(uniset, "InSC=Visarga", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VOWEL_V1: ResourceKey =
        resource_key!(uniset, "InSC=Vowel", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VOWEL_DEPENDENT_V1: ResourceKey =
        resource_key!(uniset, "InSC=VowelDep", 1);
    pub const INDIC_SYLLABIC_CATEGORY_VOWEL_INDEPENDENT_V1: ResourceKey =
        resource_key!(uniset, "InSC=VowelIndep", 1);
    pub const JOINING_GROUP_AFRICAN_FEH_V1: ResourceKey =
        resource_key!(uniset, "jg=African_Feh", 1);
    pub const JOINING_GROUP_AFRICAN_NOON_V1: ResourceKey =
        resource_key!(uniset, "jg=African_Noon", 1);
    pub const JOINING_GROUP_AFRICAN_QAF_V1: ResourceKey =
        resource_key!(uniset, "jg=African_Qaf", 1);
    pub const JOINING_GROUP_AIN_V1: ResourceKey = resource_key!(uniset, "jg=Ain", 1);
    pub const JOINING_GROUP_ALAPH_V1: ResourceKey = resource_key!(uniset, "jg=Alaph", 1);
    pub const JOINING_GROUP_ALEF_V1: ResourceKey = resource_key!(uniset, "jg=Alef", 1);
    pub const JOINING_GROUP_BEH_V1: ResourceKey = resource_key!(uniset, "jg=Beh", 1);
    pub const JOINING_GROUP_BETH_V1: ResourceKey = resource_key!(uniset, "jg=Beth", 1);
    pub const JOINING_GROUP_BURUSHASKI_YEH_BARREE_V1: ResourceKey =
        resource_key!(uniset, "jg=BurushYehBarr", 1);
    pub const JOINING_GROUP_DAL_V1: ResourceKey = resource_key!(uniset, "jg=Dal", 1);
    pub const JOINING_GROUP_DALATH_RISH_V1: ResourceKey =
        resource_key!(uniset, "jg=Dalath_Rish", 1);
    pub const JOINING_GROUP_E_V1: ResourceKey = resource_key!(uniset, "jg=E", 1);
    pub const JOINING_GROUP_FARSI_YEH_V1: ResourceKey = resource_key!(uniset, "jg=Farsi_Yeh", 1);
    pub const JOINING_GROUP_FE_V1: ResourceKey = resource_key!(uniset, "jg=Fe", 1);
    pub const JOINING_GROUP_FEH_V1: ResourceKey = resource_key!(uniset, "jg=Feh", 1);
    pub const JOINING_GROUP_FINAL_SEMKATH_V1: ResourceKey =
        resource_key!(uniset, "jg=Final_Semkath", 1);
    pub const JOINING_GROUP_GAF_V1: ResourceKey = resource_key!(uniset, "jg=Gaf", 1);
    pub const JOINING_GROUP_GAMAL_V1: ResourceKey = resource_key!(uniset, "jg=Gamal", 1);
    pub const JOINING_GROUP_HAH_V1: ResourceKey = resource_key!(uniset, "jg=Hah", 1);
    pub const JOINING_GROUP_HANIFI_ROHINGYA_KINNA_YA_V1: ResourceKey =
        resource_key!(uniset, "jg=HRohingKinYa", 1);
    pub const JOINING_GROUP_HANIFI_ROHINGYA_PA_V1: ResourceKey =
        resource_key!(uniset, "jg=HRohingPa", 1);
    pub const JOINING_GROUP_HE_V1: ResourceKey = resource_key!(uniset, "jg=He", 1);
    pub const JOINING_GROUP_HEH_V1: ResourceKey = resource_key!(uniset, "jg=Heh", 1);
    pub const JOINING_GROUP_HEH_GOAL_V1: ResourceKey = resource_key!(uniset, "jg=Heh_Goal", 1);
    pub const JOINING_GROUP_HETH_V1: ResourceKey = resource_key!(uniset, "jg=Heth", 1);
    pub const JOINING_GROUP_KAF_V1: ResourceKey = resource_key!(uniset, "jg=Kaf", 1);
    pub const JOINING_GROUP_KAPH_V1: ResourceKey = resource_key!(uniset, "jg=Kaph", 1);
    pub const JOINING_GROUP_KHAPH_V1: ResourceKey = resource_key!(uniset, "jg=Khaph", 1);
    pub const JOINING_GROUP_KNOTTED_HEH_V1: ResourceKey =
        resource_key!(uniset, "jg=Knotted_Heh", 1);
    pub const JOINING_GROUP_LAM_V1: ResourceKey = resource_key!(uniset, "jg=Lam", 1);
    pub const JOINING_GROUP_LAMADH_V1: ResourceKey = resource_key!(uniset, "jg=Lamadh", 1);
    pub const JOINING_GROUP_MALAYALAM_BHA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Bha", 1);
    pub const JOINING_GROUP_MALAYALAM_JA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Ja", 1);
    pub const JOINING_GROUP_MALAYALAM_LLA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Lla", 1);
    pub const JOINING_GROUP_MALAYALAM_LLLA_V1: ResourceKey = resource_key!(uniset, "jg=Ma_Llla", 1);
    pub const JOINING_GROUP_MALAYALAM_NGA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Nga", 1);
    pub const JOINING_GROUP_MALAYALAM_NNA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Nna", 1);
    pub const JOINING_GROUP_MALAYALAM_NNNA_V1: ResourceKey = resource_key!(uniset, "jg=Ma_Nnna", 1);
    pub const JOINING_GROUP_MALAYALAM_NYA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Nya", 1);
    pub const JOINING_GROUP_MALAYALAM_RA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Ra", 1);
    pub const JOINING_GROUP_MALAYALAM_SSA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Ssa", 1);
    pub const JOINING_GROUP_MALAYALAM_TTA_V1: ResourceKey =
        resource_key!(uniset, "jg=Malayalam_Tta", 1);
    pub const JOINING_GROUP_MANICHAEAN_ALEPH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Aleph", 1);
    pub const JOINING_GROUP_MANICHAEAN_AYIN_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Ayin", 1);
    pub const JOINING_GROUP_MANICHAEAN_BETH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Beth", 1);
    pub const JOINING_GROUP_MANICHAEAN_DALETH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Daleth", 1);
    pub const JOINING_GROUP_MANICHAEAN_DHAMEDH_V1: ResourceKey =
        resource_key!(uniset, "jg=ManichDhamedh", 1);
    pub const JOINING_GROUP_MANICHAEAN_FIVE_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Five", 1);
    pub const JOINING_GROUP_MANICHAEAN_GIMEL_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Gimel", 1);
    pub const JOINING_GROUP_MANICHAEAN_HETH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Heth", 1);
    pub const JOINING_GROUP_MANICHAEAN_HUNDRED_V1: ResourceKey =
        resource_key!(uniset, "jg=ManichHundred", 1);
    pub const JOINING_GROUP_MANICHAEAN_KAPH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Kaph", 1);
    pub const JOINING_GROUP_MANICHAEAN_LAMEDH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Lamedh", 1);
    pub const JOINING_GROUP_MANICHAEAN_MEM_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Mem", 1);
    pub const JOINING_GROUP_MANICHAEAN_NUN_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Nun", 1);
    pub const JOINING_GROUP_MANICHAEAN_ONE_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_One", 1);
    pub const JOINING_GROUP_MANICHAEAN_PE_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Pe", 1);
    pub const JOINING_GROUP_MANICHAEAN_QOPH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Qoph", 1);
    pub const JOINING_GROUP_MANICHAEAN_RESH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Resh", 1);
    pub const JOINING_GROUP_MANICHAEAN_SADHE_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Sadhe", 1);
    pub const JOINING_GROUP_MANICHAEAN_SAMEKH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Samekh", 1);
    pub const JOINING_GROUP_MANICHAEAN_TAW_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Taw", 1);
    pub const JOINING_GROUP_MANICHAEAN_TEN_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Ten", 1);
    pub const JOINING_GROUP_MANICHAEAN_TETH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Teth", 1);
    pub const JOINING_GROUP_MANICHAEAN_THAMEDH_V1: ResourceKey =
        resource_key!(uniset, "jg=ManichThamedh", 1);
    pub const JOINING_GROUP_MANICHAEAN_TWENTY_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Twenty", 1);
    pub const JOINING_GROUP_MANICHAEAN_WAW_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Waw", 1);
    pub const JOINING_GROUP_MANICHAEAN_YODH_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Yodh", 1);
    pub const JOINING_GROUP_MANICHAEAN_ZAYIN_V1: ResourceKey =
        resource_key!(uniset, "jg=Manich_Zayin", 1);
    pub const JOINING_GROUP_MEEM_V1: ResourceKey = resource_key!(uniset, "jg=Meem", 1);
    pub const JOINING_GROUP_MIM_V1: ResourceKey = resource_key!(uniset, "jg=Mim", 1);
    pub const JOINING_GROUP_NO_JOINING_GROUP_V1: ResourceKey =
        resource_key!(uniset, "jg=NoJoinGroup", 1);
    pub const JOINING_GROUP_NOON_V1: ResourceKey = resource_key!(uniset, "jg=Noon", 1);
    pub const JOINING_GROUP_NUN_V1: ResourceKey = resource_key!(uniset, "jg=Nun", 1);
    pub const JOINING_GROUP_NYA_V1: ResourceKey = resource_key!(uniset, "jg=Nya", 1);
    pub const JOINING_GROUP_PE_V1: ResourceKey = resource_key!(uniset, "jg=Pe", 1);
    pub const JOINING_GROUP_QAF_V1: ResourceKey = resource_key!(uniset, "jg=Qaf", 1);
    pub const JOINING_GROUP_QAPH_V1: ResourceKey = resource_key!(uniset, "jg=Qaph", 1);
    pub const JOINING_GROUP_REH_V1: ResourceKey = resource_key!(uniset, "jg=Reh", 1);
    pub const JOINING_GROUP_REVERSED_PE_V1: ResourceKey =
        resource_key!(uniset, "jg=Reversed_Pe", 1);
    pub const JOINING_GROUP_ROHINGYA_YEH_V1: ResourceKey =
        resource_key!(uniset, "jg=Rohingya_Yeh", 1);
    pub const JOINING_GROUP_SAD_V1: ResourceKey = resource_key!(uniset, "jg=Sad", 1);
    pub const JOINING_GROUP_SADHE_V1: ResourceKey = resource_key!(uniset, "jg=Sadhe", 1);
    pub const JOINING_GROUP_SEEN_V1: ResourceKey = resource_key!(uniset, "jg=Seen", 1);
    pub const JOINING_GROUP_SEMKATH_V1: ResourceKey = resource_key!(uniset, "jg=Semkath", 1);
    pub const JOINING_GROUP_SHIN_V1: ResourceKey = resource_key!(uniset, "jg=Shin", 1);
    pub const JOINING_GROUP_STRAIGHT_WAW_V1: ResourceKey =
        resource_key!(uniset, "jg=Straight_Waw", 1);
    pub const JOINING_GROUP_SWASH_KAF_V1: ResourceKey = resource_key!(uniset, "jg=Swash_Kaf", 1);
    pub const JOINING_GROUP_SYRIAC_WAW_V1: ResourceKey = resource_key!(uniset, "jg=Syriac_Waw", 1);
    pub const JOINING_GROUP_TAH_V1: ResourceKey = resource_key!(uniset, "jg=Tah", 1);
    pub const JOINING_GROUP_TAW_V1: ResourceKey = resource_key!(uniset, "jg=Taw", 1);
    pub const JOINING_GROUP_TEH_MARBUTA_V1: ResourceKey =
        resource_key!(uniset, "jg=Teh_Marbuta", 1);
    pub const JOINING_GROUP_HAMZA_ON_HEH_GOAL_V1: ResourceKey =
        resource_key!(uniset, "jg=TehMarbGoal", 1);
    pub const JOINING_GROUP_TETH_V1: ResourceKey = resource_key!(uniset, "jg=Teth", 1);
    pub const JOINING_GROUP_WAW_V1: ResourceKey = resource_key!(uniset, "jg=Waw", 1);
    pub const JOINING_GROUP_YEH_V1: ResourceKey = resource_key!(uniset, "jg=Yeh", 1);
    pub const JOINING_GROUP_YEH_BARREE_V1: ResourceKey = resource_key!(uniset, "jg=Yeh_Barree", 1);
    pub const JOINING_GROUP_YEH_WITH_TAIL_V1: ResourceKey =
        resource_key!(uniset, "jg=Yeh_With_Tail", 1);
    pub const JOINING_GROUP_YUDH_V1: ResourceKey = resource_key!(uniset, "jg=Yudh", 1);
    pub const JOINING_GROUP_YUDH_HE_V1: ResourceKey = resource_key!(uniset, "jg=Yudh_He", 1);
    pub const JOINING_GROUP_ZAIN_V1: ResourceKey = resource_key!(uniset, "jg=Zain", 1);
    pub const JOINING_GROUP_ZHAIN_V1: ResourceKey = resource_key!(uniset, "jg=Zhain", 1);
    pub const JOINING_TYPE_JOIN_CAUSING_V1: ResourceKey = resource_key!(uniset, "jt=C", 1);
    pub const JOINING_TYPE_DUAL_JOINING_V1: ResourceKey = resource_key!(uniset, "jt=D", 1);
    pub const JOINING_TYPE_LEFT_JOINING_V1: ResourceKey = resource_key!(uniset, "jt=L", 1);
    pub const JOINING_TYPE_RIGHT_JOINING_V1: ResourceKey = resource_key!(uniset, "jt=R", 1);
    pub const JOINING_TYPE_TRANSPARENT_V1: ResourceKey = resource_key!(uniset, "jt=T", 1);
    pub const JOINING_TYPE_NON_JOINING_V1: ResourceKey = resource_key!(uniset, "jt=U", 1);
    pub const LINE_BREAK_AMBIGUOUS_V1: ResourceKey = resource_key!(uniset, "lb=AI", 1);
    pub const LINE_BREAK_ALPHABETIC_V1: ResourceKey = resource_key!(uniset, "lb=AL", 1);
    pub const LINE_BREAK_BREAK_BOTH_V1: ResourceKey = resource_key!(uniset, "lb=B2", 1);
    pub const LINE_BREAK_BREAK_AFTER_V1: ResourceKey = resource_key!(uniset, "lb=BA", 1);
    pub const LINE_BREAK_BREAK_BEFORE_V1: ResourceKey = resource_key!(uniset, "lb=BB", 1);
    pub const LINE_BREAK_MANDATORY_BREAK_V1: ResourceKey = resource_key!(uniset, "lb=BK", 1);
    pub const LINE_BREAK_CONTINGENT_BREAK_V1: ResourceKey = resource_key!(uniset, "lb=CB", 1);
    pub const LINE_BREAK_CONDITIONAL_JAPANESE_STARTER_V1: ResourceKey =
        resource_key!(uniset, "lb=CJ", 1);
    pub const LINE_BREAK_CLOSE_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "lb=CL", 1);
    pub const LINE_BREAK_COMBINING_MARK_V1: ResourceKey = resource_key!(uniset, "lb=CM", 1);
    pub const LINE_BREAK_CLOSE_PARENTHESIS_V1: ResourceKey = resource_key!(uniset, "lb=CP", 1);
    pub const LINE_BREAK_CARRIAGE_RETURN_V1: ResourceKey = resource_key!(uniset, "lb=CR", 1);
    pub const LINE_BREAK_E_BASE_V1: ResourceKey = resource_key!(uniset, "lb=EB", 1);
    pub const LINE_BREAK_E_MODIFIER_V1: ResourceKey = resource_key!(uniset, "lb=EM", 1);
    pub const LINE_BREAK_EXCLAMATION_V1: ResourceKey = resource_key!(uniset, "lb=EX", 1);
    pub const LINE_BREAK_GLUE_V1: ResourceKey = resource_key!(uniset, "lb=GL", 1);
    pub const LINE_BREAK_H2_V1: ResourceKey = resource_key!(uniset, "lb=H2", 1);
    pub const LINE_BREAK_H3_V1: ResourceKey = resource_key!(uniset, "lb=H3", 1);
    pub const LINE_BREAK_HEBREW_LETTER_V1: ResourceKey = resource_key!(uniset, "lb=HL", 1);
    pub const LINE_BREAK_HYPHEN_V1: ResourceKey = resource_key!(uniset, "lb=HY", 1);
    pub const LINE_BREAK_IDEOGRAPHIC_V1: ResourceKey = resource_key!(uniset, "lb=ID", 1);
    pub const LINE_BREAK_INSEPERABLE_V1: ResourceKey = resource_key!(uniset, "lb=IN", 1);
    pub const LINE_BREAK_INFIX_NUMERIC_V1: ResourceKey = resource_key!(uniset, "lb=IS", 1);
    pub const LINE_BREAK_JL_V1: ResourceKey = resource_key!(uniset, "lb=JL", 1);
    pub const LINE_BREAK_JT_V1: ResourceKey = resource_key!(uniset, "lb=JT", 1);
    pub const LINE_BREAK_JV_V1: ResourceKey = resource_key!(uniset, "lb=JV", 1);
    pub const LINE_BREAK_LINE_FEED_V1: ResourceKey = resource_key!(uniset, "lb=LF", 1);
    pub const LINE_BREAK_NEXT_LINE_V1: ResourceKey = resource_key!(uniset, "lb=NL", 1);
    pub const LINE_BREAK_NONSTARTER_V1: ResourceKey = resource_key!(uniset, "lb=NS", 1);
    pub const LINE_BREAK_NUMERIC_V1: ResourceKey = resource_key!(uniset, "lb=NU", 1);
    pub const LINE_BREAK_OPEN_PUNCTUATION_V1: ResourceKey = resource_key!(uniset, "lb=OP", 1);
    pub const LINE_BREAK_POSTFIX_NUMERIC_V1: ResourceKey = resource_key!(uniset, "lb=PO", 1);
    pub const LINE_BREAK_PREFIX_NUMERIC_V1: ResourceKey = resource_key!(uniset, "lb=PR", 1);
    pub const LINE_BREAK_QUOTATION_V1: ResourceKey = resource_key!(uniset, "lb=QU", 1);
    pub const LINE_BREAK_REGIONAL_INDICATOR_V1: ResourceKey = resource_key!(uniset, "lb=RI", 1);
    pub const LINE_BREAK_COMPLEX_CONTEXT_V1: ResourceKey = resource_key!(uniset, "lb=SA", 1);
    pub const LINE_BREAK_SURROGATE_V1: ResourceKey = resource_key!(uniset, "lb=SG", 1);
    pub const LINE_BREAK_SPACE_V1: ResourceKey = resource_key!(uniset, "lb=SP", 1);
    pub const LINE_BREAK_BREAK_SYMBOLS_V1: ResourceKey = resource_key!(uniset, "lb=SY", 1);
    pub const LINE_BREAK_WORD_JOINER_V1: ResourceKey = resource_key!(uniset, "lb=WJ", 1);
    pub const LINE_BREAK_UNKNOWN_V1: ResourceKey = resource_key!(uniset, "lb=XX", 1);
    pub const LINE_BREAK_ZWSPACE_V1: ResourceKey = resource_key!(uniset, "lb=ZW", 1);
    pub const LINE_BREAK_ZWJ_V1: ResourceKey = resource_key!(uniset, "lb=ZWJ", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1: ResourceKey =
        resource_key!(uniset, "lccc=0", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_OVERLAY_V1: ResourceKey =
        resource_key!(uniset, "lccc=1", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC10_V1: ResourceKey =
        resource_key!(uniset, "lccc=10", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC103_V1: ResourceKey =
        resource_key!(uniset, "lccc=103", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC107_V1: ResourceKey =
        resource_key!(uniset, "lccc=107", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC11_V1: ResourceKey =
        resource_key!(uniset, "lccc=11", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC118_V1: ResourceKey =
        resource_key!(uniset, "lccc=118", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC12_V1: ResourceKey =
        resource_key!(uniset, "lccc=12", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC122_V1: ResourceKey =
        resource_key!(uniset, "lccc=122", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC129_V1: ResourceKey =
        resource_key!(uniset, "lccc=129", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC13_V1: ResourceKey =
        resource_key!(uniset, "lccc=13", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC130_V1: ResourceKey =
        resource_key!(uniset, "lccc=130", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC132_V1: ResourceKey =
        resource_key!(uniset, "lccc=132", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC133_V1: ResourceKey =
        resource_key!(uniset, "lccc=133", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC14_V1: ResourceKey =
        resource_key!(uniset, "lccc=14", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC15_V1: ResourceKey =
        resource_key!(uniset, "lccc=15", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC16_V1: ResourceKey =
        resource_key!(uniset, "lccc=16", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC17_V1: ResourceKey =
        resource_key!(uniset, "lccc=17", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC18_V1: ResourceKey =
        resource_key!(uniset, "lccc=18", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC19_V1: ResourceKey =
        resource_key!(uniset, "lccc=19", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC20_V1: ResourceKey =
        resource_key!(uniset, "lccc=20", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "lccc=200", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1: ResourceKey =
        resource_key!(uniset, "lccc=202", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC21_V1: ResourceKey =
        resource_key!(uniset, "lccc=21", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "lccc=214", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "lccc=216", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "lccc=218", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC22_V1: ResourceKey =
        resource_key!(uniset, "lccc=22", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_BELOW_V1: ResourceKey =
        resource_key!(uniset, "lccc=220", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "lccc=222", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_LEFT_V1: ResourceKey =
        resource_key!(uniset, "lccc=224", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "lccc=226", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1: ResourceKey =
        resource_key!(uniset, "lccc=228", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC23_V1: ResourceKey =
        resource_key!(uniset, "lccc=23", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "lccc=230", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "lccc=232", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1: ResourceKey =
        resource_key!(uniset, "lccc=233", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "lccc=234", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC24_V1: ResourceKey =
        resource_key!(uniset, "lccc=24", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1: ResourceKey =
        resource_key!(uniset, "lccc=240", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC25_V1: ResourceKey =
        resource_key!(uniset, "lccc=25", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC26_V1: ResourceKey =
        resource_key!(uniset, "lccc=26", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC27_V1: ResourceKey =
        resource_key!(uniset, "lccc=27", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC28_V1: ResourceKey =
        resource_key!(uniset, "lccc=28", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC29_V1: ResourceKey =
        resource_key!(uniset, "lccc=29", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC30_V1: ResourceKey =
        resource_key!(uniset, "lccc=30", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC31_V1: ResourceKey =
        resource_key!(uniset, "lccc=31", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC32_V1: ResourceKey =
        resource_key!(uniset, "lccc=32", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC33_V1: ResourceKey =
        resource_key!(uniset, "lccc=33", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC34_V1: ResourceKey =
        resource_key!(uniset, "lccc=34", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC35_V1: ResourceKey =
        resource_key!(uniset, "lccc=35", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC36_V1: ResourceKey =
        resource_key!(uniset, "lccc=36", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_HAN_READING_V1: ResourceKey =
        resource_key!(uniset, "lccc=6", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_NUKTA_V1: ResourceKey =
        resource_key!(uniset, "lccc=7", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1: ResourceKey =
        resource_key!(uniset, "lccc=8", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC84_V1: ResourceKey =
        resource_key!(uniset, "lccc=84", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_VIRAMA_V1: ResourceKey =
        resource_key!(uniset, "lccc=9", 1);
    pub const LEAD_CANONICAL_COMBINING_CLASS_CCC91_V1: ResourceKey =
        resource_key!(uniset, "lccc=91", 1);
    pub const NFC_QUICK_CHECK_MAYBE_V1: ResourceKey = resource_key!(uniset, "NFC_QC=M", 1);
    pub const NFC_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "NFC_QC=N", 1);
    pub const NFC_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "NFC_QC=Y", 1);
    pub const NFD_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "NFD_QC=N", 1);
    pub const NFD_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "NFD_QC=Y", 1);
    pub const NFKC_QUICK_CHECK_MAYBE_V1: ResourceKey = resource_key!(uniset, "NFKC_QC=M", 1);
    pub const NFKC_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "NFKC_QC=N", 1);
    pub const NFKC_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "NFKC_QC=Y", 1);
    pub const NFKD_QUICK_CHECK_NO_V1: ResourceKey = resource_key!(uniset, "NFKD_QC=N", 1);
    pub const NFKD_QUICK_CHECK_YES_V1: ResourceKey = resource_key!(uniset, "NFKD_QC=Y", 1);
    pub const NUMERIC_TYPE_DECIMAL_V1: ResourceKey = resource_key!(uniset, "nt=De", 1);
    pub const NUMERIC_TYPE_DIGIT_V1: ResourceKey = resource_key!(uniset, "nt=Di", 1);
    pub const NUMERIC_TYPE_NONE_V1: ResourceKey = resource_key!(uniset, "nt=None", 1);
    pub const NUMERIC_TYPE_NUMERIC_V1: ResourceKey = resource_key!(uniset, "nt=Nu", 1);
    pub const SENTENCE_BREAK_ATERM_V1: ResourceKey = resource_key!(uniset, "SB=AT", 1);
    pub const SENTENCE_BREAK_CLOSE_V1: ResourceKey = resource_key!(uniset, "SB=CL", 1);
    pub const SENTENCE_BREAK_CR_V1: ResourceKey = resource_key!(uniset, "SB=CR", 1);
    pub const SENTENCE_BREAK_EXTEND_V1: ResourceKey = resource_key!(uniset, "SB=EX", 1);
    pub const SENTENCE_BREAK_FORMAT_V1: ResourceKey = resource_key!(uniset, "SB=FO", 1);
    pub const SENTENCE_BREAK_OLETTER_V1: ResourceKey = resource_key!(uniset, "SB=LE", 1);
    pub const SENTENCE_BREAK_LF_V1: ResourceKey = resource_key!(uniset, "SB=LF", 1);
    pub const SENTENCE_BREAK_LOWER_V1: ResourceKey = resource_key!(uniset, "SB=LO", 1);
    pub const SENTENCE_BREAK_NUMERIC_V1: ResourceKey = resource_key!(uniset, "SB=NU", 1);
    pub const SENTENCE_BREAK_SCONTINUE_V1: ResourceKey = resource_key!(uniset, "SB=SC", 1);
    pub const SENTENCE_BREAK_SEP_V1: ResourceKey = resource_key!(uniset, "SB=SE", 1);
    pub const SENTENCE_BREAK_SP_V1: ResourceKey = resource_key!(uniset, "SB=SP", 1);
    pub const SENTENCE_BREAK_STERM_V1: ResourceKey = resource_key!(uniset, "SB=ST", 1);
    pub const SENTENCE_BREAK_UPPER_V1: ResourceKey = resource_key!(uniset, "SB=UP", 1);
    pub const SENTENCE_BREAK_OTHER_V1: ResourceKey = resource_key!(uniset, "SB=XX", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1: ResourceKey =
        resource_key!(uniset, "tccc=0", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_OVERLAY_V1: ResourceKey =
        resource_key!(uniset, "tccc=1", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC10_V1: ResourceKey =
        resource_key!(uniset, "tccc=10", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC103_V1: ResourceKey =
        resource_key!(uniset, "tccc=103", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC107_V1: ResourceKey =
        resource_key!(uniset, "tccc=107", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC11_V1: ResourceKey =
        resource_key!(uniset, "tccc=11", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC118_V1: ResourceKey =
        resource_key!(uniset, "tccc=118", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC12_V1: ResourceKey =
        resource_key!(uniset, "tccc=12", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC122_V1: ResourceKey =
        resource_key!(uniset, "tccc=122", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC129_V1: ResourceKey =
        resource_key!(uniset, "tccc=129", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC13_V1: ResourceKey =
        resource_key!(uniset, "tccc=13", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC130_V1: ResourceKey =
        resource_key!(uniset, "tccc=130", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC132_V1: ResourceKey =
        resource_key!(uniset, "tccc=132", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC133_V1: ResourceKey =
        resource_key!(uniset, "tccc=133", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC14_V1: ResourceKey =
        resource_key!(uniset, "tccc=14", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC15_V1: ResourceKey =
        resource_key!(uniset, "tccc=15", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC16_V1: ResourceKey =
        resource_key!(uniset, "tccc=16", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC17_V1: ResourceKey =
        resource_key!(uniset, "tccc=17", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC18_V1: ResourceKey =
        resource_key!(uniset, "tccc=18", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC19_V1: ResourceKey =
        resource_key!(uniset, "tccc=19", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC20_V1: ResourceKey =
        resource_key!(uniset, "tccc=20", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "tccc=200", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1: ResourceKey =
        resource_key!(uniset, "tccc=202", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC21_V1: ResourceKey =
        resource_key!(uniset, "tccc=21", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "tccc=214", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "tccc=216", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1: ResourceKey =
        resource_key!(uniset, "tccc=218", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC22_V1: ResourceKey =
        resource_key!(uniset, "tccc=22", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_BELOW_V1: ResourceKey =
        resource_key!(uniset, "tccc=220", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "tccc=222", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_LEFT_V1: ResourceKey =
        resource_key!(uniset, "tccc=224", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "tccc=226", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1: ResourceKey =
        resource_key!(uniset, "tccc=228", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC23_V1: ResourceKey =
        resource_key!(uniset, "tccc=23", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "tccc=230", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1: ResourceKey =
        resource_key!(uniset, "tccc=232", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1: ResourceKey =
        resource_key!(uniset, "tccc=233", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1: ResourceKey =
        resource_key!(uniset, "tccc=234", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC24_V1: ResourceKey =
        resource_key!(uniset, "tccc=24", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1: ResourceKey =
        resource_key!(uniset, "tccc=240", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC25_V1: ResourceKey =
        resource_key!(uniset, "tccc=25", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC26_V1: ResourceKey =
        resource_key!(uniset, "tccc=26", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC27_V1: ResourceKey =
        resource_key!(uniset, "tccc=27", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC28_V1: ResourceKey =
        resource_key!(uniset, "tccc=28", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC29_V1: ResourceKey =
        resource_key!(uniset, "tccc=29", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC30_V1: ResourceKey =
        resource_key!(uniset, "tccc=30", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC31_V1: ResourceKey =
        resource_key!(uniset, "tccc=31", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC32_V1: ResourceKey =
        resource_key!(uniset, "tccc=32", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC33_V1: ResourceKey =
        resource_key!(uniset, "tccc=33", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC34_V1: ResourceKey =
        resource_key!(uniset, "tccc=34", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC35_V1: ResourceKey =
        resource_key!(uniset, "tccc=35", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC36_V1: ResourceKey =
        resource_key!(uniset, "tccc=36", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_HAN_READING_V1: ResourceKey =
        resource_key!(uniset, "tccc=6", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_NUKTA_V1: ResourceKey =
        resource_key!(uniset, "tccc=7", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1: ResourceKey =
        resource_key!(uniset, "tccc=8", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC84_V1: ResourceKey =
        resource_key!(uniset, "tccc=84", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_VIRAMA_V1: ResourceKey =
        resource_key!(uniset, "tccc=9", 1);
    pub const TRAIL_CANONICAL_COMBINING_CLASS_CCC91_V1: ResourceKey =
        resource_key!(uniset, "tccc=91", 1);
    pub const VERTICAL_ORIENTATION_ROTATED_V1: ResourceKey = resource_key!(uniset, "vo=R", 1);
    pub const VERTICAL_ORIENTATION_TRANSFORMED_ROTATED_V1: ResourceKey =
        resource_key!(uniset, "vo=Tr", 1);
    pub const VERTICAL_ORIENTATION_TRANSFORMED_UPRIGHT_V1: ResourceKey =
        resource_key!(uniset, "vo=Tu", 1);
    pub const VERTICAL_ORIENTATION_UPRIGHT_V1: ResourceKey = resource_key!(uniset, "vo=U", 1);
    pub const WORD_BREAK_CR_V1: ResourceKey = resource_key!(uniset, "WB=CR", 1);
    pub const WORD_BREAK_DOUBLE_QUOTE_V1: ResourceKey = resource_key!(uniset, "WB=DQ", 1);
    pub const WORD_BREAK_E_BASE_V1: ResourceKey = resource_key!(uniset, "WB=EB", 1);
    pub const WORD_BREAK_E_BASE_GAZ_V1: ResourceKey = resource_key!(uniset, "WB=EBG", 1);
    pub const WORD_BREAK_E_MODIFIER_V1: ResourceKey = resource_key!(uniset, "WB=EM", 1);
    pub const WORD_BREAK_EXTENDNUMLET_V1: ResourceKey = resource_key!(uniset, "WB=EX", 1);
    pub const WORD_BREAK_EXTEND_V1: ResourceKey = resource_key!(uniset, "WB=Extend", 1);
    pub const WORD_BREAK_FORMAT_V1: ResourceKey = resource_key!(uniset, "WB=FO", 1);
    pub const WORD_BREAK_GLUE_AFTER_ZWJ_V1: ResourceKey = resource_key!(uniset, "WB=GAZ", 1);
    pub const WORD_BREAK_HEBREW_LETTER_V1: ResourceKey = resource_key!(uniset, "WB=HL", 1);
    pub const WORD_BREAK_KATAKANA_V1: ResourceKey = resource_key!(uniset, "WB=KA", 1);
    pub const WORD_BREAK_ALETTER_V1: ResourceKey = resource_key!(uniset, "WB=LE", 1);
    pub const WORD_BREAK_LF_V1: ResourceKey = resource_key!(uniset, "WB=LF", 1);
    pub const WORD_BREAK_MIDNUMLET_V1: ResourceKey = resource_key!(uniset, "WB=MB", 1);
    pub const WORD_BREAK_MIDLETTER_V1: ResourceKey = resource_key!(uniset, "WB=ML", 1);
    pub const WORD_BREAK_MIDNUM_V1: ResourceKey = resource_key!(uniset, "WB=MN", 1);
    pub const WORD_BREAK_NEWLINE_V1: ResourceKey = resource_key!(uniset, "WB=NL", 1);
    pub const WORD_BREAK_NUMERIC_V1: ResourceKey = resource_key!(uniset, "WB=NU", 1);
    pub const WORD_BREAK_REGIONAL_INDICATOR_V1: ResourceKey = resource_key!(uniset, "WB=RI", 1);
    pub const WORD_BREAK_SINGLE_QUOTE_V1: ResourceKey = resource_key!(uniset, "WB=SQ", 1);
    pub const WORD_BREAK_WSEGSPACE_V1: ResourceKey = resource_key!(uniset, "WB=WSegSpace", 1);
    pub const WORD_BREAK_OTHER_V1: ResourceKey = resource_key!(uniset, "WB=XX", 1);
    pub const WORD_BREAK_ZWJ_V1: ResourceKey = resource_key!(uniset, "WB=ZWJ", 1);
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "testing",
    derive(Hash, Eq)
)]
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
