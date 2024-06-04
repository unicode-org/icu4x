// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

pub mod names;
pub use names::*;

use crate::script::ScriptWithExt;
use crate::Script;

use core::ops::RangeInclusive;
use core::str;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_collections::codepointtrie::{CodePointMapRange, CodePointTrie, TrieValue};
use icu_provider::prelude::*;
use zerofrom::ZeroFrom;

use zerovec::{VarZeroVec, ZeroSlice, ZeroVecError};

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as properties;
        pub use icu_collections as collections;
        #[allow(unused_imports)] // baked data may or may not need this
        pub use icu_locale as locale;
    }
    icu_properties_data::make_provider!(Baked);
    icu_properties_data::impl_propnames_from_gcb_v1!(Baked);
    icu_properties_data::impl_propnames_from_bc_v1!(Baked);
    icu_properties_data::impl_propnames_from_ccc_v1!(Baked);
    icu_properties_data::impl_propnames_from_ea_v1!(Baked);
    icu_properties_data::impl_propnames_from_gc_v1!(Baked);
    icu_properties_data::impl_propnames_from_gcm_v1!(Baked);
    icu_properties_data::impl_propnames_from_hst_v1!(Baked);
    icu_properties_data::impl_propnames_from_insc_v1!(Baked);
    icu_properties_data::impl_propnames_from_jt_v1!(Baked);
    icu_properties_data::impl_propnames_from_lb_v1!(Baked);
    icu_properties_data::impl_propnames_from_sb_v1!(Baked);
    icu_properties_data::impl_propnames_from_sc_v1!(Baked);
    icu_properties_data::impl_propnames_from_wb_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_bc_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_ea_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_gc_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_gcb_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_hst_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_insc_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_jt_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_lb_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_sb_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_sc_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_linear_wb_v1!(Baked);
    icu_properties_data::impl_propnames_to_long_sparse_ccc_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_bc_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_ea_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_gc_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_gcb_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_hst_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_insc_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_jt_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_lb_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_sb_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear_wb_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_linear4_sc_v1!(Baked);
    icu_properties_data::impl_propnames_to_short_sparse_ccc_v1!(Baked);
    icu_properties_data::impl_props_ahex_v1!(Baked);
    icu_properties_data::impl_props_alnum_v1!(Baked);
    icu_properties_data::impl_props_alpha_v1!(Baked);
    icu_properties_data::impl_props_basic_emoji_v1!(Baked);
    icu_properties_data::impl_props_bc_v1!(Baked);
    icu_properties_data::impl_props_bidi_c_v1!(Baked);
    icu_properties_data::impl_props_bidi_m_v1!(Baked);
    icu_properties_data::impl_props_bidiauxiliaryprops_v1!(Baked);
    icu_properties_data::impl_props_blank_v1!(Baked);
    icu_properties_data::impl_props_cased_v1!(Baked);
    icu_properties_data::impl_props_ccc_v1!(Baked);
    icu_properties_data::impl_props_ci_v1!(Baked);
    icu_properties_data::impl_props_comp_ex_v1!(Baked);
    icu_properties_data::impl_props_cwcf_v1!(Baked);
    icu_properties_data::impl_props_cwcm_v1!(Baked);
    icu_properties_data::impl_props_cwkcf_v1!(Baked);
    icu_properties_data::impl_props_cwl_v1!(Baked);
    icu_properties_data::impl_props_cwt_v1!(Baked);
    icu_properties_data::impl_props_cwu_v1!(Baked);
    icu_properties_data::impl_props_dash_v1!(Baked);
    icu_properties_data::impl_props_dep_v1!(Baked);
    icu_properties_data::impl_props_di_v1!(Baked);
    icu_properties_data::impl_props_dia_v1!(Baked);
    icu_properties_data::impl_props_ea_v1!(Baked);
    icu_properties_data::impl_props_ebase_v1!(Baked);
    icu_properties_data::impl_props_ecomp_v1!(Baked);
    icu_properties_data::impl_props_emod_v1!(Baked);
    icu_properties_data::impl_props_emoji_v1!(Baked);
    icu_properties_data::impl_props_epres_v1!(Baked);
    icu_properties_data::impl_props_exemplarchars_auxiliary_v1!(Baked);
    icu_properties_data::impl_props_exemplarchars_index_v1!(Baked);
    icu_properties_data::impl_props_exemplarchars_main_v1!(Baked);
    icu_properties_data::impl_props_exemplarchars_numbers_v1!(Baked);
    icu_properties_data::impl_props_exemplarchars_punctuation_v1!(Baked);
    icu_properties_data::impl_props_ext_v1!(Baked);
    icu_properties_data::impl_props_extpict_v1!(Baked);
    icu_properties_data::impl_props_gc_v1!(Baked);
    icu_properties_data::impl_props_gcb_v1!(Baked);
    icu_properties_data::impl_props_gr_base_v1!(Baked);
    icu_properties_data::impl_props_gr_ext_v1!(Baked);
    icu_properties_data::impl_props_gr_link_v1!(Baked);
    icu_properties_data::impl_props_graph_v1!(Baked);
    icu_properties_data::impl_props_hex_v1!(Baked);
    icu_properties_data::impl_props_hst_v1!(Baked);
    icu_properties_data::impl_props_hyphen_v1!(Baked);
    icu_properties_data::impl_props_idc_v1!(Baked);
    icu_properties_data::impl_props_ideo_v1!(Baked);
    icu_properties_data::impl_props_ids_v1!(Baked);
    icu_properties_data::impl_props_idsb_v1!(Baked);
    icu_properties_data::impl_props_idst_v1!(Baked);
    icu_properties_data::impl_props_insc_v1!(Baked);
    icu_properties_data::impl_props_join_c_v1!(Baked);
    icu_properties_data::impl_props_jt_v1!(Baked);
    icu_properties_data::impl_props_lb_v1!(Baked);
    icu_properties_data::impl_props_loe_v1!(Baked);
    icu_properties_data::impl_props_lower_v1!(Baked);
    icu_properties_data::impl_props_math_v1!(Baked);
    icu_properties_data::impl_props_nchar_v1!(Baked);
    icu_properties_data::impl_props_nfcinert_v1!(Baked);
    icu_properties_data::impl_props_nfdinert_v1!(Baked);
    icu_properties_data::impl_props_nfkcinert_v1!(Baked);
    icu_properties_data::impl_props_nfkdinert_v1!(Baked);
    icu_properties_data::impl_props_pat_syn_v1!(Baked);
    icu_properties_data::impl_props_pat_ws_v1!(Baked);
    icu_properties_data::impl_props_pcm_v1!(Baked);
    icu_properties_data::impl_props_print_v1!(Baked);
    icu_properties_data::impl_props_qmark_v1!(Baked);
    icu_properties_data::impl_props_radical_v1!(Baked);
    icu_properties_data::impl_props_ri_v1!(Baked);
    icu_properties_data::impl_props_sb_v1!(Baked);
    icu_properties_data::impl_props_sc_v1!(Baked);
    icu_properties_data::impl_props_scx_v1!(Baked);
    icu_properties_data::impl_props_sd_v1!(Baked);
    icu_properties_data::impl_props_segstart_v1!(Baked);
    icu_properties_data::impl_props_sensitive_v1!(Baked);
    icu_properties_data::impl_props_sterm_v1!(Baked);
    icu_properties_data::impl_props_term_v1!(Baked);
    icu_properties_data::impl_props_uideo_v1!(Baked);
    icu_properties_data::impl_props_upper_v1!(Baked);
    icu_properties_data::impl_props_vs_v1!(Baked);
    icu_properties_data::impl_props_wb_v1!(Baked);
    icu_properties_data::impl_props_wspace_v1!(Baked);
    icu_properties_data::impl_props_xdigit_v1!(Baked);
    icu_properties_data::impl_props_xidc_v1!(Baked);
    icu_properties_data::impl_props_xids_v1!(Baked);
};

