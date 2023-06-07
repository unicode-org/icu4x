// @generated
/// Implement [`DataProvider<SentenceBreakValueToLongNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_sb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_LONG_LINEAR_SB_V1: &'static <icu_properties::provider::SentenceBreakValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x05\0\n\0\x0F\0\x15\0\x1A\0!\0(\0+\0-\x002\x007\09\0?\0OtherATermCloseFormatLowerNumericOLetterSepSpSTermUpperCRExtendLF") } };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::SentenceBreakValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::SentenceBreakValueToLongNameV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPNAMES_TO_LONG_LINEAR_SB_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::SentenceBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
