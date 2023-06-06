// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_to_long_linear_wb_v1 {
    () => {
        icu_properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x16\0\0\0\0\0\x05\0\x0C\0\x12\0\x1A\0#\0)\x000\0<\0>\0D\0F\0O\0V\0h\0u\0\x81\0\x8D\0\x93\0\x9D\0\xA7\0\xB5\0OtherALetterFormatKatakanaMidLetterMidNumNumericExtendNumLetCRExtendLFMidNumLetNewlineRegional_IndicatorHebrew_LetterSingle_QuoteDouble_QuoteE_BaseE_Base_GAZE_ModifierGlue_After_ZwjZWJ") } }
    };
}
/// Implement [`DataProvider<WordBreakValueToLongNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_wb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::WordBreakValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::WordBreakValueToLongNameV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::WordBreakValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_to_long_linear_wb_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::WordBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
