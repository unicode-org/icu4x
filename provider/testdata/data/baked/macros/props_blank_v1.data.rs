// @generated
/// Implement [`DataProvider<BlankV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_blank_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_BLANK_V1: &'static <icu_properties::provider::BlankV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\t\0\0\0\n\0\0\0 \0\0\0!\0\0\0\xA0\0\0\0\xA1\0\0\0\x80\x16\0\0\x81\x16\0\0\0 \0\0\x0B \0\0/ \0\x000 \0\0_ \0\0` \0\0\x000\0\0\x010\0\0") }, 18usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::BlankV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::BlankV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_BLANK_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::BlankV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