/// All data keys in this module.
pub const KEYS: &[DataKey] = &[
    AlnumV1Marker::KEY,
    AlphabeticV1Marker::KEY,
    AsciiHexDigitV1Marker::KEY,
    BasicEmojiV1Marker::KEY,
    bidi_data::BidiAuxiliaryPropertiesV1Marker::KEY,
    BidiControlV1Marker::KEY,
    BidiMirroredV1Marker::KEY,
    BlankV1Marker::KEY,
    CasedV1Marker::KEY,
    CaseIgnorableV1Marker::KEY,
    CaseSensitiveV1Marker::KEY,
    ChangesWhenCasefoldedV1Marker::KEY,
    ChangesWhenCasemappedV1Marker::KEY,
    ChangesWhenLowercasedV1Marker::KEY,
    ChangesWhenNfkcCasefoldedV1Marker::KEY,
    ChangesWhenTitlecasedV1Marker::KEY,
    ChangesWhenUppercasedV1Marker::KEY,
    DashV1Marker::KEY,
    DefaultIgnorableCodePointV1Marker::KEY,
    DeprecatedV1Marker::KEY,
    DiacriticV1Marker::KEY,
    EmojiComponentV1Marker::KEY,
    EmojiModifierBaseV1Marker::KEY,
    EmojiModifierV1Marker::KEY,
    EmojiPresentationV1Marker::KEY,
    EmojiV1Marker::KEY,
    ExemplarCharactersAuxiliaryV1Marker::KEY,
    ExemplarCharactersIndexV1Marker::KEY,
    ExemplarCharactersMainV1Marker::KEY,
    ExemplarCharactersNumbersV1Marker::KEY,
    ExemplarCharactersPunctuationV1Marker::KEY,
    ExtendedPictographicV1Marker::KEY,
    ExtenderV1Marker::KEY,
    FullCompositionExclusionV1Marker::KEY,
    GraphemeBaseV1Marker::KEY,
    GraphemeExtendV1Marker::KEY,
    GraphemeLinkV1Marker::KEY,
    GraphV1Marker::KEY,
    HexDigitV1Marker::KEY,
    HyphenV1Marker::KEY,
    IdContinueV1Marker::KEY,
    IdeographicV1Marker::KEY,
    IdsBinaryOperatorV1Marker::KEY,
    IdStartV1Marker::KEY,
    IdsTrinaryOperatorV1Marker::KEY,
    JoinControlV1Marker::KEY,
    LogicalOrderExceptionV1Marker::KEY,
    LowercaseV1Marker::KEY,
    MathV1Marker::KEY,
    NfcInertV1Marker::KEY,
    NfdInertV1Marker::KEY,
    NfkcInertV1Marker::KEY,
    NfkdInertV1Marker::KEY,
    NoncharacterCodePointV1Marker::KEY,
    PatternSyntaxV1Marker::KEY,
    PatternWhiteSpaceV1Marker::KEY,
    PrependedConcatenationMarkV1Marker::KEY,
    PrintV1Marker::KEY,
    QuotationMarkV1Marker::KEY,
    RadicalV1Marker::KEY,
    RegionalIndicatorV1Marker::KEY,
    ScriptWithExtensionsPropertyV1Marker::KEY,
    ScriptWithExtensionsPropertyV1Marker::KEY,
    SegmentStarterV1Marker::KEY,
    SentenceTerminalV1Marker::KEY,
    SoftDottedV1Marker::KEY,
    TerminalPunctuationV1Marker::KEY,
    UnifiedIdeographV1Marker::KEY,
    UppercaseV1Marker::KEY,
    VariationSelectorV1Marker::KEY,
    WhiteSpaceV1Marker::KEY,
    XdigitV1Marker::KEY,
    XidContinueV1Marker::KEY,
    XidStartV1Marker::KEY,
    names::BidiClassNameToValueV1Marker::KEY,
    BidiClassV1Marker::KEY,
    names::BidiClassValueToLongNameV1Marker::KEY,
    names::BidiClassValueToShortNameV1Marker::KEY,
    names::CanonicalCombiningClassNameToValueV1Marker::KEY,
    CanonicalCombiningClassV1Marker::KEY,
    names::CanonicalCombiningClassValueToLongNameV1Marker::KEY,
    names::CanonicalCombiningClassValueToShortNameV1Marker::KEY,
    names::EastAsianWidthNameToValueV1Marker::KEY,
    EastAsianWidthV1Marker::KEY,
    names::EastAsianWidthValueToLongNameV1Marker::KEY,
    names::EastAsianWidthValueToShortNameV1Marker::KEY,
    names::GeneralCategoryMaskNameToValueV1Marker::KEY,
    names::GeneralCategoryNameToValueV1Marker::KEY,
    GeneralCategoryV1Marker::KEY,
    names::GeneralCategoryValueToLongNameV1Marker::KEY,
    names::GeneralCategoryValueToShortNameV1Marker::KEY,
    names::GraphemeClusterBreakNameToValueV1Marker::KEY,
    GraphemeClusterBreakV1Marker::KEY,
    names::GraphemeClusterBreakValueToLongNameV1Marker::KEY,
    names::GraphemeClusterBreakValueToShortNameV1Marker::KEY,
    names::HangulSyllableTypeNameToValueV1Marker::KEY,
    HangulSyllableTypeV1Marker::KEY,
    names::HangulSyllableTypeValueToLongNameV1Marker::KEY,
    names::HangulSyllableTypeValueToShortNameV1Marker::KEY,
    names::IndicSyllabicCategoryNameToValueV1Marker::KEY,
    IndicSyllabicCategoryV1Marker::KEY,
    names::IndicSyllabicCategoryValueToLongNameV1Marker::KEY,
    names::IndicSyllabicCategoryValueToShortNameV1Marker::KEY,
    names::JoiningTypeNameToValueV1Marker::KEY,
    JoiningTypeV1Marker::KEY,
    names::JoiningTypeValueToLongNameV1Marker::KEY,
    names::JoiningTypeValueToShortNameV1Marker::KEY,
    names::LineBreakNameToValueV1Marker::KEY,
    LineBreakV1Marker::KEY,
    names::LineBreakValueToLongNameV1Marker::KEY,
    names::LineBreakValueToShortNameV1Marker::KEY,
    names::ScriptNameToValueV1Marker::KEY,
    ScriptV1Marker::KEY,
    names::ScriptValueToLongNameV1Marker::KEY,
    names::ScriptValueToShortNameV1Marker::KEY,
    names::SentenceBreakNameToValueV1Marker::KEY,
    SentenceBreakV1Marker::KEY,
    names::SentenceBreakValueToLongNameV1Marker::KEY,
    names::SentenceBreakValueToShortNameV1Marker::KEY,
    names::WordBreakNameToValueV1Marker::KEY,
    WordBreakV1Marker::KEY,
    names::WordBreakValueToLongNameV1Marker::KEY,
    names::WordBreakValueToShortNameV1Marker::KEY,
];

