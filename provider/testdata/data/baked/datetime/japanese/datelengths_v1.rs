// @generated
#![cfg(feature = "icu_datetime")]
type DataStruct = < :: icu_datetime :: provider :: calendar :: JapaneseDateLengthsV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN_CCP),
        ("ccp", BN_CCP),
        ("en", EN_EN_001_EN_ZA_FIL),
        ("en-001", EN_EN_001_EN_ZA_FIL),
        ("en-ZA", EN_EN_001_EN_ZA_FIL),
        ("es", ES),
        ("es-AR", ES_AR),
        ("fil", EN_EN_001_EN_ZA_FIL),
        ("fr", FR),
        ("ja", JA),
        ("ru", RU),
        ("sr", SR_SR_CYRL_SR_LATN),
        ("sr-Cyrl", SR_SR_CYRL_SR_LATN),
        ("sr-Latn", SR_SR_CYRL_SR_LATN),
        ("th", TH),
        ("tr", TR),
        ("und", UND),
    ]);
static AR_AR_EG: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 6u8, 12u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8,
                    128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 32u8,
                    15u8, 0u8, 0u8, 47u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 32u8,
                    15u8, 0u8, 0u8, 47u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 6u8, 12u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 6u8, 12u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 6u8, 12u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static BN_CCP: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8,
                    0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static EN_EN_001_EN_ZA_FIL: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8,
                    32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8,
                    0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static ES: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                    0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                    0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static ES_AR: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                    0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                    0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static FR: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                    32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static JA: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8,
                    103u8, 8u8, 128u8, 64u8, 1u8, 0u8, 101u8, 229u8, 128u8, 80u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8,
                    103u8, 8u8, 128u8, 64u8, 1u8, 0u8, 101u8, 229u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8,
                    103u8, 8u8, 128u8, 64u8, 1u8, 0u8, 101u8, 229u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 5u8, 128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8,
                    47u8, 128u8, 64u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static RU: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 32u8, 47u8, 0u8,
                    4u8, 51u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 32u8, 47u8, 0u8, 4u8, 51u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8,
                    128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 32u8, 47u8, 0u8, 4u8, 51u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8,
                    128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 46u8, 128u8, 32u8, 2u8, 0u8, 0u8, 46u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static SR_SR_CYRL_SR_LATN: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8,
                    32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8,
                    0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static TH: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 14u8, 72u8, 0u8, 0u8,
                    32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8,
                    14u8, 27u8, 0u8, 14u8, 53u8, 128u8, 0u8, 1u8, 0u8, 14u8, 23u8, 0u8, 14u8, 53u8,
                    0u8, 14u8, 72u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8, 14u8,
                    27u8, 0u8, 14u8, 53u8, 128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                    1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static TR: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 80u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 128u8, 32u8, 2u8, 0u8, 0u8, 46u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static UND: &DataStruct = &::icu_datetime::provider::calendar::DateLengthsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8,
                    4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8,
                    80u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8,
                    4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8,
                    3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 5u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 45u8, 128u8, 32u8,
                    2u8, 0u8, 0u8, 45u8, 128u8, 64u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        medium: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        short: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
