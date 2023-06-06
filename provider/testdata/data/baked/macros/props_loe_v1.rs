// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_loe_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"@\x0E\0\0E\x0E\0\0\xC0\x0E\0\0\xC5\x0E\0\0\xB5\x19\0\0\xB8\x19\0\0\xBA\x19\0\0\xBB\x19\0\0\xB5\xAA\0\0\xB7\xAA\0\0\xB9\xAA\0\0\xBA\xAA\0\0\xBB\xAA\0\0\xBD\xAA\0\0") }, 19usize)
        })
    };
}
/// Implement [`DataProvider<LogicalOrderExceptionV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_loe_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::LogicalOrderExceptionV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::LogicalOrderExceptionV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::LogicalOrderExceptionV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_loe_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::LogicalOrderExceptionV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
