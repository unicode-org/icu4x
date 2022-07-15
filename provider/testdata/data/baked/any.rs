// @generated
impl AnyProvider for BakedDataProvider {
    fn load_any(&self, key: ResourceKey, req: &DataRequest) -> Result<AnyResponse, DataError> {
        const JAPANESEERASV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_calendar::provider::JapaneseErasV1Marker::KEY.get_hash();
        const CASEMAPPINGV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_casemapping::provider::CaseMappingV1Marker::KEY.get_hash();
        const COLLATIONDATAV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_collator::provider::CollationDataV1Marker::KEY.get_hash();
        const COLLATIONDIACRITICSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_collator::provider::CollationDiacriticsV1Marker::KEY.get_hash();
        const COLLATIONJAMOV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_collator::provider::CollationJamoV1Marker::KEY.get_hash();
        const COLLATIONMETADATAV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_collator::provider::CollationMetadataV1Marker::KEY.get_hash();
        const COLLATIONREORDERINGV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_collator::provider::CollationReorderingV1Marker::KEY.get_hash();
        const COLLATIONSPECIALPRIMARIESV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY.get_hash();
        const DATEPATTERNSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_datetime::provider::calendar::DatePatternsV1Marker::KEY.get_hash();
        const DATESKELETONPATTERNSV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY.get_hash();
        const DATESYMBOLSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_datetime::provider::calendar::DateSymbolsV1Marker::KEY.get_hash();
        const TIMEPATTERNSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_datetime::provider::calendar::TimePatternsV1Marker::KEY.get_hash();
        const TIMESYMBOLSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_datetime::provider::calendar::TimeSymbolsV1Marker::KEY.get_hash();
        const EXEMPLARCITIESV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY.get_hash();
        const METAZONEGENERICNAMESLONGV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker::KEY.get_hash();
        const METAZONEGENERICNAMESSHORTV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker::KEY.get_hash();
        const METAZONEPERIODV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_datetime::provider::time_zones::MetaZonePeriodV1Marker::KEY.get_hash();
        const METAZONESPECIFICNAMESLONGV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker::KEY.get_hash();
        const METAZONESPECIFICNAMESSHORTV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker::KEY.get_hash();
        const TIMEZONEFORMATSV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY.get_hash();
        const WEEKDATAV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_datetime::provider::week_data::WeekDataV1Marker::KEY.get_hash();
        const DECIMALSYMBOLSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_decimal::provider::DecimalSymbolsV1Marker::KEY.get_hash();
        const ANDLISTV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_list::provider::AndListV1Marker::KEY.get_hash();
        const ORLISTV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_list::provider::OrListV1Marker::KEY.get_hash();
        const UNITLISTV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_list::provider::UnitListV1Marker::KEY.get_hash();
        const ALIASESV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_locale_canonicalizer::provider::AliasesV1Marker::KEY.get_hash();
        const LIKELYSUBTAGSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_locale_canonicalizer::provider::LikelySubtagsV1Marker::KEY.get_hash();
        const CANONICALCOMPOSITIONPASSTHROUGHV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::CanonicalCompositionPassthroughV1Marker::KEY.get_hash();
        const CANONICALCOMPOSITIONSV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::CanonicalCompositionsV1Marker::KEY.get_hash();
        const CANONICALDECOMPOSITIONDATAV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::CanonicalDecompositionDataV1Marker::KEY.get_hash();
        const CANONICALDECOMPOSITIONTABLESV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker::KEY.get_hash();
        const COMPATIBILITYCOMPOSITIONPASSTHROUGHV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::CompatibilityCompositionPassthroughV1Marker::KEY.get_hash();
        const COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker::KEY.get_hash();
        const COMPATIBILITYDECOMPOSITIONTABLESV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker::KEY.get_hash();
        const NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker::KEY.get_hash();
        const UTS46COMPOSITIONPASSTHROUGHV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::Uts46CompositionPassthroughV1Marker::KEY.get_hash();
        const UTS46DECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker::KEY.get_hash();
        const CARDINALV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_plurals::provider::CardinalV1Marker::KEY.get_hash();
        const ORDINALV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_plurals::provider::OrdinalV1Marker::KEY.get_hash();
        const ALPHABETICV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::AlphabeticV1Marker::KEY.get_hash();
        const ASCIIHEXDIGITV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::AsciiHexDigitV1Marker::KEY.get_hash();
        const BIDICLASSV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::BidiClassV1Marker::KEY.get_hash();
        const BIDICONTROLV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::BidiControlV1Marker::KEY.get_hash();
        const BIDIMIRROREDV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::BidiMirroredV1Marker::KEY.get_hash();
        const CANONICALCOMBININGCLASSV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::CanonicalCombiningClassV1Marker::KEY.get_hash();
        const CASEIGNORABLEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::CaseIgnorableV1Marker::KEY.get_hash();
        const CASEDV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::CasedV1Marker::KEY.get_hash();
        const CHANGESWHENCASEFOLDEDV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::ChangesWhenCasefoldedV1Marker::KEY.get_hash();
        const CHANGESWHENLOWERCASEDV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::ChangesWhenLowercasedV1Marker::KEY.get_hash();
        const CHANGESWHENNFKCCASEFOLDEDV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker::KEY.get_hash();
        const CHANGESWHENTITLECASEDV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::ChangesWhenTitlecasedV1Marker::KEY.get_hash();
        const CHANGESWHENUPPERCASEDV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::ChangesWhenUppercasedV1Marker::KEY.get_hash();
        const DASHV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::DashV1Marker::KEY.get_hash();
        const DEFAULTIGNORABLECODEPOINTV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::DefaultIgnorableCodePointV1Marker::KEY.get_hash();
        const DEPRECATEDV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::DeprecatedV1Marker::KEY.get_hash();
        const DIACRITICV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::DiacriticV1Marker::KEY.get_hash();
        const EASTASIANWIDTHV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::EastAsianWidthV1Marker::KEY.get_hash();
        const EMOJICOMPONENTV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::EmojiComponentV1Marker::KEY.get_hash();
        const EMOJIMODIFIERBASEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::EmojiModifierBaseV1Marker::KEY.get_hash();
        const EMOJIMODIFIERV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::EmojiModifierV1Marker::KEY.get_hash();
        const EMOJIPRESENTATIONV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::EmojiPresentationV1Marker::KEY.get_hash();
        const EMOJIV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::EmojiV1Marker::KEY.get_hash();
        const EXTENDEDPICTOGRAPHICV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::ExtendedPictographicV1Marker::KEY.get_hash();
        const EXTENDERV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::ExtenderV1Marker::KEY.get_hash();
        const GENERALCATEGORYV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::GeneralCategoryV1Marker::KEY.get_hash();
        const GRAPHEMEBASEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::GraphemeBaseV1Marker::KEY.get_hash();
        const GRAPHEMECLUSTERBREAKV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::GraphemeClusterBreakV1Marker::KEY.get_hash();
        const GRAPHEMEEXTENDV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::GraphemeExtendV1Marker::KEY.get_hash();
        const HEXDIGITV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::HexDigitV1Marker::KEY.get_hash();
        const IDCONTINUEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::IdContinueV1Marker::KEY.get_hash();
        const IDSTARTV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::IdStartV1Marker::KEY.get_hash();
        const IDEOGRAPHICV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::IdeographicV1Marker::KEY.get_hash();
        const IDSBINARYOPERATORV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::IdsBinaryOperatorV1Marker::KEY.get_hash();
        const IDSTRINARYOPERATORV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::IdsTrinaryOperatorV1Marker::KEY.get_hash();
        const JOINCONTROLV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::JoinControlV1Marker::KEY.get_hash();
        const LINEBREAKV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::LineBreakV1Marker::KEY.get_hash();
        const LOGICALORDEREXCEPTIONV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::LogicalOrderExceptionV1Marker::KEY.get_hash();
        const LOWERCASEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::LowercaseV1Marker::KEY.get_hash();
        const MATHV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::MathV1Marker::KEY.get_hash();
        const NONCHARACTERCODEPOINTV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::NoncharacterCodePointV1Marker::KEY.get_hash();
        const PATTERNSYNTAXV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::PatternSyntaxV1Marker::KEY.get_hash();
        const PATTERNWHITESPACEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::PatternWhiteSpaceV1Marker::KEY.get_hash();
        const QUOTATIONMARKV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::QuotationMarkV1Marker::KEY.get_hash();
        const RADICALV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::RadicalV1Marker::KEY.get_hash();
        const REGIONALINDICATORV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::RegionalIndicatorV1Marker::KEY.get_hash();
        const SCRIPTV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::ScriptV1Marker::KEY.get_hash();
        const SCRIPTWITHEXTENSIONSPROPERTYV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker::KEY.get_hash();
        const SENTENCEBREAKV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::SentenceBreakV1Marker::KEY.get_hash();
        const SENTENCETERMINALV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::SentenceTerminalV1Marker::KEY.get_hash();
        const SOFTDOTTEDV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::SoftDottedV1Marker::KEY.get_hash();
        const TERMINALPUNCTUATIONV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::TerminalPunctuationV1Marker::KEY.get_hash();
        const UNIFIEDIDEOGRAPHV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::UnifiedIdeographV1Marker::KEY.get_hash();
        const UPPERCASEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::UppercaseV1Marker::KEY.get_hash();
        const VARIATIONSELECTORV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::VariationSelectorV1Marker::KEY.get_hash();
        const WHITESPACEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::WhiteSpaceV1Marker::KEY.get_hash();
        const WORDBREAKV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::WordBreakV1Marker::KEY.get_hash();
        const XIDCONTINUEV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::XidContinueV1Marker::KEY.get_hash();
        const XIDSTARTV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_properties::provider::XidStartV1Marker::KEY.get_hash();
        const HELLOWORLDV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_provider::hello_world::HelloWorldV1Marker::KEY.get_hash();
        const LOCALEFALLBACKLIKELYSUBTAGSV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker::KEY.get_hash();
        const LOCALEFALLBACKPARENTSV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker::KEY.get_hash();
        const GRAPHEMECLUSTERBREAKDATAV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker::KEY.get_hash();
        const LINEBREAKDATAV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_segmenter::provider::LineBreakDataV1Marker::KEY.get_hash();
        const SENTENCEBREAKDATAV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_segmenter::provider::SentenceBreakDataV1Marker::KEY.get_hash();
        const UCHARDICTIONARYBREAKDATAV1MARKER: ::icu_provider::ResourceKeyHash =
            ::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker::KEY.get_hash();
        const WORDBREAKDATAV1MARKER: ::icu_provider::ResourceKeyHash = ::icu_segmenter::provider::WordBreakDataV1Marker::KEY.get_hash();
        Ok(AnyResponse {
            payload: Some(match key.get_hash() {
                JAPANESEERASV1MARKER => AnyPayload::from_static_ref::<<::icu_calendar::provider::JapaneseErasV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(calendar::japanese_v1::DATA, key, req)?,
                ),
                CASEMAPPINGV1MARKER => AnyPayload::from_static_ref::<<::icu_casemapping::provider::CaseMappingV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::casemap_v1::DATA, key, req)?,
                ),
                COLLATIONDATAV1MARKER => AnyPayload::from_static_ref::<<::icu_collator::provider::CollationDataV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(collator::data_v1::DATA, key, req)?,
                ),
                COLLATIONDIACRITICSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_collator::provider::CollationDiacriticsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(collator::dia_v1::DATA, key, req)?),
                COLLATIONJAMOV1MARKER => AnyPayload::from_static_ref::<<::icu_collator::provider::CollationJamoV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(collator::jamo_v1::DATA, key, req)?,
                ),
                COLLATIONMETADATAV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_collator::provider::CollationMetadataV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(collator::meta_v1::DATA, key, req)?),
                COLLATIONREORDERINGV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_collator::provider::CollationReorderingV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(collator::reord_v1::DATA, key, req)?),
                COLLATIONSPECIALPRIMARIESV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_collator::provider::CollationSpecialPrimariesV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(collator::prim_v1::DATA, key, req)?),
                DATEPATTERNSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::calendar::DatePatternsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(datetime::datelengths_v1_u_ca::DATA, key, req)?),
                DATESKELETONPATTERNSV1MARKER => AnyPayload::from_rc_payload::<::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker>(
                    alloc::rc::Rc::new(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(litemap_slice_get(
                        datetime::skeletons_v1_u_ca::DATA,
                        key,
                        req,
                    )?))),
                ),
                DATESYMBOLSV1MARKER => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::calendar::DateSymbolsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(datetime::datesymbols_v1_u_ca::DATA, key, req)?,
                    )
                }
                TIMEPATTERNSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::calendar::TimePatternsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(datetime::timelengths_v1_u_ca::DATA, key, req)?),
                TIMESYMBOLSV1MARKER => {
                    AnyPayload::from_static_ref::<<::icu_datetime::provider::calendar::TimeSymbolsV1Marker as DataMarker>::Yokeable>(
                        litemap_slice_get(datetime::timesymbols_v1_u_ca::DATA, key, req)?,
                    )
                }
                EXEMPLARCITIESV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(time_zone::exemplar_cities_v1::DATA, key, req)?),
                METAZONEGENERICNAMESLONGV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(time_zone::generic_long_v1::DATA, key, req)?),
                METAZONEGENERICNAMESSHORTV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(time_zone::generic_short_v1::DATA, key, req)?),
                METAZONEPERIODV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::time_zones::MetaZonePeriodV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(time_zone::metazone_period_v1::DATA, key, req)?),
                METAZONESPECIFICNAMESLONGV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(time_zone::specific_long_v1::DATA, key, req)?),
                METAZONESPECIFICNAMESSHORTV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(time_zone::specific_short_v1::DATA, key, req)?),
                TIMEZONEFORMATSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(time_zone::formats_v1::DATA, key, req)?),
                WEEKDATAV1MARKER => AnyPayload::from_static_ref::<<::icu_datetime::provider::week_data::WeekDataV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(datetime::week_data_v1_r::DATA, key, req)?,
                ),
                DECIMALSYMBOLSV1MARKER => AnyPayload::from_static_ref::<<::icu_decimal::provider::DecimalSymbolsV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(decimal::symbols_v1::DATA, key, req)?,
                ),
                ANDLISTV1MARKER => AnyPayload::from_static_ref::<<::icu_list::provider::AndListV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                    list::and_v1::DATA,
                    key,
                    req,
                )?),
                ORLISTV1MARKER => AnyPayload::from_static_ref::<<::icu_list::provider::OrListV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                    list::or_v1::DATA,
                    key,
                    req,
                )?),
                UNITLISTV1MARKER => AnyPayload::from_static_ref::<<::icu_list::provider::UnitListV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(list::unit_v1::DATA, key, req)?,
                ),
                ALIASESV1MARKER => AnyPayload::from_static_ref::<<::icu_locale_canonicalizer::provider::AliasesV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(locale_canonicalizer::aliases_v1::DATA, key, req)?,
                ),
                LIKELYSUBTAGSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_locale_canonicalizer::provider::LikelySubtagsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(locale_canonicalizer::likelysubtags_v1::DATA, key, req)?),
                CANONICALCOMPOSITIONPASSTHROUGHV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::CanonicalCompositionPassthroughV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::nfc_v1::DATA, key, req)?),
                CANONICALCOMPOSITIONSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::CanonicalCompositionsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::comp_v1::DATA, key, req)?),
                CANONICALDECOMPOSITIONDATAV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::CanonicalDecompositionDataV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::nfd_v1::DATA, key, req)?),
                CANONICALDECOMPOSITIONTABLESV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::nfdex_v1::DATA, key, req)?),
                COMPATIBILITYCOMPOSITIONPASSTHROUGHV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::CompatibilityCompositionPassthroughV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::nfkc_v1::DATA, key, req)?),
                COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::nfkd_v1::DATA, key, req)?),
                COMPATIBILITYDECOMPOSITIONTABLESV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::nfkdex_v1::DATA, key, req)?),
                NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::decomp_v1::DATA, key, req)?),
                UTS46COMPOSITIONPASSTHROUGHV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::Uts46CompositionPassthroughV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::uts46_v1::DATA, key, req)?),
                UTS46DECOMPOSITIONSUPPLEMENTV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(normalizer::uts46d_v1::DATA, key, req)?),
                CARDINALV1MARKER => AnyPayload::from_static_ref::<<::icu_plurals::provider::CardinalV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(plurals::cardinal_v1::DATA, key, req)?,
                ),
                ORDINALV1MARKER => AnyPayload::from_static_ref::<<::icu_plurals::provider::OrdinalV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(plurals::ordinal_v1::DATA, key, req)?,
                ),
                ALPHABETICV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::AlphabeticV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::alpha_v1::DATA, key, req)?,
                ),
                ASCIIHEXDIGITV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::AsciiHexDigitV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::ahex_v1::DATA, key, req)?,
                ),
                BIDICLASSV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::BidiClassV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::bc_v1::DATA, key, req)?,
                ),
                BIDICONTROLV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::BidiControlV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::bidi_c_v1::DATA, key, req)?,
                ),
                BIDIMIRROREDV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::BidiMirroredV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::bidi_m_v1::DATA, key, req)?,
                ),
                CANONICALCOMBININGCLASSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::CanonicalCombiningClassV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::ccc_v1::DATA, key, req)?),
                CASEIGNORABLEV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::CaseIgnorableV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::ci_v1::DATA, key, req)?,
                ),
                CASEDV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::CasedV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::cased_v1::DATA, key, req)?,
                ),
                CHANGESWHENCASEFOLDEDV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::ChangesWhenCasefoldedV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::cwcf_v1::DATA, key, req)?),
                CHANGESWHENLOWERCASEDV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::ChangesWhenLowercasedV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::cwl_v1::DATA, key, req)?),
                CHANGESWHENNFKCCASEFOLDEDV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::cwkcf_v1::DATA, key, req)?),
                CHANGESWHENTITLECASEDV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::ChangesWhenTitlecasedV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::cwt_v1::DATA, key, req)?),
                CHANGESWHENUPPERCASEDV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::ChangesWhenUppercasedV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::cwu_v1::DATA, key, req)?),
                DASHV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::DashV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                    props::dash_v1::DATA,
                    key,
                    req,
                )?),
                DEFAULTIGNORABLECODEPOINTV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::DefaultIgnorableCodePointV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::di_v1::DATA, key, req)?),
                DEPRECATEDV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::DeprecatedV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::dep_v1::DATA, key, req)?,
                ),
                DIACRITICV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::DiacriticV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::dia_v1::DATA, key, req)?,
                ),
                EASTASIANWIDTHV1MARKER => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EastAsianWidthV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ea_v1::DATA,
                        key,
                        req,
                    )?)
                }
                EMOJICOMPONENTV1MARKER => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiComponentV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::ecomp_v1::DATA,
                        key,
                        req,
                    )?)
                }
                EMOJIMODIFIERBASEV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::EmojiModifierBaseV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::ebase_v1::DATA, key, req)?),
                EMOJIMODIFIERV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiModifierV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::emod_v1::DATA, key, req)?,
                ),
                EMOJIPRESENTATIONV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::EmojiPresentationV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::epres_v1::DATA, key, req)?),
                EMOJIV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::EmojiV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::emoji_v1::DATA, key, req)?,
                ),
                EXTENDEDPICTOGRAPHICV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::ExtendedPictographicV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::extpict_v1::DATA, key, req)?),
                EXTENDERV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::ExtenderV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::ext_v1::DATA, key, req)?,
                ),
                GENERALCATEGORYV1MARKER => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::GeneralCategoryV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::gc_v1::DATA,
                        key,
                        req,
                    )?)
                }
                GRAPHEMEBASEV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::GraphemeBaseV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::gr_base_v1::DATA, key, req)?,
                ),
                GRAPHEMECLUSTERBREAKV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::GraphemeClusterBreakV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::gcb_v1::DATA, key, req)?),
                GRAPHEMEEXTENDV1MARKER => {
                    AnyPayload::from_static_ref::<<::icu_properties::provider::GraphemeExtendV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                        props::gr_ext_v1::DATA,
                        key,
                        req,
                    )?)
                }
                HEXDIGITV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::HexDigitV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::hex_v1::DATA, key, req)?,
                ),
                IDCONTINUEV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::IdContinueV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::idc_v1::DATA, key, req)?,
                ),
                IDSTARTV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::IdStartV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::ids_v1::DATA, key, req)?,
                ),
                IDEOGRAPHICV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::IdeographicV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::ideo_v1::DATA, key, req)?,
                ),
                IDSBINARYOPERATORV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::IdsBinaryOperatorV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::idsb_v1::DATA, key, req)?),
                IDSTRINARYOPERATORV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::IdsTrinaryOperatorV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::idst_v1::DATA, key, req)?),
                JOINCONTROLV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::JoinControlV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::join_c_v1::DATA, key, req)?,
                ),
                LINEBREAKV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::LineBreakV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::lb_v1::DATA, key, req)?,
                ),
                LOGICALORDEREXCEPTIONV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::LogicalOrderExceptionV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::loe_v1::DATA, key, req)?),
                LOWERCASEV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::LowercaseV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::lower_v1::DATA, key, req)?,
                ),
                MATHV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::MathV1Marker as DataMarker>::Yokeable>(litemap_slice_get(
                    props::math_v1::DATA,
                    key,
                    req,
                )?),
                NONCHARACTERCODEPOINTV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::NoncharacterCodePointV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::nchar_v1::DATA, key, req)?),
                PATTERNSYNTAXV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::PatternSyntaxV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::pat_syn_v1::DATA, key, req)?,
                ),
                PATTERNWHITESPACEV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::PatternWhiteSpaceV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::pat_ws_v1::DATA, key, req)?),
                QUOTATIONMARKV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::QuotationMarkV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::qmark_v1::DATA, key, req)?,
                ),
                RADICALV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::RadicalV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::radical_v1::DATA, key, req)?,
                ),
                REGIONALINDICATORV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::RegionalIndicatorV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::ri_v1::DATA, key, req)?),
                SCRIPTV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::ScriptV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::sc_v1::DATA, key, req)?,
                ),
                SCRIPTWITHEXTENSIONSPROPERTYV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::scx_v1::DATA, key, req)?),
                SENTENCEBREAKV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::SentenceBreakV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::sb_v1::DATA, key, req)?,
                ),
                SENTENCETERMINALV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::SentenceTerminalV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::sterm_v1::DATA, key, req)?),
                SOFTDOTTEDV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::SoftDottedV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::sd_v1::DATA, key, req)?,
                ),
                TERMINALPUNCTUATIONV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::TerminalPunctuationV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::term_v1::DATA, key, req)?),
                UNIFIEDIDEOGRAPHV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::UnifiedIdeographV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::uideo_v1::DATA, key, req)?),
                UPPERCASEV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::UppercaseV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::upper_v1::DATA, key, req)?,
                ),
                VARIATIONSELECTORV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_properties::provider::VariationSelectorV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(props::vs_v1::DATA, key, req)?),
                WHITESPACEV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::WhiteSpaceV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::wspace_v1::DATA, key, req)?,
                ),
                WORDBREAKV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::WordBreakV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::wb_v1::DATA, key, req)?,
                ),
                XIDCONTINUEV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::XidContinueV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::xidc_v1::DATA, key, req)?,
                ),
                XIDSTARTV1MARKER => AnyPayload::from_static_ref::<<::icu_properties::provider::XidStartV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(props::xids_v1::DATA, key, req)?,
                ),
                HELLOWORLDV1MARKER => AnyPayload::from_static_ref::<<::icu_provider::hello_world::HelloWorldV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(core::helloworld_v1::DATA, key, req)?,
                ),
                LOCALEFALLBACKLIKELYSUBTAGSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(fallback::likelysubtags_v1::DATA, key, req)?),
                LOCALEFALLBACKPARENTSV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(fallback::parents_v1::DATA, key, req)?),
                GRAPHEMECLUSTERBREAKDATAV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(segmenter::grapheme_v1::DATA, key, req)?),
                LINEBREAKDATAV1MARKER => AnyPayload::from_static_ref::<<::icu_segmenter::provider::LineBreakDataV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(segmenter::line_v1::DATA, key, req)?,
                ),
                SENTENCEBREAKDATAV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_segmenter::provider::SentenceBreakDataV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(segmenter::sentence_v1::DATA, key, req)?),
                UCHARDICTIONARYBREAKDATAV1MARKER => AnyPayload::from_static_ref::<
                    <::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker as DataMarker>::Yokeable,
                >(litemap_slice_get(segmenter::dictionary_v1::DATA, key, req)?),
                WORDBREAKDATAV1MARKER => AnyPayload::from_static_ref::<<::icu_segmenter::provider::WordBreakDataV1Marker as DataMarker>::Yokeable>(
                    litemap_slice_get(segmenter::word_v1::DATA, key, req)?,
                ),
                _ => return Err(DataErrorKind::MissingResourceKey.with_req(key, req)),
            }),
            metadata: Default::default(),
        })
    }
}