// include the specialized structs for the compact representation of Bidi property data
pub mod bidi_data;

/// A set of characters which share a particular property value.
///
/// This data enum is extensible, more backends may be added in the future.
/// Old data can be used with newer code but not vice versa.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(AlnumV1Marker, "props/alnum@1", singleton),
    marker(AlphabeticV1Marker, "props/Alpha@1", singleton),
    marker(AsciiHexDigitV1Marker, "props/AHex@1", singleton),
    marker(BidiControlV1Marker, "props/Bidi_C@1", singleton),
    marker(BidiMirroredV1Marker, "props/Bidi_M@1", singleton),
    marker(BlankV1Marker, "props/blank@1", singleton),
    marker(CasedV1Marker, "props/Cased@1", singleton),
    marker(CaseIgnorableV1Marker, "props/CI@1", singleton),
    marker(CaseSensitiveV1Marker, "props/Sensitive@1", singleton),
    marker(ChangesWhenCasefoldedV1Marker, "props/CWCF@1", singleton),
    marker(ChangesWhenCasemappedV1Marker, "props/CWCM@1", singleton),
    marker(ChangesWhenLowercasedV1Marker, "props/CWL@1", singleton),
    marker(ChangesWhenNfkcCasefoldedV1Marker, "props/CWKCF@1", singleton),
    marker(ChangesWhenTitlecasedV1Marker, "props/CWT@1", singleton),
    marker(ChangesWhenUppercasedV1Marker, "props/CWU@1", singleton),
    marker(DashV1Marker, "props/Dash@1", singleton),
    marker(DefaultIgnorableCodePointV1Marker, "props/DI@1", singleton),
    marker(DeprecatedV1Marker, "props/Dep@1", singleton),
    marker(DiacriticV1Marker, "props/Dia@1", singleton),
    marker(EmojiComponentV1Marker, "props/EComp@1", singleton),
    marker(EmojiModifierBaseV1Marker, "props/EBase@1", singleton),
    marker(EmojiModifierV1Marker, "props/EMod@1", singleton),
    marker(EmojiPresentationV1Marker, "props/EPres@1", singleton),
    marker(EmojiV1Marker, "props/Emoji@1", singleton),
    marker(ExtendedPictographicV1Marker, "props/ExtPict@1", singleton),
    marker(ExtenderV1Marker, "props/Ext@1", singleton),
    marker(FullCompositionExclusionV1Marker, "props/Comp_Ex@1", singleton),
    marker(GraphemeBaseV1Marker, "props/Gr_Base@1", singleton),
    marker(GraphemeExtendV1Marker, "props/Gr_Ext@1", singleton),
    marker(GraphemeLinkV1Marker, "props/Gr_Link@1", singleton),
    marker(GraphV1Marker, "props/graph@1", singleton),
    marker(HexDigitV1Marker, "props/Hex@1", singleton),
    marker(HyphenV1Marker, "props/Hyphen@1", singleton),
    marker(IdContinueV1Marker, "props/IDC@1", singleton),
    marker(IdeographicV1Marker, "props/Ideo@1", singleton),
    marker(IdsBinaryOperatorV1Marker, "props/IDSB@1", singleton),
    marker(IdStartV1Marker, "props/IDS@1", singleton),
    marker(IdsTrinaryOperatorV1Marker, "props/IDST@1", singleton),
    marker(JoinControlV1Marker, "props/Join_C@1", singleton),
    marker(LogicalOrderExceptionV1Marker, "props/LOE@1", singleton),
    marker(LowercaseV1Marker, "props/Lower@1", singleton),
    marker(MathV1Marker, "props/Math@1", singleton),
    marker(NfcInertV1Marker, "props/nfcinert@1", singleton),
    marker(NfdInertV1Marker, "props/nfdinert@1", singleton),
    marker(NfkcInertV1Marker, "props/nfkcinert@1", singleton),
    marker(NfkdInertV1Marker, "props/nfkdinert@1", singleton),
    marker(NoncharacterCodePointV1Marker, "props/NChar@1", singleton),
    marker(PatternSyntaxV1Marker, "props/Pat_Syn@1", singleton),
    marker(PatternWhiteSpaceV1Marker, "props/Pat_WS@1", singleton),
    marker(PrependedConcatenationMarkV1Marker, "props/PCM@1", singleton),
    marker(PrintV1Marker, "props/print@1", singleton),
    marker(QuotationMarkV1Marker, "props/QMark@1", singleton),
    marker(RadicalV1Marker, "props/Radical@1", singleton),
    marker(RegionalIndicatorV1Marker, "props/RI@1", singleton),
    marker(SegmentStarterV1Marker, "props/segstart@1", singleton),
    marker(SentenceTerminalV1Marker, "props/STerm@1", singleton),
    marker(SoftDottedV1Marker, "props/SD@1", singleton),
    marker(TerminalPunctuationV1Marker, "props/Term@1", singleton),
    marker(UnifiedIdeographV1Marker, "props/UIdeo@1", singleton),
    marker(UppercaseV1Marker, "props/Upper@1", singleton),
    marker(VariationSelectorV1Marker, "props/VS@1", singleton),
    marker(WhiteSpaceV1Marker, "props/WSpace@1", singleton),
    marker(XdigitV1Marker, "props/xdigit@1", singleton),
    marker(XidContinueV1Marker, "props/XIDC@1", singleton),
    marker(XidStartV1Marker, "props/XIDS@1", singleton)
)]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyCodePointSetV1<'data> {
    /// The set of characters, represented as an inversion list
    InversionList(#[cfg_attr(feature = "serde", serde(borrow))] CodePointInversionList<'data>),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

/// A map efficiently storing data about individual characters.
///
/// This data enum is extensible, more backends may be added in the future.
/// Old data can be used with newer code but not vice versa.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, Debug, Eq, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyCodePointMapV1<'data, T: TrieValue> {
    /// A codepoint trie storing the data
    CodePointTrie(#[cfg_attr(feature = "serde", serde(borrow))] CodePointTrie<'data, T>),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}
macro_rules! data_struct_generic {
    ($(marker($marker:ident, $ty:ident, $path:literal),)+) => {
        $(
            #[doc = core::concat!("Data marker for the '", stringify!($ty), "' Unicode property")]
            #[derive(Debug, Default)]
            #[cfg_attr(
                feature = "datagen",
                derive(databake::Bake),
                databake(path = icu_properties::provider),
            )]
            pub struct $marker;
            impl icu_provider::DataMarker for $marker {
                type Yokeable = PropertyCodePointMapV1<'static, crate::$ty>;
            }
            impl icu_provider::KeyedDataMarker for $marker {
                const KEY: icu_provider::DataKey = icu_provider::data_key!($path, icu_provider::DataKeyMetadata::construct_internal(icu_provider::_internal::LocaleFallbackPriority::Language, None, None, true));
            }
        )+
    }
}
data_struct_generic!(
    marker(BidiClassV1Marker, BidiClass, "props/bc@1"),
    marker(
        CanonicalCombiningClassV1Marker,
        CanonicalCombiningClass,
        "props/ccc@1"
    ),
    marker(EastAsianWidthV1Marker, EastAsianWidth, "props/ea@1"),
    marker(GeneralCategoryV1Marker, GeneralCategory, "props/gc@1"),
    marker(
        GraphemeClusterBreakV1Marker,
        GraphemeClusterBreak,
        "props/GCB@1"
    ),
    marker(
        HangulSyllableTypeV1Marker,
        HangulSyllableType,
        "props/hst@1"
    ),
    marker(
        IndicSyllabicCategoryV1Marker,
        IndicSyllabicCategory,
        "props/InSC@1"
    ),
    marker(JoiningTypeV1Marker, JoiningType, "props/jt@1"),
    marker(LineBreakV1Marker, LineBreak, "props/lb@1"),
    marker(ScriptV1Marker, Script, "props/sc@1"),
    marker(SentenceBreakV1Marker, SentenceBreak, "props/SB@1"),
    marker(WordBreakV1Marker, WordBreak, "props/WB@1"),
);

