// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_datetime::provider::calendar::DatePatternsV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_datetime::provider::calendar::DatePatternsV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[
            (
                "ar-EG-u-ca-buddhist",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "ar-EG-u-ca-coptic",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "ar-EG-u-ca-ethiopic",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            ("ar-EG-u-ca-gregory", AR_U_CA_GREGORY_AR_EG_U_CA_GREGORY),
            (
                "ar-EG-u-ca-indian",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "ar-EG-u-ca-japanese",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "ar-u-ca-buddhist",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "ar-u-ca-coptic",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "ar-u-ca-ethiopic",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            ("ar-u-ca-gregory", AR_U_CA_GREGORY_AR_EG_U_CA_GREGORY),
            (
                "ar-u-ca-indian",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "ar-u-ca-japanese",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
            (
                "bn-u-ca-buddhist",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "bn-u-ca-coptic",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "bn-u-ca-ethiopic",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            ("bn-u-ca-gregory", BN_U_CA_GREGORY_CCP_U_CA_GREGORY),
            (
                "bn-u-ca-indian",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "bn-u-ca-japanese",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "ccp-u-ca-buddhist",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "ccp-u-ca-coptic",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "ccp-u-ca-ethiopic",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            ("ccp-u-ca-gregory", BN_U_CA_GREGORY_CCP_U_CA_GREGORY),
            (
                "ccp-u-ca-indian",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "ccp-u-ca-japanese",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "en-001-u-ca-buddhist",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            (
                "en-001-u-ca-coptic",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            (
                "en-001-u-ca-ethiopic",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            ("en-001-u-ca-gregory", EN_001_U_CA_GREGORY),
            (
                "en-001-u-ca-indian",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            (
                "en-001-u-ca-japanese",
                EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC,
            ),
            ("en-ZA-u-ca-buddhist", EN_ZA_U_CA_BUDDHIST_EN_ZA_U_CA_COPTIC),
            ("en-ZA-u-ca-coptic", EN_ZA_U_CA_BUDDHIST_EN_ZA_U_CA_COPTIC),
            ("en-ZA-u-ca-ethiopic", EN_ZA_U_CA_BUDDHIST_EN_ZA_U_CA_COPTIC),
            ("en-ZA-u-ca-gregory", EN_ZA_U_CA_GREGORY),
            ("en-ZA-u-ca-indian", EN_ZA_U_CA_BUDDHIST_EN_ZA_U_CA_COPTIC),
            ("en-ZA-u-ca-japanese", EN_ZA_U_CA_JAPANESE),
            (
                "en-u-ca-buddhist",
                EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC,
            ),
            (
                "en-u-ca-coptic",
                EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC,
            ),
            (
                "en-u-ca-ethiopic",
                EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC,
            ),
            ("en-u-ca-gregory", EN_U_CA_GREGORY),
            (
                "en-u-ca-indian",
                EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC,
            ),
            (
                "en-u-ca-japanese",
                EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC,
            ),
            ("es-AR-u-ca-buddhist", ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC),
            ("es-AR-u-ca-coptic", ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC),
            ("es-AR-u-ca-ethiopic", ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC),
            ("es-AR-u-ca-gregory", ES_AR_U_CA_GREGORY),
            ("es-AR-u-ca-indian", ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC),
            ("es-AR-u-ca-japanese", ES_AR_U_CA_JAPANESE),
            (
                "es-u-ca-buddhist",
                ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC,
            ),
            (
                "es-u-ca-coptic",
                ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC,
            ),
            (
                "es-u-ca-ethiopic",
                ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC,
            ),
            ("es-u-ca-gregory", ES_U_CA_GREGORY),
            (
                "es-u-ca-indian",
                ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC,
            ),
            ("es-u-ca-japanese", ES_U_CA_JAPANESE),
            (
                "fil-u-ca-buddhist",
                FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC,
            ),
            (
                "fil-u-ca-coptic",
                FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC,
            ),
            (
                "fil-u-ca-ethiopic",
                FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC,
            ),
            ("fil-u-ca-gregory", FIL_U_CA_GREGORY),
            (
                "fil-u-ca-indian",
                FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC,
            ),
            (
                "fil-u-ca-japanese",
                FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC,
            ),
            (
                "fr-u-ca-buddhist",
                FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC,
            ),
            (
                "fr-u-ca-coptic",
                FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC,
            ),
            (
                "fr-u-ca-ethiopic",
                FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC,
            ),
            ("fr-u-ca-gregory", FR_U_CA_GREGORY),
            (
                "fr-u-ca-indian",
                FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC,
            ),
            ("fr-u-ca-japanese", FR_U_CA_JAPANESE),
            ("ja-u-ca-buddhist", JA_U_CA_BUDDHIST),
            (
                "ja-u-ca-coptic",
                JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC_JA_U_CA_INDIAN,
            ),
            (
                "ja-u-ca-ethiopic",
                JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC_JA_U_CA_INDIAN,
            ),
            ("ja-u-ca-gregory", JA_U_CA_GREGORY),
            (
                "ja-u-ca-indian",
                JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC_JA_U_CA_INDIAN,
            ),
            ("ja-u-ca-japanese", JA_U_CA_JAPANESE),
            (
                "ru-u-ca-buddhist",
                RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC,
            ),
            (
                "ru-u-ca-coptic",
                RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC,
            ),
            (
                "ru-u-ca-ethiopic",
                RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC,
            ),
            ("ru-u-ca-gregory", RU_U_CA_GREGORY),
            (
                "ru-u-ca-indian",
                RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC,
            ),
            (
                "ru-u-ca-japanese",
                RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC,
            ),
            (
                "sr-Cyrl-u-ca-buddhist",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Cyrl-u-ca-coptic",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Cyrl-u-ca-ethiopic",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            ("sr-Cyrl-u-ca-gregory", SR_U_CA_GREGORY_SR_CYRL_U_CA_GREGORY),
            (
                "sr-Cyrl-u-ca-indian",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Cyrl-u-ca-japanese",
                SR_U_CA_JAPANESE_SR_CYRL_U_CA_JAPANESE,
            ),
            (
                "sr-Latn-u-ca-buddhist",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Latn-u-ca-coptic",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Latn-u-ca-ethiopic",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            ("sr-Latn-u-ca-gregory", SR_U_CA_GREGORY_SR_CYRL_U_CA_GREGORY),
            (
                "sr-Latn-u-ca-indian",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Latn-u-ca-japanese",
                SR_U_CA_JAPANESE_SR_CYRL_U_CA_JAPANESE,
            ),
            (
                "sr-u-ca-buddhist",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-u-ca-coptic",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-u-ca-ethiopic",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            ("sr-u-ca-gregory", SR_U_CA_GREGORY_SR_CYRL_U_CA_GREGORY),
            (
                "sr-u-ca-indian",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            ("sr-u-ca-japanese", SR_U_CA_JAPANESE_SR_CYRL_U_CA_JAPANESE),
            ("th-u-ca-buddhist", TH_U_CA_BUDDHIST),
            (
                "th-u-ca-coptic",
                TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC_TH_U_CA_INDIAN,
            ),
            (
                "th-u-ca-ethiopic",
                TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC_TH_U_CA_INDIAN,
            ),
            ("th-u-ca-gregory", TH_U_CA_GREGORY),
            (
                "th-u-ca-indian",
                TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC_TH_U_CA_INDIAN,
            ),
            ("th-u-ca-japanese", TH_U_CA_JAPANESE),
            (
                "tr-u-ca-buddhist",
                TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC,
            ),
            (
                "tr-u-ca-coptic",
                TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC,
            ),
            (
                "tr-u-ca-ethiopic",
                TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC,
            ),
            ("tr-u-ca-gregory", TR_U_CA_GREGORY),
            (
                "tr-u-ca-indian",
                TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC,
            ),
            ("tr-u-ca-japanese", TR_U_CA_JAPANESE),
            (
                "und-u-ca-buddhist",
                UND_U_CA_BUDDHIST_UND_U_CA_COPTIC_UND_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-coptic",
                UND_U_CA_BUDDHIST_UND_U_CA_COPTIC_UND_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-ethiopic",
                UND_U_CA_BUDDHIST_UND_U_CA_COPTIC_UND_U_CA_ETHIOPIC,
            ),
            ("und-u-ca-gregory", UND_U_CA_GREGORY),
            (
                "und-u-ca-indian",
                UND_U_CA_BUDDHIST_UND_U_CA_COPTIC_UND_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-japanese",
                UND_U_CA_BUDDHIST_UND_U_CA_COPTIC_UND_U_CA_ETHIOPIC,
            ),
        ];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <::icu_datetime::provider::calendar::DatePatternsV1Marker>::KEY,
                    req,
                )
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct =
    &'static <::icu_datetime::provider::calendar::DatePatternsV1Marker as DataMarker>::Yokeable;
static AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 6u8, 12u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8,
                        32u8, 128u8, 0u8, 1u8,
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
                        128u8, 64u8, 2u8, 0u8, 32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8,
                        32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                        1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8,
                        32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                        5u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
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
                        128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
                full: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 6u8, 65u8, 0u8, 6u8, 74u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                long: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 6u8, 65u8, 0u8, 6u8, 74u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                medium: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 6u8, 65u8, 0u8, 6u8, 74u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                short: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 6u8, 65u8, 0u8, 6u8, 74u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
            },
    };
static AR_U_CA_GREGORY_AR_EG_U_CA_GREGORY: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 6u8, 12u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                        16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 2u8, 0u8, 32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8,
                        32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8,
                        32u8, 15u8, 0u8, 0u8, 47u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
                full: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 6u8, 65u8, 0u8, 6u8, 74u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                long: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 6u8, 65u8, 0u8, 6u8, 74u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
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
static BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8,
                        1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
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
                        128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static BN_U_CA_GREGORY_CCP_U_CA_GREGORY: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8,
                        1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                        16u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8,
                        32u8, 128u8, 0u8, 1u8,
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
                        128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
                full: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                long: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
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
static EN_001_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 160u8,
                    4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 160u8,
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
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
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
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H11H12,
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
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
static EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                        0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8,
                        1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
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
                        128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
                full: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                long: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
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
static EN_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8,
                    32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 160u8,
                    4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 160u8,
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
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
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
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H11H12,
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
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
static EN_ZA_U_CA_BUDDHIST_EN_ZA_U_CA_COPTIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 2u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8,
                        32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 2u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                        16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 2u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                        16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 5u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                        32u8, 2u8, 0u8, 0u8, 47u8, 128u8, 64u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
                full: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                long: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8,
                            0u8, 32u8, 128u8, 0u8, 0u8,
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
static EN_ZA_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 2u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    64u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
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
static EN_ZA_U_CA_JAPANESE: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
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
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 97u8, 0u8, 0u8, 116u8, 0u8, 0u8,
                    32u8, 128u8, 0u8, 0u8,
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
static ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 32u8,
                        4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8,
                        128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8,
                        101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8,
                        101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
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
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static ES_AR_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                    0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                    0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
                    128u8, 0u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static ES_AR_U_CA_JAPANESE: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
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
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
static ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 32u8,
                        4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8,
                        128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8,
                        101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8,
                        101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
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
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
                        128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 0u8, 0u8, 40u8, 128u8, 160u8, 4u8, 0u8,
                        0u8, 41u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static ES_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                    0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 0u8, 0u8, 100u8, 0u8, 0u8, 101u8, 0u8,
                    0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
static ES_U_CA_JAPANESE: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
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
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
static FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                        0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8,
                        1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
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
                        128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
                full: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 110u8, 0u8, 0u8, 97u8, 0u8,
                            0u8, 110u8, 0u8, 0u8, 103u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                        ])
                    },
                },
                long: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 110u8, 0u8, 0u8, 97u8, 0u8,
                            0u8, 110u8, 0u8, 0u8, 103u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
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
static FIL_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8,
                    32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                    32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 160u8,
                    4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8, 0u8, 0u8, 32u8, 128u8, 160u8,
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
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 114u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 163u8, 1u8,
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
    preferred_hour_cycle: ::icu_datetime::pattern::CoarseHourCycle::H11H12,
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 110u8, 0u8, 0u8, 97u8, 0u8, 0u8,
                    110u8, 0u8, 0u8, 103u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 110u8, 0u8, 0u8, 97u8, 0u8, 0u8,
                    110u8, 0u8, 0u8, 103u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
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
static FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                        1u8,
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
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
                full: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 224u8, 0u8, 0u8, 32u8,
                            128u8, 0u8, 0u8,
                        ])
                    },
                },
                long: ::icu_datetime::pattern::runtime::GenericPattern {
                    items: unsafe {
                        ::zerovec::ZeroVec::from_bytes_unchecked(&[
                            128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 224u8, 0u8, 0u8, 32u8,
                            128u8, 0u8, 0u8,
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
static FR_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                    32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
    length_combinations: ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 224u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                    0u8,
                ])
            },
        },
        long: ::icu_datetime::pattern::runtime::GenericPattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 0u8, 0u8, 224u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                    0u8,
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
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 0u8,
                ])
            },
        },
    },
};
static FR_U_CA_JAPANESE: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
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
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
static JA_U_CA_BUDDHIST: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 4u8, 128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8,
                    103u8, 8u8, 128u8, 64u8, 1u8, 0u8, 101u8, 229u8, 128u8, 80u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 4u8, 128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8,
                    103u8, 8u8, 128u8, 64u8, 1u8, 0u8, 101u8, 229u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8,
                    47u8, 128u8, 64u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 0u8, 1u8, 128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8,
                    47u8, 128u8, 64u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8,
                    0u8, 58u8, 128u8, 144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8,
                    0u8, 58u8, 128u8, 144u8, 2u8,
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
static JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC_JA_U_CA_INDIAN: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 1u8, 128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8,
                        103u8, 8u8, 128u8, 64u8, 1u8, 0u8, 101u8, 229u8, 0u8, 0u8, 40u8, 128u8,
                        80u8, 4u8, 0u8, 0u8, 41u8,
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
                        128u8, 0u8, 5u8, 128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8,
                        0u8, 47u8, 128u8, 64u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 5u8, 128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8,
                        0u8, 47u8, 128u8, 64u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                        0u8, 0u8, 58u8, 128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                        0u8, 0u8, 58u8, 128u8, 144u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8,
                        0u8, 0u8, 58u8, 128u8, 144u8, 2u8,
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
                        128u8, 114u8, 1u8, 0u8, 102u8, 66u8, 128u8, 128u8, 2u8, 0u8, 82u8, 6u8,
                        128u8, 144u8, 2u8, 0u8, 121u8, 210u8, 0u8, 0u8, 32u8, 128u8, 160u8, 4u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static JA_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8, 103u8, 8u8, 128u8,
                    64u8, 1u8, 0u8, 101u8, 229u8, 128u8, 80u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 94u8, 116u8, 128u8, 32u8, 1u8, 0u8, 103u8, 8u8, 128u8,
                    64u8, 1u8, 0u8, 101u8, 229u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    64u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 2u8, 0u8, 0u8, 47u8, 128u8,
                    64u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
