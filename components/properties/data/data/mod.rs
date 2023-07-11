// @generated
include!("macros.rs");
/// Implement [`DataProvider<M>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_data_provider {
    ($ provider : path) => {
        impl_propnames_from_gcb_v1!($provider);
        impl_propnames_from_sb_v1!($provider);
        impl_propnames_from_wb_v1!($provider);
        impl_propnames_from_bc_v1!($provider);
        impl_propnames_from_ccc_v1!($provider);
        impl_propnames_from_ea_v1!($provider);
        impl_propnames_from_gc_v1!($provider);
        impl_propnames_from_gcm_v1!($provider);
        impl_propnames_from_lb_v1!($provider);
        impl_propnames_from_sc_v1!($provider);
        impl_propnames_to_long_linear_gcb_v1!($provider);
        impl_propnames_to_long_linear_sb_v1!($provider);
        impl_propnames_to_long_linear_wb_v1!($provider);
        impl_propnames_to_long_linear_bc_v1!($provider);
        impl_propnames_to_long_linear_ea_v1!($provider);
        impl_propnames_to_long_linear_gc_v1!($provider);
        impl_propnames_to_long_linear_lb_v1!($provider);
        impl_propnames_to_long_linear_sc_v1!($provider);
        impl_propnames_to_long_sparse_ccc_v1!($provider);
        impl_propnames_to_short_linear_gcb_v1!($provider);
        impl_propnames_to_short_linear_sb_v1!($provider);
        impl_propnames_to_short_linear_wb_v1!($provider);
        impl_propnames_to_short_linear_bc_v1!($provider);
        impl_propnames_to_short_linear_ea_v1!($provider);
        impl_propnames_to_short_linear_gc_v1!($provider);
        impl_propnames_to_short_linear_lb_v1!($provider);
        impl_propnames_to_short_linear4_sc_v1!($provider);
        impl_propnames_to_short_sparse_ccc_v1!($provider);
        impl_props_ahex_v1!($provider);
        impl_props_alpha_v1!($provider);
        impl_props_basic_emoji_v1!($provider);
        impl_props_bidi_c_v1!($provider);
        impl_props_bidi_m_v1!($provider);
        impl_props_ci_v1!($provider);
        impl_props_cwcf_v1!($provider);
        impl_props_cwcm_v1!($provider);
        impl_props_cwkcf_v1!($provider);
        impl_props_cwl_v1!($provider);
        impl_props_cwt_v1!($provider);
        impl_props_cwu_v1!($provider);
        impl_props_cased_v1!($provider);
        impl_props_comp_ex_v1!($provider);
        impl_props_di_v1!($provider);
        impl_props_dash_v1!($provider);
        impl_props_dep_v1!($provider);
        impl_props_dia_v1!($provider);
        impl_props_ebase_v1!($provider);
        impl_props_ecomp_v1!($provider);
        impl_props_emod_v1!($provider);
        impl_props_epres_v1!($provider);
        impl_props_emoji_v1!($provider);
        impl_props_ext_v1!($provider);
        impl_props_extpict_v1!($provider);
        impl_props_gcb_v1!($provider);
        impl_props_gr_base_v1!($provider);
        impl_props_gr_ext_v1!($provider);
        impl_props_gr_link_v1!($provider);
        impl_props_hex_v1!($provider);
        impl_props_hyphen_v1!($provider);
        impl_props_idc_v1!($provider);
        impl_props_ids_v1!($provider);
        impl_props_idsb_v1!($provider);
        impl_props_idst_v1!($provider);
        impl_props_ideo_v1!($provider);
        impl_props_join_c_v1!($provider);
        impl_props_loe_v1!($provider);
        impl_props_lower_v1!($provider);
        impl_props_math_v1!($provider);
        impl_props_nchar_v1!($provider);
        impl_props_pcm_v1!($provider);
        impl_props_pat_syn_v1!($provider);
        impl_props_pat_ws_v1!($provider);
        impl_props_qmark_v1!($provider);
        impl_props_ri_v1!($provider);
        impl_props_radical_v1!($provider);
        impl_props_sb_v1!($provider);
        impl_props_sd_v1!($provider);
        impl_props_sterm_v1!($provider);
        impl_props_sensitive_v1!($provider);
        impl_props_term_v1!($provider);
        impl_props_uideo_v1!($provider);
        impl_props_upper_v1!($provider);
        impl_props_vs_v1!($provider);
        impl_props_wb_v1!($provider);
        impl_props_wspace_v1!($provider);
        impl_props_xidc_v1!($provider);
        impl_props_xids_v1!($provider);
        impl_props_alnum_v1!($provider);
        impl_props_bc_v1!($provider);
        impl_props_bidiauxiliaryprops_v1!($provider);
        impl_props_blank_v1!($provider);
        impl_props_ccc_v1!($provider);
        impl_props_ea_v1!($provider);
        impl_props_exemplarchars_auxiliary_v1!($provider);
        impl_props_exemplarchars_index_v1!($provider);
        impl_props_exemplarchars_main_v1!($provider);
        impl_props_exemplarchars_numbers_v1!($provider);
        impl_props_exemplarchars_punctuation_v1!($provider);
        impl_props_gc_v1!($provider);
        impl_props_graph_v1!($provider);
        impl_props_lb_v1!($provider);
        impl_props_nfcinert_v1!($provider);
        impl_props_nfdinert_v1!($provider);
        impl_props_nfkcinert_v1!($provider);
        impl_props_nfkdinert_v1!($provider);
        impl_props_print_v1!($provider);
        impl_props_sc_v1!($provider);
        impl_props_scx_v1!($provider);
        impl_props_segstart_v1!($provider);
        impl_props_xdigit_v1!($provider);
    };
}
#[doc(inline)]
pub use __impl_data_provider as impl_data_provider;
/// Implement [`AnyProvider`](icu_provider::AnyProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_any` constructors.
///
/// ```compile_fail
/// struct MyAnyProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                const PROPNAMES_FROM_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_GCM_V1: icu_provider::DataKeyHash = <icu_properties::provider::names::GeneralCategoryMaskNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_FROM_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_LINEAR_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_LONG_SPARSE_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_LINEAR4_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPNAMES_TO_SHORT_SPARSE_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_AHEX_V1: icu_provider::DataKeyHash = <icu_properties::provider::AsciiHexDigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_ALPHA_V1: icu_provider::DataKeyHash = <icu_properties::provider::AlphabeticV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_BASIC_EMOJI_V1: icu_provider::DataKeyHash = <icu_properties::provider::BasicEmojiV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_BIDI_C_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiControlV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_BIDI_M_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiMirroredV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CI_V1: icu_provider::DataKeyHash = <icu_properties::provider::CaseIgnorableV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CWCF_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenCasefoldedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CWCM_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenCasemappedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CWKCF_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CWL_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenLowercasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CWT_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenTitlecasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CWU_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenUppercasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CASED_V1: icu_provider::DataKeyHash = <icu_properties::provider::CasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_COMP_EX_V1: icu_provider::DataKeyHash = <icu_properties::provider::FullCompositionExclusionV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_DI_V1: icu_provider::DataKeyHash = <icu_properties::provider::DefaultIgnorableCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_DASH_V1: icu_provider::DataKeyHash = <icu_properties::provider::DashV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_DEP_V1: icu_provider::DataKeyHash = <icu_properties::provider::DeprecatedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_DIA_V1: icu_provider::DataKeyHash = <icu_properties::provider::DiacriticV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EBASE_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiModifierBaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_ECOMP_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiComponentV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EMOD_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiModifierV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EPRES_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiPresentationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EMOJI_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EXT_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExtenderV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EXTPICT_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExtendedPictographicV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_GR_BASE_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeBaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_GR_EXT_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeExtendV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_GR_LINK_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeLinkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_HEX_V1: icu_provider::DataKeyHash = <icu_properties::provider::HexDigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_HYPHEN_V1: icu_provider::DataKeyHash = <icu_properties::provider::HyphenV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_IDC_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdContinueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_IDS_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdStartV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_IDSB_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdsBinaryOperatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_IDST_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdsTrinaryOperatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_IDEO_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdeographicV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_JOIN_C_V1: icu_provider::DataKeyHash = <icu_properties::provider::JoinControlV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_LOE_V1: icu_provider::DataKeyHash = <icu_properties::provider::LogicalOrderExceptionV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_LOWER_V1: icu_provider::DataKeyHash = <icu_properties::provider::LowercaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_MATH_V1: icu_provider::DataKeyHash = <icu_properties::provider::MathV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_NCHAR_V1: icu_provider::DataKeyHash = <icu_properties::provider::NoncharacterCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_PCM_V1: icu_provider::DataKeyHash = <icu_properties::provider::PrependedConcatenationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_PAT_SYN_V1: icu_provider::DataKeyHash = <icu_properties::provider::PatternSyntaxV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_PAT_WS_V1: icu_provider::DataKeyHash = <icu_properties::provider::PatternWhiteSpaceV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_QMARK_V1: icu_provider::DataKeyHash = <icu_properties::provider::QuotationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_RI_V1: icu_provider::DataKeyHash = <icu_properties::provider::RegionalIndicatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_RADICAL_V1: icu_provider::DataKeyHash = <icu_properties::provider::RadicalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_SD_V1: icu_provider::DataKeyHash = <icu_properties::provider::SoftDottedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_STERM_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceTerminalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_SENSITIVE_V1: icu_provider::DataKeyHash = <icu_properties::provider::CaseSensitiveV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_TERM_V1: icu_provider::DataKeyHash = <icu_properties::provider::TerminalPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_UIDEO_V1: icu_provider::DataKeyHash = <icu_properties::provider::UnifiedIdeographV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_UPPER_V1: icu_provider::DataKeyHash = <icu_properties::provider::UppercaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_VS_V1: icu_provider::DataKeyHash = <icu_properties::provider::VariationSelectorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_WSPACE_V1: icu_provider::DataKeyHash = <icu_properties::provider::WhiteSpaceV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_XIDC_V1: icu_provider::DataKeyHash = <icu_properties::provider::XidContinueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_XIDS_V1: icu_provider::DataKeyHash = <icu_properties::provider::XidStartV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_ALNUM_V1: icu_provider::DataKeyHash = <icu_properties::provider::AlnumV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_BIDIAUXILIARYPROPS_V1: icu_provider::DataKeyHash = <icu_properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_BLANK_V1: icu_provider::DataKeyHash = <icu_properties::provider::BlankV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EXEMPLARCHARS_AUXILIARY_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EXEMPLARCHARS_INDEX_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EXEMPLARCHARS_MAIN_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersMainV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EXEMPLARCHARS_NUMBERS_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_EXEMPLARCHARS_PUNCTUATION_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_GRAPH_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_NFCINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfcInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_NFDINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfdInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_NFKCINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfkcInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_NFKDINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfkdInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_PRINT_V1: icu_provider::DataKeyHash = <icu_properties::provider::PrintV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_SCX_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptWithExtensionsPropertyV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_SEGSTART_V1: icu_provider::DataKeyHash = <icu_properties::provider::SegmentStarterV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const PROPS_XDIGIT_V1: icu_provider::DataKeyHash = <icu_properties::provider::XdigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    PROPNAMES_FROM_GCB_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_SB_V1 => icu_provider::DataProvider::<icu_properties::provider::SentenceBreakNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_WB_V1 => icu_provider::DataProvider::<icu_properties::provider::WordBreakNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_BC_V1 => icu_provider::DataProvider::<icu_properties::provider::BidiClassNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_CCC_V1 => icu_provider::DataProvider::<icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_EA_V1 => icu_provider::DataProvider::<icu_properties::provider::EastAsianWidthNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_GC_V1 => icu_provider::DataProvider::<icu_properties::provider::GeneralCategoryNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_GCM_V1 => icu_provider::DataProvider::<icu_properties::provider::names::GeneralCategoryMaskNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_LB_V1 => icu_provider::DataProvider::<icu_properties::provider::LineBreakNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_FROM_SC_V1 => icu_provider::DataProvider::<icu_properties::provider::ScriptNameToValueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_GCB_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphemeClusterBreakValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_SB_V1 => icu_provider::DataProvider::<icu_properties::provider::SentenceBreakValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_WB_V1 => icu_provider::DataProvider::<icu_properties::provider::WordBreakValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_BC_V1 => icu_provider::DataProvider::<icu_properties::provider::BidiClassValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_EA_V1 => icu_provider::DataProvider::<icu_properties::provider::EastAsianWidthValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_GC_V1 => icu_provider::DataProvider::<icu_properties::provider::GeneralCategoryValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_LB_V1 => icu_provider::DataProvider::<icu_properties::provider::LineBreakValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_LINEAR_SC_V1 => icu_provider::DataProvider::<icu_properties::provider::ScriptValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_LONG_SPARSE_CCC_V1 => icu_provider::DataProvider::<icu_properties::provider::CanonicalCombiningClassValueToLongNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR_GCB_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphemeClusterBreakValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR_SB_V1 => icu_provider::DataProvider::<icu_properties::provider::SentenceBreakValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR_WB_V1 => icu_provider::DataProvider::<icu_properties::provider::WordBreakValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR_BC_V1 => icu_provider::DataProvider::<icu_properties::provider::BidiClassValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR_EA_V1 => icu_provider::DataProvider::<icu_properties::provider::EastAsianWidthValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR_GC_V1 => icu_provider::DataProvider::<icu_properties::provider::GeneralCategoryValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR_LB_V1 => icu_provider::DataProvider::<icu_properties::provider::LineBreakValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_LINEAR4_SC_V1 => icu_provider::DataProvider::<icu_properties::provider::ScriptValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPNAMES_TO_SHORT_SPARSE_CCC_V1 => icu_provider::DataProvider::<icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_AHEX_V1 => icu_provider::DataProvider::<icu_properties::provider::AsciiHexDigitV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_ALPHA_V1 => icu_provider::DataProvider::<icu_properties::provider::AlphabeticV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_BASIC_EMOJI_V1 => icu_provider::DataProvider::<icu_properties::provider::BasicEmojiV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_BIDI_C_V1 => icu_provider::DataProvider::<icu_properties::provider::BidiControlV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_BIDI_M_V1 => icu_provider::DataProvider::<icu_properties::provider::BidiMirroredV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CI_V1 => icu_provider::DataProvider::<icu_properties::provider::CaseIgnorableV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CWCF_V1 => icu_provider::DataProvider::<icu_properties::provider::ChangesWhenCasefoldedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CWCM_V1 => icu_provider::DataProvider::<icu_properties::provider::ChangesWhenCasemappedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CWKCF_V1 => icu_provider::DataProvider::<icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CWL_V1 => icu_provider::DataProvider::<icu_properties::provider::ChangesWhenLowercasedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CWT_V1 => icu_provider::DataProvider::<icu_properties::provider::ChangesWhenTitlecasedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CWU_V1 => icu_provider::DataProvider::<icu_properties::provider::ChangesWhenUppercasedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CASED_V1 => icu_provider::DataProvider::<icu_properties::provider::CasedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_COMP_EX_V1 => icu_provider::DataProvider::<icu_properties::provider::FullCompositionExclusionV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_DI_V1 => icu_provider::DataProvider::<icu_properties::provider::DefaultIgnorableCodePointV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_DASH_V1 => icu_provider::DataProvider::<icu_properties::provider::DashV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_DEP_V1 => icu_provider::DataProvider::<icu_properties::provider::DeprecatedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_DIA_V1 => icu_provider::DataProvider::<icu_properties::provider::DiacriticV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EBASE_V1 => icu_provider::DataProvider::<icu_properties::provider::EmojiModifierBaseV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_ECOMP_V1 => icu_provider::DataProvider::<icu_properties::provider::EmojiComponentV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EMOD_V1 => icu_provider::DataProvider::<icu_properties::provider::EmojiModifierV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EPRES_V1 => icu_provider::DataProvider::<icu_properties::provider::EmojiPresentationV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EMOJI_V1 => icu_provider::DataProvider::<icu_properties::provider::EmojiV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EXT_V1 => icu_provider::DataProvider::<icu_properties::provider::ExtenderV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EXTPICT_V1 => icu_provider::DataProvider::<icu_properties::provider::ExtendedPictographicV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_GCB_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphemeClusterBreakV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_GR_BASE_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphemeBaseV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_GR_EXT_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphemeExtendV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_GR_LINK_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphemeLinkV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_HEX_V1 => icu_provider::DataProvider::<icu_properties::provider::HexDigitV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_HYPHEN_V1 => icu_provider::DataProvider::<icu_properties::provider::HyphenV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_IDC_V1 => icu_provider::DataProvider::<icu_properties::provider::IdContinueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_IDS_V1 => icu_provider::DataProvider::<icu_properties::provider::IdStartV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_IDSB_V1 => icu_provider::DataProvider::<icu_properties::provider::IdsBinaryOperatorV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_IDST_V1 => icu_provider::DataProvider::<icu_properties::provider::IdsTrinaryOperatorV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_IDEO_V1 => icu_provider::DataProvider::<icu_properties::provider::IdeographicV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_JOIN_C_V1 => icu_provider::DataProvider::<icu_properties::provider::JoinControlV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_LOE_V1 => icu_provider::DataProvider::<icu_properties::provider::LogicalOrderExceptionV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_LOWER_V1 => icu_provider::DataProvider::<icu_properties::provider::LowercaseV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_MATH_V1 => icu_provider::DataProvider::<icu_properties::provider::MathV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_NCHAR_V1 => icu_provider::DataProvider::<icu_properties::provider::NoncharacterCodePointV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_PCM_V1 => icu_provider::DataProvider::<icu_properties::provider::PrependedConcatenationMarkV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_PAT_SYN_V1 => icu_provider::DataProvider::<icu_properties::provider::PatternSyntaxV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_PAT_WS_V1 => icu_provider::DataProvider::<icu_properties::provider::PatternWhiteSpaceV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_QMARK_V1 => icu_provider::DataProvider::<icu_properties::provider::QuotationMarkV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_RI_V1 => icu_provider::DataProvider::<icu_properties::provider::RegionalIndicatorV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_RADICAL_V1 => icu_provider::DataProvider::<icu_properties::provider::RadicalV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_SB_V1 => icu_provider::DataProvider::<icu_properties::provider::SentenceBreakV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_SD_V1 => icu_provider::DataProvider::<icu_properties::provider::SoftDottedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_STERM_V1 => icu_provider::DataProvider::<icu_properties::provider::SentenceTerminalV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_SENSITIVE_V1 => icu_provider::DataProvider::<icu_properties::provider::CaseSensitiveV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_TERM_V1 => icu_provider::DataProvider::<icu_properties::provider::TerminalPunctuationV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_UIDEO_V1 => icu_provider::DataProvider::<icu_properties::provider::UnifiedIdeographV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_UPPER_V1 => icu_provider::DataProvider::<icu_properties::provider::UppercaseV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_VS_V1 => icu_provider::DataProvider::<icu_properties::provider::VariationSelectorV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_WB_V1 => icu_provider::DataProvider::<icu_properties::provider::WordBreakV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_WSPACE_V1 => icu_provider::DataProvider::<icu_properties::provider::WhiteSpaceV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_XIDC_V1 => icu_provider::DataProvider::<icu_properties::provider::XidContinueV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_XIDS_V1 => icu_provider::DataProvider::<icu_properties::provider::XidStartV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_ALNUM_V1 => icu_provider::DataProvider::<icu_properties::provider::AlnumV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_BC_V1 => icu_provider::DataProvider::<icu_properties::provider::BidiClassV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_BIDIAUXILIARYPROPS_V1 => icu_provider::DataProvider::<icu_properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_BLANK_V1 => icu_provider::DataProvider::<icu_properties::provider::BlankV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_CCC_V1 => icu_provider::DataProvider::<icu_properties::provider::CanonicalCombiningClassV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EA_V1 => icu_provider::DataProvider::<icu_properties::provider::EastAsianWidthV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EXEMPLARCHARS_AUXILIARY_V1 => icu_provider::DataProvider::<icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EXEMPLARCHARS_INDEX_V1 => icu_provider::DataProvider::<icu_properties::provider::ExemplarCharactersIndexV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EXEMPLARCHARS_MAIN_V1 => icu_provider::DataProvider::<icu_properties::provider::ExemplarCharactersMainV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EXEMPLARCHARS_NUMBERS_V1 => icu_provider::DataProvider::<icu_properties::provider::ExemplarCharactersNumbersV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_EXEMPLARCHARS_PUNCTUATION_V1 => icu_provider::DataProvider::<icu_properties::provider::ExemplarCharactersPunctuationV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_GC_V1 => icu_provider::DataProvider::<icu_properties::provider::GeneralCategoryV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_GRAPH_V1 => icu_provider::DataProvider::<icu_properties::provider::GraphV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_LB_V1 => icu_provider::DataProvider::<icu_properties::provider::LineBreakV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_NFCINERT_V1 => icu_provider::DataProvider::<icu_properties::provider::NfcInertV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_NFDINERT_V1 => icu_provider::DataProvider::<icu_properties::provider::NfdInertV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_NFKCINERT_V1 => icu_provider::DataProvider::<icu_properties::provider::NfkcInertV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_NFKDINERT_V1 => icu_provider::DataProvider::<icu_properties::provider::NfkdInertV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_PRINT_V1 => icu_provider::DataProvider::<icu_properties::provider::PrintV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_SC_V1 => icu_provider::DataProvider::<icu_properties::provider::ScriptV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_SCX_V1 => icu_provider::DataProvider::<icu_properties::provider::ScriptWithExtensionsPropertyV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_SEGSTART_V1 => icu_provider::DataProvider::<icu_properties::provider::SegmentStarterV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    PROPS_XDIGIT_V1 => icu_provider::DataProvider::<icu_properties::provider::XdigitV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
#[clippy::msrv = "1.61"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
