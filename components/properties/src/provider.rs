// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::script::ScriptExtensions;
use icu_codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::yoke::{self, *};
use icu_uniset::UnicodeSet;
use icu_uniset::UnicodeSetBuilder;

//
// resource key structs - the structs used directly by users of data provider
//

pub mod key {
    //! Resource keys for [`icu_uniset`](crate)
    use icu_provider::resource_key;
    use icu_provider::ResourceKey;

    /// Macro to help define resource keys and store them in a list.
    macro_rules! define_resource_keys {
        ($allkeys:ident; $count:expr; $(($k:ident, $s:literal)),+,) => {
            $(
                #[allow(missing_docs)] // These constants don't need individual documentation.
                pub const $k: ResourceKey = resource_key!(concat!("props/", $s, "@1"));
            )+

            /// The set of all resource keys supported by [`icu_uniset`](crate).
            pub const $allkeys: [ResourceKey; $count] = [$($k,)+];
        };
    }

    define_resource_keys!(ALL_SET_KEYS; 265;
        //
        // Binary property UnicodeSets
        //

        (ASCII_HEX_DIGIT_V1, "AHex"),
        (ALNUM_V1, "alnum"),
        (ALPHABETIC_V1, "Alpha"),
        (BIDI_CONTROL_V1, "Bidi_C"),
        (BIDI_MIRRORED_V1, "Bidi_M"),
        (BLANK_V1, "blank"),
        (CASED_V1, "Cased"),
        (CASE_IGNORABLE_V1, "CI"),
        (FULL_COMPOSITION_EXCLUSION_V1, "Comp_Ex"),
        (CHANGES_WHEN_CASEFOLDED_V1, "CWCF"),
        (CHANGES_WHEN_CASEMAPPED_V1, "CWCM"),
        (CHANGES_WHEN_NFKC_CASEFOLDED_V1, "CWKCF"),
        (CHANGES_WHEN_LOWERCASED_V1, "CWL"),
        (CHANGES_WHEN_TITLECASED_V1, "CWT"),
        (CHANGES_WHEN_UPPERCASED_V1, "CWU"),
        (DASH_V1, "Dash"),
        (DEPRECATED_V1, "Dep"),
        (DEFAULT_IGNORABLE_CODE_POINT_V1, "DI"),
        (DIACRITIC_V1, "Dia"),
        (EMOJI_MODIFIER_BASE_V1, "EBase"),
        (EMOJI_COMPONENT_V1, "EComp"),
        (EMOJI_MODIFIER_V1, "EMod"),
        (EMOJI_V1, "Emoji"),
        (EMOJI_PRESENTATION_V1, "EPres"),
        (EXTENDER_V1, "Ext"),
        (EXTENDED_PICTOGRAPHIC_V1, "ExtPict"),
        (GRAPH_V1, "graph"),
        (GRAPHEME_BASE_V1, "Gr_Base"),
        (GRAPHEME_EXTEND_V1, "Gr_Ext"),
        (GRAPHEME_LINK_V1, "Gr_Link"),
        (HEX_DIGIT_V1, "Hex"),
        (HYPHEN_V1, "Hyphen"),
        (ID_CONTINUE_V1, "IDC"),
        (IDEOGRAPHIC_V1, "Ideo"),
        (ID_START_V1, "IDS"),
        (IDS_BINARY_OPERATOR_V1, "IDSB"),
        (IDS_TRINARY_OPERATOR_V1, "IDST"),
        (JOIN_CONTROL_V1, "Join_C"),
        (LOGICAL_ORDER_EXCEPTION_V1, "LOE"),
        (LOWERCASE_V1, "Lower"),
        (MATH_V1, "Math"),
        (NONCHARACTER_CODE_POINT_V1, "NChar"),
        (NFC_INERT_V1, "nfcinert"),
        (NFD_INERT_V1, "nfdinert"),
        (NFKC_INERT_V1, "nfkcinert"),
        (NFKD_INERT_V1, "nfkdinert"),
        (PATTERN_SYNTAX_V1, "Pat_Syn"),
        (PATTERN_WHITE_SPACE_V1, "Pat_WS"),
        (PREPENDED_CONCATENATION_MARK_V1, "PCM"),
        (PRINT_V1, "print"),
        (QUOTATION_MARK_V1, "QMark"),
        (RADICAL_V1, "Radical"),
        (REGIONAL_INDICATOR_V1, "RI"),
        (SOFT_DOTTED_V1, "SD"),
        (SEGMENT_STARTER_V1, "segstart"),
        (CASE_SENSITIVE_V1, "Sensitive"),
        (SENTENCE_TERMINAL_V1, "STerm"),
        (TERMINAL_PUNCTUATION_V1, "Term"),
        (UNIFIED_IDEOGRAPH_V1, "UIdeo"),
        (UPPERCASE_V1, "Upper"),
        (VARIATION_SELECTOR_V1, "VS"),
        (WHITE_SPACE_V1, "WSpace"),
        (XDIGIT_V1, "xdigit"),
        (XID_CONTINUE_V1, "XIDC"),
        (XID_START_V1, "XIDS"),

        //
        // Enumerated property prop=val UnicodeSets
        //

        // Note: The ResourceKey subcategory strings are determined from the
        // short name of the enumerated property and the short name of the
        // property value.

        (GENERAL_CATEGORY_OTHER_V1, "gc=C"),
        (GENERAL_CATEGORY_CONTROL_V1, "gc=Cc"),
        (GENERAL_CATEGORY_FORMAT_V1, "gc=Cf"),
        (GENERAL_CATEGORY_UNASSIGNED_V1, "gc=Cn"),
        (GENERAL_CATEGORY_PRIVATE_USE_V1, "gc=Co"),
        (GENERAL_CATEGORY_SURROGATE_V1, "gc=Cs"),
        (GENERAL_CATEGORY_LETTER_V1, "gc=L"),
        (GENERAL_CATEGORY_CASED_LETTER_V1, "gc=LC"),
        (GENERAL_CATEGORY_LOWERCASE_LETTER_V1, "gc=Ll"),
        (GENERAL_CATEGORY_MODIFIER_LETTER_V1, "gc=Lm"),
        (GENERAL_CATEGORY_OTHER_LETTER_V1, "gc=Lo"),
        (GENERAL_CATEGORY_TITLECASE_LETTER_V1, "gc=Lt"),
        (GENERAL_CATEGORY_UPPERCASE_LETTER_V1, "gc=Lu"),
        (GENERAL_CATEGORY_MARK_V1, "gc=M"),
        (GENERAL_CATEGORY_SPACING_MARK_V1, "gc=Mc"),
        (GENERAL_CATEGORY_ENCLOSING_MARK_V1, "gc=Me"),
        (GENERAL_CATEGORY_NONSPACING_MARK_V1, "gc=Mn"),
        (GENERAL_CATEGORY_NUMBER_V1, "gc=N"),
        (GENERAL_CATEGORY_DIGIT_V1, "gc=Nd"),
        (GENERAL_CATEGORY_LETTER_NUMBER_V1, "gc=Nl"),
        (GENERAL_CATEGORY_OTHER_NUMBER_V1, "gc=No"),
        (GENERAL_CATEGORY_PUNCTUATION_V1, "gc=P"),
        (GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1, "gc=Pc"),
        (GENERAL_CATEGORY_DASH_PUNCTUATION_V1, "gc=Pd"),
        (GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1, "gc=Pe"),
        (GENERAL_CATEGORY_FINAL_PUNCTUATION_V1, "gc=Pf"),
        (GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1, "gc=Pi"),
        (GENERAL_CATEGORY_OTHER_PUNCTUATION_V1, "gc=Po"),
        (GENERAL_CATEGORY_OPEN_PUNCTUATION_V1, "gc=Ps"),
        (GENERAL_CATEGORY_SYMBOL_V1, "gc=S"),
        (GENERAL_CATEGORY_CURRENCY_SYMBOL_V1, "gc=Sc"),
        (GENERAL_CATEGORY_MODIFIER_SYMBOL_V1, "gc=Sk"),
        (GENERAL_CATEGORY_MATH_SYMBOL_V1, "gc=Sm"),
        (GENERAL_CATEGORY_OTHER_SYMBOL_V1, "gc=So"),
        (GENERAL_CATEGORY_SEPARATOR_V1, "gc=Z"),
        (GENERAL_CATEGORY_LINE_SEPARATOR_V1, "gc=Zl"),
        (GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1, "gc=Zp"),
        (GENERAL_CATEGORY_SPACE_SEPARATOR_V1, "gc=Zs"),
        (SCRIPT_ADLAM_V1, "sc=Adlm"),
        (SCRIPT_AHOM_V1, "sc=Ahom"),
        (SCRIPT_ANATOLIAN_HIEROGLYPHS_V1, "sc=Hluw"),
        (SCRIPT_ARABIC_V1, "sc=Arab"),
        (SCRIPT_ARMENIAN_V1, "sc=Armn"),
        (SCRIPT_AVESTAN_V1, "sc=Avst"),
        (SCRIPT_BALINESE_V1, "sc=Bali"),
        (SCRIPT_BAMUM_V1, "sc=Bamu"),
        (SCRIPT_BASSA_VAH_V1, "sc=Bass"),
        (SCRIPT_BATAK_V1, "sc=Batk"),
        (SCRIPT_BENGALI_V1, "sc=Beng"),
        (SCRIPT_BHAIKSUKI_V1, "sc=Bhks"),
        (SCRIPT_BOPOMOFO_V1, "sc=Bopo"),
        (SCRIPT_BRAHMI_V1, "sc=Brah"),
        (SCRIPT_BRAILLE_V1, "sc=Brai"),
        (SCRIPT_BUGINESE_V1, "sc=Bugi"),
        (SCRIPT_BUHID_V1, "sc=Buhd"),
        (SCRIPT_CANADIAN_ABORIGINAL_V1, "sc=Cans"),
        (SCRIPT_CARIAN_V1, "sc=Cari"),
        (SCRIPT_CAUCASIAN_ALBANIAN_V1, "sc=Aghb"),
        (SCRIPT_CHAKMA_V1, "sc=Cakm"),
        (SCRIPT_CHAM_V1, "sc=Cham"),
        (SCRIPT_CHEROKEE_V1, "sc=Cher"),
        (SCRIPT_CHORASMIAN_V1, "sc=Chrs"),
        (SCRIPT_COMMON_V1, "sc=Zyyy"),
        (SCRIPT_COPTIC_V1, "sc=Copt"),
        (SCRIPT_CUNEIFORM_V1, "sc=Xsux"),
        (SCRIPT_CYPRIOT_V1, "sc=Cprt"),
        (SCRIPT_CYPRO_MINOAN_V1, "sc=Cpmn"),
        (SCRIPT_CYRILLIC_V1, "sc=Cyrl"),
        (SCRIPT_DESERET_V1, "sc=Dsrt"),
        (SCRIPT_DEVANAGARI_V1, "sc=Deva"),
        (SCRIPT_DIVES_AKURU_V1, "sc=Diak"),
        (SCRIPT_DOGRA_V1, "sc=Dogr"),
        (SCRIPT_DUPLOYAN_V1, "sc=Dupl"),
        (SCRIPT_EGYPTIAN_HIEROGLYPHS_V1, "sc=Egyp"),
        (SCRIPT_ELBASAN_V1, "sc=Elba"),
        (SCRIPT_ELYMAIC_V1, "sc=Elym"),
        (SCRIPT_ETHIOPIC_V1, "sc=Ethi"),
        (SCRIPT_GEORGIAN_V1, "sc=Geor"),
        (SCRIPT_GLAGOLITIC_V1, "sc=Glag"),
        (SCRIPT_GOTHIC_V1, "sc=Goth"),
        (SCRIPT_GRANTHA_V1, "sc=Gran"),
        (SCRIPT_GREEK_V1, "sc=Grek"),
        (SCRIPT_GUJARATI_V1, "sc=Gujr"),
        (SCRIPT_GUNJALA_GONDI_V1, "sc=Gong"),
        (SCRIPT_GURMUKHI_V1, "sc=Guru"),
        (SCRIPT_HAN_V1, "sc=Hani"),
        (SCRIPT_HANGUL_V1, "sc=Hang"),
        (SCRIPT_HANIFI_ROHINGYA_V1, "sc=Rohg"),
        (SCRIPT_HANUNOO_V1, "sc=Hano"),
        (SCRIPT_HATRAN_V1, "sc=Hatr"),
        (SCRIPT_HEBREW_V1, "sc=Hebr"),
        (SCRIPT_HIRAGANA_V1, "sc=Hira"),
        (SCRIPT_IMPERIAL_ARAMAIC_V1, "sc=Armi"),
        (SCRIPT_INHERITED_V1, "sc=Zinh"),
        (SCRIPT_INSCRIPTIONAL_PAHLAVI_V1, "sc=Phli"),
        (SCRIPT_INSCRIPTIONAL_PARTHIAN_V1, "sc=Prti"),
        (SCRIPT_JAVANESE_V1, "sc=Java"),
        (SCRIPT_KAITHI_V1, "sc=Kthi"),
        (SCRIPT_KANNADA_V1, "sc=Knda"),
        (SCRIPT_KATAKANA_V1, "sc=Kana"),
        (SCRIPT_KAYAH_LI_V1, "sc=Kali"),
        (SCRIPT_KHAROSHTHI_V1, "sc=Khar"),
        (SCRIPT_KHITAN_SMALL_SCRIPT_V1, "sc=Kits"),
        (SCRIPT_KHMER_V1, "sc=Khmr"),
        (SCRIPT_KHOJKI_V1, "sc=Khoj"),
        (SCRIPT_KHUDAWADI_V1, "sc=Sind"),
        (SCRIPT_LAO_V1, "sc=Laoo"),
        (SCRIPT_LATIN_V1, "sc=Latn"),
        (SCRIPT_LEPCHA_V1, "sc=Lepc"),
        (SCRIPT_LIMBU_V1, "sc=Limb"),
        (SCRIPT_LINEAR_A_V1, "sc=Lina"),
        (SCRIPT_LINEAR_B_V1, "sc=Linb"),
        (SCRIPT_LISU_V1, "sc=Lisu"),
        (SCRIPT_LYCIAN_V1, "sc=Lyci"),
        (SCRIPT_LYDIAN_V1, "sc=Lydi"),
        (SCRIPT_MAHAJANI_V1, "sc=Mahj"),
        (SCRIPT_MAKASAR_V1, "sc=Maka"),
        (SCRIPT_MALAYALAM_V1, "sc=Mlym"),
        (SCRIPT_MANDAIC_V1, "sc=Mand"),
        (SCRIPT_MANICHAEAN_V1, "sc=Mani"),
        (SCRIPT_MARCHEN_V1, "sc=Marc"),
        (SCRIPT_MASARAM_GONDI_V1, "sc=Gonm"),
        (SCRIPT_MEDEFAIDRIN_V1, "sc=Medf"),
        (SCRIPT_MEETEI_MAYEK_V1, "sc=Mtei"),
        (SCRIPT_MENDE_KIKAKUI_V1, "sc=Mend"),
        (SCRIPT_MEROITIC_CURSIVE_V1, "sc=Merc"),
        (SCRIPT_MEROITIC_HIEROGLYPHS_V1, "sc=Mero"),
        (SCRIPT_MIAO_V1, "sc=Plrd"),
        (SCRIPT_MODI_V1, "sc=Modi"),
        (SCRIPT_MONGOLIAN_V1, "sc=Mong"),
        (SCRIPT_MRO_V1, "sc=Mroo"),
        (SCRIPT_MULTANI_V1, "sc=Mult"),
        (SCRIPT_MYANMAR_V1, "sc=Mymr"),
        (SCRIPT_NABATAEAN_V1, "sc=Nbat"),
        (SCRIPT_NANDINAGARI_V1, "sc=Nand"),
        (SCRIPT_NEW_TAI_LUE_V1, "sc=Talu"),
        (SCRIPT_NEWA_V1, "sc=Newa"),
        (SCRIPT_NKO_V1, "sc=Nkoo"),
        (SCRIPT_NUSHU_V1, "sc=Nshu"),
        (SCRIPT_NYIAKENG_PUACHUE_HMONG_V1, "sc=Hmnp"),
        (SCRIPT_OGHAM_V1, "sc=Ogam"),
        (SCRIPT_OL_CHIKI_V1, "sc=Olck"),
        (SCRIPT_OLD_HUNGARIAN_V1, "sc=Hung"),
        (SCRIPT_OLD_ITALIC_V1, "sc=Ital"),
        (SCRIPT_OLD_NORTH_ARABIAN_V1, "sc=Narb"),
        (SCRIPT_OLD_PERMIC_V1, "sc=Perm"),
        (SCRIPT_OLD_PERSIAN_V1, "sc=Xpeo"),
        (SCRIPT_OLD_SOGDIAN_V1, "sc=Sogo"),
        (SCRIPT_OLD_SOUTH_ARABIAN_V1, "sc=Sarb"),
        (SCRIPT_OLD_TURKIC_V1, "sc=Orkh"),
        (SCRIPT_OLD_UYGHUR_V1, "sc=Ougr"),
        (SCRIPT_ORIYA_V1, "sc=Orya"),
        (SCRIPT_OSAGE_V1, "sc=Osge"),
        (SCRIPT_OSMANYA_V1, "sc=Osma"),
        (SCRIPT_PAHAWH_HMONG_V1, "sc=Hmng"),
        (SCRIPT_PALMYRENE_V1, "sc=Palm"),
        (SCRIPT_PAU_CIN_HAU_V1, "sc=Pauc"),
        (SCRIPT_PHAGS_PA_V1, "sc=Phag"),
        (SCRIPT_PHOENICIAN_V1, "sc=Phnx"),
        (SCRIPT_PSALTER_PAHLAVI_V1, "sc=Phlp"),
        (SCRIPT_REJANG_V1, "sc=Rjng"),
        (SCRIPT_RUNIC_V1, "sc=Runr"),
        (SCRIPT_SAMARITAN_V1, "sc=Samr"),
        (SCRIPT_SAURASHTRA_V1, "sc=Saur"),
        (SCRIPT_SHARADA_V1, "sc=Shrd"),
        (SCRIPT_SHAVIAN_V1, "sc=Shaw"),
        (SCRIPT_SIDDHAM_V1, "sc=Sidd"),
        (SCRIPT_SIGNWRITING_V1, "sc=Sgnw"),
        (SCRIPT_SINHALA_V1, "sc=Sinh"),
        (SCRIPT_SOGDIAN_V1, "sc=Sogd"),
        (SCRIPT_SORA_SOMPENG_V1, "sc=Sora"),
        (SCRIPT_SOYOMBO_V1, "sc=Soyo"),
        (SCRIPT_SUNDANESE_V1, "sc=Sund"),
        (SCRIPT_SYLOTI_NAGRI_V1, "sc=Sylo"),
        (SCRIPT_SYRIAC_V1, "sc=Syrc"),
        (SCRIPT_TAGALOG_V1, "sc=Tglg"),
        (SCRIPT_TAGBANWA_V1, "sc=Tagb"),
        (SCRIPT_TAI_LE_V1, "sc=Tale"),
        (SCRIPT_TAI_THAM_V1, "sc=Lana"),
        (SCRIPT_TAI_VIET_V1, "sc=Tavt"),
        (SCRIPT_TAKRI_V1, "sc=Takr"),
        (SCRIPT_TAMIL_V1, "sc=Taml"),
        (SCRIPT_TANGSA_V1, "sc=Tnsa"),
        (SCRIPT_TANGUT_V1, "sc=Tang"),
        (SCRIPT_TELUGU_V1, "sc=Telu"),
        (SCRIPT_THAANA_V1, "sc=Thaa"),
        (SCRIPT_THAI_V1, "sc=Thai"),
        (SCRIPT_TIBETAN_V1, "sc=Tibt"),
        (SCRIPT_TIFINAGH_V1, "sc=Tfng"),
        (SCRIPT_TIRHUTA_V1, "sc=Tirh"),
        (SCRIPT_TOTO_V1, "sc=Toto"),
        (SCRIPT_UGARITIC_V1, "sc=Ugar"),
        (SCRIPT_UNKNOWN_V1, "sc=Zzzz"),
        (SCRIPT_VAI_V1, "sc=Vaii"),
        (SCRIPT_VITHKUQI_V1, "sc=Vith"),
        (SCRIPT_WANCHO_V1, "sc=Wcho"),
        (SCRIPT_WARANG_CITI_V1, "sc=Wara"),
        (SCRIPT_YEZIDI_V1, "sc=Yezi"),
        (SCRIPT_YI_V1, "sc=Yiii"),
        (SCRIPT_ZANABAZAR_SQUARE_V1, "sc=Zanb"),
    );

