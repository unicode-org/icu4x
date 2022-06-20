// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{ResourceKey, ResourceMarker};

/// List of all supported keys
pub fn get_all_keys() -> Vec<ResourceKey> {
    let mut v = vec![
        icu_calendar::provider::JapaneseErasV1Marker::KEY,
        icu_datetime::provider::calendar::DatePatternsV1Marker::KEY,
        icu_datetime::provider::calendar::TimePatternsV1Marker::KEY,
        icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY,
        icu_datetime::provider::calendar::DateSymbolsV1Marker::KEY,
        icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY,
        icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY,
        icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker::KEY,
        icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker::KEY,
        icu_datetime::provider::time_zones::MetaZonePeriodV1Marker::KEY,
        icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker::KEY,
        icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker::KEY,
        icu_datetime::provider::week_data::WeekDataV1Marker::KEY,
        icu_decimal::provider::DecimalSymbolsV1Marker::KEY,
        icu_list::provider::AndListV1Marker::KEY,
        icu_list::provider::OrListV1Marker::KEY,
        icu_list::provider::UnitListV1Marker::KEY,
        icu_locale_canonicalizer::provider::AliasesV1Marker::KEY,
        icu_locale_canonicalizer::provider::LikelySubtagsV1Marker::KEY,
        icu_plurals::provider::CardinalV1Marker::KEY,
        icu_plurals::provider::OrdinalV1Marker::KEY,
        icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker::KEY,
        icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_casemapping::provider::CaseMappingV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::CanonicalDecompositionDataV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::Uts46DecompositionSupplementV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::CanonicalDecompositionTablesV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::CanonicalCompositionsV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::CanonicalCompositionPassthroughV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::CompatibilityCompositionPassthroughV1Marker::KEY,
        #[cfg(feature = "experimental")]
        icu_normalizer::provider::Uts46CompositionPassthroughV1Marker::KEY,
    ];
    v.extend(icu_properties::provider::ALL_KEYS);
    #[cfg(feature = "experimental")]
    v.extend(icu_segmenter::ALL_KEYS);
    #[cfg(feature = "experimental")]
    v.extend(crate::transform::collator::ALL_KEYS);
    v
}

/// Create a data provider reading from source files that generates data for all,
/// or a subset, of ICU4X.
///
/// The macro behaves like a function that takes the following arguments:
///
/// 1. An instance of [`SourceData`](crate::SourceData) (required)
/// 2. A list of providers to instantiate (optional)
///
/// The return value is a complex type that implements all of the key data provider traits.
///
/// To create a data provider for all of ICU4X:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports all keys required by ICU4X.
/// let provider = icu_datagen::create_datagen_provider!(SourceData::default());
/// ```
///
/// To create a data provider for a subset:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports the keys for LocaleCanonicalizer.
/// let provider = icu_datagen::create_datagen_provider!(
///     SourceData::default(),
///     [
///         icu_datagen::transform::cldr::AliasesProvider,
///         icu_datagen::transform::cldr::LikelySubtagsProvider,
///     ]
/// );
/// ```
#[macro_export]
#[cfg(not(feature = "experimental"))]
macro_rules! create_datagen_provider {
    ($source_data:expr) => {
        $crate::create_datagen_provider!(
            $source_data,
            [
                $crate::transform::cldr::AliasesProvider,
                $crate::transform::cldr::CommonDateProvider,
                $crate::transform::cldr::FallbackRulesProvider,
                $crate::transform::cldr::JapaneseErasProvider,
                $crate::transform::cldr::LikelySubtagsProvider,
                $crate::transform::cldr::NumbersProvider,
                $crate::transform::cldr::PluralsProvider,
                $crate::transform::cldr::TimeZonesProvider,
                $crate::transform::cldr::WeekDataProvider,
                $crate::transform::cldr::ListProvider,
                $crate::transform::uprops::EnumeratedPropertyCodePointTrieProvider,
                $crate::transform::uprops::ScriptWithExtensionsPropertyProvider,
                $crate::transform::uprops::BinaryPropertyUnicodeSetDataProvider,
            ]
        )
    };
    ($source_data:expr, [ $($constructor:path),+, ]) => {{
        let __source = &$source_data;
        icu_provider_adapters::make_forking_provider!(
            icu_provider_adapters::fork::by_key::ForkByKeyProvider,
            [
                icu_provider::hello_world::HelloWorldProvider::new_with_placeholder_data(),
                $(<$constructor>::from(__source)),+,
            ]
        )
    }};
}

