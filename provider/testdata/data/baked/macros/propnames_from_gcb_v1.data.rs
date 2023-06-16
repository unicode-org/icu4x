// @generated
/// Implement [`DataProvider<GraphemeClusterBreakNameToValueV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_gcb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_FROM_GCB_V1: &'static <icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
                map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x1C\0\0\0\0\0\x02\0\t\0\x0B\0\r\0\x13\0\x1D\0 \0\"\0,\0.\x004\x007\0E\0F\0H\0J\0M\0R\0T\0[\0m\0o\0q\0|\0}\0~\0\x80\0CNControlCREBE_BaseE_Base_GAZEBGEME_ModifierEXExtendGAZGlue_After_ZwjLLFLVLVTOtherPPPrependRegional_IndicatorRISMSpacingMarkTVXXZWJ") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\x01\0\x02\0\r\0\r\0\x0E\0\x0E\0\x0F\0\x0F\0\x03\0\x03\0\x10\0\x10\0\x04\0\x05\0\x06\0\x07\0\0\0\x0B\0\x0B\0\x0C\0\x0C\0\n\0\n\0\x08\0\t\0\0\0\x11\0") })
                },
            };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPNAMES_FROM_GCB_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::GraphemeClusterBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
