// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_to_long_linear_lb_v1 {
    () => {
        icu_properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"*\0\0\0\0\0\x07\0\x10\0\x1A\0$\0/\0;\0J\0Z\0k\0y\0\x88\0\x93\0\x97\0\x9D\0\xA8\0\xB3\0\xC0\0\xC9\0\xD3\0\xDA\0\xEA\0\xF9\0\x07\x01\x10\x01\x1F\x01(\x01-\x01:\x01A\x01J\x01U\x01W\x01Y\x01[\x01]\x01_\x01p\x01\x8C\x01\x99\x01\xAB\x01\xB1\x01UnknownAmbiguousAlphabeticBreak_BothBreak_AfterBreak_BeforeMandatory_BreakContingent_BreakClose_PunctuationCombining_MarkCarriage_ReturnExclamationGlueHyphenIdeographicInseparableInfix_NumericLine_FeedNonstarterNumericOpen_PunctuationPostfix_NumericPrefix_NumericQuotationComplex_ContextSurrogateSpaceBreak_SymbolsZWSpaceNext_LineWord_JoinerH2H3JLJTJVClose_ParenthesisConditional_Japanese_StarterHebrew_LetterRegional_IndicatorE_BaseE_Modifier") } }
    };
}
/// Implement [`DataProvider<LineBreakValueToLongNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_lb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::LineBreakValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::LineBreakValueToLongNameV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::LineBreakValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_to_long_linear_lb_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::LineBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
