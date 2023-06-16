// @generated
/// Implement [`DataProvider<PatternSyntaxV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_pat_syn_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_PAT_SYN_V1: &'static <icu_properties::provider::PatternSyntaxV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\0\0\x000\0\0\0:\0\0\0A\0\0\0[\0\0\0_\0\0\0`\0\0\0a\0\0\0{\0\0\0\x7F\0\0\0\xA1\0\0\0\xA8\0\0\0\xA9\0\0\0\xAA\0\0\0\xAB\0\0\0\xAD\0\0\0\xAE\0\0\0\xAF\0\0\0\xB0\0\0\0\xB2\0\0\0\xB6\0\0\0\xB7\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\xD7\0\0\0\xD8\0\0\0\xF7\0\0\0\xF8\0\0\0\x10 \0\0( \0\x000 \0\0? \0\0A \0\0T \0\0U \0\0_ \0\0\x90!\0\0`$\0\0\0%\0\0v'\0\0\x94'\0\0\0,\0\0\0.\0\0\x80.\0\0\x010\0\0\x040\0\0\x080\0\0!0\0\x0000\0\x0010\0\0>\xFD\0\0@\xFD\0\0E\xFE\0\0G\xFE\0\0") }, 2760usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::PatternSyntaxV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::PatternSyntaxV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_PAT_SYN_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::PatternSyntaxV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