static JA_U_CA_JAPANESE: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
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
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8,
                    0u8, 58u8, 128u8, 144u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 96u8, 1u8, 128u8, 112u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8,
                    0u8, 58u8, 128u8, 144u8, 2u8,
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
static RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8,
                        32u8, 0u8, 4u8, 51u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                        16u8, 1u8, 0u8, 0u8, 32u8, 0u8, 4u8, 51u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8,
                        128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                        16u8, 1u8, 0u8, 0u8, 32u8, 0u8, 4u8, 51u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8,
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
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static RU_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8,
                    32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 0u8,
                    4u8, 51u8, 0u8, 0u8, 46u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 0u8, 4u8, 51u8, 0u8, 0u8, 46u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 0u8, 4u8, 51u8, 0u8, 0u8, 46u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 2u8, 0u8, 0u8, 46u8, 128u8, 32u8, 2u8, 0u8, 0u8, 46u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
static SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8,
                        1u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                        1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 128u8, 32u8, 2u8, 0u8, 0u8, 46u8, 128u8,
                        16u8, 1u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 128u8, 32u8, 1u8, 0u8, 0u8, 46u8, 128u8,
                        16u8, 1u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 0u8, 5u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static SR_U_CA_GREGORY_SR_CYRL_U_CA_GREGORY: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8,
                        0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8,
                        1u8, 0u8, 0u8, 46u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 46u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 32u8, 1u8, 0u8,
                        0u8, 46u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 46u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 128u8, 32u8, 1u8, 0u8, 0u8, 46u8, 128u8,
                        16u8, 2u8, 0u8, 0u8, 46u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static SR_U_CA_JAPANESE_SR_CYRL_U_CA_JAPANESE: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8,
                        0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 16u8,
                        1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8,
                        0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
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
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 2u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static TH_U_CA_BUDDHIST: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 14u8, 72u8, 0u8, 0u8,
                    32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8,
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
static TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC_TH_U_CA_INDIAN: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 80u8, 4u8, 0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 14u8, 72u8, 0u8,
                        0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8,
                        32u8, 128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                        0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                        0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                        16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 0u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
                        128u8, 114u8, 1u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8,
                        14u8, 44u8, 0u8, 14u8, 52u8, 0u8, 14u8, 1u8, 0u8, 14u8, 50u8, 0u8, 0u8,
                        32u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8,
                        0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 144u8, 2u8, 0u8,
                        0u8, 32u8, 0u8, 14u8, 39u8, 0u8, 14u8, 52u8, 0u8, 14u8, 25u8, 0u8, 14u8,
                        50u8, 0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 160u8, 4u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 114u8, 1u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8, 0u8,
                        14u8, 44u8, 0u8, 14u8, 52u8, 0u8, 14u8, 1u8, 0u8, 14u8, 50u8, 0u8, 0u8,
                        32u8, 128u8, 128u8, 2u8, 0u8, 0u8, 32u8, 0u8, 14u8, 25u8, 0u8, 14u8, 50u8,
                        0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 144u8, 2u8, 0u8,
                        0u8, 32u8, 0u8, 14u8, 39u8, 0u8, 14u8, 52u8, 0u8, 14u8, 25u8, 0u8, 14u8,
                        50u8, 0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 0u8, 32u8, 128u8, 160u8, 1u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static TH_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 80u8, 4u8, 0u8, 14u8, 23u8, 0u8, 14u8, 53u8, 0u8, 14u8, 72u8, 0u8, 0u8,
                    32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8,
                    128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 0u8,
                    1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 47u8, 128u8, 32u8, 1u8, 0u8, 0u8, 47u8, 128u8,
                    16u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
