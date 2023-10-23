// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_datetime_buddhist_datelengths_v1!($provider);
        impl_datetime_buddhist_datesymbols_v1!($provider);
        impl_datetime_chinese_datelengths_v1!($provider);
        impl_datetime_chinese_datesymbols_v1!($provider);
        impl_datetime_coptic_datelengths_v1!($provider);
        impl_datetime_coptic_datesymbols_v1!($provider);
        impl_datetime_dangi_datelengths_v1!($provider);
        impl_datetime_dangi_datesymbols_v1!($provider);
        impl_datetime_ethiopic_datelengths_v1!($provider);
        impl_datetime_ethiopic_datesymbols_v1!($provider);
        impl_datetime_gregory_datelengths_v1!($provider);
        impl_datetime_gregory_datesymbols_v1!($provider);
        impl_datetime_hebrew_datelengths_v1!($provider);
        impl_datetime_hebrew_datesymbols_v1!($provider);
        impl_datetime_indian_datelengths_v1!($provider);
        impl_datetime_indian_datesymbols_v1!($provider);
        impl_datetime_islamic_datelengths_v1!($provider);
        impl_datetime_islamic_datesymbols_v1!($provider);
        impl_datetime_japanese_datelengths_v1!($provider);
        impl_datetime_japanese_datesymbols_v1!($provider);
        impl_datetime_japanext_datelengths_v1!($provider);
        impl_datetime_japanext_datesymbols_v1!($provider);
        impl_datetime_patterns_buddhist_date_v1!($provider);
        impl_datetime_patterns_chinese_date_v1!($provider);
        impl_datetime_patterns_coptic_date_v1!($provider);
        impl_datetime_patterns_dangi_date_v1!($provider);
        impl_datetime_patterns_datetime_v1!($provider);
        impl_datetime_patterns_ethiopic_date_v1!($provider);
        impl_datetime_patterns_gregory_date_v1!($provider);
        impl_datetime_patterns_hebrew_date_v1!($provider);
        impl_datetime_patterns_indian_date_v1!($provider);
        impl_datetime_patterns_islamic_date_v1!($provider);
        impl_datetime_patterns_japanese_date_v1!($provider);
        impl_datetime_patterns_japanext_date_v1!($provider);
        impl_datetime_patterns_persian_date_v1!($provider);
        impl_datetime_patterns_roc_date_v1!($provider);
        impl_datetime_patterns_time_v1!($provider);
        impl_datetime_persian_datelengths_v1!($provider);
        impl_datetime_persian_datesymbols_v1!($provider);
        impl_datetime_roc_datelengths_v1!($provider);
        impl_datetime_roc_datesymbols_v1!($provider);
        impl_datetime_skeletons_v1!($provider);
        impl_datetime_symbols_buddhist_months_v1!($provider);
        impl_datetime_symbols_buddhist_years_v1!($provider);
        impl_datetime_symbols_chinese_months_v1!($provider);
        impl_datetime_symbols_chinese_years_v1!($provider);
        impl_datetime_symbols_coptic_months_v1!($provider);
        impl_datetime_symbols_coptic_years_v1!($provider);
        impl_datetime_symbols_dangi_months_v1!($provider);
        impl_datetime_symbols_dangi_years_v1!($provider);
        impl_datetime_symbols_dayperiods_v1!($provider);
        impl_datetime_symbols_ethiopic_months_v1!($provider);
        impl_datetime_symbols_ethiopic_years_v1!($provider);
        impl_datetime_symbols_gregory_months_v1!($provider);
        impl_datetime_symbols_gregory_years_v1!($provider);
        impl_datetime_symbols_hebrew_months_v1!($provider);
        impl_datetime_symbols_hebrew_years_v1!($provider);
        impl_datetime_symbols_indian_months_v1!($provider);
        impl_datetime_symbols_indian_years_v1!($provider);
        impl_datetime_symbols_islamic_months_v1!($provider);
        impl_datetime_symbols_islamic_years_v1!($provider);
        impl_datetime_symbols_japanese_months_v1!($provider);
        impl_datetime_symbols_japanese_years_v1!($provider);
        impl_datetime_symbols_japanext_months_v1!($provider);
        impl_datetime_symbols_japanext_years_v1!($provider);
        impl_datetime_symbols_persian_months_v1!($provider);
        impl_datetime_symbols_persian_years_v1!($provider);
        impl_datetime_symbols_roc_months_v1!($provider);
        impl_datetime_symbols_roc_years_v1!($provider);
        impl_datetime_symbols_weekdays_v1!($provider);
        impl_datetime_timelengths_v1!($provider);
        impl_datetime_timesymbols_v1!($provider);
        impl_time_zone_exemplar_cities_v1!($provider);
        impl_time_zone_formats_v1!($provider);
        impl_time_zone_generic_long_v1!($provider);
        impl_time_zone_generic_short_v1!($provider);
        impl_time_zone_specific_long_v1!($provider);
        impl_time_zone_specific_short_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::datetime::provider::calendar::BuddhistDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::BuddhistDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::BuddhistDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::BuddhistDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::ChineseDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::ChineseDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::CopticDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::CopticDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::CopticDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DangiDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DangiDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DangiDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DangiDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::EthiopianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::EthiopianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::EthiopianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::GregorianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::GregorianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::GregorianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::GregorianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::HebrewDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::HebrewDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::HebrewDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::HebrewDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IndianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IndianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IndianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IndianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IslamicDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IslamicDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IslamicDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IslamicDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DateTimePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DateTimePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocDatePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::TimePatternV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::TimePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::PersianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::PersianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::PersianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::PersianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::RocDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::RocDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::RocDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::RocDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DateSkeletonPatternsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DateSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DayPeriodSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DayPeriodSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocMonthSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocMonthSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocYearSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::WeekdaySymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::WeekdaySymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::TimeLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::TimeSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::TimeSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::ExemplarCitiesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::ExemplarCitiesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::TimeZoneFormatsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.67"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
