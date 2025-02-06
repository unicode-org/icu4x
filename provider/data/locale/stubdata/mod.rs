// @generated
include!("exemplar_characters_punctuation_v1.rs.data");
include!("script_direction_v1.rs.data");
include!("likely_subtags_for_language_v1.rs.data");
include!("likely_subtags_for_script_region_v1.rs.data");
include!("exemplar_characters_main_v1.rs.data");
include!("exemplar_characters_auxiliary_v1.rs.data");
include!("aliases_v2.rs.data");
include!("exemplar_characters_index_v1.rs.data");
include!("likely_subtags_extended_v1.rs.data");
include!("parents_v1.rs.data");
include!("exemplar_characters_numbers_v1.rs.data");
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
        #[clippy::msrv = "1.81"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::marker::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_exemplar_characters_punctuation_v1!($provider);
        impl_script_direction_v1!($provider);
        impl_likely_subtags_for_language_v1!($provider);
        impl_likely_subtags_for_script_region_v1!($provider);
        impl_exemplar_characters_main_v1!($provider);
        impl_exemplar_characters_auxiliary_v1!($provider);
        impl_aliases_v2!($provider);
        impl_exemplar_characters_index_v1!($provider);
        impl_likely_subtags_extended_v1!($provider);
        impl_parents_v1!($provider);
        impl_exemplar_characters_numbers_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.81"]
        impl icu_provider::any::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.id.hashed() {
                    h if h == <icu::locale::provider::ExemplarCharactersPunctuationV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::ExemplarCharactersPunctuationV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::ScriptDirectionV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::ScriptDirectionV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::LikelySubtagsForLanguageV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::LikelySubtagsForLanguageV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::LikelySubtagsForScriptRegionV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::LikelySubtagsForScriptRegionV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::ExemplarCharactersMainV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::ExemplarCharactersMainV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::ExemplarCharactersAuxiliaryV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::ExemplarCharactersAuxiliaryV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::AliasesV2 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::AliasesV2>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::ExemplarCharactersIndexV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::ExemplarCharactersIndexV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::LikelySubtagsExtendedV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::LikelySubtagsExtendedV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::ParentsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::ParentsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::locale::provider::ExemplarCharactersNumbersV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::locale::provider::ExemplarCharactersNumbersV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MarkerNotFound.with_req(marker, req)),
                }
            }
        }
    };
}
