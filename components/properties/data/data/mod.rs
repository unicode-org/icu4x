// @generated
include!("macros.rs");
/// Implement `DataProvider<M>` on the given struct using the data
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
/// Implement `AnyProvider` on the given struct using the data
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
        #[clippy::msrv = "1.65"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::properties::provider::GraphemeClusterBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphemeClusterBreakNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::SentenceBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::SentenceBreakNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::WordBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::WordBreakNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BidiClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BidiClassNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::CanonicalCombiningClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::CanonicalCombiningClassNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EastAsianWidthNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EastAsianWidthNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GeneralCategoryNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GeneralCategoryNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::names::GeneralCategoryMaskNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::names::GeneralCategoryMaskNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::LineBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::LineBreakNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ScriptNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ScriptNameToValueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GraphemeClusterBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphemeClusterBreakValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::SentenceBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::SentenceBreakValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::WordBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::WordBreakValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BidiClassValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BidiClassValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EastAsianWidthValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EastAsianWidthValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GeneralCategoryValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GeneralCategoryValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::LineBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::LineBreakValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ScriptValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ScriptValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::CanonicalCombiningClassValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::CanonicalCombiningClassValueToLongNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::SentenceBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::SentenceBreakValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::WordBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::WordBreakValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BidiClassValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BidiClassValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EastAsianWidthValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EastAsianWidthValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GeneralCategoryValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GeneralCategoryValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::LineBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::LineBreakValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ScriptValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ScriptValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::CanonicalCombiningClassValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::CanonicalCombiningClassValueToShortNameV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::AsciiHexDigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::AsciiHexDigitV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::AlphabeticV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::AlphabeticV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BasicEmojiV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BasicEmojiV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BidiControlV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BidiControlV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BidiMirroredV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BidiMirroredV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::CaseIgnorableV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::CaseIgnorableV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ChangesWhenCasefoldedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ChangesWhenCasefoldedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ChangesWhenCasemappedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ChangesWhenCasemappedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ChangesWhenNfkcCasefoldedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ChangesWhenNfkcCasefoldedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ChangesWhenLowercasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ChangesWhenLowercasedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ChangesWhenTitlecasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ChangesWhenTitlecasedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ChangesWhenUppercasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ChangesWhenUppercasedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::CasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::CasedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::FullCompositionExclusionV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::FullCompositionExclusionV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::DefaultIgnorableCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::DefaultIgnorableCodePointV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::DashV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::DashV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::DeprecatedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::DeprecatedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::DiacriticV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::DiacriticV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EmojiModifierBaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EmojiModifierBaseV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EmojiComponentV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EmojiComponentV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EmojiModifierV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EmojiModifierV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EmojiPresentationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EmojiPresentationV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EmojiV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EmojiV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ExtenderV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ExtenderV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ExtendedPictographicV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ExtendedPictographicV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GraphemeClusterBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphemeClusterBreakV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GraphemeBaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphemeBaseV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GraphemeExtendV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphemeExtendV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GraphemeLinkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphemeLinkV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::HexDigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::HexDigitV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::HyphenV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::HyphenV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::IdContinueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::IdContinueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::IdStartV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::IdStartV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::IdsBinaryOperatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::IdsBinaryOperatorV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::IdsTrinaryOperatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::IdsTrinaryOperatorV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::IdeographicV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::IdeographicV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::JoinControlV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::JoinControlV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::LogicalOrderExceptionV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::LogicalOrderExceptionV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::LowercaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::LowercaseV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::MathV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::MathV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::NoncharacterCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::NoncharacterCodePointV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::PrependedConcatenationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::PrependedConcatenationMarkV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::PatternSyntaxV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::PatternSyntaxV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::PatternWhiteSpaceV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::PatternWhiteSpaceV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::QuotationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::QuotationMarkV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::RegionalIndicatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::RegionalIndicatorV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::RadicalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::RadicalV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::SentenceBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::SentenceBreakV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::SoftDottedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::SoftDottedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::SentenceTerminalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::SentenceTerminalV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::CaseSensitiveV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::CaseSensitiveV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::TerminalPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::TerminalPunctuationV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::UnifiedIdeographV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::UnifiedIdeographV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::UppercaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::UppercaseV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::VariationSelectorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::VariationSelectorV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::WordBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::WordBreakV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::WhiteSpaceV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::WhiteSpaceV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::XidContinueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::XidContinueV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::XidStartV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::XidStartV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::AlnumV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::AlnumV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BidiClassV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BidiClassV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::BlankV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::BlankV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::CanonicalCombiningClassV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::CanonicalCombiningClassV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::EastAsianWidthV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::EastAsianWidthV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ExemplarCharactersIndexV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ExemplarCharactersMainV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ExemplarCharactersMainV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ExemplarCharactersNumbersV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ExemplarCharactersPunctuationV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GeneralCategoryV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GeneralCategoryV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::GraphV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::GraphV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::LineBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::LineBreakV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::NfcInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::NfcInertV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::NfdInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::NfdInertV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::NfkcInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::NfkcInertV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::NfkdInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::NfkdInertV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::PrintV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::PrintV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ScriptV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ScriptV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::ScriptWithExtensionsPropertyV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::ScriptWithExtensionsPropertyV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::SegmentStarterV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::SegmentStarterV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::properties::provider::XdigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::properties::provider::XdigitV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
#[clippy::msrv = "1.65"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
