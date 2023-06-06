// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_dep_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"I\x01\0\0J\x01\0\0s\x06\0\0t\x06\0\0w\x0F\0\0x\x0F\0\0y\x0F\0\0z\x0F\0\0\xA3\x17\0\0\xA5\x17\0\0j \0\0p \0\0)#\0\0+#\0\0\x01\0\x0E\0\x02\0\x0E\0") }, 15usize)
        })
    };
}
/// Implement [`DataProvider<DeprecatedV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_dep_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::DeprecatedV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::DeprecatedV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::DeprecatedV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_dep_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::DeprecatedV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
