// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::script::ScriptWithExtensions;
use icu_codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::prelude::*;
use icu_uniset::UnicodeSet;

/// A set of characters with a particular property.
#[derive(Debug, Eq, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct UnicodePropertyV1<'data> {
    /// The set of characters, represented as an inversion list
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub inv_list: UnicodeSet<'data>,
}

/// A map efficiently storing data about individual characters.
#[derive(Clone, Debug, Eq, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct UnicodePropertyMapV1<'data, T: TrieValue> {
    /// A codepoint trie storing the data
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub code_point_trie: CodePointTrie<'data, T>,
}

/// A data structure efficiently storing `Script` and `Script_Extensions` property data.
#[icu_provider::data_struct(ScriptWithExtensionsPropertyV1Marker = "props/scx@1")]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ScriptWithExtensionsPropertyV1<'data> {
    /// A special data structure for `Script` and `Script_Extensions`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub data: ScriptWithExtensions<'data>,
}

macro_rules! expand {
    (
        ($(($bin_marker:ident, $bin_s:literal),)+),
        ($(($enum_marker:ident, $enum_s:literal, $value_ty:ident),)+)
    ) => {

            $(
                #[doc = core::concat!("Data marker for the '", $bin_s, "' Unicode property")]
                pub struct $bin_marker;

                impl DataMarker for $bin_marker {
                    type Yokeable = UnicodePropertyV1<'static>;
                }
                impl ResourceMarker for $bin_marker {
                    const KEY: ResourceKey = resource_key!(concat!("props/", $bin_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $bin_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $bin_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ icu_properties::provider::$bin_marker }
                    }
                }
            )+

            $(
                #[doc = core::concat!("Data marker for the '", $enum_s, "' Unicode property")]
                pub struct $enum_marker;

                impl DataMarker for $enum_marker {
                    type Yokeable = UnicodePropertyMapV1<'static, crate::$value_ty>;
                }

                impl ResourceMarker for $enum_marker {
                    const KEY: ResourceKey = resource_key!(concat!("props/", $enum_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $enum_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $enum_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ icu_properties::provider::$enum_marker }
                    }
                }
            )+

            #[cfg(feature = "datagen")]
            /// The set of all resource keys supported by [`icu_uniset`](crate).
            pub const ALL_KEYS: [ResourceKey; 75] = [
                $(
                    $bin_marker::KEY,
                )+
                $(
                    $enum_marker::KEY,
                )+
                ScriptWithExtensionsPropertyV1Marker::KEY,
            ];
    };
}

expand!(
    (
        // Binary properties
        (AsciiHexDigitV1Marker, "AHex"),
        (AlnumV1Marker, "alnum"),
        (AlphabeticV1Marker, "Alpha"),
        (BidiControlV1Marker, "Bidi_C"),
        (BidiMirroredV1Marker, "Bidi_M"),
        (BlankV1Marker, "blank"),
        (CasedV1Marker, "Cased"),
        (CaseIgnorableV1Marker, "CI"),
        (FullCompositionExclusionV1Marker, "Comp_Ex"),
        (ChangesWhenCasefoldedV1Marker, "CWCF"),
        (ChangesWhenCasemappedV1Marker, "CWCM"),
        (ChangesWhenNfkcCasefoldedV1Marker, "CWKCF"),
        (ChangesWhenLowercasedV1Marker, "CWL"),
        (ChangesWhenTitlecasedV1Marker, "CWT"),
        (ChangesWhenUppercasedV1Marker, "CWU"),
        (DashV1Marker, "Dash"),
        (DeprecatedV1Marker, "Dep"),
        (DefaultIgnorableCodePointV1Marker, "DI"),
        (DiacriticV1Marker, "Dia"),
        (EmojiModifierBaseV1Marker, "EBase"),
        (EmojiComponentV1Marker, "EComp"),
        (EmojiModifierV1Marker, "EMod"),
        (EmojiV1Marker, "Emoji"),
        (EmojiPresentationV1Marker, "EPres"),
        (ExtenderV1Marker, "Ext"),
        (ExtendedPictographicV1Marker, "ExtPict"),
        (GraphV1Marker, "graph"),
        (GraphemeBaseV1Marker, "Gr_Base"),
        (GraphemeExtendV1Marker, "Gr_Ext"),
        (GraphemeLinkV1Marker, "Gr_Link"),
        (HexDigitV1Marker, "Hex"),
        (HyphenV1Marker, "Hyphen"),
        (IdContinueV1Marker, "IDC"),
        (IdeographicV1Marker, "Ideo"),
        (IdStartV1Marker, "IDS"),
        (IdsBinaryOperatorV1Marker, "IDSB"),
        (IdsTrinaryOperatorV1Marker, "IDST"),
        (JoinControlV1Marker, "Join_C"),
        (LogicalOrderExceptionV1Marker, "LOE"),
        (LowercaseV1Marker, "Lower"),
        (MathV1Marker, "Math"),
        (NoncharacterCodePointV1Marker, "NChar"),
        (NfcInertV1Marker, "nfcinert"),
        (NfdInertV1Marker, "nfdinert"),
        (NfkcInertV1Marker, "nfkcinert"),
        (NfkdInertV1Marker, "nfkdinert"),
        (PatternSyntaxV1Marker, "Pat_Syn"),
        (PatternWhiteSpaceV1Marker, "Pat_WS"),
        (PrependedConcatenationMarkV1Marker, "PCM"),
        (PrintV1Marker, "print"),
        (QuotationMarkV1Marker, "QMark"),
        (RadicalV1Marker, "Radical"),
        (RegionalIndicatorV1Marker, "RI"),
        (SoftDottedV1Marker, "SD"),
        (SegmentStarterV1Marker, "segstart"),
        (CaseSensitiveV1Marker, "Sensitive"),
        (SentenceTerminalV1Marker, "STerm"),
        (TerminalPunctuationV1Marker, "Term"),
        (UnifiedIdeographV1Marker, "UIdeo"),
        (UppercaseV1Marker, "Upper"),
        (VariationSelectorV1Marker, "VS"),
        (WhiteSpaceV1Marker, "WSpace"),
        (XdigitV1Marker, "xdigit"),
        (XidContinueV1Marker, "XIDC"),
        (XidStartV1Marker, "XIDS"),
    ),
    (
        // Enum properties
        (
            CanonicalCombiningClassV1Marker,
            "ccc",
            CanonicalCombiningClass
        ),
        (GeneralCategoryV1Marker, "gc", GeneralCategory),
        (BidiClassV1Marker, "bc", BidiClass),
        (ScriptV1Marker, "sc", Script),
        (EastAsianWidthV1Marker, "ea", EastAsianWidth),
        (LineBreakV1Marker, "lb", LineBreak),
        (GraphemeClusterBreakV1Marker, "GCB", GraphemeClusterBreak),
        (WordBreakV1Marker, "WB", WordBreak),
        (SentenceBreakV1Marker, "SB", SentenceBreak),
    )
);
