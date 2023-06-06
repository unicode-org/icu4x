// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_to_long_linear_gc_v1 {
    () => {
        icu_properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x1D\0\0\0\0\0\n\0\x1A\0*\0:\0I\0U\0d\0r\0~\0\x8C\0\x99\0\xA5\0\xB4\0\xC2\0\xD5\0\xDC\0\xE2\0\xED\0\xF6\0\x06\x01\x16\x01'\x01<\x01M\x01X\x01g\x01v\x01\x82\x01UnassignedUppercase_LetterLowercase_LetterTitlecase_LetterModifier_LetterOther_LetterNonspacing_MarkEnclosing_MarkSpacing_MarkDecimal_NumberLetter_NumberOther_NumberSpace_SeparatorLine_SeparatorParagraph_SeparatorControlFormatPrivate_UseSurrogateDash_PunctuationOpen_PunctuationClose_PunctuationConnector_PunctuationOther_PunctuationMath_SymbolCurrency_SymbolModifier_SymbolOther_SymbolInitial_Punctuation") } }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_propnames_to_long_linear_gc_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_properties::provider::GeneralCategoryValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_to_long_linear_gc_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<GeneralCategoryValueToLongNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_gc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::GeneralCategoryValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::GeneralCategoryValueToLongNameV1Marker>, icu_provider::DataError> {
                let lookup = lookup_propnames_to_long_linear_gc_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::GeneralCategoryValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
