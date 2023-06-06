// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_nchar_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xD0\xFD\0\0\xF0\xFD\0\0\xFE\xFF\0\0\0\0\x01\0\xFE\xFF\x01\0\0\0\x02\0\xFE\xFF\x02\0\0\0\x03\0\xFE\xFF\x03\0\0\0\x04\0\xFE\xFF\x04\0\0\0\x05\0\xFE\xFF\x05\0\0\0\x06\0\xFE\xFF\x06\0\0\0\x07\0\xFE\xFF\x07\0\0\0\x08\0\xFE\xFF\x08\0\0\0\t\0\xFE\xFF\t\0\0\0\n\0\xFE\xFF\n\0\0\0\x0B\0\xFE\xFF\x0B\0\0\0\x0C\0\xFE\xFF\x0C\0\0\0\r\0\xFE\xFF\r\0\0\0\x0E\0\xFE\xFF\x0E\0\0\0\x0F\0\xFE\xFF\x0F\0\0\0\x10\0\xFE\xFF\x10\0\0\0\x11\0") }, 66usize)
        })
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_props_nchar_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_properties::provider::NoncharacterCodePointV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_nchar_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<NoncharacterCodePointV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_nchar_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::NoncharacterCodePointV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::NoncharacterCodePointV1Marker>, icu_provider::DataError> {
                let lookup = lookup_props_nchar_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::NoncharacterCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
