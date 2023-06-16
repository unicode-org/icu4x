// @generated
/// Implement [`DataProvider<WordBreakNameToValueV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_wb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_FROM_WB_V1: &'static <icu_properties::provider::WordBreakNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
                map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b")\0\0\0\0\0\x07\0\t\0\x15\0\x17\0\x19\0\x1F\0)\0,\0.\08\0:\0@\0L\0N\0T\0W\0e\0r\0t\0v\0~\0\x80\0\x82\0\x84\0\x8D\0\x93\0\x9C\0\x9E\0\xA0\0\xA7\0\xA9\0\xAB\0\xB2\0\xB7\0\xC9\0\xCB\0\xD7\0\xD9\0\xE2\0\xE4\0ALetterCRDouble_QuoteDQEBE_BaseE_Base_GAZEBGEME_ModifierEXExtendExtendNumLetFOFormatGAZGlue_After_ZwjHebrew_LetterHLKAKatakanaLELFMBMidLetterMidNumMidNumLetMLMNNewlineNLNUNumericOtherRegional_IndicatorRISingle_QuoteSQWSegSpaceXXZWJ") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\x08\0\x10\0\x10\0\x11\0\x11\0\x12\0\x12\0\x13\0\x13\0\x07\0\t\0\x07\0\x02\0\x02\0\x14\0\x14\0\x0E\0\x0E\0\x03\0\x03\0\x01\0\n\0\x0B\0\x04\0\x05\0\x0B\0\x04\0\x05\0\x0C\0\x0C\0\x06\0\x06\0\0\0\r\0\r\0\x0F\0\x0F\0\x16\0\0\0\x15\0") })
                },
            };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::WordBreakNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::WordBreakNameToValueV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPNAMES_FROM_WB_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::WordBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
