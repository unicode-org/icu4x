// @generated
impl AnyProvider for BakedDataProvider {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        const JAPANESEERASV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_calendar::provider::JapaneseErasV1Marker::KEY.get_hash();
        const CASEMAPPINGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_casemapping::provider::CaseMappingV1Marker::KEY.get_hash();
        const COLLATIONDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationDataV1Marker::KEY.get_hash();
        const COLLATIONDIACRITICSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationDiacriticsV1Marker::KEY.get_hash();
        const COLLATIONJAMOV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationJamoV1Marker::KEY.get_hash();
        const COLLATIONMETADATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationMetadataV1Marker::KEY.get_hash();
        const COLLATIONREORDERINGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationReorderingV1Marker::KEY.get_hash();
        const COLLATIONSPECIALPRIMARIESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY.get_hash();
        const DATEPATTERNSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::DatePatternsV1Marker::KEY.get_hash();
        const DATESKELETONPATTERNSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY.get_hash();
        const DATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::DateSymbolsV1Marker::KEY.get_hash();
        const TIMELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::TimeLengthsV1Marker::KEY.get_hash();
        const TIMESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::TimeSymbolsV1Marker::KEY.get_hash();
        const EXEMPLARCITIESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY.get_hash();
        const METAZONEGENERICNAMESLONGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker::KEY.get_hash();
        const METAZONEGENERICNAMESSHORTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker::KEY.get_hash();
        const METAZONESPECIFICNAMESLONGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker::KEY.get_hash();
        const METAZONESPECIFICNAMESSHORTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker::KEY
                .get_hash();
        const TIMEZONEFORMATSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY.get_hash();
        const WEEKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::week_data::WeekDataV1Marker::KEY.get_hash();
        const DECIMALSYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_decimal::provider::DecimalSymbolsV1Marker::KEY.get_hash();
        const ANDLISTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_list::provider::AndListV1Marker::KEY.get_hash();
        const ORLISTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_list::provider::OrListV1Marker::KEY.get_hash();
        const UNITLISTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_list::provider::UnitListV1Marker::KEY.get_hash();
        const ALIASESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_locale_canonicalizer::provider::AliasesV1Marker::KEY.get_hash();
        const LIKELYSUBTAGSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_locale_canonicalizer::provider::LikelySubtagsV1Marker::KEY.get_hash();
        const CANONICALCOMPOSITIONPASSTHROUGHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CanonicalCompositionPassthroughV1Marker::KEY.get_hash();
        const CANONICALCOMPOSITIONSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CanonicalCompositionsV1Marker::KEY.get_hash();
        const CANONICALDECOMPOSITIONDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CanonicalDecompositionDataV1Marker::KEY.get_hash();
        const CANONICALDECOMPOSITIONTABLESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker::KEY.get_hash();
        const COMPATIBILITYCOMPOSITIONPASSTHROUGHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CompatibilityCompositionPassthroughV1Marker::KEY.get_hash();
        const COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker::KEY
                .get_hash();
        const COMPATIBILITYDECOMPOSITIONTABLESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker::KEY.get_hash();
        const NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker::KEY.get_hash();
        const UTS46COMPOSITIONPASSTHROUGHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::Uts46CompositionPassthroughV1Marker::KEY.get_hash();
        const UTS46DECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker::KEY.get_hash();
        const CARDINALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_plurals::provider::CardinalV1Marker::KEY.get_hash();
        const ORDINALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_plurals::provider::OrdinalV1Marker::KEY.get_hash();
        const ALPHABETICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::AlphabeticV1Marker::KEY.get_hash();
        const ASCIIHEXDIGITV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::AsciiHexDigitV1Marker::KEY.get_hash();
        const BIDICLASSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::BidiClassV1Marker::KEY.get_hash();
        const BIDICONTROLV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::BidiControlV1Marker::KEY.get_hash();
        const BIDIMIRROREDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::BidiMirroredV1Marker::KEY.get_hash();
        const CANONICALCOMBININGCLASSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::CanonicalCombiningClassV1Marker::KEY.get_hash();
        const CASEIGNORABLEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::CaseIgnorableV1Marker::KEY.get_hash();
        const CASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::CasedV1Marker::KEY.get_hash();
        const CHANGESWHENCASEFOLDEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenCasefoldedV1Marker::KEY.get_hash();
        const CHANGESWHENLOWERCASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenLowercasedV1Marker::KEY.get_hash();
        const CHANGESWHENNFKCCASEFOLDEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker::KEY.get_hash();
        const CHANGESWHENTITLECASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenTitlecasedV1Marker::KEY.get_hash();
        const CHANGESWHENUPPERCASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenUppercasedV1Marker::KEY.get_hash();
        const DASHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DashV1Marker::KEY.get_hash();
        const DEFAULTIGNORABLECODEPOINTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DefaultIgnorableCodePointV1Marker::KEY.get_hash();
        const DEPRECATEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DeprecatedV1Marker::KEY.get_hash();
        const DIACRITICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DiacriticV1Marker::KEY.get_hash();
        const EASTASIANWIDTHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EastAsianWidthV1Marker::KEY.get_hash();
        const EMOJICOMPONENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiComponentV1Marker::KEY.get_hash();
        const EMOJIMODIFIERBASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiModifierBaseV1Marker::KEY.get_hash();
        const EMOJIMODIFIERV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiModifierV1Marker::KEY.get_hash();
        const EMOJIPRESENTATIONV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiPresentationV1Marker::KEY.get_hash();
        const EMOJIV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiV1Marker::KEY.get_hash();
        const EXTENDEDPICTOGRAPHICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ExtendedPictographicV1Marker::KEY.get_hash();
        const EXTENDERV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ExtenderV1Marker::KEY.get_hash();
        const GENERALCATEGORYV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GeneralCategoryV1Marker::KEY.get_hash();
        const GRAPHEMEBASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GraphemeBaseV1Marker::KEY.get_hash();
        const GRAPHEMECLUSTERBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GraphemeClusterBreakV1Marker::KEY.get_hash();
        const GRAPHEMEEXTENDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GraphemeExtendV1Marker::KEY.get_hash();
        const HEXDIGITV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::HexDigitV1Marker::KEY.get_hash();
        const IDCONTINUEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdContinueV1Marker::KEY.get_hash();
        const IDSTARTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdStartV1Marker::KEY.get_hash();
        const IDEOGRAPHICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdeographicV1Marker::KEY.get_hash();
        const IDSBINARYOPERATORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdsBinaryOperatorV1Marker::KEY.get_hash();
        const IDSTRINARYOPERATORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdsTrinaryOperatorV1Marker::KEY.get_hash();
        const JOINCONTROLV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::JoinControlV1Marker::KEY.get_hash();
        const LINEBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::LineBreakV1Marker::KEY.get_hash();
        const LOGICALORDEREXCEPTIONV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::LogicalOrderExceptionV1Marker::KEY.get_hash();
        const LOWERCASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::LowercaseV1Marker::KEY.get_hash();
        const MATHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::MathV1Marker::KEY.get_hash();
        const NONCHARACTERCODEPOINTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::NoncharacterCodePointV1Marker::KEY.get_hash();
        const PATTERNSYNTAXV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::PatternSyntaxV1Marker::KEY.get_hash();
        const PATTERNWHITESPACEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::PatternWhiteSpaceV1Marker::KEY.get_hash();
        const QUOTATIONMARKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::QuotationMarkV1Marker::KEY.get_hash();
        const RADICALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::RadicalV1Marker::KEY.get_hash();
        const REGIONALINDICATORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::RegionalIndicatorV1Marker::KEY.get_hash();
        const SCRIPTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ScriptV1Marker::KEY.get_hash();
        const SCRIPTWITHEXTENSIONSPROPERTYV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker::KEY.get_hash();
        const SENTENCEBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::SentenceBreakV1Marker::KEY.get_hash();
        const SENTENCETERMINALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::SentenceTerminalV1Marker::KEY.get_hash();
        const SOFTDOTTEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::SoftDottedV1Marker::KEY.get_hash();
        const TERMINALPUNCTUATIONV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::TerminalPunctuationV1Marker::KEY.get_hash();
        const UNIFIEDIDEOGRAPHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::UnifiedIdeographV1Marker::KEY.get_hash();
        const UPPERCASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::UppercaseV1Marker::KEY.get_hash();
        const VARIATIONSELECTORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::VariationSelectorV1Marker::KEY.get_hash();
        const WHITESPACEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::WhiteSpaceV1Marker::KEY.get_hash();
        const WORDBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::WordBreakV1Marker::KEY.get_hash();
        const XIDCONTINUEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::XidContinueV1Marker::KEY.get_hash();
        const XIDSTARTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::XidStartV1Marker::KEY.get_hash();
        const HELLOWORLDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_provider::hello_world::HelloWorldV1Marker::KEY.get_hash();
        const LOCALEFALLBACKLIKELYSUBTAGSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker::KEY
                .get_hash();
        const LOCALEFALLBACKPARENTSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker::KEY
                .get_hash();
        const LSTMDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::LstmDataV1Marker::KEY.get_hash();
        const GRAPHEMECLUSTERBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker::KEY.get_hash();
        const LINEBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::LineBreakDataV1Marker::KEY.get_hash();
        const SENTENCEBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::SentenceBreakDataV1Marker::KEY.get_hash();
        const UCHARDICTIONARYBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker::KEY.get_hash();
        const WORDBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::WordBreakDataV1Marker::KEY.get_hash();
        const METAZONEPERIODV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_timezone::provider::MetaZonePeriodV1Marker::KEY.get_hash();
        Ok(AnyResponse {
            payload: Some(match key.get_hash() {
                JAPANESEERASV1MARKER => calendar::japanese_v1_u_ca::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CASEMAPPINGV1MARKER => props::casemap_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COLLATIONDATAV1MARKER => collator::data_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COLLATIONDIACRITICSV1MARKER => collator::dia_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COLLATIONJAMOV1MARKER => collator::jamo_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COLLATIONMETADATAV1MARKER => collator::meta_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COLLATIONREORDERINGV1MARKER => collator::reord_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COLLATIONSPECIALPRIMARIESV1MARKER => collator::prim_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                DATEPATTERNSV1MARKER => datetime::datelengths_v1_u_ca::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                DATESKELETONPATTERNSV1MARKER => datetime::skeletons_v1_u_ca::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(|&data| {
                        AnyPayload::from_rc_payload::<
                            ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker,
                        >(alloc::rc::Rc::new(DataPayload::from_owned(
                            zerofrom::ZeroFrom::zero_from(data),
                        )))
                    }),
                DATESYMBOLSV1MARKER => datetime::datesymbols_v1_u_ca::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                TIMELENGTHSV1MARKER => datetime::timelengths_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                TIMESYMBOLSV1MARKER => datetime::timesymbols_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EXEMPLARCITIESV1MARKER => time_zone::exemplar_cities_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                METAZONEGENERICNAMESLONGV1MARKER => time_zone::generic_long_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                METAZONEGENERICNAMESSHORTV1MARKER => time_zone::generic_short_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                METAZONESPECIFICNAMESLONGV1MARKER => time_zone::specific_long_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                METAZONESPECIFICNAMESSHORTV1MARKER => time_zone::specific_short_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                TIMEZONEFORMATSV1MARKER => time_zone::formats_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                WEEKDATAV1MARKER => datetime::week_data_v1_r::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                DECIMALSYMBOLSV1MARKER => decimal::symbols_v1_u_nu::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                ANDLISTV1MARKER => list::and_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                ORLISTV1MARKER => list::or_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                UNITLISTV1MARKER => list::unit_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                ALIASESV1MARKER => locale_canonicalizer::aliases_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LIKELYSUBTAGSV1MARKER => locale_canonicalizer::likelysubtags_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CANONICALCOMPOSITIONPASSTHROUGHV1MARKER => normalizer::nfc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CANONICALCOMPOSITIONSV1MARKER => normalizer::comp_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CANONICALDECOMPOSITIONDATAV1MARKER => normalizer::nfd_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CANONICALDECOMPOSITIONTABLESV1MARKER => normalizer::nfdex_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COMPATIBILITYCOMPOSITIONPASSTHROUGHV1MARKER => normalizer::nfkc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::nfkd_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                COMPATIBILITYDECOMPOSITIONTABLESV1MARKER => normalizer::nfkdex_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::decomp_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                UTS46COMPOSITIONPASSTHROUGHV1MARKER => normalizer::uts46_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                UTS46DECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::uts46d_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CARDINALV1MARKER => plurals::cardinal_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                ORDINALV1MARKER => plurals::ordinal_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                ALPHABETICV1MARKER => props::alpha_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                ASCIIHEXDIGITV1MARKER => props::ahex_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                BIDICLASSV1MARKER => props::bc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                BIDICONTROLV1MARKER => props::bidi_c_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                BIDIMIRROREDV1MARKER => props::bidi_m_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CANONICALCOMBININGCLASSV1MARKER => props::ccc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CASEIGNORABLEV1MARKER => props::ci_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CASEDV1MARKER => props::cased_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CHANGESWHENCASEFOLDEDV1MARKER => props::cwcf_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CHANGESWHENLOWERCASEDV1MARKER => props::cwl_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CHANGESWHENNFKCCASEFOLDEDV1MARKER => props::cwkcf_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CHANGESWHENTITLECASEDV1MARKER => props::cwt_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                CHANGESWHENUPPERCASEDV1MARKER => props::cwu_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                DASHV1MARKER => props::dash_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                DEFAULTIGNORABLECODEPOINTV1MARKER => props::di_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                DEPRECATEDV1MARKER => props::dep_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                DIACRITICV1MARKER => props::dia_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EASTASIANWIDTHV1MARKER => props::ea_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EMOJICOMPONENTV1MARKER => props::ecomp_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EMOJIMODIFIERBASEV1MARKER => props::ebase_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EMOJIMODIFIERV1MARKER => props::emod_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EMOJIPRESENTATIONV1MARKER => props::epres_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EMOJIV1MARKER => props::emoji_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EXTENDEDPICTOGRAPHICV1MARKER => props::extpict_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                EXTENDERV1MARKER => props::ext_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                GENERALCATEGORYV1MARKER => props::gc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                GRAPHEMEBASEV1MARKER => props::gr_base_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                GRAPHEMECLUSTERBREAKV1MARKER => props::gcb_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                GRAPHEMEEXTENDV1MARKER => props::gr_ext_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                HEXDIGITV1MARKER => props::hex_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                IDCONTINUEV1MARKER => props::idc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                IDSTARTV1MARKER => props::ids_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                IDEOGRAPHICV1MARKER => props::ideo_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                IDSBINARYOPERATORV1MARKER => props::idsb_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                IDSTRINARYOPERATORV1MARKER => props::idst_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                JOINCONTROLV1MARKER => props::join_c_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LINEBREAKV1MARKER => props::lb_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LOGICALORDEREXCEPTIONV1MARKER => props::loe_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LOWERCASEV1MARKER => props::lower_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                MATHV1MARKER => props::math_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                NONCHARACTERCODEPOINTV1MARKER => props::nchar_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                PATTERNSYNTAXV1MARKER => props::pat_syn_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                PATTERNWHITESPACEV1MARKER => props::pat_ws_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                QUOTATIONMARKV1MARKER => props::qmark_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                RADICALV1MARKER => props::radical_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                REGIONALINDICATORV1MARKER => props::ri_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                SCRIPTV1MARKER => props::sc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                SCRIPTWITHEXTENSIONSPROPERTYV1MARKER => props::scx_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                SENTENCEBREAKV1MARKER => props::sb_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                SENTENCETERMINALV1MARKER => props::sterm_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                SOFTDOTTEDV1MARKER => props::sd_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                TERMINALPUNCTUATIONV1MARKER => props::term_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                UNIFIEDIDEOGRAPHV1MARKER => props::uideo_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                UPPERCASEV1MARKER => props::upper_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                VARIATIONSELECTORV1MARKER => props::vs_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                WHITESPACEV1MARKER => props::wspace_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                WORDBREAKV1MARKER => props::wb_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                XIDCONTINUEV1MARKER => props::xidc_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                XIDSTARTV1MARKER => props::xids_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                HELLOWORLDV1MARKER => core::helloworld_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LOCALEFALLBACKLIKELYSUBTAGSV1MARKER => fallback::likelysubtags_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LOCALEFALLBACKPARENTSV1MARKER => fallback::parents_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LSTMDATAV1MARKER => segmenter::lstm_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                GRAPHEMECLUSTERBREAKDATAV1MARKER => segmenter::grapheme_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                LINEBREAKDATAV1MARKER => segmenter::line_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                SENTENCEBREAKDATAV1MARKER => segmenter::sentence_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                UCHARDICTIONARYBREAKDATAV1MARKER => segmenter::dictionary_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                WORDBREAKDATAV1MARKER => segmenter::word_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                METAZONEPERIODV1MARKER => time_zone::metazone_period_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .map(AnyPayload::from_static_ref),
                _ => return Err(DataErrorKind::MissingDataKey.with_req(key, req)),
            })
            .ok_or_else(|| DataErrorKind::MissingLocale.with_req(key, req))?,
            metadata: Default::default(),
        })
    }
}
