// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_ideo_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x060\0\0\x080\0\0!0\0\0*0\0\080\0\0;0\0\0\x004\0\0\xC0M\0\0\0N\0\0\0\xA0\0\0\0\xF9\0\0n\xFA\0\0p\xFA\0\0\xDA\xFA\0\0\xE4o\x01\0\xE5o\x01\0\0p\x01\0\xF8\x87\x01\0\0\x88\x01\0\xD6\x8C\x01\0\0\x8D\x01\0\t\x8D\x01\0p\xB1\x01\0\xFC\xB2\x01\0\0\0\x02\0\xE0\xA6\x02\0\0\xA7\x02\0:\xB7\x02\0@\xB7\x02\0\x1E\xB8\x02\0 \xB8\x02\0\xA2\xCE\x02\0\xB0\xCE\x02\0\xE1\xEB\x02\0\0\xF8\x02\0\x1E\xFA\x02\0\0\0\x03\0K\x13\x03\0P\x13\x03\0\xB0#\x03\0") }, 105854usize)
        })
    };
}
/// Implement [`DataProvider<IdeographicV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ideo_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::IdeographicV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::IdeographicV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::IdeographicV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_ideo_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::IdeographicV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
