// @generated
/// Implement `DataProvider<SentenceBreakValueToLongNameV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_sb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_LONG_LINEAR_SB_V1: &'static <icu::properties::provider::SentenceBreakValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x05\0\n\0\x0F\0\x15\0\x1A\0!\0(\0+\0-\x002\x007\09\0?\0OtherATermCloseFormatLowerNumericOLetterSepSpSTermUpperCRExtendLF") } };
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::properties::provider::SentenceBreakValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakValueToLongNameV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_TO_LONG_LINEAR_SB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::SentenceBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