static TH_U_CA_JAPANESE: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
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
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
static TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 80u8,
                        4u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 5u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 128u8,
                        32u8, 2u8, 0u8, 0u8, 46u8, 128u8, 16u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static TR_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 80u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 64u8, 1u8, 0u8, 0u8, 46u8, 128u8, 32u8, 2u8, 0u8, 0u8, 46u8, 128u8,
                    16u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
static TR_U_CA_JAPANESE: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
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
    time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8, 128u8,
                    144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
static UND_U_CA_BUDDHIST_UND_U_CA_COPTIC_UND_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::DatePatternsV1 {
        date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8,
                        32u8, 128u8, 80u8, 4u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        32u8, 4u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            medium: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 1u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8,
                        32u8, 3u8, 0u8, 0u8, 32u8, 128u8, 64u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
            short: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 0u8, 5u8, 0u8, 0u8, 32u8, 128u8, 16u8, 1u8, 0u8, 0u8, 45u8, 128u8,
                        32u8, 2u8, 0u8, 0u8, 45u8, 128u8, 64u8, 2u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
            },
        },
        time_h11_h12: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
            full: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
                    ])
                },
                time_granularity: ::icu_datetime::pattern::TimeGranularity::Seconds,
            },
            long: ::icu_datetime::pattern::runtime::Pattern {
                items: unsafe {
                    ::zerovec::ZeroVec::from_bytes_unchecked(&[
                        128u8, 113u8, 1u8, 0u8, 0u8, 58u8, 128u8, 128u8, 2u8, 0u8, 0u8, 58u8,
                        128u8, 144u8, 2u8, 0u8, 0u8, 32u8, 128u8, 96u8, 1u8,
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
        length_combinations:
            ::icu_datetime::provider::calendar::patterns::GenericLengthPatternsV1 {
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
static UND_U_CA_GREGORY: DataStruct = &::icu_datetime::provider::calendar::DatePatternsV1 {
    date: ::icu_datetime::provider::calendar::patterns::LengthPatternsV1 {
        full: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    64u8, 1u8, 0u8, 0u8, 44u8, 0u8, 0u8, 32u8, 128u8, 80u8, 4u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        long: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 4u8, 0u8, 0u8, 32u8, 128u8,
                    64u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        medium: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 0u8, 32u8, 128u8, 32u8, 3u8, 0u8, 0u8, 32u8, 128u8,
                    64u8, 1u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
        short: ::icu_datetime::pattern::runtime::Pattern {
            items: unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 16u8, 1u8, 0u8, 0u8, 45u8, 128u8, 32u8, 2u8, 0u8, 0u8, 45u8, 128u8,
                    64u8, 2u8,
                ])
            },
            time_granularity: ::icu_datetime::pattern::TimeGranularity::None,
        },
    },
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
