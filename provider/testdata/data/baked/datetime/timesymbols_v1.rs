// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_datetime::provider::calendar::TimeSymbolsV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_datetime::provider::calendar::TimeSymbolsV1Marker>, DataError>
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
            (
                "ar-EG-u-ca-gregory",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
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
            (
                "ar-u-ca-gregory",
                AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC,
            ),
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
            (
                "bn-u-ca-gregory",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
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
            (
                "ccp-u-ca-gregory",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
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
            (
                "en-001-u-ca-gregory",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            (
                "en-001-u-ca-indian",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            (
                "en-001-u-ca-japanese",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            (
                "en-ZA-u-ca-buddhist",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            ("en-ZA-u-ca-coptic", EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC),
            (
                "en-ZA-u-ca-ethiopic",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            (
                "en-ZA-u-ca-gregory",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
            ("en-ZA-u-ca-indian", EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC),
            (
                "en-ZA-u-ca-japanese",
                EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC,
            ),
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
            (
                "en-u-ca-gregory",
                EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC,
            ),
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
            ("es-AR-u-ca-gregory", ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC),
            ("es-AR-u-ca-indian", ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC),
            ("es-AR-u-ca-japanese", ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC),
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
            (
                "es-u-ca-gregory",
                ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC,
            ),
            (
                "es-u-ca-indian",
                ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC,
            ),
            (
                "es-u-ca-japanese",
                ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC,
            ),
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
            (
                "fil-u-ca-gregory",
                FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC,
            ),
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
            (
                "fr-u-ca-gregory",
                FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC,
            ),
            (
                "fr-u-ca-indian",
                FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC,
            ),
            (
                "fr-u-ca-japanese",
                FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC,
            ),
            (
                "ja-u-ca-buddhist",
                JA_U_CA_BUDDHIST_JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC,
            ),
            (
                "ja-u-ca-coptic",
                JA_U_CA_BUDDHIST_JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC,
            ),
            (
                "ja-u-ca-ethiopic",
                JA_U_CA_BUDDHIST_JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC,
            ),
            (
                "ja-u-ca-gregory",
                JA_U_CA_BUDDHIST_JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC,
            ),
            (
                "ja-u-ca-indian",
                JA_U_CA_BUDDHIST_JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC,
            ),
            (
                "ja-u-ca-japanese",
                JA_U_CA_BUDDHIST_JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC,
            ),
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
            (
                "ru-u-ca-gregory",
                RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC,
            ),
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
            (
                "sr-Cyrl-u-ca-gregory",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Cyrl-u-ca-indian",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Cyrl-u-ca-japanese",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-Latn-u-ca-buddhist",
                SR_LATN_U_CA_BUDDHIST_SR_LATN_U_CA_COPTIC,
            ),
            (
                "sr-Latn-u-ca-coptic",
                SR_LATN_U_CA_BUDDHIST_SR_LATN_U_CA_COPTIC,
            ),
            (
                "sr-Latn-u-ca-ethiopic",
                SR_LATN_U_CA_BUDDHIST_SR_LATN_U_CA_COPTIC,
            ),
            (
                "sr-Latn-u-ca-gregory",
                SR_LATN_U_CA_BUDDHIST_SR_LATN_U_CA_COPTIC,
            ),
            (
                "sr-Latn-u-ca-indian",
                SR_LATN_U_CA_BUDDHIST_SR_LATN_U_CA_COPTIC,
            ),
            (
                "sr-Latn-u-ca-japanese",
                SR_LATN_U_CA_BUDDHIST_SR_LATN_U_CA_COPTIC,
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
            (
                "sr-u-ca-gregory",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-u-ca-indian",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "sr-u-ca-japanese",
                SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC,
            ),
            (
                "th-u-ca-buddhist",
                TH_U_CA_BUDDHIST_TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC,
            ),
            (
                "th-u-ca-coptic",
                TH_U_CA_BUDDHIST_TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC,
            ),
            (
                "th-u-ca-ethiopic",
                TH_U_CA_BUDDHIST_TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC,
            ),
            (
                "th-u-ca-gregory",
                TH_U_CA_BUDDHIST_TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC,
            ),
            (
                "th-u-ca-indian",
                TH_U_CA_BUDDHIST_TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC,
            ),
            (
                "th-u-ca-japanese",
                TH_U_CA_BUDDHIST_TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC,
            ),
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
            (
                "tr-u-ca-gregory",
                TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC,
            ),
            (
                "tr-u-ca-indian",
                TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC,
            ),
            (
                "tr-u-ca-japanese",
                TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-buddhist",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-coptic",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-ethiopic",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-gregory",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-indian",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
            (
                "und-u-ca-japanese",
                BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC,
            ),
        ];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <::icu_datetime::provider::calendar::TimeSymbolsV1Marker>::KEY,
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
    &'static <::icu_datetime::provider::calendar::TimeSymbolsV1Marker as DataMarker>::Yokeable;
static AR_U_CA_BUDDHIST_AR_U_CA_COPTIC_AR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("ص"),
                    pm: ::alloc::borrow::Cow::Borrowed("م"),
                    noon: None,
                    midnight: None,
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("ص"),
                    pm: ::alloc::borrow::Cow::Borrowed("م"),
                    noon: None,
                    midnight: None,
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("ص"),
                    pm: ::alloc::borrow::Cow::Borrowed("م"),
                    noon: None,
                    midnight: None,
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: None,
                    short: None,
                    wide: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("صباح\u{64b}ا"),
                        pm: ::alloc::borrow::Cow::Borrowed("مساء\u{64b}"),
                        noon: None,
                        midnight: None,
                    }),
                },
            ),
        },
    };
static BN_U_CA_BUDDHIST_BN_U_CA_COPTIC_BN_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: None,
                    midnight: None,
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: None,
                    midnight: None,
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: None,
                    midnight: None,
                },
            },
            stand_alone: None,
        },
    };
