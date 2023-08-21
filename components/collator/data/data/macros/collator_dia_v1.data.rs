// @generated
/// Implement `DataProvider<CollationDiacriticsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_collator_dia_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_COLLATOR_DIA_V1: &'static <icu::collator::provider::CollationDiacriticsV1Marker as icu_provider::DataMarker>::Yokeable = &icu::collator::provider::CollationDiacriticsV1 { secondaries: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x8A\0\x88\0\x8E\0\x9A\0\xA4\0\xB4\0\x8C\0\x9C\0\x96\0\xB6\0\x92\0\x98\0\x90\0\xA6\0\xA6\0\xB8\0\xBA\0\xBC\0\xA6\0\x84\0\x86\0\xA6\0\xA8\0\xA8\0\xA8\0\xA8\0\xA6\0\xBE\0\xA8\0\xA8\0\xA8\0\xA8\0\xA8\0\xC0\0\xC2\0\xC4\0\xC6\0\xC8\0\xCA\0\xA0\0\xA2\0\xA8\0\xA8\0\xA8\0\xA8\0\xCC\0\xCE\0\xA8\0\xD0\0\xD2\0\x82\0\xA8\0\xD4\0\xB2\0\xAA\0\xAA\0\x9E\0\xD6\0\xA8\0\xA8\0\xA8\0\xA6\0\xA6\0\xA6\0\0\0\0\0\x94\0\0\0\0\0\xD8\0\xA6\0\xA8\0\xA8\0\xA8\0\xA6\0\xA6\0\xA6\0\xA8\0\xA8") } };
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::collator::provider::CollationDiacriticsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::collator::provider::CollationDiacriticsV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_COLLATOR_DIA_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::collator::provider::CollationDiacriticsV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
