// @generated
/// Implement [`DataProvider<LineBreakNameToValueV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_lb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_FROM_LB_V1: &'static <icu_properties::provider::LineBreakNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
                map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"Q\0\0\0\0\0\x02\0\x04\0\x0E\0\x17\0\x19\0\x1B\0\x1D\0\x1F\0*\x006\0@\0M\0\\\0^\0`\0b\0s\0\x84\0\x86\0\x94\0\xA3\0\xBF\0\xCF\0\xD1\0\xD3\0\xD5\0\xDB\0\xDD\0\xE7\0\xE9\0\xF4\0\xF6\0\xFA\0\xFC\0\xFE\0\x0B\x01\r\x01\x0F\x01\x15\x01\x17\x01\"\x01$\x011\x01<\x01G\x01I\x01K\x01M\x01O\x01Q\x01Z\x01i\x01r\x01t\x01~\x01\x80\x01\x82\x01\x89\x01\x8B\x01\x9B\x01\x9D\x01\xAC\x01\xAE\x01\xBC\x01\xBE\x01\xC7\x01\xD9\x01\xDB\x01\xDD\x01\xDF\x01\xE1\x01\xE6\x01\xEF\x01\xF1\x01\xF8\x01\xFA\x01\x05\x02\x07\x02\t\x02\x0C\x02AIALAlphabeticAmbiguousB2BABBBKBreak_AfterBreak_BeforeBreak_BothBreak_SymbolsCarriage_ReturnCBCJCLClose_ParenthesisClose_PunctuationCMCombining_MarkComplex_ContextConditional_Japanese_StarterContingent_BreakCPCREBE_BaseEME_ModifierEXExclamationGLGlueH2H3Hebrew_LetterHLHYHyphenIDIdeographicINInfix_NumericInseparableInseperableISJLJTJVLFLine_FeedMandatory_BreakNext_LineNLNonstarterNSNUNumericOPOpen_PunctuationPOPostfix_NumericPRPrefix_NumericQUQuotationRegional_IndicatorRISASGSPSpaceSurrogateSYUnknownWJWord_JoinerXXZWZWJZWSpace") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\x02\0\x02\0\x01\0\x03\0\x04\0\x05\0\x06\0\x04\0\x05\0\x03\0\x1B\0\n\0\x07\0%\0\x08\0$\0\x08\0\t\0\t\0\x18\0%\0\x07\0$\0\n\0(\0(\0)\0)\0\x0B\0\x0B\0\x0C\0\x0C\0\x1F\0 \0&\0&\0\r\0\r\0\x0E\0\x0E\0\x0F\0\x10\0\x0F\0\x0F\0\x10\0!\0\"\0#\0\x11\0\x11\0\x06\0\x1D\0\x1D\0\x12\0\x12\0\x13\0\x13\0\x14\0\x14\0\x15\0\x15\0\x16\0\x16\0\x17\0\x17\0'\0'\0\x18\0\x19\0\x1A\0\x1A\0\x19\0\x1B\0\0\0\x1E\0\x1E\0\0\0\x1C\0*\0\x1C\0") })
                },
            };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::LineBreakNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::LineBreakNameToValueV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_FROM_LB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::LineBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