    define_resource_keys!(ALL_MAP_KEYS; 8;
        //
        // Enumerated property CodePointMaps
        //

        // ResourceKey subcategory string is the short alias of the property

        (CANONICAL_COMBINING_CLASS_V1, "ccc"),
        (GENERAL_CATEGORY_V1, "gc"),
        (SCRIPT_V1, "sc"),
        (EAST_ASIAN_WIDTH_V1, "ea"),
        (LINE_BREAK_V1, "lb"),
        (GRAPHEME_CLUSTER_BREAK_V1, "GCB"),
        (WORD_BREAK_V1, "WB"),
        (SENTENCE_BREAK_V1, "SB"),

    );

    define_resource_keys!(ALL_SCRIPT_EXTENSIONS_KEYS; 1;
        //
        // Script_Extensions + Script data
        //

        // ResourceKey subcategory string is the short alias of Script_Extensions

        (SCRIPT_EXTENSIONS_V1, "scx"),
    );
}

//
// UnicodeProperty
//

/// A set of characters with a particular property.
#[icu_provider::data_struct]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct UnicodePropertyV1<'data> {
    /// The set of characters, represented as an inversion list
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub inv_list: UnicodeSet<'data>,
}

impl Default for UnicodePropertyV1<'static> {
    /// Default empty property
    fn default() -> UnicodePropertyV1<'static> {
        UnicodePropertyV1 {
            inv_list: UnicodeSetBuilder::new().build(),
        }
    }
}

