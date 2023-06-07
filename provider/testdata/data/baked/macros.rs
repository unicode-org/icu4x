// @generated
#[macro_use]
mod macros {
    #[macro_use]
    mod calendar_japanese_v1;
    #[macro_use]
    mod calendar_japanext_v1;
    #[macro_use]
    mod collator_data_v1;
    #[macro_use]
    mod collator_dia_v1;
    #[macro_use]
    mod collator_jamo_v1;
    #[macro_use]
    mod collator_meta_v1;
    #[macro_use]
    mod collator_prim_v1;
    #[macro_use]
    mod collator_reord_v1;
    #[macro_use]
    mod compactdecimal_long_v1;
    #[macro_use]
    mod compactdecimal_short_v1;
    #[macro_use]
    mod core_helloworld_v1;
    #[macro_use]
    mod datetime_buddhist_datelengths_v1;
    #[macro_use]
    mod datetime_buddhist_datesymbols_v1;
    #[macro_use]
    mod datetime_coptic_datelengths_v1;
    #[macro_use]
    mod datetime_coptic_datesymbols_v1;
    #[macro_use]
    mod datetime_ethiopic_datelengths_v1;
    #[macro_use]
    mod datetime_ethiopic_datesymbols_v1;
    #[macro_use]
    mod datetime_gregory_datelengths_v1;
    #[macro_use]
    mod datetime_gregory_datesymbols_v1;
    #[macro_use]
    mod datetime_indian_datelengths_v1;
    #[macro_use]
    mod datetime_indian_datesymbols_v1;
    #[macro_use]
    mod datetime_japanese_datelengths_v1;
    #[macro_use]
    mod datetime_japanese_datesymbols_v1;
    #[macro_use]
    mod datetime_japanext_datelengths_v1;
    #[macro_use]
    mod datetime_japanext_datesymbols_v1;
    #[macro_use]
    mod datetime_skeletons_v1;
    #[macro_use]
    mod datetime_timelengths_v1;
    #[macro_use]
    mod datetime_timesymbols_v1;
    #[macro_use]
    mod datetime_week_data_v1;
    #[macro_use]
    mod decimal_symbols_v1;
    #[macro_use]
    mod displaynames_languages_v1;
    #[macro_use]
    mod displaynames_locales_v1;
    #[macro_use]
    mod displaynames_regions_v1;
    #[macro_use]
    mod displaynames_scripts_v1;
    #[macro_use]
    mod displaynames_variants_v1;
    #[macro_use]
    mod fallback_likelysubtags_v1;
    #[macro_use]
    mod fallback_parents_v1;
    #[macro_use]
    mod fallback_supplement_co_v1;
    #[macro_use]
    mod list_and_v1;
    #[macro_use]
    mod list_or_v1;
    #[macro_use]
    mod list_unit_v1;
    #[macro_use]
    mod locid_transform_aliases_v1;
    #[macro_use]
    mod locid_transform_likelysubtags_v1;
    #[macro_use]
    mod locid_transform_likelysubtags_ext_v1;
    #[macro_use]
    mod locid_transform_likelysubtags_l_v1;
    #[macro_use]
    mod locid_transform_likelysubtags_sr_v1;
    #[macro_use]
    mod normalizer_comp_v1;
    #[macro_use]
    mod normalizer_decomp_v1;
    #[macro_use]
    mod normalizer_nfd_v1;
    #[macro_use]
    mod normalizer_nfdex_v1;
    #[macro_use]
    mod normalizer_nfkd_v1;
    #[macro_use]
    mod normalizer_nfkdex_v1;
    #[macro_use]
    mod normalizer_uts46d_v1;
    #[macro_use]
    mod plurals_cardinal_v1;
    #[macro_use]
    mod plurals_ordinal_v1;
    #[macro_use]
    mod propnames_from_gcb_v1;
    #[macro_use]
    mod propnames_from_sb_v1;
    #[macro_use]
    mod propnames_from_wb_v1;
    #[macro_use]
    mod propnames_from_bc_v1;
    #[macro_use]
    mod propnames_from_ccc_v1;
    #[macro_use]
    mod propnames_from_ea_v1;
    #[macro_use]
    mod propnames_from_gc_v1;
    #[macro_use]
    mod propnames_from_gcm_v1;
    #[macro_use]
    mod propnames_from_lb_v1;
    #[macro_use]
    mod propnames_from_sc_v1;
    #[macro_use]
    mod propnames_to_long_linear_gcb_v1;
    #[macro_use]
    mod propnames_to_long_linear_sb_v1;
    #[macro_use]
    mod propnames_to_long_linear_wb_v1;
    #[macro_use]
    mod propnames_to_long_linear_bc_v1;
    #[macro_use]
    mod propnames_to_long_linear_ea_v1;
    #[macro_use]
    mod propnames_to_long_linear_gc_v1;
    #[macro_use]
    mod propnames_to_long_linear_lb_v1;
    #[macro_use]
    mod propnames_to_long_linear_sc_v1;
    #[macro_use]
    mod propnames_to_long_sparse_ccc_v1;
    #[macro_use]
    mod propnames_to_short_linear_gcb_v1;
    #[macro_use]
    mod propnames_to_short_linear_sb_v1;
    #[macro_use]
    mod propnames_to_short_linear_wb_v1;
    #[macro_use]
    mod propnames_to_short_linear_bc_v1;
    #[macro_use]
    mod propnames_to_short_linear_ea_v1;
    #[macro_use]
    mod propnames_to_short_linear_gc_v1;
    #[macro_use]
    mod propnames_to_short_linear_lb_v1;
    #[macro_use]
    mod propnames_to_short_linear4_sc_v1;
    #[macro_use]
    mod propnames_to_short_sparse_ccc_v1;
    #[macro_use]
    mod props_ahex_v1;
    #[macro_use]
    mod props_alpha_v1;
    #[macro_use]
    mod props_basic_emoji_v1;
    #[macro_use]
    mod props_bidi_c_v1;
    #[macro_use]
    mod props_bidi_m_v1;
    #[macro_use]
    mod props_ci_v1;
    #[macro_use]
    mod props_cwcf_v1;
    #[macro_use]
    mod props_cwcm_v1;
    #[macro_use]
    mod props_cwkcf_v1;
    #[macro_use]
    mod props_cwl_v1;
    #[macro_use]
    mod props_cwt_v1;
    #[macro_use]
    mod props_cwu_v1;
    #[macro_use]
    mod props_cased_v1;
    #[macro_use]
    mod props_comp_ex_v1;
    #[macro_use]
    mod props_di_v1;
    #[macro_use]
    mod props_dash_v1;
    #[macro_use]
    mod props_dep_v1;
    #[macro_use]
    mod props_dia_v1;
    #[macro_use]
    mod props_ebase_v1;
    #[macro_use]
    mod props_ecomp_v1;
    #[macro_use]
    mod props_emod_v1;
    #[macro_use]
    mod props_epres_v1;
    #[macro_use]
    mod props_emoji_v1;
    #[macro_use]
    mod props_ext_v1;
    #[macro_use]
    mod props_extpict_v1;
    #[macro_use]
    mod props_gcb_v1;
    #[macro_use]
    mod props_gr_base_v1;
    #[macro_use]
    mod props_gr_ext_v1;
    #[macro_use]
    mod props_gr_link_v1;
    #[macro_use]
    mod props_hex_v1;
    #[macro_use]
    mod props_hyphen_v1;
    #[macro_use]
    mod props_idc_v1;
    #[macro_use]
    mod props_ids_v1;
    #[macro_use]
    mod props_idsb_v1;
    #[macro_use]
    mod props_idst_v1;
    #[macro_use]
    mod props_ideo_v1;
    #[macro_use]
    mod props_join_c_v1;
    #[macro_use]
    mod props_loe_v1;
    #[macro_use]
    mod props_lower_v1;
    #[macro_use]
    mod props_math_v1;
    #[macro_use]
    mod props_nchar_v1;
    #[macro_use]
    mod props_pcm_v1;
    #[macro_use]
    mod props_pat_syn_v1;
    #[macro_use]
    mod props_pat_ws_v1;
    #[macro_use]
    mod props_qmark_v1;
    #[macro_use]
    mod props_ri_v1;
    #[macro_use]
    mod props_radical_v1;
    #[macro_use]
    mod props_sb_v1;
    #[macro_use]
    mod props_sd_v1;
    #[macro_use]
    mod props_sterm_v1;
    #[macro_use]
    mod props_sensitive_v1;
    #[macro_use]
    mod props_term_v1;
    #[macro_use]
    mod props_uideo_v1;
    #[macro_use]
    mod props_upper_v1;
    #[macro_use]
    mod props_vs_v1;
    #[macro_use]
    mod props_wb_v1;
    #[macro_use]
    mod props_wspace_v1;
    #[macro_use]
    mod props_xidc_v1;
    #[macro_use]
    mod props_xids_v1;
    #[macro_use]
    mod props_alnum_v1;
    #[macro_use]
    mod props_bc_v1;
    #[macro_use]
    mod props_bidiauxiliaryprops_v1;
    #[macro_use]
    mod props_blank_v1;
    #[macro_use]
    mod props_casemap_v1;
    #[macro_use]
    mod props_ccc_v1;
    #[macro_use]
    mod props_ea_v1;
    #[macro_use]
    mod props_exemplarchars_auxiliary_v1;
    #[macro_use]
    mod props_exemplarchars_index_v1;
    #[macro_use]
    mod props_exemplarchars_main_v1;
    #[macro_use]
    mod props_exemplarchars_numbers_v1;
    #[macro_use]
    mod props_exemplarchars_punctuation_v1;
    #[macro_use]
    mod props_gc_v1;
    #[macro_use]
    mod props_graph_v1;
    #[macro_use]
    mod props_lb_v1;
    #[macro_use]
    mod props_nfcinert_v1;
    #[macro_use]
    mod props_nfdinert_v1;
    #[macro_use]
    mod props_nfkcinert_v1;
    #[macro_use]
    mod props_nfkdinert_v1;
    #[macro_use]
    mod props_print_v1;
    #[macro_use]
    mod props_sc_v1;
    #[macro_use]
    mod props_scx_v1;
    #[macro_use]
    mod props_segstart_v1;
    #[macro_use]
    mod props_xdigit_v1;
    #[macro_use]
    mod relativetime_long_day_v1;
    #[macro_use]
    mod relativetime_long_hour_v1;
    #[macro_use]
    mod relativetime_long_minute_v1;
    #[macro_use]
    mod relativetime_long_month_v1;
    #[macro_use]
    mod relativetime_long_quarter_v1;
    #[macro_use]
    mod relativetime_long_second_v1;
    #[macro_use]
    mod relativetime_long_week_v1;
    #[macro_use]
    mod relativetime_long_year_v1;
    #[macro_use]
    mod relativetime_narrow_day_v1;
    #[macro_use]
    mod relativetime_narrow_hour_v1;
    #[macro_use]
    mod relativetime_narrow_minute_v1;
    #[macro_use]
    mod relativetime_narrow_month_v1;
    #[macro_use]
    mod relativetime_narrow_quarter_v1;
    #[macro_use]
    mod relativetime_narrow_second_v1;
    #[macro_use]
    mod relativetime_narrow_week_v1;
    #[macro_use]
    mod relativetime_narrow_year_v1;
    #[macro_use]
    mod relativetime_short_day_v1;
    #[macro_use]
    mod relativetime_short_hour_v1;
    #[macro_use]
    mod relativetime_short_minute_v1;
    #[macro_use]
    mod relativetime_short_month_v1;
    #[macro_use]
    mod relativetime_short_quarter_v1;
    #[macro_use]
    mod relativetime_short_second_v1;
    #[macro_use]
    mod relativetime_short_week_v1;
    #[macro_use]
    mod relativetime_short_year_v1;
    #[macro_use]
    mod segmenter_dictionary_w_auto_v1;
    #[macro_use]
    mod segmenter_dictionary_wl_ext_v1;
    #[macro_use]
    mod segmenter_grapheme_v1;
    #[macro_use]
    mod segmenter_line_v1;
    #[macro_use]
    mod segmenter_lstm_wl_auto_v1;
    #[macro_use]
    mod segmenter_sentence_v1;
    #[macro_use]
    mod segmenter_word_v1;
    #[macro_use]
    mod time_zone_exemplar_cities_v1;
    #[macro_use]
    mod time_zone_formats_v1;
    #[macro_use]
    mod time_zone_generic_long_v1;
    #[macro_use]
    mod time_zone_generic_short_v1;
    #[macro_use]
    mod time_zone_metazone_period_v1;
    #[macro_use]
    mod time_zone_specific_long_v1;
    #[macro_use]
    mod time_zone_specific_short_v1;
}
#[doc(inline)]
pub use __impl_calendar_japanese_v1 as impl_calendar_japanese_v1;
#[doc(inline)]
pub use __impl_calendar_japanext_v1 as impl_calendar_japanext_v1;
#[doc(inline)]
pub use __impl_collator_data_v1 as impl_collator_data_v1;
#[doc(inline)]
pub use __impl_collator_dia_v1 as impl_collator_dia_v1;
#[doc(inline)]
pub use __impl_collator_jamo_v1 as impl_collator_jamo_v1;
#[doc(inline)]
pub use __impl_collator_meta_v1 as impl_collator_meta_v1;
#[doc(inline)]
pub use __impl_collator_prim_v1 as impl_collator_prim_v1;
#[doc(inline)]
pub use __impl_collator_reord_v1 as impl_collator_reord_v1;
#[doc(inline)]
pub use __impl_compactdecimal_long_v1 as impl_compactdecimal_long_v1;
#[doc(inline)]
pub use __impl_compactdecimal_short_v1 as impl_compactdecimal_short_v1;
#[doc(inline)]
pub use __impl_core_helloworld_v1 as impl_core_helloworld_v1;
#[doc(inline)]
pub use __impl_datetime_buddhist_datelengths_v1 as impl_datetime_buddhist_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_buddhist_datesymbols_v1 as impl_datetime_buddhist_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_coptic_datelengths_v1 as impl_datetime_coptic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_coptic_datesymbols_v1 as impl_datetime_coptic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_ethiopic_datelengths_v1 as impl_datetime_ethiopic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_ethiopic_datesymbols_v1 as impl_datetime_ethiopic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_gregory_datelengths_v1 as impl_datetime_gregory_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_gregory_datesymbols_v1 as impl_datetime_gregory_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_indian_datelengths_v1 as impl_datetime_indian_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_indian_datesymbols_v1 as impl_datetime_indian_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_japanese_datelengths_v1 as impl_datetime_japanese_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_japanese_datesymbols_v1 as impl_datetime_japanese_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_japanext_datelengths_v1 as impl_datetime_japanext_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_japanext_datesymbols_v1 as impl_datetime_japanext_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_skeletons_v1 as impl_datetime_skeletons_v1;
#[doc(inline)]
pub use __impl_datetime_timelengths_v1 as impl_datetime_timelengths_v1;
#[doc(inline)]
pub use __impl_datetime_timesymbols_v1 as impl_datetime_timesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_week_data_v1 as impl_datetime_week_data_v1;
#[doc(inline)]
pub use __impl_decimal_symbols_v1 as impl_decimal_symbols_v1;
#[doc(inline)]
pub use __impl_displaynames_languages_v1 as impl_displaynames_languages_v1;
#[doc(inline)]
pub use __impl_displaynames_locales_v1 as impl_displaynames_locales_v1;
#[doc(inline)]
pub use __impl_displaynames_regions_v1 as impl_displaynames_regions_v1;
#[doc(inline)]
pub use __impl_displaynames_scripts_v1 as impl_displaynames_scripts_v1;
#[doc(inline)]
pub use __impl_displaynames_variants_v1 as impl_displaynames_variants_v1;
#[doc(inline)]
pub use __impl_fallback_likelysubtags_v1 as impl_fallback_likelysubtags_v1;
#[doc(inline)]
pub use __impl_fallback_parents_v1 as impl_fallback_parents_v1;
#[doc(inline)]
pub use __impl_fallback_supplement_co_v1 as impl_fallback_supplement_co_v1;
#[doc(inline)]
pub use __impl_list_and_v1 as impl_list_and_v1;
#[doc(inline)]
pub use __impl_list_or_v1 as impl_list_or_v1;
#[doc(inline)]
pub use __impl_list_unit_v1 as impl_list_unit_v1;
#[doc(inline)]
pub use __impl_locid_transform_aliases_v1 as impl_locid_transform_aliases_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_ext_v1 as impl_locid_transform_likelysubtags_ext_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_l_v1 as impl_locid_transform_likelysubtags_l_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_sr_v1 as impl_locid_transform_likelysubtags_sr_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_v1 as impl_locid_transform_likelysubtags_v1;
#[doc(inline)]
pub use __impl_normalizer_comp_v1 as impl_normalizer_comp_v1;
#[doc(inline)]
pub use __impl_normalizer_decomp_v1 as impl_normalizer_decomp_v1;
#[doc(inline)]
pub use __impl_normalizer_nfd_v1 as impl_normalizer_nfd_v1;
#[doc(inline)]
pub use __impl_normalizer_nfdex_v1 as impl_normalizer_nfdex_v1;
#[doc(inline)]
pub use __impl_normalizer_nfkd_v1 as impl_normalizer_nfkd_v1;
#[doc(inline)]
pub use __impl_normalizer_nfkdex_v1 as impl_normalizer_nfkdex_v1;
#[doc(inline)]
pub use __impl_normalizer_uts46d_v1 as impl_normalizer_uts46d_v1;
#[doc(inline)]
pub use __impl_plurals_cardinal_v1 as impl_plurals_cardinal_v1;
#[doc(inline)]
pub use __impl_plurals_ordinal_v1 as impl_plurals_ordinal_v1;
#[doc(inline)]
pub use __impl_propnames_from_bc_v1 as impl_propnames_from_bc_v1;
#[doc(inline)]
pub use __impl_propnames_from_ccc_v1 as impl_propnames_from_ccc_v1;
#[doc(inline)]
pub use __impl_propnames_from_ea_v1 as impl_propnames_from_ea_v1;
#[doc(inline)]
pub use __impl_propnames_from_gc_v1 as impl_propnames_from_gc_v1;
#[doc(inline)]
pub use __impl_propnames_from_gcb_v1 as impl_propnames_from_gcb_v1;
#[doc(inline)]
pub use __impl_propnames_from_gcm_v1 as impl_propnames_from_gcm_v1;
#[doc(inline)]
pub use __impl_propnames_from_lb_v1 as impl_propnames_from_lb_v1;
#[doc(inline)]
pub use __impl_propnames_from_sb_v1 as impl_propnames_from_sb_v1;
#[doc(inline)]
pub use __impl_propnames_from_sc_v1 as impl_propnames_from_sc_v1;
#[doc(inline)]
pub use __impl_propnames_from_wb_v1 as impl_propnames_from_wb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_bc_v1 as impl_propnames_to_long_linear_bc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_ea_v1 as impl_propnames_to_long_linear_ea_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_gc_v1 as impl_propnames_to_long_linear_gc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_gcb_v1 as impl_propnames_to_long_linear_gcb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_lb_v1 as impl_propnames_to_long_linear_lb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_sb_v1 as impl_propnames_to_long_linear_sb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_sc_v1 as impl_propnames_to_long_linear_sc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_wb_v1 as impl_propnames_to_long_linear_wb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_sparse_ccc_v1 as impl_propnames_to_long_sparse_ccc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear4_sc_v1 as impl_propnames_to_short_linear4_sc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_bc_v1 as impl_propnames_to_short_linear_bc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_ea_v1 as impl_propnames_to_short_linear_ea_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_gc_v1 as impl_propnames_to_short_linear_gc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_gcb_v1 as impl_propnames_to_short_linear_gcb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_lb_v1 as impl_propnames_to_short_linear_lb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_sb_v1 as impl_propnames_to_short_linear_sb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_wb_v1 as impl_propnames_to_short_linear_wb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_sparse_ccc_v1 as impl_propnames_to_short_sparse_ccc_v1;
#[doc(inline)]
pub use __impl_props_ahex_v1 as impl_props_ahex_v1;
#[doc(inline)]
pub use __impl_props_alnum_v1 as impl_props_alnum_v1;
#[doc(inline)]
pub use __impl_props_alpha_v1 as impl_props_alpha_v1;
#[doc(inline)]
pub use __impl_props_basic_emoji_v1 as impl_props_basic_emoji_v1;
#[doc(inline)]
pub use __impl_props_bc_v1 as impl_props_bc_v1;
#[doc(inline)]
pub use __impl_props_bidi_c_v1 as impl_props_bidi_c_v1;
#[doc(inline)]
pub use __impl_props_bidi_m_v1 as impl_props_bidi_m_v1;
#[doc(inline)]
pub use __impl_props_bidiauxiliaryprops_v1 as impl_props_bidiauxiliaryprops_v1;
#[doc(inline)]
pub use __impl_props_blank_v1 as impl_props_blank_v1;
#[doc(inline)]
pub use __impl_props_cased_v1 as impl_props_cased_v1;
#[doc(inline)]
pub use __impl_props_casemap_v1 as impl_props_casemap_v1;
#[doc(inline)]
pub use __impl_props_ccc_v1 as impl_props_ccc_v1;
#[doc(inline)]
pub use __impl_props_ci_v1 as impl_props_ci_v1;
#[doc(inline)]
pub use __impl_props_comp_ex_v1 as impl_props_comp_ex_v1;
#[doc(inline)]
pub use __impl_props_cwcf_v1 as impl_props_cwcf_v1;
#[doc(inline)]
pub use __impl_props_cwcm_v1 as impl_props_cwcm_v1;
#[doc(inline)]
pub use __impl_props_cwkcf_v1 as impl_props_cwkcf_v1;
#[doc(inline)]
pub use __impl_props_cwl_v1 as impl_props_cwl_v1;
#[doc(inline)]
pub use __impl_props_cwt_v1 as impl_props_cwt_v1;
#[doc(inline)]
pub use __impl_props_cwu_v1 as impl_props_cwu_v1;
#[doc(inline)]
pub use __impl_props_dash_v1 as impl_props_dash_v1;
#[doc(inline)]
pub use __impl_props_dep_v1 as impl_props_dep_v1;
#[doc(inline)]
pub use __impl_props_di_v1 as impl_props_di_v1;
#[doc(inline)]
pub use __impl_props_dia_v1 as impl_props_dia_v1;
#[doc(inline)]
pub use __impl_props_ea_v1 as impl_props_ea_v1;
#[doc(inline)]
pub use __impl_props_ebase_v1 as impl_props_ebase_v1;
#[doc(inline)]
pub use __impl_props_ecomp_v1 as impl_props_ecomp_v1;
#[doc(inline)]
pub use __impl_props_emod_v1 as impl_props_emod_v1;
#[doc(inline)]
pub use __impl_props_emoji_v1 as impl_props_emoji_v1;
#[doc(inline)]
pub use __impl_props_epres_v1 as impl_props_epres_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_auxiliary_v1 as impl_props_exemplarchars_auxiliary_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_index_v1 as impl_props_exemplarchars_index_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_main_v1 as impl_props_exemplarchars_main_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_numbers_v1 as impl_props_exemplarchars_numbers_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_punctuation_v1 as impl_props_exemplarchars_punctuation_v1;
#[doc(inline)]
pub use __impl_props_ext_v1 as impl_props_ext_v1;
#[doc(inline)]
pub use __impl_props_extpict_v1 as impl_props_extpict_v1;
#[doc(inline)]
pub use __impl_props_gc_v1 as impl_props_gc_v1;
#[doc(inline)]
pub use __impl_props_gcb_v1 as impl_props_gcb_v1;
#[doc(inline)]
pub use __impl_props_gr_base_v1 as impl_props_gr_base_v1;
#[doc(inline)]
pub use __impl_props_gr_ext_v1 as impl_props_gr_ext_v1;
#[doc(inline)]
pub use __impl_props_gr_link_v1 as impl_props_gr_link_v1;
#[doc(inline)]
pub use __impl_props_graph_v1 as impl_props_graph_v1;
#[doc(inline)]
pub use __impl_props_hex_v1 as impl_props_hex_v1;
#[doc(inline)]
pub use __impl_props_hyphen_v1 as impl_props_hyphen_v1;
#[doc(inline)]
pub use __impl_props_idc_v1 as impl_props_idc_v1;
#[doc(inline)]
pub use __impl_props_ideo_v1 as impl_props_ideo_v1;
#[doc(inline)]
pub use __impl_props_ids_v1 as impl_props_ids_v1;
#[doc(inline)]
pub use __impl_props_idsb_v1 as impl_props_idsb_v1;
#[doc(inline)]
pub use __impl_props_idst_v1 as impl_props_idst_v1;
#[doc(inline)]
pub use __impl_props_join_c_v1 as impl_props_join_c_v1;
#[doc(inline)]
pub use __impl_props_lb_v1 as impl_props_lb_v1;
#[doc(inline)]
pub use __impl_props_loe_v1 as impl_props_loe_v1;
#[doc(inline)]
pub use __impl_props_lower_v1 as impl_props_lower_v1;
#[doc(inline)]
pub use __impl_props_math_v1 as impl_props_math_v1;
#[doc(inline)]
pub use __impl_props_nchar_v1 as impl_props_nchar_v1;
#[doc(inline)]
pub use __impl_props_nfcinert_v1 as impl_props_nfcinert_v1;
#[doc(inline)]
pub use __impl_props_nfdinert_v1 as impl_props_nfdinert_v1;
#[doc(inline)]
pub use __impl_props_nfkcinert_v1 as impl_props_nfkcinert_v1;
#[doc(inline)]
pub use __impl_props_nfkdinert_v1 as impl_props_nfkdinert_v1;
#[doc(inline)]
pub use __impl_props_pat_syn_v1 as impl_props_pat_syn_v1;
#[doc(inline)]
pub use __impl_props_pat_ws_v1 as impl_props_pat_ws_v1;
#[doc(inline)]
pub use __impl_props_pcm_v1 as impl_props_pcm_v1;
#[doc(inline)]
pub use __impl_props_print_v1 as impl_props_print_v1;
#[doc(inline)]
pub use __impl_props_qmark_v1 as impl_props_qmark_v1;
#[doc(inline)]
pub use __impl_props_radical_v1 as impl_props_radical_v1;
#[doc(inline)]
pub use __impl_props_ri_v1 as impl_props_ri_v1;
#[doc(inline)]
pub use __impl_props_sb_v1 as impl_props_sb_v1;
#[doc(inline)]
pub use __impl_props_sc_v1 as impl_props_sc_v1;
#[doc(inline)]
pub use __impl_props_scx_v1 as impl_props_scx_v1;
#[doc(inline)]
pub use __impl_props_sd_v1 as impl_props_sd_v1;
#[doc(inline)]
pub use __impl_props_segstart_v1 as impl_props_segstart_v1;
#[doc(inline)]
pub use __impl_props_sensitive_v1 as impl_props_sensitive_v1;
#[doc(inline)]
pub use __impl_props_sterm_v1 as impl_props_sterm_v1;
#[doc(inline)]
pub use __impl_props_term_v1 as impl_props_term_v1;
#[doc(inline)]
pub use __impl_props_uideo_v1 as impl_props_uideo_v1;
#[doc(inline)]
pub use __impl_props_upper_v1 as impl_props_upper_v1;
#[doc(inline)]
pub use __impl_props_vs_v1 as impl_props_vs_v1;
#[doc(inline)]
pub use __impl_props_wb_v1 as impl_props_wb_v1;
#[doc(inline)]
pub use __impl_props_wspace_v1 as impl_props_wspace_v1;
#[doc(inline)]
pub use __impl_props_xdigit_v1 as impl_props_xdigit_v1;
#[doc(inline)]
pub use __impl_props_xidc_v1 as impl_props_xidc_v1;
#[doc(inline)]
pub use __impl_props_xids_v1 as impl_props_xids_v1;
#[doc(inline)]
pub use __impl_relativetime_long_day_v1 as impl_relativetime_long_day_v1;
#[doc(inline)]
pub use __impl_relativetime_long_hour_v1 as impl_relativetime_long_hour_v1;
#[doc(inline)]
pub use __impl_relativetime_long_minute_v1 as impl_relativetime_long_minute_v1;
#[doc(inline)]
pub use __impl_relativetime_long_month_v1 as impl_relativetime_long_month_v1;
#[doc(inline)]
pub use __impl_relativetime_long_quarter_v1 as impl_relativetime_long_quarter_v1;
#[doc(inline)]
pub use __impl_relativetime_long_second_v1 as impl_relativetime_long_second_v1;
#[doc(inline)]
pub use __impl_relativetime_long_week_v1 as impl_relativetime_long_week_v1;
#[doc(inline)]
pub use __impl_relativetime_long_year_v1 as impl_relativetime_long_year_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_day_v1 as impl_relativetime_narrow_day_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_hour_v1 as impl_relativetime_narrow_hour_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_minute_v1 as impl_relativetime_narrow_minute_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_month_v1 as impl_relativetime_narrow_month_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_quarter_v1 as impl_relativetime_narrow_quarter_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_second_v1 as impl_relativetime_narrow_second_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_week_v1 as impl_relativetime_narrow_week_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_year_v1 as impl_relativetime_narrow_year_v1;
#[doc(inline)]
pub use __impl_relativetime_short_day_v1 as impl_relativetime_short_day_v1;
#[doc(inline)]
pub use __impl_relativetime_short_hour_v1 as impl_relativetime_short_hour_v1;
#[doc(inline)]
pub use __impl_relativetime_short_minute_v1 as impl_relativetime_short_minute_v1;
#[doc(inline)]
pub use __impl_relativetime_short_month_v1 as impl_relativetime_short_month_v1;
#[doc(inline)]
pub use __impl_relativetime_short_quarter_v1 as impl_relativetime_short_quarter_v1;
#[doc(inline)]
pub use __impl_relativetime_short_second_v1 as impl_relativetime_short_second_v1;
#[doc(inline)]
pub use __impl_relativetime_short_week_v1 as impl_relativetime_short_week_v1;
#[doc(inline)]
pub use __impl_relativetime_short_year_v1 as impl_relativetime_short_year_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_w_auto_v1 as impl_segmenter_dictionary_w_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_wl_ext_v1 as impl_segmenter_dictionary_wl_ext_v1;
#[doc(inline)]
pub use __impl_segmenter_grapheme_v1 as impl_segmenter_grapheme_v1;
#[doc(inline)]
pub use __impl_segmenter_line_v1 as impl_segmenter_line_v1;
#[doc(inline)]
pub use __impl_segmenter_lstm_wl_auto_v1 as impl_segmenter_lstm_wl_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_sentence_v1 as impl_segmenter_sentence_v1;
#[doc(inline)]
pub use __impl_segmenter_word_v1 as impl_segmenter_word_v1;
#[doc(inline)]
pub use __impl_time_zone_exemplar_cities_v1 as impl_time_zone_exemplar_cities_v1;
#[doc(inline)]
pub use __impl_time_zone_formats_v1 as impl_time_zone_formats_v1;
#[doc(inline)]
pub use __impl_time_zone_generic_long_v1 as impl_time_zone_generic_long_v1;
#[doc(inline)]
pub use __impl_time_zone_generic_short_v1 as impl_time_zone_generic_short_v1;
#[doc(inline)]
pub use __impl_time_zone_metazone_period_v1 as impl_time_zone_metazone_period_v1;
#[doc(inline)]
pub use __impl_time_zone_specific_long_v1 as impl_time_zone_specific_long_v1;
#[doc(inline)]
pub use __impl_time_zone_specific_short_v1 as impl_time_zone_specific_short_v1;
/// Implement [`DataProvider<M>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/macros.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_data_provider {
    ($ provider : path) => {
        #[cfg(feature = "icu_calendar")]
        impl_calendar_japanese_v1!($provider);
        #[cfg(feature = "icu_calendar")]
        impl_calendar_japanext_v1!($provider);
        #[cfg(feature = "icu_collator")]
        impl_collator_data_v1!($provider);
        #[cfg(feature = "icu_collator")]
        impl_collator_dia_v1!($provider);
        #[cfg(feature = "icu_collator")]
        impl_collator_jamo_v1!($provider);
        #[cfg(feature = "icu_collator")]
        impl_collator_meta_v1!($provider);
        #[cfg(feature = "icu_collator")]
        impl_collator_prim_v1!($provider);
        #[cfg(feature = "icu_collator")]
        impl_collator_reord_v1!($provider);
        #[cfg(feature = "icu_compactdecimal")]
        impl_compactdecimal_long_v1!($provider);
        #[cfg(feature = "icu_compactdecimal")]
        impl_compactdecimal_short_v1!($provider);
        impl_core_helloworld_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_buddhist_datelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_buddhist_datesymbols_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_coptic_datelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_coptic_datesymbols_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_ethiopic_datelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_ethiopic_datesymbols_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_gregory_datelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_gregory_datesymbols_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_indian_datelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_indian_datesymbols_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_japanese_datelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_japanese_datesymbols_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_japanext_datelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_japanext_datesymbols_v1!($provider);
        #[cfg(feature = "icu_datetime_experimental")]
        impl_datetime_skeletons_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_timelengths_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_datetime_timesymbols_v1!($provider);
        #[cfg(feature = "icu_calendar")]
        impl_datetime_week_data_v1!($provider);
        #[cfg(feature = "icu_decimal")]
        impl_decimal_symbols_v1!($provider);
        #[cfg(feature = "icu_displaynames")]
        impl_displaynames_languages_v1!($provider);
        #[cfg(feature = "icu_displaynames")]
        impl_displaynames_locales_v1!($provider);
        #[cfg(feature = "icu_displaynames")]
        impl_displaynames_regions_v1!($provider);
        #[cfg(feature = "icu_displaynames")]
        impl_displaynames_scripts_v1!($provider);
        #[cfg(feature = "icu_displaynames")]
        impl_displaynames_variants_v1!($provider);
        impl_fallback_likelysubtags_v1!($provider);
        impl_fallback_parents_v1!($provider);
        impl_fallback_supplement_co_v1!($provider);
        #[cfg(feature = "icu_list")]
        impl_list_and_v1!($provider);
        #[cfg(feature = "icu_list")]
        impl_list_or_v1!($provider);
        #[cfg(feature = "icu_list")]
        impl_list_unit_v1!($provider);
        #[cfg(feature = "icu_locid_transform")]
        impl_locid_transform_aliases_v1!($provider);
        #[cfg(feature = "icu_locid_transform")]
        impl_locid_transform_likelysubtags_v1!($provider);
        #[cfg(feature = "icu_locid_transform")]
        impl_locid_transform_likelysubtags_ext_v1!($provider);
        #[cfg(feature = "icu_locid_transform")]
        impl_locid_transform_likelysubtags_l_v1!($provider);
        #[cfg(feature = "icu_locid_transform")]
        impl_locid_transform_likelysubtags_sr_v1!($provider);
        #[cfg(feature = "icu_normalizer")]
        impl_normalizer_comp_v1!($provider);
        #[cfg(feature = "icu_normalizer")]
        impl_normalizer_decomp_v1!($provider);
        #[cfg(feature = "icu_normalizer")]
        impl_normalizer_nfd_v1!($provider);
        #[cfg(feature = "icu_normalizer")]
        impl_normalizer_nfdex_v1!($provider);
        #[cfg(feature = "icu_normalizer")]
        impl_normalizer_nfkd_v1!($provider);
        #[cfg(feature = "icu_normalizer")]
        impl_normalizer_nfkdex_v1!($provider);
        #[cfg(feature = "icu_normalizer")]
        impl_normalizer_uts46d_v1!($provider);
        #[cfg(feature = "icu_plurals")]
        impl_plurals_cardinal_v1!($provider);
        #[cfg(feature = "icu_plurals")]
        impl_plurals_ordinal_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_gcb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_sb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_wb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_bc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_ccc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_ea_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_gc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_gcm_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_lb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_from_sc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_gcb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_sb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_wb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_bc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_ea_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_gc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_lb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_linear_sc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_long_sparse_ccc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear_gcb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear_sb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear_wb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear_bc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear_ea_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear_gc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear_lb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_linear4_sc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_propnames_to_short_sparse_ccc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ahex_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_alpha_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_basic_emoji_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_bidi_c_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_bidi_m_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ci_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_cwcf_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_cwcm_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_cwkcf_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_cwl_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_cwt_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_cwu_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_cased_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_comp_ex_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_di_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_dash_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_dep_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_dia_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ebase_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ecomp_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_emod_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_epres_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_emoji_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ext_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_extpict_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_gcb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_gr_base_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_gr_ext_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_gr_link_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_hex_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_hyphen_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_idc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ids_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_idsb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_idst_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ideo_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_join_c_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_loe_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_lower_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_math_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_nchar_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_pcm_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_pat_syn_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_pat_ws_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_qmark_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ri_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_radical_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_sb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_sd_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_sterm_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_sensitive_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_term_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_uideo_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_upper_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_vs_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_wb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_wspace_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_xidc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_xids_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_alnum_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_bc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_bidiauxiliaryprops_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_blank_v1!($provider);
        #[cfg(feature = "icu_casemapping")]
        impl_props_casemap_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ccc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_ea_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_exemplarchars_auxiliary_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_exemplarchars_index_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_exemplarchars_main_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_exemplarchars_numbers_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_exemplarchars_punctuation_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_gc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_graph_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_lb_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_nfcinert_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_nfdinert_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_nfkcinert_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_nfkdinert_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_print_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_sc_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_scx_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_segstart_v1!($provider);
        #[cfg(feature = "icu_properties")]
        impl_props_xdigit_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_day_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_hour_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_minute_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_month_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_quarter_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_second_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_week_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_long_year_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_day_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_hour_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_minute_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_month_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_quarter_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_second_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_week_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_narrow_year_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_day_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_hour_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_minute_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_month_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_quarter_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_second_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_week_v1!($provider);
        #[cfg(feature = "icu_relativetime")]
        impl_relativetime_short_year_v1!($provider);
        #[cfg(feature = "icu_segmenter")]
        impl_segmenter_dictionary_w_auto_v1!($provider);
        #[cfg(feature = "icu_segmenter")]
        impl_segmenter_dictionary_wl_ext_v1!($provider);
        #[cfg(feature = "icu_segmenter")]
        impl_segmenter_grapheme_v1!($provider);
        #[cfg(feature = "icu_segmenter")]
        impl_segmenter_line_v1!($provider);
        #[cfg(feature = "icu_segmenter")]
        impl_segmenter_lstm_wl_auto_v1!($provider);
        #[cfg(feature = "icu_segmenter")]
        impl_segmenter_sentence_v1!($provider);
        #[cfg(feature = "icu_segmenter")]
        impl_segmenter_word_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_time_zone_exemplar_cities_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_time_zone_formats_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_time_zone_generic_long_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_time_zone_generic_short_v1!($provider);
        #[cfg(feature = "icu_timezone")]
        impl_time_zone_metazone_period_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_time_zone_specific_long_v1!($provider);
        #[cfg(feature = "icu_datetime")]
        impl_time_zone_specific_short_v1!($provider);
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
/// include!("/path/to/generated/macros.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                #[cfg(feature = "icu_calendar")]
                const CALENDAR_JAPANESE_V1: icu_provider::DataKeyHash = <icu_calendar::provider::JapaneseErasV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_calendar")]
                const CALENDAR_JAPANEXT_V1: icu_provider::DataKeyHash = <icu_calendar::provider::JapaneseExtendedErasV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATOR_DATA_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATOR_DIA_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationDiacriticsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATOR_JAMO_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationJamoV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATOR_META_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationMetadataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATOR_PRIM_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATOR_REORD_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationReorderingV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_compactdecimal")]
                const COMPACTDECIMAL_LONG_V1: icu_provider::DataKeyHash = <icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_compactdecimal")]
                const COMPACTDECIMAL_SHORT_V1: icu_provider::DataKeyHash = <icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const CORE_HELLOWORLD_V1: icu_provider::DataKeyHash = <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_BUDDHIST_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_BUDDHIST_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_COPTIC_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::CopticDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_COPTIC_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_ETHIOPIC_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_ETHIOPIC_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_GREGORY_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::GregorianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_GREGORY_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_INDIAN_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::IndianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_INDIAN_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::IndianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_JAPANESE_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_JAPANESE_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_JAPANEXT_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_JAPANEXT_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime_experimental")]
                const DATETIME_SKELETONS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_TIMELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const DATETIME_TIMESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::TimeSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_calendar")]
                const DATETIME_WEEK_DATA_V1: icu_provider::DataKeyHash = <icu_calendar::provider::WeekDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_decimal")]
                const DECIMAL_SYMBOLS_V1: icu_provider::DataKeyHash = <icu_decimal::provider::DecimalSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_displaynames")]
                const DISPLAYNAMES_LANGUAGES_V1: icu_provider::DataKeyHash = <icu_displaynames::provider::LanguageDisplayNamesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_displaynames")]
                const DISPLAYNAMES_LOCALES_V1: icu_provider::DataKeyHash = <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_displaynames")]
                const DISPLAYNAMES_REGIONS_V1: icu_provider::DataKeyHash = <icu_displaynames::provider::RegionDisplayNamesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_displaynames")]
                const DISPLAYNAMES_SCRIPTS_V1: icu_provider::DataKeyHash = <icu_displaynames::provider::ScriptDisplayNamesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_displaynames")]
                const DISPLAYNAMES_VARIANTS_V1: icu_provider::DataKeyHash = <icu_displaynames::provider::VariantDisplayNamesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const FALLBACK_LIKELYSUBTAGS_V1: icu_provider::DataKeyHash = <icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const FALLBACK_PARENTS_V1: icu_provider::DataKeyHash = <icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const FALLBACK_SUPPLEMENT_CO_V1: icu_provider::DataKeyHash = <icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_list")]
                const LIST_AND_V1: icu_provider::DataKeyHash = <icu_list::provider::AndListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_list")]
                const LIST_OR_V1: icu_provider::DataKeyHash = <icu_list::provider::OrListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_list")]
                const LIST_UNIT_V1: icu_provider::DataKeyHash = <icu_list::provider::UnitListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_locid_transform")]
                const LOCID_TRANSFORM_ALIASES_V1: icu_provider::DataKeyHash = <icu_locid_transform::provider::AliasesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_locid_transform")]
                const LOCID_TRANSFORM_LIKELYSUBTAGS_V1: icu_provider::DataKeyHash = <icu_locid_transform::provider::LikelySubtagsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_locid_transform")]
                const LOCID_TRANSFORM_LIKELYSUBTAGS_EXT_V1: icu_provider::DataKeyHash = <icu_locid_transform::provider::LikelySubtagsExtendedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_locid_transform")]
                const LOCID_TRANSFORM_LIKELYSUBTAGS_L_V1: icu_provider::DataKeyHash = <icu_locid_transform::provider::LikelySubtagsForLanguageV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_locid_transform")]
                const LOCID_TRANSFORM_LIKELYSUBTAGS_SR_V1: icu_provider::DataKeyHash = <icu_locid_transform::provider::LikelySubtagsForScriptRegionV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NORMALIZER_COMP_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CanonicalCompositionsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NORMALIZER_DECOMP_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NORMALIZER_NFD_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CanonicalDecompositionDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NORMALIZER_NFDEX_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CanonicalDecompositionTablesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NORMALIZER_NFKD_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NORMALIZER_NFKDEX_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NORMALIZER_UTS46D_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::Uts46DecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_plurals")]
                const PLURALS_CARDINAL_V1: icu_provider::DataKeyHash = <icu_plurals::provider::CardinalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_plurals")]
                const PLURALS_ORDINAL_V1: icu_provider::DataKeyHash = <icu_plurals::provider::OrdinalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_GCM_V1: icu_provider::DataKeyHash = <icu_properties::provider::names::GeneralCategoryMaskNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_FROM_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_LINEAR_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_LONG_SPARSE_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_LINEAR4_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPNAMES_TO_SHORT_SPARSE_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_AHEX_V1: icu_provider::DataKeyHash = <icu_properties::provider::AsciiHexDigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_ALPHA_V1: icu_provider::DataKeyHash = <icu_properties::provider::AlphabeticV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_BASIC_EMOJI_V1: icu_provider::DataKeyHash = <icu_properties::provider::BasicEmojiV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_BIDI_C_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiControlV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_BIDI_M_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiMirroredV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CI_V1: icu_provider::DataKeyHash = <icu_properties::provider::CaseIgnorableV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CWCF_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenCasefoldedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CWCM_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenCasemappedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CWKCF_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CWL_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenLowercasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CWT_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenTitlecasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CWU_V1: icu_provider::DataKeyHash = <icu_properties::provider::ChangesWhenUppercasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CASED_V1: icu_provider::DataKeyHash = <icu_properties::provider::CasedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_COMP_EX_V1: icu_provider::DataKeyHash = <icu_properties::provider::FullCompositionExclusionV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_DI_V1: icu_provider::DataKeyHash = <icu_properties::provider::DefaultIgnorableCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_DASH_V1: icu_provider::DataKeyHash = <icu_properties::provider::DashV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_DEP_V1: icu_provider::DataKeyHash = <icu_properties::provider::DeprecatedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_DIA_V1: icu_provider::DataKeyHash = <icu_properties::provider::DiacriticV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EBASE_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiModifierBaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_ECOMP_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiComponentV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EMOD_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiModifierV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EPRES_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiPresentationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EMOJI_V1: icu_provider::DataKeyHash = <icu_properties::provider::EmojiV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EXT_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExtenderV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EXTPICT_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExtendedPictographicV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_GCB_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeClusterBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_GR_BASE_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeBaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_GR_EXT_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeExtendV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_GR_LINK_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphemeLinkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_HEX_V1: icu_provider::DataKeyHash = <icu_properties::provider::HexDigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_HYPHEN_V1: icu_provider::DataKeyHash = <icu_properties::provider::HyphenV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_IDC_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdContinueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_IDS_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdStartV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_IDSB_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdsBinaryOperatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_IDST_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdsTrinaryOperatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_IDEO_V1: icu_provider::DataKeyHash = <icu_properties::provider::IdeographicV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_JOIN_C_V1: icu_provider::DataKeyHash = <icu_properties::provider::JoinControlV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_LOE_V1: icu_provider::DataKeyHash = <icu_properties::provider::LogicalOrderExceptionV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_LOWER_V1: icu_provider::DataKeyHash = <icu_properties::provider::LowercaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_MATH_V1: icu_provider::DataKeyHash = <icu_properties::provider::MathV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_NCHAR_V1: icu_provider::DataKeyHash = <icu_properties::provider::NoncharacterCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_PCM_V1: icu_provider::DataKeyHash = <icu_properties::provider::PrependedConcatenationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_PAT_SYN_V1: icu_provider::DataKeyHash = <icu_properties::provider::PatternSyntaxV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_PAT_WS_V1: icu_provider::DataKeyHash = <icu_properties::provider::PatternWhiteSpaceV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_QMARK_V1: icu_provider::DataKeyHash = <icu_properties::provider::QuotationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_RI_V1: icu_provider::DataKeyHash = <icu_properties::provider::RegionalIndicatorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_RADICAL_V1: icu_provider::DataKeyHash = <icu_properties::provider::RadicalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_SB_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_SD_V1: icu_provider::DataKeyHash = <icu_properties::provider::SoftDottedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_STERM_V1: icu_provider::DataKeyHash = <icu_properties::provider::SentenceTerminalV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_SENSITIVE_V1: icu_provider::DataKeyHash = <icu_properties::provider::CaseSensitiveV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_TERM_V1: icu_provider::DataKeyHash = <icu_properties::provider::TerminalPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_UIDEO_V1: icu_provider::DataKeyHash = <icu_properties::provider::UnifiedIdeographV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_UPPER_V1: icu_provider::DataKeyHash = <icu_properties::provider::UppercaseV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_VS_V1: icu_provider::DataKeyHash = <icu_properties::provider::VariationSelectorV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_WB_V1: icu_provider::DataKeyHash = <icu_properties::provider::WordBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_WSPACE_V1: icu_provider::DataKeyHash = <icu_properties::provider::WhiteSpaceV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_XIDC_V1: icu_provider::DataKeyHash = <icu_properties::provider::XidContinueV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_XIDS_V1: icu_provider::DataKeyHash = <icu_properties::provider::XidStartV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_ALNUM_V1: icu_provider::DataKeyHash = <icu_properties::provider::AlnumV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_BC_V1: icu_provider::DataKeyHash = <icu_properties::provider::BidiClassV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_BIDIAUXILIARYPROPS_V1: icu_provider::DataKeyHash = <icu_properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_BLANK_V1: icu_provider::DataKeyHash = <icu_properties::provider::BlankV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_casemapping")]
                const PROPS_CASEMAP_V1: icu_provider::DataKeyHash = <icu_casemapping::provider::CaseMappingV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_CCC_V1: icu_provider::DataKeyHash = <icu_properties::provider::CanonicalCombiningClassV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EA_V1: icu_provider::DataKeyHash = <icu_properties::provider::EastAsianWidthV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EXEMPLARCHARS_AUXILIARY_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EXEMPLARCHARS_INDEX_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EXEMPLARCHARS_MAIN_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersMainV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EXEMPLARCHARS_NUMBERS_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersNumbersV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_EXEMPLARCHARS_PUNCTUATION_V1: icu_provider::DataKeyHash = <icu_properties::provider::ExemplarCharactersPunctuationV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_GC_V1: icu_provider::DataKeyHash = <icu_properties::provider::GeneralCategoryV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_GRAPH_V1: icu_provider::DataKeyHash = <icu_properties::provider::GraphV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_LB_V1: icu_provider::DataKeyHash = <icu_properties::provider::LineBreakV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_NFCINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfcInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_NFDINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfdInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_NFKCINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfkcInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_NFKDINERT_V1: icu_provider::DataKeyHash = <icu_properties::provider::NfkdInertV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_PRINT_V1: icu_provider::DataKeyHash = <icu_properties::provider::PrintV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_SC_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_SCX_V1: icu_provider::DataKeyHash = <icu_properties::provider::ScriptWithExtensionsPropertyV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_SEGSTART_V1: icu_provider::DataKeyHash = <icu_properties::provider::SegmentStarterV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PROPS_XDIGIT_V1: icu_provider::DataKeyHash = <icu_properties::provider::XdigitV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_DAY_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_HOUR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_MINUTE_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_MONTH_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_QUARTER_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_SECOND_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_WEEK_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_LONG_YEAR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_DAY_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_HOUR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_MINUTE_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_MONTH_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_QUARTER_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_SECOND_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_WEEK_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_NARROW_YEAR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_DAY_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_HOUR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_MINUTE_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_MONTH_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_QUARTER_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_SECOND_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_WEEK_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const RELATIVETIME_SHORT_YEAR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SEGMENTER_DICTIONARY_W_AUTO_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SEGMENTER_DICTIONARY_WL_EXT_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SEGMENTER_GRAPHEME_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::GraphemeClusterBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SEGMENTER_LINE_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::LineBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SEGMENTER_LSTM_WL_AUTO_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::LstmForWordLineAutoV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SEGMENTER_SENTENCE_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::SentenceBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SEGMENTER_WORD_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::WordBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIME_ZONE_EXEMPLAR_CITIES_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::ExemplarCitiesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIME_ZONE_FORMATS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIME_ZONE_GENERIC_LONG_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIME_ZONE_GENERIC_SHORT_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_timezone")]
                const TIME_ZONE_METAZONE_PERIOD_V1: icu_provider::DataKeyHash = <icu_timezone::provider::MetazonePeriodV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIME_ZONE_SPECIFIC_LONG_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIME_ZONE_SPECIFIC_SHORT_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    #[cfg(feature = "icu_calendar")]
                    CALENDAR_JAPANESE_V1 => Self::lookup_calendar_japanese_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_calendar")]
                    CALENDAR_JAPANEXT_V1 => Self::lookup_calendar_japanext_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATOR_DATA_V1 => Self::lookup_collator_data_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATOR_DIA_V1 => Self::lookup_collator_dia_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATOR_JAMO_V1 => Self::lookup_collator_jamo_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATOR_META_V1 => Self::lookup_collator_meta_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATOR_PRIM_V1 => Self::lookup_collator_prim_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATOR_REORD_V1 => Self::lookup_collator_reord_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_compactdecimal")]
                    COMPACTDECIMAL_LONG_V1 => Self::lookup_compactdecimal_long_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_compactdecimal")]
                    COMPACTDECIMAL_SHORT_V1 => Self::lookup_compactdecimal_short_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    CORE_HELLOWORLD_V1 => Self::lookup_core_helloworld_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_BUDDHIST_DATELENGTHS_V1 => Self::lookup_datetime_buddhist_datelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_BUDDHIST_DATESYMBOLS_V1 => Self::lookup_datetime_buddhist_datesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_COPTIC_DATELENGTHS_V1 => Self::lookup_datetime_coptic_datelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_COPTIC_DATESYMBOLS_V1 => Self::lookup_datetime_coptic_datesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_ETHIOPIC_DATELENGTHS_V1 => Self::lookup_datetime_ethiopic_datelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_ETHIOPIC_DATESYMBOLS_V1 => Self::lookup_datetime_ethiopic_datesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_GREGORY_DATELENGTHS_V1 => Self::lookup_datetime_gregory_datelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_GREGORY_DATESYMBOLS_V1 => Self::lookup_datetime_gregory_datesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_INDIAN_DATELENGTHS_V1 => Self::lookup_datetime_indian_datelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_INDIAN_DATESYMBOLS_V1 => Self::lookup_datetime_indian_datesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_JAPANESE_DATELENGTHS_V1 => Self::lookup_datetime_japanese_datelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_JAPANESE_DATESYMBOLS_V1 => Self::lookup_datetime_japanese_datesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_JAPANEXT_DATELENGTHS_V1 => Self::lookup_datetime_japanext_datelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_JAPANEXT_DATESYMBOLS_V1 => Self::lookup_datetime_japanext_datesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime_experimental")]
                    DATETIME_SKELETONS_V1 => Self::lookup_datetime_skeletons_v1(&req.locale).map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::<icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker>::from_owned).map(icu_provider::DataPayload::wrap_into_any_payload),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_TIMELENGTHS_V1 => Self::lookup_datetime_timelengths_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    DATETIME_TIMESYMBOLS_V1 => Self::lookup_datetime_timesymbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_calendar")]
                    DATETIME_WEEK_DATA_V1 => Self::lookup_datetime_week_data_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_decimal")]
                    DECIMAL_SYMBOLS_V1 => Self::lookup_decimal_symbols_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_displaynames")]
                    DISPLAYNAMES_LANGUAGES_V1 => Self::lookup_displaynames_languages_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_displaynames")]
                    DISPLAYNAMES_LOCALES_V1 => Self::lookup_displaynames_locales_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_displaynames")]
                    DISPLAYNAMES_REGIONS_V1 => Self::lookup_displaynames_regions_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_displaynames")]
                    DISPLAYNAMES_SCRIPTS_V1 => Self::lookup_displaynames_scripts_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_displaynames")]
                    DISPLAYNAMES_VARIANTS_V1 => Self::lookup_displaynames_variants_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    FALLBACK_LIKELYSUBTAGS_V1 => Self::lookup_fallback_likelysubtags_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    FALLBACK_PARENTS_V1 => Self::lookup_fallback_parents_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    FALLBACK_SUPPLEMENT_CO_V1 => Self::lookup_fallback_supplement_co_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_list")]
                    LIST_AND_V1 => Self::lookup_list_and_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_list")]
                    LIST_OR_V1 => Self::lookup_list_or_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_list")]
                    LIST_UNIT_V1 => Self::lookup_list_unit_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_locid_transform")]
                    LOCID_TRANSFORM_ALIASES_V1 => Self::lookup_locid_transform_aliases_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_locid_transform")]
                    LOCID_TRANSFORM_LIKELYSUBTAGS_V1 => Self::lookup_locid_transform_likelysubtags_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_locid_transform")]
                    LOCID_TRANSFORM_LIKELYSUBTAGS_EXT_V1 => Self::lookup_locid_transform_likelysubtags_ext_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_locid_transform")]
                    LOCID_TRANSFORM_LIKELYSUBTAGS_L_V1 => Self::lookup_locid_transform_likelysubtags_l_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_locid_transform")]
                    LOCID_TRANSFORM_LIKELYSUBTAGS_SR_V1 => Self::lookup_locid_transform_likelysubtags_sr_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NORMALIZER_COMP_V1 => Self::lookup_normalizer_comp_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NORMALIZER_DECOMP_V1 => Self::lookup_normalizer_decomp_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NORMALIZER_NFD_V1 => Self::lookup_normalizer_nfd_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NORMALIZER_NFDEX_V1 => Self::lookup_normalizer_nfdex_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NORMALIZER_NFKD_V1 => Self::lookup_normalizer_nfkd_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NORMALIZER_NFKDEX_V1 => Self::lookup_normalizer_nfkdex_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NORMALIZER_UTS46D_V1 => Self::lookup_normalizer_uts46d_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_plurals")]
                    PLURALS_CARDINAL_V1 => Self::lookup_plurals_cardinal_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_plurals")]
                    PLURALS_ORDINAL_V1 => Self::lookup_plurals_ordinal_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_GCB_V1 => Self::lookup_propnames_from_gcb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_SB_V1 => Self::lookup_propnames_from_sb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_WB_V1 => Self::lookup_propnames_from_wb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_BC_V1 => Self::lookup_propnames_from_bc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_CCC_V1 => Self::lookup_propnames_from_ccc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_EA_V1 => Self::lookup_propnames_from_ea_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_GC_V1 => Self::lookup_propnames_from_gc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_GCM_V1 => Self::lookup_propnames_from_gcm_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_LB_V1 => Self::lookup_propnames_from_lb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_FROM_SC_V1 => Self::lookup_propnames_from_sc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_GCB_V1 => Self::lookup_propnames_to_long_linear_gcb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_SB_V1 => Self::lookup_propnames_to_long_linear_sb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_WB_V1 => Self::lookup_propnames_to_long_linear_wb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_BC_V1 => Self::lookup_propnames_to_long_linear_bc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_EA_V1 => Self::lookup_propnames_to_long_linear_ea_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_GC_V1 => Self::lookup_propnames_to_long_linear_gc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_LB_V1 => Self::lookup_propnames_to_long_linear_lb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_LINEAR_SC_V1 => Self::lookup_propnames_to_long_linear_sc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_LONG_SPARSE_CCC_V1 => Self::lookup_propnames_to_long_sparse_ccc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR_GCB_V1 => Self::lookup_propnames_to_short_linear_gcb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR_SB_V1 => Self::lookup_propnames_to_short_linear_sb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR_WB_V1 => Self::lookup_propnames_to_short_linear_wb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR_BC_V1 => Self::lookup_propnames_to_short_linear_bc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR_EA_V1 => Self::lookup_propnames_to_short_linear_ea_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR_GC_V1 => Self::lookup_propnames_to_short_linear_gc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR_LB_V1 => Self::lookup_propnames_to_short_linear_lb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_LINEAR4_SC_V1 => Self::lookup_propnames_to_short_linear4_sc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPNAMES_TO_SHORT_SPARSE_CCC_V1 => Self::lookup_propnames_to_short_sparse_ccc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_AHEX_V1 => Self::lookup_props_ahex_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_ALPHA_V1 => Self::lookup_props_alpha_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_BASIC_EMOJI_V1 => Self::lookup_props_basic_emoji_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_BIDI_C_V1 => Self::lookup_props_bidi_c_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_BIDI_M_V1 => Self::lookup_props_bidi_m_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CI_V1 => Self::lookup_props_ci_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CWCF_V1 => Self::lookup_props_cwcf_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CWCM_V1 => Self::lookup_props_cwcm_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CWKCF_V1 => Self::lookup_props_cwkcf_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CWL_V1 => Self::lookup_props_cwl_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CWT_V1 => Self::lookup_props_cwt_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CWU_V1 => Self::lookup_props_cwu_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CASED_V1 => Self::lookup_props_cased_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_COMP_EX_V1 => Self::lookup_props_comp_ex_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_DI_V1 => Self::lookup_props_di_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_DASH_V1 => Self::lookup_props_dash_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_DEP_V1 => Self::lookup_props_dep_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_DIA_V1 => Self::lookup_props_dia_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EBASE_V1 => Self::lookup_props_ebase_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_ECOMP_V1 => Self::lookup_props_ecomp_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EMOD_V1 => Self::lookup_props_emod_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EPRES_V1 => Self::lookup_props_epres_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EMOJI_V1 => Self::lookup_props_emoji_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EXT_V1 => Self::lookup_props_ext_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EXTPICT_V1 => Self::lookup_props_extpict_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_GCB_V1 => Self::lookup_props_gcb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_GR_BASE_V1 => Self::lookup_props_gr_base_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_GR_EXT_V1 => Self::lookup_props_gr_ext_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_GR_LINK_V1 => Self::lookup_props_gr_link_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_HEX_V1 => Self::lookup_props_hex_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_HYPHEN_V1 => Self::lookup_props_hyphen_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_IDC_V1 => Self::lookup_props_idc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_IDS_V1 => Self::lookup_props_ids_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_IDSB_V1 => Self::lookup_props_idsb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_IDST_V1 => Self::lookup_props_idst_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_IDEO_V1 => Self::lookup_props_ideo_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_JOIN_C_V1 => Self::lookup_props_join_c_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_LOE_V1 => Self::lookup_props_loe_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_LOWER_V1 => Self::lookup_props_lower_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_MATH_V1 => Self::lookup_props_math_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_NCHAR_V1 => Self::lookup_props_nchar_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_PCM_V1 => Self::lookup_props_pcm_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_PAT_SYN_V1 => Self::lookup_props_pat_syn_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_PAT_WS_V1 => Self::lookup_props_pat_ws_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_QMARK_V1 => Self::lookup_props_qmark_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_RI_V1 => Self::lookup_props_ri_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_RADICAL_V1 => Self::lookup_props_radical_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_SB_V1 => Self::lookup_props_sb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_SD_V1 => Self::lookup_props_sd_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_STERM_V1 => Self::lookup_props_sterm_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_SENSITIVE_V1 => Self::lookup_props_sensitive_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_TERM_V1 => Self::lookup_props_term_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_UIDEO_V1 => Self::lookup_props_uideo_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_UPPER_V1 => Self::lookup_props_upper_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_VS_V1 => Self::lookup_props_vs_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_WB_V1 => Self::lookup_props_wb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_WSPACE_V1 => Self::lookup_props_wspace_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_XIDC_V1 => Self::lookup_props_xidc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_XIDS_V1 => Self::lookup_props_xids_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_ALNUM_V1 => Self::lookup_props_alnum_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_BC_V1 => Self::lookup_props_bc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_BIDIAUXILIARYPROPS_V1 => Self::lookup_props_bidiauxiliaryprops_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_BLANK_V1 => Self::lookup_props_blank_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_casemapping")]
                    PROPS_CASEMAP_V1 => Self::lookup_props_casemap_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_CCC_V1 => Self::lookup_props_ccc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EA_V1 => Self::lookup_props_ea_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EXEMPLARCHARS_AUXILIARY_V1 => Self::lookup_props_exemplarchars_auxiliary_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EXEMPLARCHARS_INDEX_V1 => Self::lookup_props_exemplarchars_index_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EXEMPLARCHARS_MAIN_V1 => Self::lookup_props_exemplarchars_main_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EXEMPLARCHARS_NUMBERS_V1 => Self::lookup_props_exemplarchars_numbers_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_EXEMPLARCHARS_PUNCTUATION_V1 => Self::lookup_props_exemplarchars_punctuation_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_GC_V1 => Self::lookup_props_gc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_GRAPH_V1 => Self::lookup_props_graph_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_LB_V1 => Self::lookup_props_lb_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_NFCINERT_V1 => Self::lookup_props_nfcinert_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_NFDINERT_V1 => Self::lookup_props_nfdinert_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_NFKCINERT_V1 => Self::lookup_props_nfkcinert_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_NFKDINERT_V1 => Self::lookup_props_nfkdinert_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_PRINT_V1 => Self::lookup_props_print_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_SC_V1 => Self::lookup_props_sc_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_SCX_V1 => Self::lookup_props_scx_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_SEGSTART_V1 => Self::lookup_props_segstart_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PROPS_XDIGIT_V1 => Self::lookup_props_xdigit_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_DAY_V1 => Self::lookup_relativetime_long_day_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_HOUR_V1 => Self::lookup_relativetime_long_hour_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_MINUTE_V1 => Self::lookup_relativetime_long_minute_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_MONTH_V1 => Self::lookup_relativetime_long_month_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_QUARTER_V1 => Self::lookup_relativetime_long_quarter_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_SECOND_V1 => Self::lookup_relativetime_long_second_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_WEEK_V1 => Self::lookup_relativetime_long_week_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_LONG_YEAR_V1 => Self::lookup_relativetime_long_year_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_DAY_V1 => Self::lookup_relativetime_narrow_day_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_HOUR_V1 => Self::lookup_relativetime_narrow_hour_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_MINUTE_V1 => Self::lookup_relativetime_narrow_minute_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_MONTH_V1 => Self::lookup_relativetime_narrow_month_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_QUARTER_V1 => Self::lookup_relativetime_narrow_quarter_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_SECOND_V1 => Self::lookup_relativetime_narrow_second_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_WEEK_V1 => Self::lookup_relativetime_narrow_week_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_NARROW_YEAR_V1 => Self::lookup_relativetime_narrow_year_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_DAY_V1 => Self::lookup_relativetime_short_day_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_HOUR_V1 => Self::lookup_relativetime_short_hour_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_MINUTE_V1 => Self::lookup_relativetime_short_minute_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_MONTH_V1 => Self::lookup_relativetime_short_month_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_QUARTER_V1 => Self::lookup_relativetime_short_quarter_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_SECOND_V1 => Self::lookup_relativetime_short_second_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_WEEK_V1 => Self::lookup_relativetime_short_week_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    RELATIVETIME_SHORT_YEAR_V1 => Self::lookup_relativetime_short_year_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SEGMENTER_DICTIONARY_W_AUTO_V1 => Self::lookup_segmenter_dictionary_w_auto_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SEGMENTER_DICTIONARY_WL_EXT_V1 => Self::lookup_segmenter_dictionary_wl_ext_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SEGMENTER_GRAPHEME_V1 => Self::lookup_segmenter_grapheme_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SEGMENTER_LINE_V1 => Self::lookup_segmenter_line_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SEGMENTER_LSTM_WL_AUTO_V1 => Self::lookup_segmenter_lstm_wl_auto_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SEGMENTER_SENTENCE_V1 => Self::lookup_segmenter_sentence_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SEGMENTER_WORD_V1 => Self::lookup_segmenter_word_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIME_ZONE_EXEMPLAR_CITIES_V1 => Self::lookup_time_zone_exemplar_cities_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIME_ZONE_FORMATS_V1 => Self::lookup_time_zone_formats_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIME_ZONE_GENERIC_LONG_V1 => Self::lookup_time_zone_generic_long_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIME_ZONE_GENERIC_SHORT_V1 => Self::lookup_time_zone_generic_short_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_timezone")]
                    TIME_ZONE_METAZONE_PERIOD_V1 => Self::lookup_time_zone_metazone_period_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIME_ZONE_SPECIFIC_LONG_V1 => Self::lookup_time_zone_specific_long_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIME_ZONE_SPECIFIC_SHORT_V1 => Self::lookup_time_zone_specific_short_v1(&req.locale).map(icu_provider::AnyPayload::from_static_ref),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey),
                }
                .map(|payload| icu_provider::AnyResponse { payload: Some(payload), metadata: Default::default() })
                .map_err(|e| e.with_req(key, req))
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