static EN_001_U_CA_BUDDHIST_EN_001_U_CA_COPTIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("am"),
                    pm: ::alloc::borrow::Cow::Borrowed("pm"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("noon")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("midnight")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a"),
                    pm: ::alloc::borrow::Cow::Borrowed("p"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("n")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("mi")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("am"),
                    pm: ::alloc::borrow::Cow::Borrowed("pm"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("noon")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("midnight")),
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("am"),
                        pm: ::alloc::borrow::Cow::Borrowed("pm"),
                        noon: Some(::alloc::borrow::Cow::Borrowed("noon")),
                        midnight: Some(::alloc::borrow::Cow::Borrowed("midnight")),
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
static EN_U_CA_BUDDHIST_EN_U_CA_COPTIC_EN_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("noon")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("midnight")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a"),
                    pm: ::alloc::borrow::Cow::Borrowed("p"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("n")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("mi")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("noon")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("midnight")),
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("AM"),
                        pm: ::alloc::borrow::Cow::Borrowed("PM"),
                        noon: Some(::alloc::borrow::Cow::Borrowed("noon")),
                        midnight: Some(::alloc::borrow::Cow::Borrowed("midnight")),
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
static ES_AR_U_CA_BUDDHIST_ES_AR_U_CA_COPTIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                    pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                    noon: Some(::alloc::borrow::Cow::Borrowed("mediodía")),
                    midnight: None,
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                    pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                    noon: Some(::alloc::borrow::Cow::Borrowed("del mediodía")),
                    midnight: None,
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                    pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                    noon: Some(::alloc::borrow::Cow::Borrowed("mediodía")),
                    midnight: None,
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                        pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                        noon: Some(::alloc::borrow::Cow::Borrowed("m.")),
                        midnight: None,
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
static ES_U_CA_BUDDHIST_ES_U_CA_COPTIC_ES_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                    pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                    noon: Some(::alloc::borrow::Cow::Borrowed("del mediodía")),
                    midnight: None,
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                    pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                    noon: Some(::alloc::borrow::Cow::Borrowed("del mediodía")),
                    midnight: None,
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                    pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                    noon: Some(::alloc::borrow::Cow::Borrowed("del mediodía")),
                    midnight: None,
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                        pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                        noon: Some(::alloc::borrow::Cow::Borrowed("mediodía")),
                        midnight: None,
                    }),
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                        pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                        noon: Some(::alloc::borrow::Cow::Borrowed("mediodía")),
                        midnight: None,
                    }),
                    short: None,
                    wide: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                        pm: ::alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                        noon: Some(::alloc::borrow::Cow::Borrowed("mediodía")),
                        midnight: None,
                    }),
                },
            ),
        },
    };
static FIL_U_CA_BUDDHIST_FIL_U_CA_COPTIC_FIL_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("hatinggabi")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("am"),
                    pm: ::alloc::borrow::Cow::Borrowed("pm"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("hatinggabi")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("hatinggabi")),
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("AM"),
                        pm: ::alloc::borrow::Cow::Borrowed("PM"),
                        noon: Some(::alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                        midnight: Some(::alloc::borrow::Cow::Borrowed("hatinggabi")),
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
static FR_U_CA_BUDDHIST_FR_U_CA_COPTIC_FR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("midi")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("minuit")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("midi")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("minuit")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("midi")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("minuit")),
                },
            },
            stand_alone: None,
        },
    };
