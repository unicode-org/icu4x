// @generated
/// Implement `DataProvider<PersianYearSymbolsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_symbols_persian_years_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::datetime::provider::neo::PersianYearSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::datetime::provider::neo::PersianYearSymbolsV1Marker>, icu_provider::DataError> {
                static FR_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0A. P.") })
                });
                static RO_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0A.P.") })
                });
                static UND_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0AP") })
                });
                static SV_X_5: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Anno Persarum") })
                });
                static FI_X_5: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Anno Persico") })
                });
                static MK_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\x90\xD0\x9F") })
                });
                static RU_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xBF\xD0\xB5\xD1\x80\xD1\x81. \xD0\xB3\xD0\xBE\xD0\xB4") })
                });
                static RU_X_5: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xBF\xD0\xB5\xD1\x80\xD1\x81\xD0\xB8\xD0\xB4\xD1\x81\xD0\xBA\xD0\xB8\xD0\xB9 \xD0\xB3\xD0\xBE\xD0\xB4") })
                });
                static HE_X_5: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD7\x94\xD7\xA1\xD7\xA4\xD7\x99\xD7\xA8\xD7\x94 \xD7\x94\xD7\xA4\xD7\xA8\xD7\xA1\xD7\x99\xD7\xAA") })
                });
                static FA_X_5: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x87\xD8\xAC\xD8\xB1\xDB\x8C \xD8\xB4\xD9\x85\xD8\xB3\xDB\x8C") })
                });
                static AR_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x87\xE2\x80\x8D.\xD8\xB4") })
                });
                static FA_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x87\xE2\x80\x8D.\xD8\xB4.") })
                });
                static TH_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB8\x9B\xE0\xB8\xB5\xE0\xB9\x80\xE0\xB8\x9B\xE0\xB8\xAD\xE0\xB8\xA3\xE0\xB9\x8C\xE0\xB9\x80\xE0\xB8\x8B\xE0\xB8\xB5\xE0\xB8\xA2") })
                });
                static LO_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xBA\x9B\xE0\xBA\xB5\xE0\xBB\x80\xE0\xBA\x9B\xE0\xBA\xB5\xE0\xBB\x80\xE0\xBA\x8A\xE0\xBA\x8D") })
                });
                static YUE_HANS_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE6\xB3\xA2\xE6\x96\xAF\xE5\x8E\x86") })
                });
                static YUE_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE6\xB3\xA2\xE6\x96\xAF\xE6\x9B\x86") })
                });
                static FF_ADLM_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xF0\x9E\xA4\x80\xF0\x9E\xA4\x86") })
                });
                static SC_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0a.p.") })
                });
                static SC_X_5: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0annu persianu") })
                });
                static UZ_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0forsiy") })
                });
                static LV_X_3: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0pers. gads") })
                });
                static LV_X_5: <icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ah") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0persie\xC5\xA1u gads") })
                });
                static VALUES: [&<icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 54usize] = [&AR_X_3, &AR_X_3, &AR_X_3, &FA_X_3, &FA_X_3, &FA_X_5, &FF_ADLM_X_3, &FF_ADLM_X_3, &FF_ADLM_X_3, &FI_X_5, &FR_X_3, &FR_X_3, &FI_X_5, &HE_X_5, &LO_X_3, &LO_X_3, &LO_X_3, &LV_X_3, &LV_X_3, &LV_X_5, &MK_X_3, &MK_X_3, &MK_X_3, &RO_X_3, &RO_X_3, &FI_X_5, &RU_X_3, &RU_X_3, &RU_X_5, &SC_X_3, &SC_X_3, &SC_X_5, &SV_X_5, &TH_X_3, &TH_X_3, &TH_X_3, &UND_X_3, &UND_X_3, &UND_X_3, &UZ_X_3, &UZ_X_3, &UZ_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3];
                static KEYS: [&str; 54usize] = ["ar-x-3", "ar-x-4", "ar-x-5", "fa-x-3", "fa-x-4", "fa-x-5", "ff-Adlm-x-3", "ff-Adlm-x-4", "ff-Adlm-x-5", "fi-x-5", "fr-x-3", "fr-x-4", "fr-x-5", "he-x-5", "lo-x-3", "lo-x-4", "lo-x-5", "lv-x-3", "lv-x-4", "lv-x-5", "mk-x-3", "mk-x-4", "mk-x-5", "ro-x-3", "ro-x-4", "ro-x-5", "ru-x-3", "ru-x-4", "ru-x-5", "sc-x-3", "sc-x-4", "sc-x-5", "sv-x-5", "th-x-3", "th-x-4", "th-x-5", "und-x-3", "und-x-4", "und-x-5", "uz-x-3", "uz-x-4", "uz-x-5", "yue-Hans-x-3", "yue-Hans-x-4", "yue-Hans-x-5", "yue-x-3", "yue-x-4", "yue-x-5", "zh-Hant-x-3", "zh-Hant-x-4", "zh-Hant-x-5", "zh-x-3", "zh-x-4", "zh-x-5"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if fallback_iterator.get().is_und() {
                            return Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu::datetime::provider::neo::PersianYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY, req));
                        }
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}
