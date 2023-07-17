// @generated
/// Implement [`DataProvider<WeekDataV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_week_data_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_calendar::provider::WeekDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_calendar::provider::WeekDataV1Marker>, icu_provider::DataError> {
                static UND_MV: <icu_calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_calendar::provider::WeekDataV1 { first_weekday: icu_calendar::types::IsoWeekday::Friday, min_week_days: 1u8 };
                static UND: <icu_calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_calendar::provider::WeekDataV1 { first_weekday: icu_calendar::types::IsoWeekday::Monday, min_week_days: 1u8 };
                static UND_AD: <icu_calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_calendar::provider::WeekDataV1 { first_weekday: icu_calendar::types::IsoWeekday::Monday, min_week_days: 4u8 };
                static UND_AE: <icu_calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_calendar::provider::WeekDataV1 { first_weekday: icu_calendar::types::IsoWeekday::Saturday, min_week_days: 1u8 };
                static UND_AG: <icu_calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_calendar::provider::WeekDataV1 { first_weekday: icu_calendar::types::IsoWeekday::Sunday, min_week_days: 1u8 };
                static UND_PT: <icu_calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::Yokeable = icu_calendar::provider::WeekDataV1 { first_weekday: icu_calendar::types::IsoWeekday::Sunday, min_week_days: 4u8 };
                static VALUES: [&<icu_calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::Yokeable; 155usize] = [&UND, &UND_AD, &UND_AE, &UND_AE, &UND_AG, &UND, &UND, &UND, &UND_AD, &UND, &UND_AG, &UND_AD, &UND, &UND_AD, &UND, &UND, &UND_AG, &UND_AD, &UND_AD, &UND_AE, &UND, &UND, &UND_AG, &UND_AG, &UND_AG, &UND_AG, &UND, &UND_AG, &UND_AG, &UND_AD, &UND, &UND, &UND, &UND_AG, &UND, &UND, &UND_AD, &UND_AD, &UND_AE, &UND_AD, &UND_AG, &UND_AG, &UND_AE, &UND, &UND_AD, &UND_AE, &UND_AD, &UND_AG, &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND, &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND_AG, &UND_AG, &UND_AG, &UND_AG, &UND, &UND_AD, &UND_AG, &UND_AD, &UND_AG, &UND_AD, &UND_AG, &UND_AE, &UND_AE, &UND_AD, &UND_AD, &UND_AD, &UND_AG, &UND_AE, &UND_AG, &UND_AG, &UND, &UND_AG, &UND_AG, &UND_AE, &UND, &UND_AG, &UND, &UND_AD, &UND, &UND_AD, &UND_AD, &UND, &UND_AE, &UND_AD, &UND, &UND, &UND_AG, &UND, &UND_AG, &UND, &UND_AG, &UND_AD, &UND_AG, &UND_MV, &UND_AG, &UND, &UND_AG, &UND_AG, &UND_AD, &UND_AD, &UND_AG, &UND, &UND_AE, &UND_AG, &UND_AG, &UND_AG, &UND_AG, &UND_AD, &UND_AG, &UND_PT, &UND_AG, &UND_AE, &UND_AD, &UND, &UND, &UND_AD, &UND_AG, &UND_AE, &UND_AD, &UND_AG, &UND, &UND_AD, &UND_AD, &UND_AD, &UND_AG, &UND_AE, &UND_AG, &UND, &UND, &UND, &UND_AG, &UND_AG, &UND, &UND_AG, &UND_AG, &UND, &UND, &UND_AD, &UND_AG, &UND_AG, &UND, &UND_AG, &UND, &UND_AG, &UND_AG, &UND_AG];
                static KEYS: [&str; 155usize] = ["und", "und-AD", "und-AE", "und-AF", "und-AG", "und-AI", "und-AL", "und-AM", "und-AN", "und-AR", "und-AS", "und-AT", "und-AU", "und-AX", "und-AZ", "und-BA", "und-BD", "und-BE", "und-BG", "und-BH", "und-BM", "und-BN", "und-BR", "und-BS", "und-BT", "und-BW", "und-BY", "und-BZ", "und-CA", "und-CH", "und-CL", "und-CM", "und-CN", "und-CO", "und-CR", "und-CY", "und-CZ", "und-DE", "und-DJ", "und-DK", "und-DM", "und-DO", "und-DZ", "und-EC", "und-EE", "und-EG", "und-ES", "und-ET", "und-FI", "und-FJ", "und-FO", "und-FR", "und-GB", "und-GE", "und-GF", "und-GG", "und-GI", "und-GP", "und-GR", "und-GT", "und-GU", "und-HK", "und-HN", "und-HR", "und-HU", "und-ID", "und-IE", "und-IL", "und-IM", "und-IN", "und-IQ", "und-IR", "und-IS", "und-IT", "und-JE", "und-JM", "und-JO", "und-JP", "und-KE", "und-KG", "und-KH", "und-KR", "und-KW", "und-KZ", "und-LA", "und-LB", "und-LI", "und-LK", "und-LT", "und-LU", "und-LV", "und-LY", "und-MC", "und-MD", "und-ME", "und-MH", "und-MK", "und-MM", "und-MN", "und-MO", "und-MQ", "und-MT", "und-MV", "und-MX", "und-MY", "und-MZ", "und-NI", "und-NL", "und-NO", "und-NP", "und-NZ", "und-OM", "und-PA", "und-PE", "und-PH", "und-PK", "und-PL", "und-PR", "und-PT", "und-PY", "und-QA", "und-RE", "und-RO", "und-RS", "und-RU", "und-SA", "und-SD", "und-SE", "und-SG", "und-SI", "und-SJ", "und-SK", "und-SM", "und-SV", "und-SY", "und-TH", "und-TJ", "und-TM", "und-TR", "und-TT", "und-TW", "und-UA", "und-UM", "und-US", "und-UY", "und-UZ", "und-VA", "und-VE", "und-VI", "und-VN", "und-WS", "und-XK", "und-YE", "und-ZA", "und-ZW"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_calendar::provider::WeekDataV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
