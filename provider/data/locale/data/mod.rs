// @generated
include!("fallback_likelysubtags_v1.rs.data");
include!("fallback_parents_v1.rs.data");
include!("fallback_supplement_co_v1.rs.data");
include!("locid_transform_aliases_v2.rs.data");
include!("locid_transform_likelysubtags_ext_v1.rs.data");
include!("locid_transform_likelysubtags_l_v1.rs.data");
include!("locid_transform_likelysubtags_sr_v1.rs.data");
include!("locid_transform_script_dir_v1.rs.data");
/// Marks a type as a data provider. You can then use macros like
/// `impl_core_helloworld_v1` to add implementations.
///
/// ```ignore
/// struct MyProvider;
/// const _: () = {
///     include!("path/to/generated/macros.rs");
///     make_provider!(MyProvider);
///     impl_core_helloworld_v1!(MyProvider);
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __make_provider {
    ($ name : ty) => {
        #[clippy::msrv = "1.70"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_fallback_likelysubtags_v1!($provider);
        impl_fallback_parents_v1!($provider);
        impl_fallback_supplement_co_v1!($provider);
        impl_locid_transform_aliases_v2!($provider);
        impl_locid_transform_likelysubtags_ext_v1!($provider);
        impl_locid_transform_likelysubtags_l_v1!($provider);
        impl_locid_transform_likelysubtags_sr_v1!($provider);
        impl_locid_transform_script_dir_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.70"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.path.hashed() {
                    h if h == <icu::locale::provider::LocaleFallbackLikelySubtagsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::LocaleFallbackLikelySubtagsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::LocaleFallbackParentsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::LocaleFallbackParentsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::CollationFallbackSupplementV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::CollationFallbackSupplementV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::AliasesV2Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::AliasesV2Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::LikelySubtagsExtendedV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::LikelySubtagsExtendedV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::LikelySubtagsForLanguageV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::LikelySubtagsForLanguageV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::LikelySubtagsForScriptRegionV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::LikelySubtagsForScriptRegionV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::ScriptDirectionV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::locale::provider::ScriptDirectionV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataMarker.with_req(marker, req)),
                }
            }
        }
    };
}
