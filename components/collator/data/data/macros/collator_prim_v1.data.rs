// @generated
/// Implement [`DataProvider<CollationSpecialPrimariesV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_collator_prim_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_COLLATOR_PRIM_V1: &'static <icu_collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::DataMarker>::Yokeable = &icu_collator::provider::CollationSpecialPrimariesV1 { last_primaries: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\x05\0\x0C\x8A\r\0\x0E") }, numeric_primary: 15u8 };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_collator::provider::CollationSpecialPrimariesV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_collator::provider::CollationSpecialPrimariesV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_COLLATOR_PRIM_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
