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
            icu::calendar::provider::ChineseCacheV1Marker = "calendar/chinesecache@1",
            icu::calendar::provider::DangiCacheV1Marker = "calendar/dangicache@1",
            icu::calendar::provider::JapaneseErasV1Marker = "calendar/japanese@1",
            icu::calendar::provider::IslamicObservationalCacheV1Marker = "calendar/islamicobservationalcache@1",
            icu::calendar::provider::IslamicUmmAlQuraCacheV1Marker = "calendar/islamicummalquracache@1",
            icu::calendar::provider::JapaneseExtendedErasV1Marker = "calendar/japanext@1",
            icu::calendar::provider::WeekDataV2Marker = "datetime/week_data@2",
            icu::casemap::provider::CaseMapV1Marker = "props/casemap@1",
            icu::casemap::provider::CaseMapUnfoldV1Marker = "props/casemap_unfold@1",
            icu::collator::provider::CollationRootV1Marker = "collator/root@1",
            icu::collator::provider::CollationTailoringV1Marker = "collator/tailoring@1",
            icu::collator::provider::CollationDiacriticsV1Marker = "collator/dia@1",
            icu::collator::provider::CollationJamoV1Marker = "collator/jamo@1",
            icu::collator::provider::CollationMetadataV1Marker = "collator/meta@1",
            icu::collator::provider::CollationReorderingV1Marker = "collator/reord@1",
            icu::collator::provider::CollationSpecialPrimariesV1Marker = "collator/prim@1",
            icu::datetime::provider::time_zones::LocationsV1Marker = "time_zone/locations@1",
            icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker = "time_zone/generic_long@1",
            icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker = "time_zone/generic_short@1",
            icu::datetime::provider::time_zones::MetazonePeriodV1Marker = "time_zone/metazone_period@1",
            icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker = "time_zone/specific_long@1",
            icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker = "time_zone/specific_short@1",
            icu::datetime::provider::time_zones::TimeZoneEssentialsV1Marker = "time_zone/essentials@1",
            icu::timezone::provider::ZoneOffsetPeriodV1Marker = "time_zone/offset_period@1",
            icu::decimal::provider::DecimalSymbolsV2Marker = "decimal/symbols@2",
            icu::list::provider::AndListV2Marker = "list/and@2",
            icu::list::provider::OrListV2Marker = "list/or@2",
            icu::list::provider::UnitListV2Marker = "list/unit@2",
            icu::locale::provider::AliasesV2Marker = "locale/aliases@2",
            icu::locale::provider::ExemplarCharactersAuxiliaryV1Marker = "locale/exemplarchars/auxiliary@1",
            icu::locale::provider::ExemplarCharactersIndexV1Marker = "locale/exemplarchars/index@1",
            icu::locale::provider::ExemplarCharactersMainV1Marker = "locale/exemplarchars/main@1",
            icu::locale::provider::ExemplarCharactersNumbersV1Marker = "locale/exemplarchars/numbers@1",
            icu::locale::provider::ExemplarCharactersPunctuationV1Marker = "locale/exemplarchars/punctuation@1",
            icu::locale::provider::LikelySubtagsExtendedV1Marker = "locale/likelysubtags_ext@1",
            icu::locale::provider::LikelySubtagsForLanguageV1Marker = "locale/likelysubtags_l@1",
            icu::locale::provider::LikelySubtagsForScriptRegionV1Marker = "locale/likelysubtags_sr@1",
            icu::locale::provider::ParentsV1Marker = "locale/parents@1",
            icu::locale::provider::ScriptDirectionV1Marker = "locale/script_dir@1",
            icu::normalizer::provider::CanonicalCompositionsV1Marker = "normalizer/comp@1",
            icu::normalizer::provider::CanonicalDecompositionDataV1Marker = "normalizer/nfd@1",
            icu::normalizer::provider::CanonicalDecompositionTablesV1Marker = "normalizer/nfdex@1",
            icu::normalizer::provider::CompatibilityDecompositionSupplementV1Marker = "normalizer/nfkd@1",
            icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker = "normalizer/nfkdex@1",
            icu::normalizer::provider::NonRecursiveDecompositionSupplementV1Marker = "normalizer/decomp@1",
            icu::normalizer::provider::Uts46DecompositionSupplementV1Marker = "normalizer/uts46d@1",
            icu::plurals::provider::CardinalV1Marker = "plurals/cardinal@1",
            icu::plurals::provider::OrdinalV1Marker = "plurals/ordinal@1",
            icu::properties::provider::AlnumV1Marker = "props/alnum@1",
            icu::properties::provider::AlphabeticV1Marker = "props/Alpha@1",
            icu::properties::provider::AsciiHexDigitV1Marker = "props/AHex@1",
            icu::properties::provider::BasicEmojiV1Marker = "props/Basic_Emoji@1",
            icu::properties::provider::BidiMirroringGlyphV1Marker = "props/Bidi_G@1",
            icu::properties::provider::BidiClassV1Marker = "props/bc@1",
            icu::properties::provider::BidiControlV1Marker = "props/Bidi_C@1",
            icu::properties::provider::BidiMirroredV1Marker = "props/Bidi_M@1",
            icu::properties::provider::BlankV1Marker = "props/blank@1",
            icu::properties::provider::CanonicalCombiningClassV1Marker = "props/ccc@1",
            icu::properties::provider::CasedV1Marker = "props/Cased@1",
            icu::properties::provider::CaseIgnorableV1Marker = "props/CI@1",
            icu::properties::provider::CaseSensitiveV1Marker = "props/Sensitive@1",
            icu::properties::provider::ChangesWhenCasefoldedV1Marker = "props/CWCF@1",
            icu::properties::provider::ChangesWhenCasemappedV1Marker = "props/CWCM@1",
            icu::properties::provider::ChangesWhenLowercasedV1Marker = "props/CWL@1",
            icu::properties::provider::ChangesWhenNfkcCasefoldedV1Marker = "props/CWKCF@1",
            icu::properties::provider::ChangesWhenTitlecasedV1Marker = "props/CWT@1",
            icu::properties::provider::ChangesWhenUppercasedV1Marker = "props/CWU@1",
            icu::properties::provider::DashV1Marker = "props/Dash@1",
            icu::properties::provider::DefaultIgnorableCodePointV1Marker = "props/DI@1",
            icu::properties::provider::DeprecatedV1Marker = "props/Dep@1",
            icu::properties::provider::DiacriticV1Marker = "props/Dia@1",
            icu::properties::provider::EastAsianWidthV1Marker = "props/ea@1",
            icu::properties::provider::EmojiComponentV1Marker = "props/EComp@1",
            icu::properties::provider::EmojiModifierBaseV1Marker = "props/EBase@1",
            icu::properties::provider::EmojiModifierV1Marker = "props/EMod@1",
            icu::properties::provider::EmojiPresentationV1Marker = "props/EPres@1",
            icu::properties::provider::EmojiV1Marker = "props/Emoji@1",
            icu::properties::provider::ExtendedPictographicV1Marker = "props/ExtPict@1",
            icu::properties::provider::ExtenderV1Marker = "props/Ext@1",
            icu::properties::provider::FullCompositionExclusionV1Marker = "props/Comp_Ex@1",
            icu::properties::provider::GeneralCategoryV1Marker = "props/gc@1",
            icu::properties::provider::GraphemeBaseV1Marker = "props/Gr_Base@1",
            icu::properties::provider::GraphemeClusterBreakV1Marker = "props/GCB@1",
            icu::properties::provider::GraphemeExtendV1Marker = "props/Gr_Ext@1",
            icu::properties::provider::GraphemeLinkV1Marker = "props/Gr_Link@1",
            icu::properties::provider::GraphV1Marker = "props/graph@1",
            icu::properties::provider::HangulSyllableTypeV1Marker = "props/hst@1",
            icu::properties::provider::HexDigitV1Marker = "props/Hex@1",
            icu::properties::provider::HyphenV1Marker = "props/Hyphen@1",
            icu::properties::provider::IdContinueV1Marker = "props/IDC@1",
            icu::properties::provider::IdeographicV1Marker = "props/Ideo@1",
            icu::properties::provider::IdsBinaryOperatorV1Marker = "props/IDSB@1",
            icu::properties::provider::IdStartV1Marker = "props/IDS@1",
            icu::properties::provider::IdsTrinaryOperatorV1Marker = "props/IDST@1",
            icu::properties::provider::IndicSyllabicCategoryV1Marker = "props/InSC@1",
            icu::properties::provider::JoinControlV1Marker = "props/Join_C@1",
            icu::properties::provider::JoiningTypeV1Marker = "props/jt@1",
            icu::properties::provider::LineBreakV1Marker = "props/lb@1",
            icu::properties::provider::LogicalOrderExceptionV1Marker = "props/LOE@1",
            icu::properties::provider::LowercaseV1Marker = "props/Lower@1",
            icu::properties::provider::MathV1Marker = "props/Math@1",
            icu::properties::provider::BidiClassNameToValueV2Marker = "propnames/from/bc@2",
            icu::properties::provider::BidiClassValueToLongNameV1Marker = "propnames/to/long/linear/bc@1",
            icu::properties::provider::BidiClassValueToShortNameV1Marker = "propnames/to/short/linear/bc@1",
            icu::properties::provider::CanonicalCombiningClassNameToValueV2Marker = "propnames/from/ccc@2",
            icu::properties::provider::CanonicalCombiningClassValueToLongNameV1Marker = "propnames/to/long/sparse/ccc@1",
            icu::properties::provider::CanonicalCombiningClassValueToShortNameV1Marker = "propnames/to/short/sparse/ccc@1",
            icu::properties::provider::EastAsianWidthNameToValueV2Marker = "propnames/from/ea@2",
            icu::properties::provider::EastAsianWidthValueToLongNameV1Marker = "propnames/to/long/linear/ea@1",
            icu::properties::provider::EastAsianWidthValueToShortNameV1Marker = "propnames/to/short/linear/ea@1",
            icu::properties::provider::GeneralCategoryMaskNameToValueV2Marker = "propnames/from/gcm@2",
            icu::properties::provider::GeneralCategoryNameToValueV2Marker = "propnames/from/gc@2",
            icu::properties::provider::GeneralCategoryValueToLongNameV1Marker = "propnames/to/long/linear/gc@1",
            icu::properties::provider::GeneralCategoryValueToShortNameV1Marker = "propnames/to/short/linear/gc@1",
            icu::properties::provider::GraphemeClusterBreakNameToValueV2Marker = "propnames/from/GCB@2",
            icu::properties::provider::GraphemeClusterBreakValueToLongNameV1Marker = "propnames/to/long/linear/GCB@1",
            icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker = "propnames/to/short/linear/GCB@1",
            icu::properties::provider::HangulSyllableTypeNameToValueV2Marker = "propnames/from/hst@2",
            icu::properties::provider::HangulSyllableTypeValueToLongNameV1Marker = "propnames/to/long/linear/hst@1",
            icu::properties::provider::HangulSyllableTypeValueToShortNameV1Marker = "propnames/to/short/linear/hst@1",
            icu::properties::provider::IndicSyllabicCategoryNameToValueV2Marker = "propnames/from/InSC@2",
            icu::properties::provider::IndicSyllabicCategoryValueToLongNameV1Marker = "propnames/to/long/linear/InSC@1",
            icu::properties::provider::IndicSyllabicCategoryValueToShortNameV1Marker = "propnames/to/short/linear/InSC@1",
            icu::properties::provider::JoiningTypeNameToValueV2Marker = "propnames/from/jt@2",
            icu::properties::provider::JoiningTypeValueToLongNameV1Marker = "propnames/to/long/linear/jt@1",
            icu::properties::provider::JoiningTypeValueToShortNameV1Marker = "propnames/to/short/linear/jt@1",
            icu::properties::provider::LineBreakNameToValueV2Marker = "propnames/from/lb@2",
            icu::properties::provider::LineBreakValueToLongNameV1Marker = "propnames/to/long/linear/lb@1",
            icu::properties::provider::LineBreakValueToShortNameV1Marker = "propnames/to/short/linear/lb@1",
            icu::properties::provider::ScriptNameToValueV2Marker = "propnames/from/sc@2",
            icu::properties::provider::ScriptValueToLongNameV1Marker = "propnames/to/long/linear/sc@1",
            icu::properties::provider::ScriptValueToShortNameV1Marker = "propnames/to/short/linear4/sc@1",
            icu::properties::provider::SentenceBreakNameToValueV2Marker = "propnames/from/SB@2",
            icu::properties::provider::SentenceBreakValueToLongNameV1Marker = "propnames/to/long/linear/SB@1",
            icu::properties::provider::SentenceBreakValueToShortNameV1Marker = "propnames/to/short/linear/SB@1",
            icu::properties::provider::WordBreakNameToValueV2Marker = "propnames/from/WB@2",
            icu::properties::provider::WordBreakValueToLongNameV1Marker = "propnames/to/long/linear/WB@1",
            icu::properties::provider::WordBreakValueToShortNameV1Marker = "propnames/to/short/linear/WB@1",
            icu::properties::provider::NfcInertV1Marker = "props/nfcinert@1",
            icu::properties::provider::NfdInertV1Marker = "props/nfdinert@1",
            icu::properties::provider::NfkcInertV1Marker = "props/nfkcinert@1",
            icu::properties::provider::NfkdInertV1Marker = "props/nfkdinert@1",
            icu::properties::provider::NoncharacterCodePointV1Marker = "props/NChar@1",
            icu::properties::provider::PatternSyntaxV1Marker = "props/Pat_Syn@1",
            icu::properties::provider::PatternWhiteSpaceV1Marker = "props/Pat_WS@1",
            icu::properties::provider::PrependedConcatenationMarkV1Marker = "props/PCM@1",
            icu::properties::provider::PrintV1Marker = "props/print@1",
            icu::properties::provider::QuotationMarkV1Marker = "props/QMark@1",
            icu::properties::provider::RadicalV1Marker = "props/Radical@1",
            icu::properties::provider::RegionalIndicatorV1Marker = "props/RI@1",
            icu::properties::provider::ScriptV1Marker = "props/sc@1",
            icu::properties::provider::ScriptWithExtensionsPropertyV1Marker = "props/scx@1",
            icu::properties::provider::SegmentStarterV1Marker = "props/segstart@1",
            icu::properties::provider::SentenceBreakV1Marker = "props/SB@1",
            icu::properties::provider::SentenceTerminalV1Marker = "props/STerm@1",
            icu::properties::provider::SoftDottedV1Marker = "props/SD@1",
            icu::properties::provider::TerminalPunctuationV1Marker = "props/Term@1",
            icu::properties::provider::UnifiedIdeographV1Marker = "props/UIdeo@1",
            icu::properties::provider::UppercaseV1Marker = "props/Upper@1",
            icu::properties::provider::VariationSelectorV1Marker = "props/VS@1",
            icu::properties::provider::WhiteSpaceV1Marker = "props/WSpace@1",
            icu::properties::provider::WordBreakV1Marker = "props/WB@1",
            icu::properties::provider::XdigitV1Marker = "props/xdigit@1",
            icu::properties::provider::XidContinueV1Marker = "props/XIDC@1",
            icu::properties::provider::XidStartV1Marker = "props/XIDS@1",
            icu::segmenter::provider::DictionaryForWordLineExtendedV1Marker = "segmenter/dictionary/wl_ext@1",
            icu::segmenter::provider::DictionaryForWordOnlyAutoV1Marker = "segmenter/dictionary/w_auto@1",
            icu::segmenter::provider::GraphemeClusterBreakDataV2Marker = "segmenter/grapheme@2",
            icu::segmenter::provider::LineBreakDataV2Marker = "segmenter/line@2",
            icu::segmenter::provider::LstmForWordLineAutoV1Marker = "segmenter/lstm/wl_auto@1",
            icu::segmenter::provider::SentenceBreakDataOverrideV1Marker = "segmenter/sentence/override@1",
            icu::segmenter::provider::SentenceBreakDataV2Marker = "segmenter/sentence@2",
            icu::segmenter::provider::WordBreakDataOverrideV1Marker = "segmenter/word/override@1",
            icu::segmenter::provider::WordBreakDataV2Marker = "segmenter/word@2",
            icu::timezone::provider::names::Bcp47ToIanaMapV1Marker = "time_zone/bcp47_to_iana@1",
            icu::timezone::provider::names::IanaToBcp47MapV3Marker = "time_zone/iana_to_bcp47@3",
            icu::timezone::provider::windows::WindowsZonesToBcp47MapV1Marker = "time_zone/windows_zones_to_bcp47@1",
            icu::datetime::provider::neo::WeekdayNamesV1Marker = "datetime/names/weekdays@1",
            icu::datetime::provider::neo::DayPeriodNamesV1Marker = "datetime/names/dayperiods@1",
            icu::datetime::provider::neo::GluePatternV1Marker = "datetime/patterns/glue@1",
            icu::datetime::provider::neo::BuddhistYearNamesV1Marker = "datetime/names/buddhist/years@1",
            icu::datetime::provider::neo::ChineseYearNamesV1Marker = "datetime/names/chinese/years@1",
            icu::datetime::provider::neo::CopticYearNamesV1Marker = "datetime/names/coptic/years@1",
            icu::datetime::provider::neo::DangiYearNamesV1Marker = "datetime/names/dangi/years@1",
            icu::datetime::provider::neo::EthiopianYearNamesV1Marker = "datetime/names/ethiopic/years@1",
            icu::datetime::provider::neo::GregorianYearNamesV1Marker = "datetime/names/gregory/years@1",
            icu::datetime::provider::neo::HebrewYearNamesV1Marker = "datetime/names/hebrew/years@1",
            icu::datetime::provider::neo::IndianYearNamesV1Marker = "datetime/names/indian/years@1",
            icu::datetime::provider::neo::IslamicYearNamesV1Marker = "datetime/names/islamic/years@1",
            icu::datetime::provider::neo::JapaneseYearNamesV1Marker = "datetime/names/japanese/years@1",
            icu::datetime::provider::neo::JapaneseExtendedYearNamesV1Marker = "datetime/names/japanext/years@1",
            icu::datetime::provider::neo::PersianYearNamesV1Marker = "datetime/names/persian/years@1",
            icu::datetime::provider::neo::RocYearNamesV1Marker = "datetime/names/roc/years@1",
            icu::datetime::provider::neo::BuddhistMonthNamesV1Marker = "datetime/names/buddhist/months@1",
            icu::datetime::provider::neo::ChineseMonthNamesV1Marker = "datetime/names/chinese/months@1",
            icu::datetime::provider::neo::CopticMonthNamesV1Marker = "datetime/names/coptic/months@1",
            icu::datetime::provider::neo::DangiMonthNamesV1Marker = "datetime/names/dangi/months@1",
            icu::datetime::provider::neo::EthiopianMonthNamesV1Marker = "datetime/names/ethiopic/months@1",
            icu::datetime::provider::neo::GregorianMonthNamesV1Marker = "datetime/names/gregory/months@1",
            icu::datetime::provider::neo::HebrewMonthNamesV1Marker = "datetime/names/hebrew/months@1",
            icu::datetime::provider::neo::IndianMonthNamesV1Marker = "datetime/names/indian/months@1",
            icu::datetime::provider::neo::IslamicMonthNamesV1Marker = "datetime/names/islamic/months@1",
            icu::datetime::provider::neo::JapaneseMonthNamesV1Marker = "datetime/names/japanese/months@1",
            icu::datetime::provider::neo::JapaneseExtendedMonthNamesV1Marker = "datetime/names/japanext/months@1",
            icu::datetime::provider::neo::PersianMonthNamesV1Marker = "datetime/names/persian/months@1",
            icu::datetime::provider::neo::RocMonthNamesV1Marker = "datetime/names/roc/months@1",
            icu::datetime::provider::BuddhistDateNeoSkeletonPatternsV1Marker = "datetime/patterns/buddhist/skeleton@1",
            icu::datetime::provider::ChineseDateNeoSkeletonPatternsV1Marker = "datetime/patterns/chinese/skeleton@1",
            icu::datetime::provider::CopticDateNeoSkeletonPatternsV1Marker = "datetime/patterns/coptic/skeleton@1",
            icu::datetime::provider::DangiDateNeoSkeletonPatternsV1Marker = "datetime/patterns/dangi/skeleton@1",
            icu::datetime::provider::EthiopianDateNeoSkeletonPatternsV1Marker = "datetime/patterns/ethiopic/skeleton@1",
            icu::datetime::provider::GregorianDateNeoSkeletonPatternsV1Marker = "datetime/patterns/gregory/skeleton@1",
            icu::datetime::provider::HebrewDateNeoSkeletonPatternsV1Marker = "datetime/patterns/hebrew/skeleton@1",
            icu::datetime::provider::IndianDateNeoSkeletonPatternsV1Marker = "datetime/patterns/indian/skeleton@1",
            icu::datetime::provider::IslamicDateNeoSkeletonPatternsV1Marker = "datetime/patterns/islamic/skeleton@1",
            icu::datetime::provider::JapaneseDateNeoSkeletonPatternsV1Marker = "datetime/patterns/japanese/skeleton@1",
            icu::datetime::provider::JapaneseExtendedDateNeoSkeletonPatternsV1Marker = "datetime/patterns/japanext/skeleton@1",
            icu::datetime::provider::PersianDateNeoSkeletonPatternsV1Marker = "datetime/patterns/persian/skeleton@1",
            icu::datetime::provider::RocDateNeoSkeletonPatternsV1Marker = "datetime/patterns/roc/skeleton@1",
            icu::datetime::provider::TimeNeoSkeletonPatternsV1Marker = "datetime/patterns/time_skeleton@1",
            #[experimental]
            icu::experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker = "compactdecimal/long@1",
            icu::experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker = "compactdecimal/short@1",
            icu::experimental::dimension::provider::currency_compact::ShortCurrencyCompactV1Marker = "currency/compact@1",
            icu::experimental::dimension::provider::currency_displayname::CurrencyDisplaynameV1Marker = "currency/displayname@1",
            icu::experimental::dimension::provider::currency::CurrencyEssentialsV1Marker = "currency/essentials@1",
            icu::experimental::dimension::provider::currency_patterns::CurrencyPatternsDataV1Marker = "currency/patterns@1",
            icu::experimental::dimension::provider::extended_currency::CurrencyExtendedDataV1Marker = "currency/extended@1",
            icu::experimental::dimension::provider::percent::PercentEssentialsV1Marker = "percent/essentials@1",
            icu::experimental::dimension::provider::units::UnitsDisplayNameV1Marker = "units/displaynames@1",
            icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1Marker = "units/essentials@1",
            icu::experimental::duration::provider::DigitalDurationDataV1Marker = "duration/digital@1",
            icu::experimental::displaynames::provider::RegionDisplayNamesV1Marker = "displaynames/regions@1",
            icu::experimental::displaynames::provider::LanguageDisplayNamesV1Marker = "displaynames/languages@1",
            icu::experimental::displaynames::provider::LocaleDisplayNamesV1Marker = "displaynames/locales@1",
            icu::experimental::displaynames::provider::ScriptDisplayNamesV1Marker = "displaynames/scripts@1",
            icu::experimental::displaynames::provider::VariantDisplayNamesV1Marker = "displaynames/variants@1",
            icu::experimental::measure::provider::trie::UnitsTrieV1Marker = "units/trie@1",
            icu::experimental::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker = "relativetime/long/second@1",
            icu::experimental::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker = "relativetime/short/second@1",
            icu::experimental::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker = "relativetime/narrow/second@1",
            icu::experimental::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker = "relativetime/long/minute@1",
            icu::experimental::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker = "relativetime/short/minute@1",
            icu::experimental::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker = "relativetime/narrow/minute@1",
            icu::experimental::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker = "relativetime/long/hour@1",
            icu::experimental::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker = "relativetime/short/hour@1",
            icu::experimental::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker = "relativetime/narrow/hour@1",
            icu::experimental::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker = "relativetime/long/day@1",
            icu::experimental::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker = "relativetime/short/day@1",
            icu::experimental::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker = "relativetime/narrow/day@1",
            icu::experimental::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker = "relativetime/long/week@1",
            icu::experimental::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker = "relativetime/short/week@1",
            icu::experimental::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker = "relativetime/narrow/week@1",
            icu::experimental::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker = "relativetime/long/month@1",
            icu::experimental::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker = "relativetime/short/month@1",
            icu::experimental::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker = "relativetime/narrow/month@1",
            icu::experimental::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker = "relativetime/long/quarter@1",
            icu::experimental::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker = "relativetime/short/quarter@1",
            icu::experimental::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker = "relativetime/narrow/quarter@1",
            icu::experimental::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker = "relativetime/long/year@1",
            icu::experimental::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker = "relativetime/short/year@1",
            icu::experimental::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker = "relativetime/narrow/year@1",
            icu::experimental::personnames::provider::PersonNamesFormatV1Marker = "personnames/personnames@1",
            icu::experimental::transliterate::provider::TransliteratorRulesV1Marker = "transliterator/rules@1",
            icu::experimental::units::provider::UnitsInfoV1Marker = "units/info@1",
            icu::plurals::provider::PluralRangesV1Marker = "plurals/ranges@1",
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
                if let Some(colliding_marker) = map.insert(marker.path.hashed(), marker) {
                    println!(
                        "{:?} and {:?} collide at {:?}",
                        marker.path,
                        colliding_marker.path,
                        marker.path.hashed(),
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

            $(
                assert_eq!(<$marker>::INFO.path.as_str(), $path);
            )+
            $(
                assert_eq!(<$emarker>::INFO.path.as_str(), $epath);
            )+
        }
    }
}

registry!(cb);
