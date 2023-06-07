// @generated
/// Implement [`DataProvider<VariationSelectorV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_vs_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_VS_V1: &'static <icu_properties::provider::VariationSelectorV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x0B\x18\0\0\x0E\x18\0\0\x0F\x18\0\0\x10\x18\0\0\0\xFE\0\0\x10\xFE\0\0\0\x01\x0E\0\xF0\x01\x0E\0") }, 260usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::VariationSelectorV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::VariationSelectorV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPS_VS_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::VariationSelectorV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
