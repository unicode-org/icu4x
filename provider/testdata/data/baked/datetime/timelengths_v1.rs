// @generated
type DataStruct = < :: icu_datetime :: provider :: calendar :: TimeLengthsV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_slice_unchecked(&[
        ("ar", AR_AR_EG_BN_CCP_EN_EN_001_FIL),
        ("ar-EG", AR_AR_EG_BN_CCP_EN_EN_001_FIL),
        ("bn", AR_AR_EG_BN_CCP_EN_EN_001_FIL),
        ("ccp", AR_AR_EG_BN_CCP_EN_EN_001_FIL),
        ("en", AR_AR_EG_BN_CCP_EN_EN_001_FIL),
        ("en-001", AR_AR_EG_BN_CCP_EN_EN_001_FIL),
        ("en-ZA", EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND),
        ("es", ES),
        ("es-AR", ES_AR),
        ("fil", AR_AR_EG_BN_CCP_EN_EN_001_FIL),
        ("fr", EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND),
        ("ja", JA),
        ("ru", EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND),
        ("sr", EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND),
        ("sr-Cyrl", EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND),
        ("sr-Latn", EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND),
        ("th", TH),
        ("tr", TR),
        ("und", EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND),
    ]);
static AR_AR_EG_BN_CCP_EN_EN_001_FIL: &DataStruct =
    &::icu_datetime::provider::calendar::TimeLengthsV1 {
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        160u8, 4u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        160u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8,
                        128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
            },
        },
        time_h23_h24: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
            },
        },
        preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H11H12,
    };
static EN_ZA_FR_RU_SR_SR_CYRL_SR_LATN_UND: &DataStruct =
    &::icu_datetime::provider::calendar::TimeLengthsV1 {
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        163u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        163u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8,
                        128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
            },
        },
        time_h23_h24: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 4u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
            },
        },
        preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H23H24,
    };
static ES: &DataStruct = &::icu_datetime::provider::calendar::TimeLengthsV1 {
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8, 128u8,
                    96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    time_h23_h24: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 0u8, 0u8, 40u8, 128u8, 160u8, 4u8, 0u8, 0u8, 41u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H23H24,
};
static ES_AR: &DataStruct = &::icu_datetime::provider::calendar::TimeLengthsV1 {
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8, 128u8,
                    96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    time_h23_h24: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H23H24,
};
static JA: &DataStruct = &::icu_datetime::provider::calendar::TimeLengthsV1 {
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8,
                    0u8, 58u8, 128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8,
                    0u8, 58u8, 128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8,
                    0u8, 58u8, 128u8, 144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    time_h23_h24: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 102u8, 66u8, 128u8, 128u8, 2u8, 0u8, 82u8, 6u8, 128u8,
                    144u8, 2u8, 0u8, 121u8, 210u8, 0u8, 0u8, 32u8, 128u8, 160u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H23H24,
};
static TH: &DataStruct = &::icu_datetime::provider::calendar::TimeLengthsV1 {
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8, 128u8,
                    96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    time_h23_h24: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8, 14u8,
                    44u8, 0u8, 14u8, 52u8, 0u8, 14u8, 1u8, 0u8, 14u8, 50u8, 0u8, 0u8, 32u8, 128u8,
                    128u8, 2u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8, 14u8, 23u8,
                    0u8, 14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 0u8, 14u8,
                    39u8, 0u8, 14u8, 52u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8, 14u8, 23u8, 0u8,
                    14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 160u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 1u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8, 14u8,
                    44u8, 0u8, 14u8, 52u8, 0u8, 14u8, 1u8, 0u8, 14u8, 50u8, 0u8, 0u8, 32u8, 128u8,
                    128u8, 2u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8, 14u8, 23u8,
                    0u8, 14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 0u8, 14u8,
                    39u8, 0u8, 14u8, 52u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8, 14u8, 23u8, 0u8,
                    14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H23H24,
};
static TR: &DataStruct = &::icu_datetime::provider::calendar::TimeLengthsV1 {
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8,
                    128u8, 2u8, 0u8, 0u8, 58u8, 128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8,
                    128u8, 2u8, 0u8, 0u8, 58u8, 128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8,
                    1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8,
                    128u8, 2u8, 0u8, 0u8, 58u8, 128u8, 144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8,
                    128u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    time_h23_h24: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Minutes,
        },
    },
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H23H24,
};
