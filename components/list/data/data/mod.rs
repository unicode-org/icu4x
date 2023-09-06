// @generated
include!("macros.rs");
/// Implement `DataProvider<M>` on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_data_provider {
    ($ provider : path) => {
        impl_list_and_v1!($provider);
        impl_list_or_v1!($provider);
        impl_list_unit_v1!($provider);
    };
}
#[doc(inline)]
pub use __impl_data_provider as impl_data_provider;
/// Implement `AnyProvider` on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_any` constructors.
///
/// ```compile_fail
/// struct MyAnyProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.66"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::list::provider::AndListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::list::provider::AndListV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::list::provider::OrListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::list::provider::OrListV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::list::provider::UnitListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::list::provider::UnitListV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
#[clippy::msrv = "1.66"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
