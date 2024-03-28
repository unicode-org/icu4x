// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

macro_rules! registry {
    ($(#[cfg($feature:meta)] $($marker:path = $path:literal,)+)+) => {
        /// List of all keys that are available.
        ///
        /// Note that since v1.3, `all_keys` also contains experimental keys for which the
        /// corresponding Cargo features has been enabled.
        // Excludes the hello world key, as that generally should not be generated.
        pub fn all_keys() -> Vec<DataKey> {
            #[cfg(features = "experimental_components")]
            log::warn!("The icu_datagen crates has been built with the `experimental_components` feature, so `all_keys` returns experimental keys");
            vec![
                $(
                    $(
                        #[cfg($feature)]
                        <$marker>::KEY,
                    )+
                )+
            ]
        }

        /// Parses a human-readable key identifier into a [`DataKey`].
        //  Supports the hello world key
        /// # Example
        /// ```
        /// # use icu_provider::KeyedDataMarker;
        /// assert_eq!(
        ///     icu_datagen::key("list/and@1"),
        ///     Some(icu::list::provider::AndListV1Marker::KEY),
        /// );
        /// ```
        pub fn key<S: AsRef<str>>(string: S) -> Option<DataKey> {
            use once_cell::sync::OnceCell;
            static LOOKUP: OnceCell<std::collections::HashMap<&'static str, Result<DataKey, &'static str>>> = OnceCell::new();
            let lookup = LOOKUP.get_or_init(|| {
                [
                    ("core/helloworld@1", Ok(icu_provider::hello_world::HelloWorldV1Marker::KEY)),
                    $(
                        $(
                            #[cfg($feature)]
                            ($path, Ok(<$marker>::KEY)),
                            #[cfg(not($feature))]
                            ($path, Err(stringify!($feature))),
                        )+
                    )+
                ]
                .into_iter()
                .collect()
            });
            let path = string.as_ref();
            match lookup.get(path).copied() {
                None => {
                    log::warn!("Unknown key {path:?}");
                    None
                },
                Some(Err(feature)) => {
                    log::warn!("Key {path:?} requires {feature}");
                    None
                },
                Some(Ok(key)) => Some(key)
            }
        }

        #[test]
        fn test_paths_correct() {
            $(
                $(
                    #[cfg($feature)]
                    assert_eq!(<$marker>::KEY.path().get(), $path);
                )+
            )+
        }

        #[macro_export]
        #[doc(hidden)]
        macro_rules! make_exportable_provider {
            ($ty:ty) => {
                icu_provider::make_exportable_provider!(
                    $ty,
                    [
                        icu_provider::hello_world::HelloWorldV1Marker,
                        $(
                            $(
                                #[cfg($feature)]
                                $marker,
                            )+
                        )+
                    ]
                );
            }
        }
        pub(crate) use make_exportable_provider;


        #[cfg(feature = "baked_exporter")]
        pub(crate) fn key_to_marker_bake(key: DataKey, env: &databake::CrateEnv) -> databake::TokenStream {
            use databake::Bake;
            // This is a bit naughty, we need the marker's type, but we're actually
            // baking its value. This works as long as all markers are unit structs.
            if key.path() == icu_provider::hello_world::HelloWorldV1Marker::KEY.path() {
                return icu_provider::hello_world::HelloWorldV1Marker.bake(env);
            }
            $(
                $(
                    #[cfg($feature)]
                    if key == <$marker>::KEY {
                        return $marker.bake(env);
                    }
                )+
            )+
            unreachable!("unregistered key {key:?}")
        }

        #[cfg(test)]
        pub fn deserialize_and_measure<Measurement>(key: DataKey, buf: DataPayload<BufferMarker>, measure: impl Fn() -> Measurement) -> Result<(Measurement, DataPayload<icu_provider::datagen::ExportMarker>), DataError> {
            if key.path() == icu_provider::hello_world::HelloWorldV1Marker::KEY.path() {
                let deserialized: DataPayload<icu_provider::hello_world::HelloWorldV1Marker> = buf.into_deserialized(icu_provider::buf::BufferFormat::Postcard1)?;
                return Ok((measure(), icu_provider::dynutil::UpcastDataPayload::upcast(deserialized)));
            }
            $(
                $(
                    #[cfg($feature)]
                    if key == <$marker>::KEY {
                        let deserialized: DataPayload<$marker> = buf.into_deserialized(icu_provider::buf::BufferFormat::Postcard1)?;
                        return Ok((measure(), icu_provider::dynutil::UpcastDataPayload::upcast(deserialized)));
                    }
                )+
            )+
            unreachable!("unregistered key {key:?}")
        }
    }
}

registry!(
    #[cfg(all())]
    icu_calendar::provider::ChineseCacheV1Marker = "calendar/chinesecache@1",
    icu_calendar::provider::DangiCacheV1Marker = "calendar/dangicache@1",
    icu_calendar::provider::JapaneseErasV1Marker = "calendar/japanese@1",
    icu_calendar::provider::JapaneseExtendedErasV1Marker = "calendar/japanext@1",
    icu_calendar::provider::WeekDataV1Marker = "datetime/week_data@1",
    icu_calendar::provider::WeekDataV2Marker = "datetime/week_data@2",
    #[cfg(all())]
    icu_casemap::provider::CaseMapV1Marker = "props/casemap@1",
    icu_casemap::provider::CaseMapUnfoldV1Marker = "props/casemap_unfold@1",
    #[cfg(all())]
    icu_collator::provider::CollationDataV1Marker = "collator/data@1",
    icu_collator::provider::CollationDiacriticsV1Marker = "collator/dia@1",
    icu_collator::provider::CollationJamoV1Marker = "collator/jamo@1",
    icu_collator::provider::CollationMetadataV1Marker = "collator/meta@1",
    icu_collator::provider::CollationReorderingV1Marker = "collator/reord@1",
    icu_collator::provider::CollationSpecialPrimariesV1Marker = "collator/prim@1",
    #[cfg(feature = "experimental_components")]
    icu_experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker =
        "compactdecimal/long@1",
    icu_experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker =
        "compactdecimal/short@1",
    #[cfg(all())]
    icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker =
        "datetime/buddhist/datelengths@1",
    icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker =
        "datetime/buddhist/datesymbols@1",
    icu_datetime::provider::calendar::ChineseDateLengthsV1Marker = "datetime/chinese/datelengths@1",
    icu_datetime::provider::calendar::ChineseDateSymbolsV1Marker = "datetime/chinese/datesymbols@1",
    icu_datetime::provider::calendar::CopticDateLengthsV1Marker = "datetime/coptic/datelengths@1",
    icu_datetime::provider::calendar::CopticDateSymbolsV1Marker = "datetime/coptic/datesymbols@1",
    icu_datetime::provider::calendar::DangiDateLengthsV1Marker = "datetime/dangi/datelengths@1",
    icu_datetime::provider::calendar::DangiDateSymbolsV1Marker = "datetime/dangi/datesymbols@1",
    icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker = "datetime/skeletons@1",
    icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker =
        "datetime/ethiopic/datelengths@1",
    icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker =
        "datetime/ethiopic/datesymbols@1",
    icu_datetime::provider::calendar::GregorianDateLengthsV1Marker =
        "datetime/gregory/datelengths@1",
    icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker =
        "datetime/gregory/datesymbols@1",
    icu_datetime::provider::calendar::IndianDateLengthsV1Marker = "datetime/indian/datelengths@1",
    icu_datetime::provider::calendar::IndianDateSymbolsV1Marker = "datetime/indian/datesymbols@1",
    icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker =
        "datetime/japanese/datelengths@1",
    icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker =
        "datetime/japanese/datesymbols@1",
    icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker =
        "datetime/japanext/datelengths@1",
    icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker =
        "datetime/japanext/datesymbols@1",
    icu_datetime::provider::calendar::HebrewDateLengthsV1Marker = "datetime/hebrew/datelengths@1",
    icu_datetime::provider::calendar::HebrewDateSymbolsV1Marker = "datetime/hebrew/datesymbols@1",
    icu_datetime::provider::calendar::IslamicDateLengthsV1Marker = "datetime/islamic/datelengths@1",
    icu_datetime::provider::calendar::IslamicDateSymbolsV1Marker = "datetime/islamic/datesymbols@1",
    icu_datetime::provider::calendar::PersianDateLengthsV1Marker = "datetime/persian/datelengths@1",
    icu_datetime::provider::calendar::PersianDateSymbolsV1Marker = "datetime/persian/datesymbols@1",
    icu_datetime::provider::calendar::RocDateLengthsV1Marker = "datetime/roc/datelengths@1",
    icu_datetime::provider::calendar::RocDateSymbolsV1Marker = "datetime/roc/datesymbols@1",
    icu_datetime::provider::calendar::TimeLengthsV1Marker = "datetime/timelengths@1",
    icu_datetime::provider::calendar::TimeSymbolsV1Marker = "datetime/timesymbols@1",
    // new datetime symbols stuff (unused so far)
    icu_datetime::provider::neo::WeekdayNamesV1Marker = "datetime/symbols/weekdays@1",
    icu_datetime::provider::neo::DayPeriodNamesV1Marker = "datetime/symbols/dayperiods@1",
    icu_datetime::provider::neo::TimePatternV1Marker = "datetime/patterns/time@1",
    icu_datetime::provider::neo::DateTimePatternV1Marker = "datetime/patterns/datetime@1",
    icu_datetime::provider::neo::BuddhistYearNamesV1Marker = "datetime/symbols/buddhist/years@1",
    icu_datetime::provider::neo::ChineseYearNamesV1Marker = "datetime/symbols/chinese/years@1",
    icu_datetime::provider::neo::CopticYearNamesV1Marker = "datetime/symbols/coptic/years@1",
    icu_datetime::provider::neo::DangiYearNamesV1Marker = "datetime/symbols/dangi/years@1",
    icu_datetime::provider::neo::EthiopianYearNamesV1Marker = "datetime/symbols/ethiopic/years@1",
    icu_datetime::provider::neo::GregorianYearNamesV1Marker = "datetime/symbols/gregory/years@1",
    icu_datetime::provider::neo::HebrewYearNamesV1Marker = "datetime/symbols/hebrew/years@1",
    icu_datetime::provider::neo::IndianYearNamesV1Marker = "datetime/symbols/indian/years@1",
    icu_datetime::provider::neo::IslamicYearNamesV1Marker = "datetime/symbols/islamic/years@1",
    icu_datetime::provider::neo::JapaneseYearNamesV1Marker = "datetime/symbols/japanese/years@1",
    icu_datetime::provider::neo::JapaneseExtendedYearNamesV1Marker =
        "datetime/symbols/japanext/years@1",
    icu_datetime::provider::neo::PersianYearNamesV1Marker = "datetime/symbols/persian/years@1",
    icu_datetime::provider::neo::RocYearNamesV1Marker = "datetime/symbols/roc/years@1",
    icu_datetime::provider::neo::BuddhistMonthNamesV1Marker = "datetime/symbols/buddhist/months@1",
    icu_datetime::provider::neo::ChineseMonthNamesV1Marker = "datetime/symbols/chinese/months@1",
    icu_datetime::provider::neo::CopticMonthNamesV1Marker = "datetime/symbols/coptic/months@1",
    icu_datetime::provider::neo::DangiMonthNamesV1Marker = "datetime/symbols/dangi/months@1",
    icu_datetime::provider::neo::EthiopianMonthNamesV1Marker = "datetime/symbols/ethiopic/months@1",
    icu_datetime::provider::neo::GregorianMonthNamesV1Marker = "datetime/symbols/gregory/months@1",
    icu_datetime::provider::neo::HebrewMonthNamesV1Marker = "datetime/symbols/hebrew/months@1",
    icu_datetime::provider::neo::IndianMonthNamesV1Marker = "datetime/symbols/indian/months@1",
    icu_datetime::provider::neo::IslamicMonthNamesV1Marker = "datetime/symbols/islamic/months@1",
    icu_datetime::provider::neo::JapaneseMonthNamesV1Marker = "datetime/symbols/japanese/months@1",
    icu_datetime::provider::neo::JapaneseExtendedMonthNamesV1Marker =
        "datetime/symbols/japanext/months@1",
    icu_datetime::provider::neo::PersianMonthNamesV1Marker = "datetime/symbols/persian/months@1",
    icu_datetime::provider::neo::RocMonthNamesV1Marker = "datetime/symbols/roc/months@1",
    icu_datetime::provider::neo::BuddhistDatePatternV1Marker = "datetime/patterns/buddhist/date@1",
    icu_datetime::provider::neo::ChineseDatePatternV1Marker = "datetime/patterns/chinese/date@1",
    icu_datetime::provider::neo::CopticDatePatternV1Marker = "datetime/patterns/coptic/date@1",
    icu_datetime::provider::neo::DangiDatePatternV1Marker = "datetime/patterns/dangi/date@1",
    icu_datetime::provider::neo::EthiopianDatePatternV1Marker = "datetime/patterns/ethiopic/date@1",
    icu_datetime::provider::neo::GregorianDatePatternV1Marker = "datetime/patterns/gregory/date@1",
    icu_datetime::provider::neo::HebrewDatePatternV1Marker = "datetime/patterns/hebrew/date@1",
    icu_datetime::provider::neo::IndianDatePatternV1Marker = "datetime/patterns/indian/date@1",
    icu_datetime::provider::neo::IslamicDatePatternV1Marker = "datetime/patterns/islamic/date@1",
    icu_datetime::provider::neo::JapaneseDatePatternV1Marker = "datetime/patterns/japanese/date@1",
    icu_datetime::provider::neo::JapaneseExtendedDatePatternV1Marker =
        "datetime/patterns/japanext/date@1",
    icu_datetime::provider::neo::PersianDatePatternV1Marker = "datetime/patterns/persian/date@1",
    icu_datetime::provider::neo::RocDatePatternV1Marker = "datetime/patterns/roc/date@1",
    icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker =
        "time_zone/generic_long@1",
    icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker =
        "time_zone/generic_short@1",
    icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker =
        "time_zone/specific_long@1",
    icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker =
        "time_zone/specific_short@1",
    icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker = "time_zone/formats@1",
    icu_datetime::provider::time_zones::ExemplarCitiesV1Marker = "time_zone/exemplar_cities@1",
    #[cfg(all())]
    icu_decimal::provider::DecimalSymbolsV1Marker = "decimal/symbols@1",
    #[cfg(feature = "experimental_components")]
    icu_experimental::dimension::provider::currency::CurrencyEssentialsV1Marker =
        "currency/essentials@1",
    icu_experimental::dimension::provider::percent::PercentEssentialsV1Marker =
        "percent/essentials@1",
    #[cfg(feature = "experimental_components")]
    icu_experimental::displaynames::provider::RegionDisplayNamesV1Marker = "displaynames/regions@1",
    icu_experimental::displaynames::provider::LanguageDisplayNamesV1Marker =
        "displaynames/languages@1",
    icu_experimental::displaynames::provider::LocaleDisplayNamesV1Marker = "displaynames/locales@1",
    icu_experimental::displaynames::provider::ScriptDisplayNamesV1Marker = "displaynames/scripts@1",
    icu_experimental::displaynames::provider::VariantDisplayNamesV1Marker =
        "displaynames/variants@1",
    #[cfg(all())]
    icu_list::provider::AndListV1Marker = "list/and@1",
    icu_list::provider::OrListV1Marker = "list/or@1",
    icu_list::provider::UnitListV1Marker = "list/unit@1",
    #[cfg(all())]
    icu_locid_transform::provider::AliasesV1Marker = "locid_transform/aliases@1",
    icu_locid_transform::provider::AliasesV2Marker = "locid_transform/aliases@2",
    icu_locid_transform::provider::CollationFallbackSupplementV1Marker = "fallback/supplement/co@1",
    icu_locid_transform::provider::LikelySubtagsV1Marker = "locid_transform/likelysubtags@1",
    icu_locid_transform::provider::LikelySubtagsExtendedV1Marker =
        "locid_transform/likelysubtags_ext@1",
    icu_locid_transform::provider::LikelySubtagsForLanguageV1Marker =
        "locid_transform/likelysubtags_l@1",
    icu_locid_transform::provider::LikelySubtagsForScriptRegionV1Marker =
        "locid_transform/likelysubtags_sr@1",
    icu_locid_transform::provider::LocaleFallbackLikelySubtagsV1Marker = "fallback/likelysubtags@1",
    icu_locid_transform::provider::LocaleFallbackParentsV1Marker = "fallback/parents@1",
    icu_locid_transform::provider::ScriptDirectionV1Marker = "locid_transform/script_dir@1",
    #[cfg(all())]
    icu_normalizer::provider::CanonicalCompositionsV1Marker = "normalizer/comp@1",
    icu_normalizer::provider::CanonicalDecompositionDataV1Marker = "normalizer/nfd@1",
    icu_normalizer::provider::CanonicalDecompositionTablesV1Marker = "normalizer/nfdex@1",
    icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker = "normalizer/nfkd@1",
    icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker = "normalizer/nfkdex@1",
    icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker = "normalizer/decomp@1",
    icu_normalizer::provider::Uts46DecompositionSupplementV1Marker = "normalizer/uts46d@1",
    #[cfg(all())]
    icu_plurals::provider::CardinalV1Marker = "plurals/cardinal@1",
    icu_plurals::provider::OrdinalV1Marker = "plurals/ordinal@1",
    icu_plurals::provider::PluralRangesV1Marker = "plurals/ranges@1",
    #[cfg(all())]
    icu_properties::provider::AlnumV1Marker = "props/alnum@1",
    icu_properties::provider::AlphabeticV1Marker = "props/Alpha@1",
    icu_properties::provider::AsciiHexDigitV1Marker = "props/AHex@1",
    icu_properties::provider::BasicEmojiV1Marker = "props/Basic_Emoji@1",
    icu_properties::provider::BidiClassV1Marker = "props/bc@1",
    icu_properties::provider::BidiClassNameToValueV1Marker = "propnames/from/bc@1",
    icu_properties::provider::BidiClassValueToLongNameV1Marker = "propnames/to/long/linear/bc@1",
    icu_properties::provider::BidiClassValueToShortNameV1Marker = "propnames/to/short/linear/bc@1",
    icu_properties::provider::BidiControlV1Marker = "props/Bidi_C@1",
    icu_properties::provider::BidiMirroredV1Marker = "props/Bidi_M@1",
    icu_properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker =
        "props/bidiauxiliaryprops@1",
    icu_properties::provider::BlankV1Marker = "props/blank@1",
    icu_properties::provider::CanonicalCombiningClassV1Marker = "props/ccc@1",
    icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker = "propnames/from/ccc@1",
    icu_properties::provider::CanonicalCombiningClassValueToLongNameV1Marker =
        "propnames/to/long/sparse/ccc@1",
    icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker =
        "propnames/to/short/sparse/ccc@1",
    icu_properties::provider::CasedV1Marker = "props/Cased@1",
    icu_properties::provider::CaseIgnorableV1Marker = "props/CI@1",
    icu_properties::provider::CaseSensitiveV1Marker = "props/Sensitive@1",
    icu_properties::provider::ChangesWhenCasefoldedV1Marker = "props/CWCF@1",
    icu_properties::provider::ChangesWhenCasemappedV1Marker = "props/CWCM@1",
    icu_properties::provider::ChangesWhenLowercasedV1Marker = "props/CWL@1",
    icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker = "props/CWKCF@1",
    icu_properties::provider::ChangesWhenTitlecasedV1Marker = "props/CWT@1",
    icu_properties::provider::ChangesWhenUppercasedV1Marker = "props/CWU@1",
    icu_properties::provider::DashV1Marker = "props/Dash@1",
    icu_properties::provider::DefaultIgnorableCodePointV1Marker = "props/DI@1",
    icu_properties::provider::DeprecatedV1Marker = "props/Dep@1",
    icu_properties::provider::DiacriticV1Marker = "props/Dia@1",
    icu_properties::provider::EastAsianWidthV1Marker = "props/ea@1",
    icu_properties::provider::EastAsianWidthNameToValueV1Marker = "propnames/from/ea@1",
    icu_properties::provider::EastAsianWidthValueToLongNameV1Marker =
        "propnames/to/long/linear/ea@1",
    icu_properties::provider::EastAsianWidthValueToShortNameV1Marker =
        "propnames/to/short/linear/ea@1",
    icu_properties::provider::EmojiComponentV1Marker = "props/EComp@1",
    icu_properties::provider::EmojiModifierBaseV1Marker = "props/EBase@1",
    icu_properties::provider::EmojiModifierV1Marker = "props/EMod@1",
    icu_properties::provider::EmojiPresentationV1Marker = "props/EPres@1",
    icu_properties::provider::EmojiV1Marker = "props/Emoji@1",
    icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker =
        "props/exemplarchars/auxiliary@1",
    icu_properties::provider::ExemplarCharactersIndexV1Marker = "props/exemplarchars/index@1",
    icu_properties::provider::ExemplarCharactersMainV1Marker = "props/exemplarchars/main@1",
    icu_properties::provider::ExemplarCharactersNumbersV1Marker = "props/exemplarchars/numbers@1",
    icu_properties::provider::ExemplarCharactersPunctuationV1Marker =
        "props/exemplarchars/punctuation@1",
    icu_properties::provider::ExtendedPictographicV1Marker = "props/ExtPict@1",
    icu_properties::provider::ExtenderV1Marker = "props/Ext@1",
    icu_properties::provider::FullCompositionExclusionV1Marker = "props/Comp_Ex@1",
    icu_properties::provider::GeneralCategoryV1Marker = "props/gc@1",
    icu_properties::provider::GeneralCategoryMaskNameToValueV1Marker = "propnames/from/gcm@1",
    icu_properties::provider::GeneralCategoryNameToValueV1Marker = "propnames/from/gc@1",
    icu_properties::provider::GeneralCategoryValueToLongNameV1Marker =
        "propnames/to/long/linear/gc@1",
    icu_properties::provider::GeneralCategoryValueToShortNameV1Marker =
        "propnames/to/short/linear/gc@1",
    icu_properties::provider::GraphemeBaseV1Marker = "props/Gr_Base@1",
    icu_properties::provider::GraphemeClusterBreakV1Marker = "props/GCB@1",
    icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker = "propnames/from/GCB@1",
    icu_properties::provider::GraphemeClusterBreakValueToLongNameV1Marker =
        "propnames/to/long/linear/GCB@1",
    icu_properties::provider::GraphemeClusterBreakValueToShortNameV1Marker =
        "propnames/to/short/linear/GCB@1",
    icu_properties::provider::GraphemeExtendV1Marker = "props/Gr_Ext@1",
    icu_properties::provider::GraphemeLinkV1Marker = "props/Gr_Link@1",
    icu_properties::provider::HexDigitV1Marker = "props/Hex@1",
    icu_properties::provider::HyphenV1Marker = "props/Hyphen@1",
    icu_properties::provider::IdContinueV1Marker = "props/IDC@1",
    icu_properties::provider::IdeographicV1Marker = "props/Ideo@1",
    icu_properties::provider::IdsBinaryOperatorV1Marker = "props/IDSB@1",
    icu_properties::provider::IdStartV1Marker = "props/IDS@1",
    icu_properties::provider::IdsTrinaryOperatorV1Marker = "props/IDST@1",
    icu_properties::provider::IndicSyllabicCategoryV1Marker = "props/InSC@1",
    icu_properties::provider::IndicSyllabicCategoryNameToValueV1Marker = "propnames/from/InSC@1",
    icu_properties::provider::IndicSyllabicCategoryValueToLongNameV1Marker =
        "propnames/to/long/linear/InSC@1",
    icu_properties::provider::IndicSyllabicCategoryValueToShortNameV1Marker =
        "propnames/to/short/linear/InSC@1",
    icu_properties::provider::GraphV1Marker = "props/graph@1",
    icu_properties::provider::JoinControlV1Marker = "props/Join_C@1",
    icu_properties::provider::JoiningTypeV1Marker = "props/jt@1",
    icu_properties::provider::JoiningTypeNameToValueV1Marker = "propnames/from/jt@1",
    icu_properties::provider::JoiningTypeValueToLongNameV1Marker = "propnames/to/long/linear/jt@1",
    icu_properties::provider::JoiningTypeValueToShortNameV1Marker =
        "propnames/to/short/linear/jt@1",
    icu_properties::provider::LineBreakV1Marker = "props/lb@1",
    icu_properties::provider::LineBreakNameToValueV1Marker = "propnames/from/lb@1",
    icu_properties::provider::LineBreakValueToLongNameV1Marker = "propnames/to/long/linear/lb@1",
    icu_properties::provider::LineBreakValueToShortNameV1Marker = "propnames/to/short/linear/lb@1",
    icu_properties::provider::LogicalOrderExceptionV1Marker = "props/LOE@1",
    icu_properties::provider::LowercaseV1Marker = "props/Lower@1",
    icu_properties::provider::MathV1Marker = "props/Math@1",
    icu_properties::provider::NfcInertV1Marker = "props/nfcinert@1",
    icu_properties::provider::NfdInertV1Marker = "props/nfdinert@1",
    icu_properties::provider::NfkcInertV1Marker = "props/nfkcinert@1",
    icu_properties::provider::NfkdInertV1Marker = "props/nfkdinert@1",
    icu_properties::provider::NoncharacterCodePointV1Marker = "props/NChar@1",
    icu_properties::provider::PatternSyntaxV1Marker = "props/Pat_Syn@1",
    icu_properties::provider::PatternWhiteSpaceV1Marker = "props/Pat_WS@1",
    icu_properties::provider::PrependedConcatenationMarkV1Marker = "props/PCM@1",
    icu_properties::provider::PrintV1Marker = "props/print@1",
    icu_properties::provider::QuotationMarkV1Marker = "props/QMark@1",
    icu_properties::provider::RadicalV1Marker = "props/Radical@1",
    icu_properties::provider::RegionalIndicatorV1Marker = "props/RI@1",
    icu_properties::provider::ScriptV1Marker = "props/sc@1",
    icu_properties::provider::ScriptNameToValueV1Marker = "propnames/from/sc@1",
    icu_properties::provider::ScriptValueToLongNameV1Marker = "propnames/to/long/linear/sc@1",
    icu_properties::provider::ScriptValueToShortNameV1Marker = "propnames/to/short/linear4/sc@1",
    icu_properties::provider::ScriptWithExtensionsPropertyV1Marker = "props/scx@1",
    icu_properties::provider::SegmentStarterV1Marker = "props/segstart@1",
    icu_properties::provider::SentenceBreakV1Marker = "props/SB@1",
    icu_properties::provider::SentenceBreakNameToValueV1Marker = "propnames/from/SB@1",
    icu_properties::provider::SentenceBreakValueToLongNameV1Marker =
        "propnames/to/long/linear/SB@1",
    icu_properties::provider::SentenceBreakValueToShortNameV1Marker =
        "propnames/to/short/linear/SB@1",
    icu_properties::provider::SentenceTerminalV1Marker = "props/STerm@1",
    icu_properties::provider::SoftDottedV1Marker = "props/SD@1",
    icu_properties::provider::TerminalPunctuationV1Marker = "props/Term@1",
    icu_properties::provider::UnifiedIdeographV1Marker = "props/UIdeo@1",
    icu_properties::provider::UppercaseV1Marker = "props/Upper@1",
    icu_properties::provider::WordBreakNameToValueV1Marker = "propnames/from/WB@1",
    icu_properties::provider::WordBreakValueToLongNameV1Marker = "propnames/to/long/linear/WB@1",
    icu_properties::provider::WordBreakValueToShortNameV1Marker = "propnames/to/short/linear/WB@1",
    icu_properties::provider::XdigitV1Marker = "props/xdigit@1",
    icu_properties::provider::XidContinueV1Marker = "props/XIDC@1",
    icu_properties::provider::XidStartV1Marker = "props/XIDS@1",
    icu_properties::provider::VariationSelectorV1Marker = "props/VS@1",
    icu_properties::provider::WhiteSpaceV1Marker = "props/WSpace@1",
    icu_properties::provider::WordBreakV1Marker = "props/WB@1",
    #[cfg(feature = "experimental_components")]
    icu_experimental::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker =
        "relativetime/long/second@1",
    icu_experimental::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker =
        "relativetime/short/second@1",
    icu_experimental::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/second@1",
    icu_experimental::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker =
        "relativetime/long/minute@1",
    icu_experimental::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker =
        "relativetime/short/minute@1",
    icu_experimental::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/minute@1",
    icu_experimental::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker =
        "relativetime/long/hour@1",
    icu_experimental::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker =
        "relativetime/short/hour@1",
    icu_experimental::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/hour@1",
    icu_experimental::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker =
        "relativetime/long/day@1",
    icu_experimental::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker =
        "relativetime/short/day@1",
    icu_experimental::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/day@1",
    icu_experimental::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker =
        "relativetime/long/week@1",
    icu_experimental::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker =
        "relativetime/short/week@1",
    icu_experimental::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/week@1",
    icu_experimental::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker =
        "relativetime/long/month@1",
    icu_experimental::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker =
        "relativetime/short/month@1",
    icu_experimental::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/month@1",
    icu_experimental::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker =
        "relativetime/long/quarter@1",
    icu_experimental::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker =
        "relativetime/short/quarter@1",
    icu_experimental::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/quarter@1",
    icu_experimental::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker =
        "relativetime/long/year@1",
    icu_experimental::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker =
        "relativetime/short/year@1",
    icu_experimental::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker =
        "relativetime/narrow/year@1",
    #[cfg(all())]
    icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker =
        "segmenter/dictionary/wl_ext@1",
    icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker = "segmenter/dictionary/w_auto@1",
    icu_segmenter::provider::GraphemeClusterBreakDataV1Marker = "segmenter/grapheme@1",
    icu_segmenter::provider::LineBreakDataV1Marker = "segmenter/line@1",
    icu_segmenter::provider::LstmForWordLineAutoV1Marker = "segmenter/lstm/wl_auto@1",
    icu_segmenter::provider::SentenceBreakDataV1Marker = "segmenter/sentence@1",
    icu_segmenter::provider::WordBreakDataV1Marker = "segmenter/word@1",
    #[cfg(all())]
    icu_timezone::provider::MetazonePeriodV1Marker = "time_zone/metazone_period@1",
    icu_timezone::provider::names::Bcp47ToIanaMapV1Marker = "time_zone/bcp47_to_iana@1",
    icu_timezone::provider::names::IanaToBcp47MapV1Marker = "time_zone/iana_to_bcp47@1",
    icu_timezone::provider::names::IanaToBcp47MapV2Marker = "time_zone/iana_to_bcp47@2",
    #[cfg(feature = "experimental_components")]
    icu_experimental::transliterate::provider::TransliteratorRulesV1Marker =
        "transliterator/rules@1",
    #[cfg(feature = "experimental_components")]
    icu_experimental::units::provider::UnitsInfoV1Marker = "units/info@1",
);

/// Same as `all_keys`.
///
/// Note that since v1.3, `all_keys` also contains experimental keys for which the
/// corresponding Cargo features has been enabled.
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
#[deprecated(
    since = "1.3.0",
    note = "use `all_keys` with the required cargo features"
)]
#[cfg(feature = "legacy_api")]
pub fn all_keys_with_experimental() -> Vec<DataKey> {
    all_keys()
}

#[test]
fn no_key_collisions() {
    let mut map = std::collections::BTreeMap::new();
    let mut failed = false;
    for key in all_keys() {
        if let Some(colliding_key) = map.insert(key.hashed(), key) {
            println!(
                "{:?} and {:?} collide at {:?}",
                key.path(),
                colliding_key.path(),
                key.hashed()
            );
            failed = true;
        }
    }
    if failed {
        panic!();
    }
}