/// A set of characters and strings which share a particular property value.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(BasicEmojiV1Marker, "props/Basic_Emoji@1", singleton),
    marker(ExemplarCharactersAuxiliaryV1Marker, "props/exemplarchars/auxiliary@1"),
    marker(ExemplarCharactersIndexV1Marker, "props/exemplarchars/index@1"),
    marker(ExemplarCharactersMainV1Marker, "props/exemplarchars/main@1"),
    marker(ExemplarCharactersNumbersV1Marker, "props/exemplarchars/numbers@1"),
    marker(
        ExemplarCharactersPunctuationV1Marker,
        "props/exemplarchars/punctuation@1"
    )
)]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyUnicodeSetV1<'data> {
    /// A set representing characters in an inversion list, and the strings in a list.
    CPInversionListStrList(
        #[cfg_attr(feature = "serde", serde(borrow))] CodePointInversionListAndStringList<'data>,
    ),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

impl<'data> PropertyUnicodeSetV1<'data> {
    #[inline]
    pub(crate) fn contains(&self, s: &str) -> bool {
        match *self {
            Self::CPInversionListStrList(ref l) => l.contains(s),
        }
    }

    #[inline]
    pub(crate) fn contains32(&self, cp: u32) -> bool {
        match *self {
            Self::CPInversionListStrList(ref l) => l.contains32(cp),
        }
    }

