// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_hyphen_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"-\0\0\0.\0\0\0\xAD\0\0\0\xAE\0\0\0\x8A\x05\0\0\x8B\x05\0\0\x06\x18\0\0\x07\x18\0\0\x10 \0\0\x12 \0\0\x17.\0\0\x18.\0\0\xFB0\0\0\xFC0\0\0c\xFE\0\0d\xFE\0\0\r\xFF\0\0\x0E\xFF\0\0e\xFF\0\0f\xFF\0\0") }, 11usize)
        })
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_props_hyphen_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_properties::provider::HyphenV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_hyphen_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<HyphenV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_hyphen_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::HyphenV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::HyphenV1Marker>, icu_provider::DataError> {
                let lookup = lookup_props_hyphen_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::HyphenV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