impl<'data> UnicodePropertyV1<'data> {
    /// Creates a [`UnicodePropertyV1`] for the given [`UnicodeSet`].
    pub fn from_owned_uniset(set: UnicodeSet<'data>) -> UnicodePropertyV1<'data> {
        UnicodePropertyV1 { inv_list: set }
    }
}

impl<'data> From<UnicodePropertyV1<'data>> for UnicodeSet<'data> {
    fn from(prop: UnicodePropertyV1<'data>) -> UnicodeSet<'data> {
        prop.inv_list
    }
}

//
// UnicodePropertyMap
//

/// A map efficiently storing data about individual characters.
#[derive(Debug, Eq, PartialEq, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct UnicodePropertyMapV1<'data, T: TrieValue> {
    /// A codepoint trie storing the data
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub code_point_trie: CodePointTrie<'data, T>,
}

impl<'data, T: TrieValue> Clone for UnicodePropertyMapV1<'data, T>
where
    <T as zerovec::ule::AsULE>::ULE: Clone,
{
    fn clone(&self) -> Self {
        UnicodePropertyMapV1 {
            code_point_trie: self.code_point_trie.clone(),
        }
    }
}

/// Marker type for UnicodePropertyMapV1.
/// This is generated by hand because icu_provider::data_struct doesn't support generics yet.
pub struct UnicodePropertyMapV1Marker<T: TrieValue> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T: TrieValue> icu_provider::DataMarker for UnicodePropertyMapV1Marker<T> {
    type Yokeable = UnicodePropertyMapV1<'static, T>;
}

//
// Script_Extensions
//

/// A data structure efficiently storing `Script` and `Script_Extensions` property data.
#[icu_provider::data_struct]
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ScriptExtensionsPropertyV1<'data> {
    /// A special data structure for `Script` and `Script_Extensions`.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub data: ScriptExtensions<'data>,
}