    #[inline]
    pub(crate) fn contains_char(&self, ch: char) -> bool {
        match *self {
            Self::CPInversionListStrList(ref l) => l.contains_char(ch),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_inversion_list_string_list(
        l: CodePointInversionListAndStringList<'static>,
    ) -> Self {
        Self::CPInversionListStrList(l)
    }

    #[inline]
    pub(crate) fn as_code_point_inversion_list_string_list(
        &'_ self,
    ) -> Option<&'_ CodePointInversionListAndStringList<'data>> {
        match *self {
            Self::CPInversionListStrList(ref l) => Some(l),
            // any other backing data structure that cannot return a CPInversionListStrList in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_inversion_list_string_list(
        &self,
    ) -> CodePointInversionListAndStringList<'_> {
        match *self {
            Self::CPInversionListStrList(ref t) => ZeroFrom::zero_from(t),
        }
    }
}

/// A struct that efficiently stores `Script` and `Script_Extensions` property data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(
    ScriptWithExtensionsPropertyV1Marker,
    "props/scx@1",
    singleton
))]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ScriptWithExtensionsPropertyV1<'data> {
    /// Note: The `ScriptWithExt` values in this array will assume a 12-bit layout. The 2
    /// higher order bits 11..10 will indicate how to deduce the Script value and
    /// Script_Extensions value, nearly matching the representation
    /// [in ICU](https://github.com/unicode-org/icu/blob/main/icu4c/source/common/uprops.h):
    ///
    /// | High order 2 bits value | Script                                                 | Script_Extensions                                              |
    /// |-------------------------|--------------------------------------------------------|----------------------------------------------------------------|
    /// | 3                       | First value in sub-array, index given by lower 10 bits | Sub-array excluding first value, index given by lower 10 bits  |
    /// | 2                       | Script=Inherited                                       | Entire sub-array, index given by lower 10 bits                 |
    /// | 1                       | Script=Common                                          | Entire sub-array, index given by lower 10 bits                 |
    /// | 0                       | Value in lower 10 bits                                 | `[ Script value ]` single-element array                        |
    ///
    /// When the lower 10 bits of the value are used as an index, that index is
    /// used for the outer-level vector of the nested `extensions` structure.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, ScriptWithExt>,

    /// This companion structure stores Script_Extensions values, which are
    /// themselves arrays / vectors. This structure only stores the values for
    /// cases in which `scx(cp) != [ sc(cp) ]`. Each sub-vector is distinct. The
    /// sub-vector represents the Script_Extensions array value for a code point,
    /// and may also indicate Script value, as described for the `trie` field.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub extensions: VarZeroVec<'data, ZeroSlice<Script>>,
}