static JA_U_CA_BUDDHIST_JA_U_CA_COPTIC_JA_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("午前"),
                    pm: ::alloc::borrow::Cow::Borrowed("午後"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("正午")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("真夜中")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("午前"),
                    pm: ::alloc::borrow::Cow::Borrowed("午後"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("正午")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("真夜中")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("午前"),
                    pm: ::alloc::borrow::Cow::Borrowed("午後"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("正午")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("真夜中")),
                },
            },
            stand_alone: None,
        },
    };
static RU_U_CA_BUDDHIST_RU_U_CA_COPTIC_RU_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("полд.")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("полн.")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("полд.")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("полн.")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("полдень")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("полночь")),
                },
            },
            stand_alone: None,
        },
    };
static SR_LATN_U_CA_BUDDHIST_SR_LATN_U_CA_COPTIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("podne")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("ponoć")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("podne")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("ponoć")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("podne")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("ponoć")),
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("pre podne"),
                        pm: ::alloc::borrow::Cow::Borrowed("po podne"),
                        noon: Some(::alloc::borrow::Cow::Borrowed("podne")),
                        midnight: Some(::alloc::borrow::Cow::Borrowed("ponoć")),
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
static SR_U_CA_BUDDHIST_SR_U_CA_COPTIC_SR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("подне")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("поноћ")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("подне")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("поноћ")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("AM"),
                    pm: ::alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("подне")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("поноћ")),
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("пре подне"),
                        pm: ::alloc::borrow::Cow::Borrowed("по подне"),
                        noon: Some(::alloc::borrow::Cow::Borrowed("подне")),
                        midnight: Some(::alloc::borrow::Cow::Borrowed("поноћ")),
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
static TH_U_CA_BUDDHIST_TH_U_CA_COPTIC_TH_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("ก\u{e48}อนเท\u{e35}\u{e48}ยง"),
                    pm: ::alloc::borrow::Cow::Borrowed("หล\u{e31}งเท\u{e35}\u{e48}ยง"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed(
                        "เท\u{e35}\u{e48}ยงค\u{e37}น",
                    )),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("a"),
                    pm: ::alloc::borrow::Cow::Borrowed("p"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed(
                        "เท\u{e35}\u{e48}ยงค\u{e37}น",
                    )),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("ก\u{e48}อนเท\u{e35}\u{e48}ยง"),
                    pm: ::alloc::borrow::Cow::Borrowed("หล\u{e31}งเท\u{e35}\u{e48}ยง"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed(
                        "เท\u{e35}\u{e48}ยงค\u{e37}น",
                    )),
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("ก\u{e48}อนเท\u{e35}\u{e48}ยง"),
                        pm: ::alloc::borrow::Cow::Borrowed("หล\u{e31}งเท\u{e35}\u{e48}ยง"),
                        noon: Some(::alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                        midnight: Some(::alloc::borrow::Cow::Borrowed(
                            "เท\u{e35}\u{e48}ยงค\u{e37}น",
                        )),
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
static TR_U_CA_BUDDHIST_TR_U_CA_COPTIC_TR_U_CA_ETHIOPIC: DataStruct =
    &::icu_datetime::provider::calendar::TimeSymbolsV1 {
        day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
            format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
                abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("ÖÖ"),
                    pm: ::alloc::borrow::Cow::Borrowed("ÖS"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("öğle")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("gece yarısı")),
                },
                narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("öö"),
                    pm: ::alloc::borrow::Cow::Borrowed("ös"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("ö")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("gece")),
                },
                short: None,
                wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: ::alloc::borrow::Cow::Borrowed("ÖÖ"),
                    pm: ::alloc::borrow::Cow::Borrowed("ÖS"),
                    noon: Some(::alloc::borrow::Cow::Borrowed("öğle")),
                    midnight: Some(::alloc::borrow::Cow::Borrowed("gece yarısı")),
                },
            },
            stand_alone: Some(
                ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                    abbreviated: None,
                    narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                        am: ::alloc::borrow::Cow::Borrowed("ÖÖ"),
                        pm: ::alloc::borrow::Cow::Borrowed("ÖS"),
                        noon: Some(::alloc::borrow::Cow::Borrowed("öğle")),
                        midnight: Some(::alloc::borrow::Cow::Borrowed("gece yarısı")),
                    }),
                    short: None,
                    wide: None,
                },
            ),
        },
    };
