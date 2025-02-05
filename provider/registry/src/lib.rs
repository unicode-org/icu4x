// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Exposes the list of all known `DataMarker`s.
//!
//! This is modeled as a macro that accepts a callback macro of the shape:
//!
//! ```
//! macro_rules! cb {
//!     ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
//!         // Do something for each marker, or each experimental marker
//!     };
//! }
//! ```
//!
//! Calling this as `registry!(cb);` evaluates `cb` with the list of markers.

#[macro_export]
/// See the crate docs.
macro_rules! registry(
    ($cb:ident) => {
        cb!(
            icu::calendar::provider::ChineseCacheV1 = "calendar/chinesecache@1",
            icu::calendar::provider::DangiCacheV1 = "calendar/dangicache@1",
            icu::calendar::provider::JapaneseErasV1 = "calendar/japanese@1",
            icu::calendar::provider::IslamicObservationalCacheV1 = "calendar/islamicobservationalcache@1",
            icu::calendar::provider::IslamicUmmAlQuraCacheV1 = "calendar/islamicummalquracache@1",
            icu::calendar::provider::JapaneseExtendedErasV1 = "calendar/japanext@1",
            icu::calendar::provider::WeekDataV2 = "datetime/week_data@2",
            icu::casemap::provider::CaseMapV1 = "props/casemap@1",
            icu::casemap::provider::CaseMapUnfoldV1 = "props/casemap_unfold@1",
            icu::collator::provider::CollationRootV1 = "collator/root@1",
            icu::collator::provider::CollationTailoringV1 = "collator/tailoring@1",
            icu::collator::provider::CollationDiacriticsV1 = "collator/dia@1",
            icu::collator::provider::CollationJamoV1 = "collator/jamo@1",
            icu::collator::provider::CollationMetadataV1 = "collator/meta@1",
            icu::collator::provider::CollationReorderingV1 = "collator/reord@1",
            icu::collator::provider::CollationSpecialPrimariesV1 = "collator/prim@1",
            icu::datetime::provider::time_zones::LocationsV1 = "time_zone/locations@1",
            icu::datetime::provider::time_zones::LocationsRootV1 = "time_zone/locations_root@1",
            icu::datetime::provider::time_zones::ExemplarCitiesV1 = "time_zone/exemplars@1",
            icu::datetime::provider::time_zones::ExemplarCitiesRootV1 = "time_zone/exemplars_root@1",
            icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1 = "time_zone/generic_long@1",
            icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1 = "time_zone/generic_short@1",
            icu::datetime::provider::time_zones::MetazonePeriodV1 = "time_zone/metazone_period@1",
            icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1 = "time_zone/specific_long@1",
            icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 = "time_zone/specific_short@1",
            icu::datetime::provider::time_zones::TimeZoneEssentialsV1 = "time_zone/essentials@1",
            icu::timezone::provider::ZoneOffsetPeriodV1 = "time_zone/offset_period@1",
            icu::decimal::provider::DecimalDigitsV1 = "decimal/digits@1",
            icu::decimal::provider::DecimalSymbolsV2 = "decimal/symbols@2",
            icu::list::provider::AndListV2 = "list/and@2",
            icu::list::provider::OrListV2 = "list/or@2",
            icu::list::provider::UnitListV2 = "list/unit@2",
            icu::locale::provider::AliasesV2 = "locale/aliases@2",
            icu::locale::provider::ExemplarCharactersAuxiliaryV1 = "locale/exemplarchars/auxiliary@1",
            icu::locale::provider::ExemplarCharactersIndexV1 = "locale/exemplarchars/index@1",
            icu::locale::provider::ExemplarCharactersMainV1 = "locale/exemplarchars/main@1",
            icu::locale::provider::ExemplarCharactersNumbersV1 = "locale/exemplarchars/numbers@1",
            icu::locale::provider::ExemplarCharactersPunctuationV1 = "locale/exemplarchars/punctuation@1",
            icu::locale::provider::LikelySubtagsExtendedV1 = "locale/likelysubtags_ext@1",
            icu::locale::provider::LikelySubtagsForLanguageV1 = "locale/likelysubtags_l@1",
            icu::locale::provider::LikelySubtagsForScriptRegionV1 = "locale/likelysubtags_sr@1",
            icu::locale::provider::ParentsV1 = "locale/parents@1",
            icu::locale::provider::ScriptDirectionV1 = "locale/script_dir@1",
            icu::normalizer::provider::CanonicalCompositionsV1 = "normalizer/comp@1",
            icu::normalizer::provider::CanonicalDecompositionDataV2 = "normalizer/nfd@2",
            icu::normalizer::provider::CanonicalDecompositionTablesV1 = "normalizer/nfdex@1",
            icu::normalizer::provider::CompatibilityDecompositionDataV2 = "normalizer/nfkd@2",
            icu::normalizer::provider::CompatibilityDecompositionTablesV1 = "normalizer/nfkdex@1",
            icu::normalizer::provider::NonRecursiveDecompositionSupplementV1 = "normalizer/decomp@1",
            icu::normalizer::provider::Uts46DecompositionDataV2 = "normalizer/uts46d@2",
            icu::plurals::provider::CardinalV1 = "plurals/cardinal@1",
            icu::plurals::provider::OrdinalV1 = "plurals/ordinal@1",
            icu::properties::provider::AlnumV1 = "props/alnum@1",
            icu::properties::provider::AlphabeticV1 = "props/Alpha@1",
            icu::properties::provider::AsciiHexDigitV1 = "props/AHex@1",
            icu::properties::provider::BasicEmojiV1 = "props/Basic_Emoji@1",
            icu::properties::provider::BidiMirroringGlyphV1 = "props/Bidi_G@1",
            icu::properties::provider::BidiClassV1 = "props/bc@1",
            icu::properties::provider::BidiControlV1 = "props/Bidi_C@1",
            icu::properties::provider::BidiMirroredV1 = "props/Bidi_M@1",
            icu::properties::provider::BlankV1 = "props/blank@1",
            icu::properties::provider::CanonicalCombiningClassV1 = "props/ccc@1",
            icu::properties::provider::CasedV1 = "props/Cased@1",
            icu::properties::provider::CaseIgnorableV1 = "props/CI@1",
            icu::properties::provider::CaseSensitiveV1 = "props/Sensitive@1",
            icu::properties::provider::ChangesWhenCasefoldedV1 = "props/CWCF@1",
            icu::properties::provider::ChangesWhenCasemappedV1 = "props/CWCM@1",
            icu::properties::provider::ChangesWhenLowercasedV1 = "props/CWL@1",
            icu::properties::provider::ChangesWhenNfkcCasefoldedV1 = "props/CWKCF@1",
            icu::properties::provider::ChangesWhenTitlecasedV1 = "props/CWT@1",
            icu::properties::provider::ChangesWhenUppercasedV1 = "props/CWU@1",
            icu::properties::provider::DashV1 = "props/Dash@1",
            icu::properties::provider::DefaultIgnorableCodePointV1 = "props/DI@1",
            icu::properties::provider::DeprecatedV1 = "props/Dep@1",
            icu::properties::provider::DiacriticV1 = "props/Dia@1",
            icu::properties::provider::EastAsianWidthV1 = "props/ea@1",
            icu::properties::provider::EmojiComponentV1 = "props/EComp@1",
            icu::properties::provider::EmojiModifierBaseV1 = "props/EBase@1",
            icu::properties::provider::EmojiModifierV1 = "props/EMod@1",
            icu::properties::provider::EmojiPresentationV1 = "props/EPres@1",
            icu::properties::provider::EmojiV1 = "props/Emoji@1",
            icu::properties::provider::ExtendedPictographicV1 = "props/ExtPict@1",
            icu::properties::provider::ExtenderV1 = "props/Ext@1",
            icu::properties::provider::FullCompositionExclusionV1 = "props/Comp_Ex@1",
            icu::properties::provider::GeneralCategoryV1 = "props/gc@1",
            icu::properties::provider::GraphemeBaseV1 = "props/Gr_Base@1",
            icu::properties::provider::GraphemeClusterBreakV1 = "props/GCB@1",
            icu::properties::provider::GraphemeExtendV1 = "props/Gr_Ext@1",
            icu::properties::provider::GraphemeLinkV1 = "props/Gr_Link@1",
            icu::properties::provider::GraphV1 = "props/graph@1",
            icu::properties::provider::HangulSyllableTypeV1 = "props/hst@1",
            icu::properties::provider::HexDigitV1 = "props/Hex@1",
            icu::properties::provider::HyphenV1 = "props/Hyphen@1",
            icu::properties::provider::IdContinueV1 = "props/IDC@1",
            icu::properties::provider::IdeographicV1 = "props/Ideo@1",
            icu::properties::provider::IdsBinaryOperatorV1 = "props/IDSB@1",
            icu::properties::provider::IdStartV1 = "props/IDS@1",
            icu::properties::provider::IdsTrinaryOperatorV1 = "props/IDST@1",
            icu::properties::provider::IndicSyllabicCategoryV1 = "props/InSC@1",
            icu::properties::provider::JoinControlV1 = "props/Join_C@1",
            icu::properties::provider::JoiningTypeV1 = "props/jt@1",
            icu::properties::provider::LineBreakV1 = "props/lb@1",
            icu::properties::provider::LogicalOrderExceptionV1 = "props/LOE@1",
            icu::properties::provider::LowercaseV1 = "props/Lower@1",
            icu::properties::provider::MathV1 = "props/Math@1",
            icu::properties::provider::BidiClassNameToValueV2 = "propnames/from/bc@2",
            icu::properties::provider::BidiClassValueToLongNameV1 = "propnames/to/long/linear/bc@1",
            icu::properties::provider::BidiClassValueToShortNameV1 = "propnames/to/short/linear/bc@1",
            icu::properties::provider::CanonicalCombiningClassNameToValueV2 = "propnames/from/ccc@2",
            icu::properties::provider::CanonicalCombiningClassValueToLongNameV1 = "propnames/to/long/sparse/ccc@1",
            icu::properties::provider::CanonicalCombiningClassValueToShortNameV1 = "propnames/to/short/sparse/ccc@1",
            icu::properties::provider::EastAsianWidthNameToValueV2 = "propnames/from/ea@2",
            icu::properties::provider::EastAsianWidthValueToLongNameV1 = "propnames/to/long/linear/ea@1",
            icu::properties::provider::EastAsianWidthValueToShortNameV1 = "propnames/to/short/linear/ea@1",
            icu::properties::provider::GeneralCategoryMaskNameToValueV2 = "propnames/from/gcm@2",
            icu::properties::provider::GeneralCategoryNameToValueV2 = "propnames/from/gc@2",
            icu::properties::provider::GeneralCategoryValueToLongNameV1 = "propnames/to/long/linear/gc@1",
            icu::properties::provider::GeneralCategoryValueToShortNameV1 = "propnames/to/short/linear/gc@1",
            icu::properties::provider::GraphemeClusterBreakNameToValueV2 = "propnames/from/GCB@2",
            icu::properties::provider::GraphemeClusterBreakValueToLongNameV1 = "propnames/to/long/linear/GCB@1",
            icu::properties::provider::GraphemeClusterBreakValueToShortNameV1 = "propnames/to/short/linear/GCB@1",
            icu::properties::provider::HangulSyllableTypeNameToValueV2 = "propnames/from/hst@2",
            icu::properties::provider::HangulSyllableTypeValueToLongNameV1 = "propnames/to/long/linear/hst@1",
            icu::properties::provider::HangulSyllableTypeValueToShortNameV1 = "propnames/to/short/linear/hst@1",
            icu::properties::provider::IndicSyllabicCategoryNameToValueV2 = "propnames/from/InSC@2",
            icu::properties::provider::IndicSyllabicCategoryValueToLongNameV1 = "propnames/to/long/linear/InSC@1",
            icu::properties::provider::IndicSyllabicCategoryValueToShortNameV1 = "propnames/to/short/linear/InSC@1",
            icu::properties::provider::JoiningTypeNameToValueV2 = "propnames/from/jt@2",
            icu::properties::provider::JoiningTypeValueToLongNameV1 = "propnames/to/long/linear/jt@1",
            icu::properties::provider::JoiningTypeValueToShortNameV1 = "propnames/to/short/linear/jt@1",
            icu::properties::provider::LineBreakNameToValueV2 = "propnames/from/lb@2",
            icu::properties::provider::LineBreakValueToLongNameV1 = "propnames/to/long/linear/lb@1",
            icu::properties::provider::LineBreakValueToShortNameV1 = "propnames/to/short/linear/lb@1",
            icu::properties::provider::ScriptNameToValueV2 = "propnames/from/sc@2",
            icu::properties::provider::ScriptValueToLongNameV1 = "propnames/to/long/linear/sc@1",
            icu::properties::provider::ScriptValueToShortNameV1 = "propnames/to/short/linear4/sc@1",
            icu::properties::provider::SentenceBreakNameToValueV2 = "propnames/from/SB@2",
            icu::properties::provider::SentenceBreakValueToLongNameV1 = "propnames/to/long/linear/SB@1",
            icu::properties::provider::SentenceBreakValueToShortNameV1 = "propnames/to/short/linear/SB@1",
            icu::properties::provider::WordBreakNameToValueV2 = "propnames/from/WB@2",
            icu::properties::provider::WordBreakValueToLongNameV1 = "propnames/to/long/linear/WB@1",
            icu::properties::provider::WordBreakValueToShortNameV1 = "propnames/to/short/linear/WB@1",
            icu::properties::provider::NfcInertV1 = "props/nfcinert@1",
            icu::properties::provider::NfdInertV1 = "props/nfdinert@1",
            icu::properties::provider::NfkcInertV1 = "props/nfkcinert@1",
            icu::properties::provider::NfkdInertV1 = "props/nfkdinert@1",
            icu::properties::provider::NoncharacterCodePointV1 = "props/NChar@1",
            icu::properties::provider::PatternSyntaxV1 = "props/Pat_Syn@1",
            icu::properties::provider::PatternWhiteSpaceV1 = "props/Pat_WS@1",
            icu::properties::provider::PrependedConcatenationMarkV1 = "props/PCM@1",
            icu::properties::provider::PrintV1 = "props/print@1",
            icu::properties::provider::QuotationMarkV1 = "props/QMark@1",
            icu::properties::provider::RadicalV1 = "props/Radical@1",
            icu::properties::provider::RegionalIndicatorV1 = "props/RI@1",
            icu::properties::provider::ScriptV1 = "props/sc@1",
            icu::properties::provider::ScriptWithExtensionsPropertyV1 = "props/scx@1",
            icu::properties::provider::SegmentStarterV1 = "props/segstart@1",
            icu::properties::provider::SentenceBreakV1 = "props/SB@1",
            icu::properties::provider::SentenceTerminalV1 = "props/STerm@1",
            icu::properties::provider::SoftDottedV1 = "props/SD@1",
            icu::properties::provider::TerminalPunctuationV1 = "props/Term@1",
            icu::properties::provider::UnifiedIdeographV1 = "props/UIdeo@1",
            icu::properties::provider::UppercaseV1 = "props/Upper@1",
            icu::properties::provider::VariationSelectorV1 = "props/VS@1",
            icu::properties::provider::WhiteSpaceV1 = "props/WSpace@1",
            icu::properties::provider::WordBreakV1 = "props/WB@1",
            icu::properties::provider::XdigitV1 = "props/xdigit@1",
            icu::properties::provider::XidContinueV1 = "props/XIDC@1",
            icu::properties::provider::XidStartV1 = "props/XIDS@1",
            icu::segmenter::provider::DictionaryForWordLineExtendedV1 = "segmenter/dictionary/wl_ext@1",
            icu::segmenter::provider::DictionaryForWordOnlyAutoV1 = "segmenter/dictionary/w_auto@1",
            icu::segmenter::provider::GraphemeClusterBreakDataV2 = "segmenter/grapheme@2",
            icu::segmenter::provider::LineBreakDataV2 = "segmenter/line@2",
            icu::segmenter::provider::LstmForWordLineAutoV1 = "segmenter/lstm/wl_auto@1",
            icu::segmenter::provider::SentenceBreakDataOverrideV1 = "segmenter/sentence/override@1",
            icu::segmenter::provider::SentenceBreakDataV2 = "segmenter/sentence@2",
            icu::segmenter::provider::WordBreakDataOverrideV1 = "segmenter/word/override@1",
            icu::segmenter::provider::WordBreakDataV2 = "segmenter/word@2",
            icu::timezone::provider::names::Bcp47ToIanaMapV1 = "time_zone/bcp47_to_iana@1",
            icu::timezone::provider::names::IanaToBcp47MapV3 = "time_zone/iana_to_bcp47@3",
            icu::timezone::provider::windows::WindowsZonesToBcp47MapV1 = "time_zone/windows_zones_to_bcp47@1",
            icu::datetime::provider::neo::WeekdayNamesV1 = "datetime/names/weekdays@1",
            icu::datetime::provider::neo::DayPeriodNamesV1 = "datetime/names/dayperiods@1",
            icu::datetime::provider::neo::GluePatternV1 = "datetime/patterns/glue@1",
            icu::datetime::provider::neo::BuddhistYearNamesV1 = "datetime/names/buddhist/years@1",
            icu::datetime::provider::neo::ChineseYearNamesV1 = "datetime/names/chinese/years@1",
            icu::datetime::provider::neo::CopticYearNamesV1 = "datetime/names/coptic/years@1",
            icu::datetime::provider::neo::DangiYearNamesV1 = "datetime/names/dangi/years@1",
            icu::datetime::provider::neo::EthiopianYearNamesV1 = "datetime/names/ethiopic/years@1",
            icu::datetime::provider::neo::GregorianYearNamesV1 = "datetime/names/gregory/years@1",
            icu::datetime::provider::neo::HebrewYearNamesV1 = "datetime/names/hebrew/years@1",
            icu::datetime::provider::neo::IndianYearNamesV1 = "datetime/names/indian/years@1",
            icu::datetime::provider::neo::IslamicYearNamesV1 = "datetime/names/islamic/years@1",
            icu::datetime::provider::neo::JapaneseYearNamesV1 = "datetime/names/japanese/years@1",
            icu::datetime::provider::neo::JapaneseExtendedYearNamesV1 = "datetime/names/japanext/years@1",
            icu::datetime::provider::neo::PersianYearNamesV1 = "datetime/names/persian/years@1",
            icu::datetime::provider::neo::RocYearNamesV1 = "datetime/names/roc/years@1",
            icu::datetime::provider::neo::BuddhistMonthNamesV1 = "datetime/names/buddhist/months@1",
            icu::datetime::provider::neo::ChineseMonthNamesV1 = "datetime/names/chinese/months@1",
            icu::datetime::provider::neo::CopticMonthNamesV1 = "datetime/names/coptic/months@1",
            icu::datetime::provider::neo::DangiMonthNamesV1 = "datetime/names/dangi/months@1",
            icu::datetime::provider::neo::EthiopianMonthNamesV1 = "datetime/names/ethiopic/months@1",
            icu::datetime::provider::neo::GregorianMonthNamesV1 = "datetime/names/gregory/months@1",
            icu::datetime::provider::neo::HebrewMonthNamesV1 = "datetime/names/hebrew/months@1",
            icu::datetime::provider::neo::IndianMonthNamesV1 = "datetime/names/indian/months@1",
            icu::datetime::provider::neo::IslamicMonthNamesV1 = "datetime/names/islamic/months@1",
            icu::datetime::provider::neo::JapaneseMonthNamesV1 = "datetime/names/japanese/months@1",
            icu::datetime::provider::neo::JapaneseExtendedMonthNamesV1 = "datetime/names/japanext/months@1",
            icu::datetime::provider::neo::PersianMonthNamesV1 = "datetime/names/persian/months@1",
            icu::datetime::provider::neo::RocMonthNamesV1 = "datetime/names/roc/months@1",
            icu::datetime::provider::BuddhistDateNeoSkeletonPatternsV1 = "datetime/patterns/buddhist/skeleton@1",
            icu::datetime::provider::ChineseDateNeoSkeletonPatternsV1 = "datetime/patterns/chinese/skeleton@1",
            icu::datetime::provider::CopticDateNeoSkeletonPatternsV1 = "datetime/patterns/coptic/skeleton@1",
            icu::datetime::provider::DangiDateNeoSkeletonPatternsV1 = "datetime/patterns/dangi/skeleton@1",
            icu::datetime::provider::EthiopianDateNeoSkeletonPatternsV1 = "datetime/patterns/ethiopic/skeleton@1",
            icu::datetime::provider::GregorianDateNeoSkeletonPatternsV1 = "datetime/patterns/gregory/skeleton@1",
            icu::datetime::provider::HebrewDateNeoSkeletonPatternsV1 = "datetime/patterns/hebrew/skeleton@1",
            icu::datetime::provider::IndianDateNeoSkeletonPatternsV1 = "datetime/patterns/indian/skeleton@1",
            icu::datetime::provider::IslamicDateNeoSkeletonPatternsV1 = "datetime/patterns/islamic/skeleton@1",
            icu::datetime::provider::JapaneseDateNeoSkeletonPatternsV1 = "datetime/patterns/japanese/skeleton@1",
            icu::datetime::provider::JapaneseExtendedDateNeoSkeletonPatternsV1 = "datetime/patterns/japanext/skeleton@1",
            icu::datetime::provider::PersianDateNeoSkeletonPatternsV1 = "datetime/patterns/persian/skeleton@1",
            icu::datetime::provider::RocDateNeoSkeletonPatternsV1 = "datetime/patterns/roc/skeleton@1",
            icu::datetime::provider::TimeNeoSkeletonPatternsV1 = "datetime/patterns/time_skeleton@1",
            #[experimental]
            icu::experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1 = "compactdecimal/long@1",
            icu::experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1 = "compactdecimal/short@1",
            icu::experimental::dimension::provider::currency_compact::ShortCurrencyCompactV1 = "currency/compact@1",
            icu::experimental::dimension::provider::currency_displayname::CurrencyDisplaynameV1 = "currency/displayname@1",
            icu::experimental::dimension::provider::currency::CurrencyEssentialsV1 = "currency/essentials@1",
            icu::experimental::dimension::provider::currency_patterns::CurrencyPatternsDataV1 = "currency/patterns@1",
            icu::experimental::dimension::provider::extended_currency::CurrencyExtendedDataV1 = "currency/extended@1",
            icu::experimental::dimension::provider::percent::PercentEssentialsV1 = "percent/essentials@1",
            icu::experimental::dimension::provider::units::UnitsDisplayNameV1 = "units/displaynames@1",
            icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1 = "units/essentials@1",
            icu::experimental::duration::provider::DigitalDurationDataV1 = "duration/digital@1",
            icu::experimental::displaynames::provider::RegionDisplayNamesV1 = "displaynames/regions@1",
            icu::experimental::displaynames::provider::LanguageDisplayNamesV1 = "displaynames/languages@1",
            icu::experimental::displaynames::provider::LocaleDisplayNamesV1 = "displaynames/locales@1",
            icu::experimental::displaynames::provider::ScriptDisplayNamesV1 = "displaynames/scripts@1",
            icu::experimental::displaynames::provider::VariantDisplayNamesV1 = "displaynames/variants@1",
            icu::experimental::measure::provider::trie::UnitsTrieV1 = "units/trie@1",
            icu::experimental::relativetime::provider::LongSecondRelativeV1 = "relativetime/long/second@1",
            icu::experimental::relativetime::provider::ShortSecondRelativeV1 = "relativetime/short/second@1",
            icu::experimental::relativetime::provider::NarrowSecondRelativeV1 = "relativetime/narrow/second@1",
            icu::experimental::relativetime::provider::LongMinuteRelativeV1 = "relativetime/long/minute@1",
            icu::experimental::relativetime::provider::ShortMinuteRelativeV1 = "relativetime/short/minute@1",
            icu::experimental::relativetime::provider::NarrowMinuteRelativeV1 = "relativetime/narrow/minute@1",
            icu::experimental::relativetime::provider::LongHourRelativeV1 = "relativetime/long/hour@1",
            icu::experimental::relativetime::provider::ShortHourRelativeV1 = "relativetime/short/hour@1",
            icu::experimental::relativetime::provider::NarrowHourRelativeV1 = "relativetime/narrow/hour@1",
            icu::experimental::relativetime::provider::LongDayRelativeV1 = "relativetime/long/day@1",
            icu::experimental::relativetime::provider::ShortDayRelativeV1 = "relativetime/short/day@1",
            icu::experimental::relativetime::provider::NarrowDayRelativeV1 = "relativetime/narrow/day@1",
            icu::experimental::relativetime::provider::LongWeekRelativeV1 = "relativetime/long/week@1",
            icu::experimental::relativetime::provider::ShortWeekRelativeV1 = "relativetime/short/week@1",
            icu::experimental::relativetime::provider::NarrowWeekRelativeV1 = "relativetime/narrow/week@1",
            icu::experimental::relativetime::provider::LongMonthRelativeV1 = "relativetime/long/month@1",
            icu::experimental::relativetime::provider::ShortMonthRelativeV1 = "relativetime/short/month@1",
            icu::experimental::relativetime::provider::NarrowMonthRelativeV1 = "relativetime/narrow/month@1",
            icu::experimental::relativetime::provider::LongQuarterRelativeV1 = "relativetime/long/quarter@1",
            icu::experimental::relativetime::provider::ShortQuarterRelativeV1 = "relativetime/short/quarter@1",
            icu::experimental::relativetime::provider::NarrowQuarterRelativeV1 = "relativetime/narrow/quarter@1",
            icu::experimental::relativetime::provider::LongYearRelativeV1 = "relativetime/long/year@1",
            icu::experimental::relativetime::provider::ShortYearRelativeV1 = "relativetime/short/year@1",
            icu::experimental::relativetime::provider::NarrowYearRelativeV1 = "relativetime/narrow/year@1",
            icu::experimental::personnames::provider::PersonNamesFormatV1 = "personnames/personnames@1",
            icu::experimental::transliterate::provider::TransliteratorRulesV1 = "transliterator/rules@1",
            icu::experimental::units::provider::UnitsInfoV1 = "units/info@1",
            icu::plurals::provider::PluralRangesV1 = "plurals/ranges@1",
        );
    }
);

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        #[test]
        fn no_marker_collisions() {
            use icu_provider::prelude::*;

            let mut map = std::collections::BTreeMap::new();
            let mut failed = false;
            for marker in [
                $(
                    <$marker>::INFO,
                )+
                $(
                    <$emarker>::INFO,
                )+
            ] {
                if let Some(colliding_marker) = map.insert(marker.id.hashed(), marker) {
                    println!(
                        "{:?} and {:?} collide at {:?}",
                        marker.id,
                        colliding_marker.id,
                        marker.id.hashed(),
                    );
                    failed = true;
                }
            }
            if failed {
                panic!();
            }
        }

        #[test]
        fn test_paths_correct() {
            use icu_provider::prelude::*;
            use icu_provider::marker::data_marker_id;

            $(
                assert_eq!(<$marker>::INFO.id, data_marker_id!($path));
            )+
            $(
                assert_eq!(<$emarker>::INFO.id, data_marker_id!($epath));
            )+
        }
    }
}

registry!(cb);
