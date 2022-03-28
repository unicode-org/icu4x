// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::script::ScriptWithExtensions;
use icu_codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::{yoke, zerofrom};
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

    define_resource_keys!(ALL_SET_KEYS; 65;
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
#[icu_provider::data_struct(UnicodePropertyV1Marker)]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct UnicodePropertyV1<'data> {
    /// The set of characters, represented as an inversion list
    #[cfg_attr(feature = "serialize", serde(borrow))]
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
#[derive(Debug, Eq, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct UnicodePropertyMapV1<'data, T: TrieValue> {
    /// A codepoint trie storing the data
    #[cfg_attr(feature = "serialize", serde(borrow))]
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
#[icu_provider::data_struct(ScriptWithExtensionsPropertyV1Marker)]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct ScriptWithExtensionsPropertyV1<'data> {
    /// A special data structure for `Script` and `Script_Extensions`.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub data: ScriptWithExtensions<'data>,
}