// See CodePointSetData for documentation of these functions
impl<'data> PropertyCodePointSetV1<'data> {
    #[inline]
    pub(crate) fn contains(&self, ch: char) -> bool {
        match *self {
            Self::InversionList(ref l) => l.contains(ch),
        }
    }

    #[inline]
    pub(crate) fn contains32(&self, ch: u32) -> bool {
        match *self {
            Self::InversionList(ref l) => l.contains32(ch),
        }
    }

    #[inline]
    pub(crate) fn iter_ranges(&self) -> impl Iterator<Item = RangeInclusive<u32>> + '_ {
        match *self {
            Self::InversionList(ref l) => l.iter_ranges(),
        }
    }

    #[inline]
    pub(crate) fn iter_ranges_complemented(
        &self,
    ) -> impl Iterator<Item = RangeInclusive<u32>> + '_ {
        match *self {
            Self::InversionList(ref l) => l.iter_ranges_complemented(),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_inversion_list(l: CodePointInversionList<'static>) -> Self {
        Self::InversionList(l)
    }

    #[inline]
    pub(crate) fn as_code_point_inversion_list(
        &'_ self,
    ) -> Option<&'_ CodePointInversionList<'data>> {
        match *self {
            Self::InversionList(ref l) => Some(l),
            // any other backing data structure that cannot return a CPInvList in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_inversion_list(&self) -> CodePointInversionList<'_> {
        match *self {
            Self::InversionList(ref t) => ZeroFrom::zero_from(t),
        }
    }
}

// See CodePointMapData for documentation of these functions
impl<'data, T: TrieValue> PropertyCodePointMapV1<'data, T> {
    #[inline]
    pub(crate) fn get32(&self, ch: u32) -> T {
        match *self {
            Self::CodePointTrie(ref t) => t.get32(ch),
        }
    }

    #[inline]
    pub(crate) fn try_into_converted<P>(
        self,
    ) -> Result<PropertyCodePointMapV1<'data, P>, ZeroVecError>
    where
        P: TrieValue,
    {
        match self {
            Self::CodePointTrie(t) => t
                .try_into_converted()
                .map(PropertyCodePointMapV1::CodePointTrie),
        }
    }

    #[inline]
    pub(crate) fn get_set_for_value(&self, value: T) -> CodePointInversionList<'static> {
        match *self {
            Self::CodePointTrie(ref t) => t.get_set_for_value(value),
        }
    }

    #[inline]
    pub(crate) fn iter_ranges(&self) -> impl Iterator<Item = CodePointMapRange<T>> + '_ {
        match *self {
            Self::CodePointTrie(ref t) => t.iter_ranges(),
        }
    }
    #[inline]
    pub(crate) fn iter_ranges_mapped<'a, U: Eq + 'a>(
        &'a self,
        map: impl FnMut(T) -> U + Copy + 'a,
    ) -> impl Iterator<Item = CodePointMapRange<U>> + 'a {
        match *self {
            Self::CodePointTrie(ref t) => t.iter_ranges_mapped(map),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_trie(trie: CodePointTrie<'static, T>) -> Self {
        Self::CodePointTrie(trie)
    }

    #[inline]
    pub(crate) fn as_code_point_trie(&self) -> Option<&CodePointTrie<'data, T>> {
        match *self {
            Self::CodePointTrie(ref t) => Some(t),
            // any other backing data structure that cannot return a CPT in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_trie(&self) -> CodePointTrie<'_, T> {
        match *self {
            Self::CodePointTrie(ref t) => ZeroFrom::zero_from(t),
        }
    }
}
