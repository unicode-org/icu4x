// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_to_short_linear_lb_v1 {
    () => {
        icu_properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"*\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x14\0\x16\0\x18\0\x1A\0\x1C\0\x1E\0 \0\"\0$\0&\0(\0*\0,\0.\x000\x002\x004\x006\08\0:\0<\0>\0@\0B\0D\0F\0H\0J\0L\0N\0P\0R\0XXAIALB2BABBBKCBCLCMCREXGLHYIDINISLFNSNUOPPOPRQUSASGSPSYZWNLWJH2H3JLJTJVCPCJHLRIEBEM") } }
    };
}
/// Implement [`DataProvider<LineBreakValueToShortNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_short_linear_lb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::LineBreakValueToShortNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::LineBreakValueToShortNameV1Marker>, icu_provider::DataError> {
                req.locale
                    .is_empty()
                    .then(|| {
                        static ANCHOR: <icu_properties::provider::LineBreakValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_to_short_linear_lb_v1!();
                        &ANCHOR
                    })
                    .map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from)
                    .map(icu_provider::DataPayload::from_owned)
                    .map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) })
                    .ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::LineBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
