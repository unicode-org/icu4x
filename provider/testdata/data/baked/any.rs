// @generated
impl AnyProvider for BakedDataProvider {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        #[cfg(feature = "icu_calendar")]
        const JAPANESEERASV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_calendar::provider::JapaneseErasV1Marker::KEY.hashed();
        #[cfg(feature = "icu_calendar")]
        const JAPANESEEXTENDEDERASV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_calendar::provider::JapaneseExtendedErasV1Marker::KEY.hashed();
        #[cfg(feature = "icu_calendar")]
        const WEEKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_calendar::provider::WeekDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_casemapping")]
        const CASEMAPPINGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_casemapping::provider::CaseMappingV1Marker::KEY.hashed();
        #[cfg(feature = "icu_collator")]
        const COLLATIONDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_collator")]
        const COLLATIONDIACRITICSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationDiacriticsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_collator")]
        const COLLATIONJAMOV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationJamoV1Marker::KEY.hashed();
        #[cfg(feature = "icu_collator")]
        const COLLATIONMETADATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationMetadataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_collator")]
        const COLLATIONREORDERINGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationReorderingV1Marker::KEY.hashed();
        #[cfg(feature = "icu_collator")]
        const COLLATIONSPECIALPRIMARIESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const BUDDHISTDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const BUDDHISTDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const COPTICDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::CopticDateLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const COPTICDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::CopticDateSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime_experimental")]
        const DATESKELETONPATTERNSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const ETHIOPIANDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const ETHIOPIANDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const GREGORIANDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const GREGORIANDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const INDIANDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::IndianDateLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const INDIANDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::IndianDateSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const JAPANESEDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const JAPANESEDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const JAPANESEEXTENDEDDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const JAPANESEEXTENDEDDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const TIMELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::TimeLengthsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const TIMESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::calendar::TimeSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const EXEMPLARCITIESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const METAZONEGENERICNAMESLONGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const METAZONEGENERICNAMESSHORTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const METAZONESPECIFICNAMESLONGV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const METAZONESPECIFICNAMESSHORTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker::KEY.hashed();
        #[cfg(feature = "icu_datetime")]
        const TIMEZONEFORMATSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_decimal")]
        const DECIMALSYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_decimal::provider::DecimalSymbolsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_displaynames")]
        const TERRITORYDISPLAYNAMESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_displaynames::provider::TerritoryDisplayNamesV1Marker::KEY.hashed();
        #[cfg(feature = "icu_list")]
        const ANDLISTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_list::provider::AndListV1Marker::KEY.hashed();
        #[cfg(feature = "icu_list")]
        const ORLISTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_list::provider::OrListV1Marker::KEY.hashed();
        #[cfg(feature = "icu_list")]
        const UNITLISTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_list::provider::UnitListV1Marker::KEY.hashed();
        #[cfg(feature = "icu_locid_transform")]
        const ALIASESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_locid_transform::provider::AliasesV1Marker::KEY.hashed();
        #[cfg(feature = "icu_locid_transform")]
        const LIKELYSUBTAGSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_locid_transform::provider::LikelySubtagsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_normalizer")]
        const CANONICALCOMPOSITIONSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CanonicalCompositionsV1Marker::KEY.hashed();
        #[cfg(feature = "icu_normalizer")]
        const CANONICALDECOMPOSITIONDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CanonicalDecompositionDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_normalizer")]
        const CANONICALDECOMPOSITIONTABLESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker::KEY.hashed();
        #[cfg(feature = "icu_normalizer")]
        const COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker::KEY.hashed();
        #[cfg(feature = "icu_normalizer")]
        const COMPATIBILITYDECOMPOSITIONTABLESV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker::KEY.hashed();
        #[cfg(feature = "icu_normalizer")]
        const NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker::KEY.hashed();
        #[cfg(feature = "icu_normalizer")]
        const UTS46DECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker::KEY.hashed();
        #[cfg(feature = "icu_plurals")]
        const CARDINALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_plurals::provider::CardinalV1Marker::KEY.hashed();
        #[cfg(feature = "icu_plurals")]
        const ORDINALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_plurals::provider::OrdinalV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const ALPHABETICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::AlphabeticV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const ASCIIHEXDIGITV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::AsciiHexDigitV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const BASICEMOJIV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::BasicEmojiV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const BIDICLASSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::BidiClassV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const BIDICONTROLV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::BidiControlV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const BIDIMIRROREDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::BidiMirroredV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CANONICALCOMBININGCLASSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::CanonicalCombiningClassV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CASEIGNORABLEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::CaseIgnorableV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::CasedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CHANGESWHENCASEFOLDEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenCasefoldedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CHANGESWHENLOWERCASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenLowercasedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CHANGESWHENNFKCCASEFOLDEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CHANGESWHENTITLECASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenTitlecasedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const CHANGESWHENUPPERCASEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ChangesWhenUppercasedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const DASHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DashV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const DEFAULTIGNORABLECODEPOINTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DefaultIgnorableCodePointV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const DEPRECATEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DeprecatedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const DIACRITICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::DiacriticV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EASTASIANWIDTHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EastAsianWidthV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJICOMPONENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiComponentV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIMODIFIERBASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiModifierBaseV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIMODIFIERV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiModifierV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIPRESENTATIONV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiPresentationV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EXTENDEDPICTOGRAPHICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ExtendedPictographicV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EXTENDERV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ExtenderV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const GENERALCATEGORYV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GeneralCategoryV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const GRAPHEMEBASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GraphemeBaseV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const GRAPHEMECLUSTERBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GraphemeClusterBreakV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const GRAPHEMEEXTENDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GraphemeExtendV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const HEXDIGITV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::HexDigitV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const IDCONTINUEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdContinueV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const IDSTARTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdStartV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const IDEOGRAPHICV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdeographicV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const IDSBINARYOPERATORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdsBinaryOperatorV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const IDSTRINARYOPERATORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::IdsTrinaryOperatorV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const JOINCONTROLV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::JoinControlV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const LINEBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::LineBreakV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const LOGICALORDEREXCEPTIONV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::LogicalOrderExceptionV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const LOWERCASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::LowercaseV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const MATHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::MathV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const NONCHARACTERCODEPOINTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::NoncharacterCodePointV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const PATTERNSYNTAXV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::PatternSyntaxV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const PATTERNWHITESPACEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::PatternWhiteSpaceV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const QUOTATIONMARKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::QuotationMarkV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const RADICALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::RadicalV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const REGIONALINDICATORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::RegionalIndicatorV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const SCRIPTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ScriptV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const SCRIPTWITHEXTENSIONSPROPERTYV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const SENTENCEBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::SentenceBreakV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const SENTENCETERMINALV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::SentenceTerminalV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const SOFTDOTTEDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::SoftDottedV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const TERMINALPUNCTUATIONV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::TerminalPunctuationV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const UNIFIEDIDEOGRAPHV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::UnifiedIdeographV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const UPPERCASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::UppercaseV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const VARIATIONSELECTORV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::VariationSelectorV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const WHITESPACEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::WhiteSpaceV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const WORDBREAKV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::WordBreakV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const XIDCONTINUEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::XidContinueV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const XIDSTARTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::XidStartV1Marker::KEY.hashed();
        const HELLOWORLDV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_provider::hello_world::HelloWorldV1Marker::KEY.hashed();
        const COLLATIONFALLBACKSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker::KEY
                .hashed();
        const LOCALEFALLBACKLIKELYSUBTAGSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker::KEY
                .hashed();
        const LOCALEFALLBACKPARENTSV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker::KEY
                .hashed();
        #[cfg(feature = "icu_segmenter")]
        const GRAPHEMECLUSTERBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_segmenter")]
        const LINEBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::LineBreakDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_segmenter")]
        const LSTMDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::LstmDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_segmenter")]
        const SENTENCEBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::SentenceBreakDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_segmenter")]
        const UCHARDICTIONARYBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_segmenter")]
        const WORDBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_segmenter::provider::WordBreakDataV1Marker::KEY.hashed();
        #[cfg(feature = "icu_timezone")]
        const METAZONEPERIODV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_timezone::provider::MetazonePeriodV1Marker::KEY.hashed();
        #[allow(clippy::match_single_binding)]
        match key.hashed() {
            #[cfg(feature = "icu_calendar")]
            JAPANESEERASV1MARKER => calendar::japanese_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_calendar")]
            JAPANESEEXTENDEDERASV1MARKER => calendar::japanext_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_calendar")]
            WEEKDATAV1MARKER => datetime::week_data_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_casemapping")]
            CASEMAPPINGV1MARKER => props::casemap_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_collator")]
            COLLATIONDATAV1MARKER => collator::data_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_collator")]
            COLLATIONDIACRITICSV1MARKER => collator::dia_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_collator")]
            COLLATIONJAMOV1MARKER => collator::jamo_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_collator")]
            COLLATIONMETADATAV1MARKER => collator::meta_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_collator")]
            COLLATIONREORDERINGV1MARKER => collator::reord_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_collator")]
            COLLATIONSPECIALPRIMARIESV1MARKER => collator::prim_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            BUDDHISTDATELENGTHSV1MARKER => datetime::buddhist::datelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            BUDDHISTDATESYMBOLSV1MARKER => datetime::buddhist::datesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            COPTICDATELENGTHSV1MARKER => datetime::coptic::datelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            COPTICDATESYMBOLSV1MARKER => datetime::coptic::datesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime_experimental")]
            DATESKELETONPATTERNSV1MARKER => {
                datetime::skeletons_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .copied()
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(
                        DataPayload::<
                            ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker,
                        >::from_owned,
                    )
                    .map(DataPayload::wrap_into_any_payload)
                    .ok_or(DataErrorKind::MissingLocale)
            }
            #[cfg(feature = "icu_datetime")]
            ETHIOPIANDATELENGTHSV1MARKER => datetime::ethiopic::datelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            ETHIOPIANDATESYMBOLSV1MARKER => datetime::ethiopic::datesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            GREGORIANDATELENGTHSV1MARKER => datetime::gregory::datelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            GREGORIANDATESYMBOLSV1MARKER => datetime::gregory::datesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            INDIANDATELENGTHSV1MARKER => datetime::indian::datelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            INDIANDATESYMBOLSV1MARKER => datetime::indian::datesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            JAPANESEDATELENGTHSV1MARKER => datetime::japanese::datelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            JAPANESEDATESYMBOLSV1MARKER => datetime::japanese::datesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            JAPANESEEXTENDEDDATELENGTHSV1MARKER => datetime::japanext::datelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            JAPANESEEXTENDEDDATESYMBOLSV1MARKER => datetime::japanext::datesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            TIMELENGTHSV1MARKER => datetime::timelengths_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            TIMESYMBOLSV1MARKER => datetime::timesymbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            EXEMPLARCITIESV1MARKER => time_zone::exemplar_cities_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            METAZONEGENERICNAMESLONGV1MARKER => time_zone::generic_long_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            METAZONEGENERICNAMESSHORTV1MARKER => time_zone::generic_short_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            METAZONESPECIFICNAMESLONGV1MARKER => time_zone::specific_long_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            METAZONESPECIFICNAMESSHORTV1MARKER => time_zone::specific_short_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_datetime")]
            TIMEZONEFORMATSV1MARKER => time_zone::formats_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_decimal")]
            DECIMALSYMBOLSV1MARKER => decimal::symbols_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_displaynames")]
            TERRITORYDISPLAYNAMESV1MARKER => displaynames::territories_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_list")]
            ANDLISTV1MARKER => list::and_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_list")]
            ORLISTV1MARKER => list::or_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_list")]
            UNITLISTV1MARKER => list::unit_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_locid_transform")]
            ALIASESV1MARKER => locid_transform::aliases_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_locid_transform")]
            LIKELYSUBTAGSV1MARKER => locid_transform::likelysubtags_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_normalizer")]
            CANONICALCOMPOSITIONSV1MARKER => normalizer::comp_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_normalizer")]
            CANONICALDECOMPOSITIONDATAV1MARKER => normalizer::nfd_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_normalizer")]
            CANONICALDECOMPOSITIONTABLESV1MARKER => normalizer::nfdex_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_normalizer")]
            COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::nfkd_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_normalizer")]
            COMPATIBILITYDECOMPOSITIONTABLESV1MARKER => normalizer::nfkdex_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_normalizer")]
            NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::decomp_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_normalizer")]
            UTS46DECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::uts46d_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_plurals")]
            CARDINALV1MARKER => plurals::cardinal_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_plurals")]
            ORDINALV1MARKER => plurals::ordinal_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            ALPHABETICV1MARKER => props::alpha_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            ASCIIHEXDIGITV1MARKER => props::ahex_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            BASICEMOJIV1MARKER => props::basic_emoji_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            BIDICLASSV1MARKER => props::bc_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            BIDICONTROLV1MARKER => props::bidi_c_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            BIDIMIRROREDV1MARKER => props::bidi_m_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CANONICALCOMBININGCLASSV1MARKER => props::ccc_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CASEIGNORABLEV1MARKER => props::ci_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CASEDV1MARKER => props::cased_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CHANGESWHENCASEFOLDEDV1MARKER => props::cwcf_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CHANGESWHENLOWERCASEDV1MARKER => props::cwl_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CHANGESWHENNFKCCASEFOLDEDV1MARKER => props::cwkcf_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CHANGESWHENTITLECASEDV1MARKER => props::cwt_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            CHANGESWHENUPPERCASEDV1MARKER => props::cwu_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            DASHV1MARKER => props::dash_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            DEFAULTIGNORABLECODEPOINTV1MARKER => props::di_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            DEPRECATEDV1MARKER => props::dep_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            DIACRITICV1MARKER => props::dia_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EASTASIANWIDTHV1MARKER => props::ea_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJICOMPONENTV1MARKER => props::ecomp_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIMODIFIERBASEV1MARKER => props::ebase_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIMODIFIERV1MARKER => props::emod_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIPRESENTATIONV1MARKER => props::epres_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIV1MARKER => props::emoji_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EXTENDEDPICTOGRAPHICV1MARKER => props::extpict_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EXTENDERV1MARKER => props::ext_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            GENERALCATEGORYV1MARKER => props::gc_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            GRAPHEMEBASEV1MARKER => props::gr_base_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            GRAPHEMECLUSTERBREAKV1MARKER => props::gcb_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            GRAPHEMEEXTENDV1MARKER => props::gr_ext_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            HEXDIGITV1MARKER => props::hex_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            IDCONTINUEV1MARKER => props::idc_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            IDSTARTV1MARKER => props::ids_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            IDEOGRAPHICV1MARKER => props::ideo_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            IDSBINARYOPERATORV1MARKER => props::idsb_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            IDSTRINARYOPERATORV1MARKER => props::idst_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            JOINCONTROLV1MARKER => props::join_c_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            LINEBREAKV1MARKER => props::lb_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            LOGICALORDEREXCEPTIONV1MARKER => props::loe_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            LOWERCASEV1MARKER => props::lower_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            MATHV1MARKER => props::math_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            NONCHARACTERCODEPOINTV1MARKER => props::nchar_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            PATTERNSYNTAXV1MARKER => props::pat_syn_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            PATTERNWHITESPACEV1MARKER => props::pat_ws_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            QUOTATIONMARKV1MARKER => props::qmark_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            RADICALV1MARKER => props::radical_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            REGIONALINDICATORV1MARKER => props::ri_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            SCRIPTV1MARKER => props::sc_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            SCRIPTWITHEXTENSIONSPROPERTYV1MARKER => props::scx_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            SENTENCEBREAKV1MARKER => props::sb_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            SENTENCETERMINALV1MARKER => props::sterm_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            SOFTDOTTEDV1MARKER => props::sd_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            TERMINALPUNCTUATIONV1MARKER => props::term_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            UNIFIEDIDEOGRAPHV1MARKER => props::uideo_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            UPPERCASEV1MARKER => props::upper_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            VARIATIONSELECTORV1MARKER => props::vs_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            WHITESPACEV1MARKER => props::wspace_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            WORDBREAKV1MARKER => props::wb_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            XIDCONTINUEV1MARKER => props::xidc_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            XIDSTARTV1MARKER => props::xids_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            HELLOWORLDV1MARKER => core::helloworld_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            COLLATIONFALLBACKSUPPLEMENTV1MARKER => fallback::supplement::co_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            LOCALEFALLBACKLIKELYSUBTAGSV1MARKER => fallback::likelysubtags_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            LOCALEFALLBACKPARENTSV1MARKER => fallback::parents_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_segmenter")]
            GRAPHEMECLUSTERBREAKDATAV1MARKER => segmenter::grapheme_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_segmenter")]
            LINEBREAKDATAV1MARKER => segmenter::line_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_segmenter")]
            LSTMDATAV1MARKER => segmenter::lstm_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_segmenter")]
            SENTENCEBREAKDATAV1MARKER => segmenter::sentence_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_segmenter")]
            UCHARDICTIONARYBREAKDATAV1MARKER => segmenter::dictionary_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_segmenter")]
            WORDBREAKDATAV1MARKER => segmenter::word_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_timezone")]
            METAZONEPERIODV1MARKER => time_zone::metazone_period_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            _ => Err(DataErrorKind::MissingDataKey),
        }
        .map_err(|e| e.with_req(key, req))
        .map(|payload| AnyResponse {
            payload: Some(payload),
            metadata: Default::default(),
        })
    }
}
