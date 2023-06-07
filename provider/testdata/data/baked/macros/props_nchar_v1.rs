// @generated
/// Implement [`DataProvider<NoncharacterCodePointV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_nchar_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_NCHAR_V1: &'static <icu_properties::provider::NoncharacterCodePointV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xD0\xFD\0\0\xF0\xFD\0\0\xFE\xFF\0\0\0\0\x01\0\xFE\xFF\x01\0\0\0\x02\0\xFE\xFF\x02\0\0\0\x03\0\xFE\xFF\x03\0\0\0\x04\0\xFE\xFF\x04\0\0\0\x05\0\xFE\xFF\x05\0\0\0\x06\0\xFE\xFF\x06\0\0\0\x07\0\xFE\xFF\x07\0\0\0\x08\0\xFE\xFF\x08\0\0\0\t\0\xFE\xFF\t\0\0\0\n\0\xFE\xFF\n\0\0\0\x0B\0\xFE\xFF\x0B\0\0\0\x0C\0\xFE\xFF\x0C\0\0\0\r\0\xFE\xFF\r\0\0\0\x0E\0\xFE\xFF\x0E\0\0\0\x0F\0\xFE\xFF\x0F\0\0\0\x10\0\xFE\xFF\x10\0\0\0\x11\0") }, 66usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::NoncharacterCodePointV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::NoncharacterCodePointV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPS_NCHAR_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::NoncharacterCodePointV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
