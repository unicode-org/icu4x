// @generated
/// Implement [`DataProvider<JapaneseErasV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_calendar_japanese_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_CALENDAR_JAPANESE_V1: &'static <icu_calendar::provider::JapaneseErasV1Marker as icu_provider::DataMarker>::Yokeable = &icu_calendar::provider::JapaneseErasV1 { dates_to_eras: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"L\x07\0\0\t\x08meiji\0\0\0\0\0\0\0\0\0\0\0x\x07\0\0\x07\x1Etaisho\0\0\0\0\0\0\0\0\0\0\x86\x07\0\0\x0C\x19showa\0\0\0\0\0\0\0\0\0\0\0\xC5\x07\0\0\x01\x08heisei\0\0\0\0\0\0\0\0\0\0\xE3\x07\0\0\x05\x01reiwa\0\0\0\0\0\0\0\0\0\0\0") } };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_calendar::provider::JapaneseErasV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_calendar::provider::JapaneseErasV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_CALENDAR_JAPANESE_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_calendar::provider::JapaneseErasV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
