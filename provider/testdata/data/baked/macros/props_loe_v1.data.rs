// @generated
/// Implement [`DataProvider<LogicalOrderExceptionV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_loe_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_LOE_V1: &'static <icu_properties::provider::LogicalOrderExceptionV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"@\x0E\0\0E\x0E\0\0\xC0\x0E\0\0\xC5\x0E\0\0\xB5\x19\0\0\xB8\x19\0\0\xBA\x19\0\0\xBB\x19\0\0\xB5\xAA\0\0\xB7\xAA\0\0\xB9\xAA\0\0\xBA\xAA\0\0\xBB\xAA\0\0\xBD\xAA\0\0") }, 19usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::LogicalOrderExceptionV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::LogicalOrderExceptionV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_LOE_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::LogicalOrderExceptionV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
