// @generated
impl AnyProvider for BakedDataProvider {
    fn load_any(&self, key: ResourceKey, req: &DataRequest) -> Result<AnyResponse, DataError> {
        Ok(AnyResponse {
            payload: Some(match key {
                <::icu_calendar::provider::JapaneseErasV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_calendar::provider::JapaneseErasV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        calendar::japanese_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_casemapping::provider::CaseMappingV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_casemapping::provider::CaseMappingV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::casemap_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_collator::provider::CollationDataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_collator::provider::CollationDataV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        collator::data_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_collator::provider::CollationDiacriticsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_collator::provider::CollationDiacriticsV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        collator::dia_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_collator::provider::CollationJamoV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_collator::provider::CollationJamoV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        collator::jamo_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_collator::provider::CollationMetadataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_collator::provider::CollationMetadataV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        collator::meta_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_collator::provider::CollationReorderingV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_collator::provider::CollationReorderingV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        collator::reord_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_collator::provider::CollationSpecialPrimariesV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_collator::provider::CollationSpecialPrimariesV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(collator::prim_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::calendar::DatePatternsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::calendar::DatePatternsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(datetime::datelengths_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_rc_payload::<::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker>(alloc::rc::Rc::new(
                        DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(litemap_slice_get(datetime::skeletons_v1::DATA, key, req)?)),
                    ))
                }
                <::icu_datetime::provider::calendar::DateSymbolsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::calendar::DateSymbolsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(datetime::datesymbols_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::calendar::TimePatternsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::calendar::TimePatternsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(datetime::timelengths_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::calendar::TimeSymbolsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::calendar::TimeSymbolsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(datetime::timesymbols_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(time_zone::exemplar_cities_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(time_zone::generic_long_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(time_zone::generic_short_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::time_zones::MetaZonePeriodV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::time_zones::MetaZonePeriodV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(time_zone::metazone_period_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(time_zone::specific_long_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(time_zone::specific_short_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(time_zone::formats_v1::DATA, key, req)?,
                    )
                }
                <::icu_datetime::provider::week_data::WeekDataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::week_data::WeekDataV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        datetime::week_data_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_decimal::provider::DecimalSymbolsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_decimal::provider::DecimalSymbolsV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        decimal::symbols_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_list::provider::AndListV1Marker as ResourceMarker>::KEY => AnyPayload::from_static_ref::<
                    <::icu_list::provider::AndListV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(list::and_v1::DATA, key, req)?),
                <::icu_list::provider::OrListV1Marker as ResourceMarker>::KEY => AnyPayload::from_static_ref::<
                    <::icu_list::provider::OrListV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(list::or_v1::DATA, key, req)?),
                <::icu_list::provider::UnitListV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_list::provider::UnitListV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        list::unit_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_locale_canonicalizer::provider::AliasesV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_locale_canonicalizer::provider::AliasesV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        locale_canonicalizer::aliases_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_locale_canonicalizer::provider::LikelySubtagsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_locale_canonicalizer::provider::LikelySubtagsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(locale_canonicalizer::likelysubtags_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::CanonicalCompositionPassthroughV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::CanonicalCompositionPassthroughV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::nfc_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::CanonicalCompositionsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::CanonicalCompositionsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::comp_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::CanonicalDecompositionDataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::CanonicalDecompositionDataV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::nfd_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::nfdex_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::CompatibilityCompositionPassthroughV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::CompatibilityCompositionPassthroughV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::nfkc_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::nfkd_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::nfkdex_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::Uts46CompositionPassthroughV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::Uts46CompositionPassthroughV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::uts46_v1::DATA, key, req)?,
                    )
                }
                <::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(normalizer::uts46d_v1::DATA, key, req)?,
                    )
                }
                <::icu_plurals::provider::CardinalV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_plurals::provider::CardinalV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        plurals::cardinal_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_plurals::provider::OrdinalV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_plurals::provider::OrdinalV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        plurals::ordinal_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::AlphabeticV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::AlphabeticV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::alpha_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::AsciiHexDigitV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::AsciiHexDigitV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ahex_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::BidiClassV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::BidiClassV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::bc_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::BidiControlV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::BidiControlV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::bidi_c_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::BidiMirroredV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::BidiMirroredV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::bidi_m_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::CanonicalCombiningClassV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::CanonicalCombiningClassV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::ccc_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::CaseIgnorableV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::CaseIgnorableV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ci_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::CasedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::CasedV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::cased_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::ChangesWhenCasefoldedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ChangesWhenCasefoldedV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::cwcf_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::ChangesWhenLowercasedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ChangesWhenLowercasedV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::cwl_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::cwkcf_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::ChangesWhenTitlecasedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ChangesWhenTitlecasedV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::cwt_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::ChangesWhenUppercasedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ChangesWhenUppercasedV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::cwu_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::DashV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::DashV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::dash_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::DefaultIgnorableCodePointV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::DefaultIgnorableCodePointV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::di_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::DeprecatedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::DeprecatedV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::dep_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::DiacriticV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::DiacriticV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::dia_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::EastAsianWidthV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EastAsianWidthV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ea_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::EmojiComponentV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiComponentV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ecomp_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::EmojiModifierBaseV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiModifierBaseV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ebase_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::EmojiModifierV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiModifierV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::emod_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::EmojiPresentationV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiPresentationV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::epres_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::EmojiV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::emoji_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::ExtendedPictographicV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ExtendedPictographicV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::extpict_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::ExtenderV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ExtenderV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ext_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::GeneralCategoryV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::GeneralCategoryV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::gc_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::GraphemeBaseV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::GraphemeBaseV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::gr_base_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::GraphemeClusterBreakV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::GraphemeClusterBreakV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::gcb_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::GraphemeExtendV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::GraphemeExtendV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::gr_ext_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::HexDigitV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::HexDigitV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::hex_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::IdContinueV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::IdContinueV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::idc_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::IdStartV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::IdStartV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ids_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::IdeographicV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::IdeographicV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ideo_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::IdsBinaryOperatorV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::IdsBinaryOperatorV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::idsb_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::IdsTrinaryOperatorV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::IdsTrinaryOperatorV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::idst_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::JoinControlV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::JoinControlV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::join_c_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::LineBreakV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::LineBreakV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::lb_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::LogicalOrderExceptionV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::LogicalOrderExceptionV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::loe_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::LowercaseV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::LowercaseV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::lower_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::MathV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::MathV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::math_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::NoncharacterCodePointV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::NoncharacterCodePointV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::nchar_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::PatternSyntaxV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::PatternSyntaxV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::pat_syn_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::PatternWhiteSpaceV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::PatternWhiteSpaceV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::pat_ws_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::QuotationMarkV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::QuotationMarkV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::qmark_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::RadicalV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::RadicalV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::radical_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::RegionalIndicatorV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::RegionalIndicatorV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ri_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::ScriptV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ScriptV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::sc_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::scx_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::SentenceBreakV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::SentenceBreakV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::sb_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::SentenceTerminalV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::SentenceTerminalV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::sterm_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::SoftDottedV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::SoftDottedV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::sd_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::TerminalPunctuationV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::TerminalPunctuationV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(props::term_v1::DATA, key, req)?,
                    )
                }
                <::icu_properties::provider::UnifiedIdeographV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::UnifiedIdeographV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::uideo_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::UppercaseV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::UppercaseV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::upper_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::VariationSelectorV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::VariationSelectorV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::vs_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::WhiteSpaceV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::WhiteSpaceV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::wspace_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::WordBreakV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::WordBreakV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::wb_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::XidContinueV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::XidContinueV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::xidc_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_properties::provider::XidStartV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::XidStartV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::xids_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<
                        <::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker as DataMarker>::Yokeable,
                    >(litemap_slice_get(fallback::likelysubtags_v1::DATA, key, req)?)
                }
                <::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(fallback::parents_v1::DATA, key, req)?,
                    )
                }
                <::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(segmenter::grapheme_v1::DATA, key, req)?,
                    )
                }
                <::icu_segmenter::provider::LineBreakDataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_segmenter::provider::LineBreakDataV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        segmenter::line_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_segmenter::provider::SentenceBreakDataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_segmenter::provider::SentenceBreakDataV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        segmenter::sentence_v1::DATA,
                        key,
                        req,
                    )?)
                }
                <::icu_segmenter::provider::WordBreakDataV1Marker as ResourceMarker>::KEY => {
                    AnyPayload::from_static_ref::<<::icu_segmenter::provider::WordBreakDataV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        segmenter::word_v1::DATA,
                        key,
                        req,
                    )?)
                }
                _ => return Err(DataErrorKind::MissingResourceKey.with_req(key, req)),
            }),
            metadata: Default::default(),
        })
    }
}
