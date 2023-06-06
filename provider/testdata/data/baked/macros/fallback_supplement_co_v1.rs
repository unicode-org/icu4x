// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_fallback_supplement_co_v1 {
    () => {
        icu_provider_adapters::fallback::provider::LocaleFallbackSupplementV1 {
            parents: unsafe {
                #[allow(unused_unsafe)]
                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0yue") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"zh\0\x01Hant\0\0\0\0") })
            },
            unicode_extension_defaults: unsafe {
                #[allow(unused_unsafe)]
                zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"co") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0zhzh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0pinyinstroke") })
            },
        }
    };
}
/// Implement [`DataProvider<CollationFallbackSupplementV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fallback_supplement_co_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker as icu_provider::DataMarker>::Yokeable = singleton_fallback_supplement_co_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