/// Create a data provider reading from source files that generates data for all,
/// or a subset, of ICU4X.
///
/// The macro behaves like a function that takes the following arguments:
///
/// 1. An instance of [`SourceData`](crate::SourceData) (required)
/// 2. A list of providers to instantiate (optional)
///
/// The return value is a complex type that implements all of the key data provider traits.
///
/// To create a data provider for all of ICU4X:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports all keys required by ICU4X.
/// let provider = icu_datagen::create_datagen_provider!(SourceData::default());
/// ```
///
/// To create a data provider for a subset:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports the keys for LocaleCanonicalizer.
/// let provider = icu_datagen::create_datagen_provider!(
///     SourceData::default(),
///     [
///         icu_datagen::transform::cldr::AliasesProvider,
///         icu_datagen::transform::cldr::LikelySubtagsProvider,
///     ]
/// );
/// ```
#[macro_export]
#[cfg(feature = "experimental")]
macro_rules! create_datagen_provider {
    ($source_data:expr) => {
        $crate::create_datagen_provider!(
            $source_data,
            [
                $crate::transform::cldr::AliasesProvider,
                $crate::transform::cldr::CommonDateProvider,
                $crate::transform::cldr::FallbackRulesProvider,
                $crate::transform::cldr::JapaneseErasProvider,
                $crate::transform::cldr::LikelySubtagsProvider,
                $crate::transform::cldr::NumbersProvider,
                $crate::transform::cldr::PluralsProvider,
                $crate::transform::cldr::TimeZonesProvider,
                $crate::transform::cldr::WeekDataProvider,
                $crate::transform::cldr::ListProvider,
                $crate::transform::uprops::CaseMappingDataProvider,
                $crate::transform::uprops::EnumeratedPropertyCodePointTrieProvider,
                $crate::transform::uprops::ScriptWithExtensionsPropertyProvider,
                $crate::transform::uprops::BinaryPropertyUnicodeSetDataProvider,
                $crate::transform::segmenter::SegmenterRuleProvider,
                $crate::transform::uprops::CanonicalDecompositionDataProvider,
                $crate::transform::uprops::CompatibilityDecompositionSupplementProvider,
                $crate::transform::uprops::Uts46DecompositionSupplementProvider,
                $crate::transform::uprops::CanonicalDecompositionTablesProvider,
                $crate::transform::uprops::CompatibilityDecompositionTablesProvider,
                $crate::transform::uprops::CanonicalCompositionsProvider,
                $crate::transform::uprops::CanonicalCompositionPassthroughProvider,
                $crate::transform::uprops::CompatibilityCompositionPassthroughProvider,
                $crate::transform::uprops::Uts46CompositionPassthroughProvider,
                $crate::transform::collator::CollationProvider,
            ]
        )
    };
    ($source_data:expr, [ $($constructor:path),+, ]) => {{
        let __source = &$source_data;
        icu_provider_adapters::make_forking_provider!(
            icu_provider_adapters::fork::by_key::ForkByKeyProvider,
            [
                icu_provider::hello_world::HelloWorldProvider::new_with_placeholder_data(),
                $(<$constructor>::from(__source)),+,
            ]
        )
    }};
}

#[test]
fn no_key_collisions() {
    let mut map = std::collections::BTreeMap::new();
    let mut failed = false;
    for key in get_all_keys() {
        if let Some(colliding_key) = map.insert(key.get_hash(), key) {
            println!(
                "{:?} and {:?} collide at {:?}",
                key.get_path(),
                colliding_key.get_path(),
                key.get_hash()
            );
            failed = true;
        }
    }
    if failed {
        panic!();
    }
}
