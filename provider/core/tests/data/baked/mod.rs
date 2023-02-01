// @generated
mod core;
use ::icu_provider::prelude::*;
/// Implement [`DataProvider<M>`] on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// This macro can only be called from its definition-site, i.e. right
/// after `include!`-ing the generated module.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : path) => {
        impl DataProvider<::icu_provider::hello_world::HelloWorldV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_provider::hello_world::HelloWorldV1Marker>, DataError> {
                core::helloworld_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_provider::hello_world::HelloWorldV1Marker::KEY, req))
            }
        }
    };
}
/// Implement [`AnyProvider`] on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_any` constructors.
///
/// This macro can only be called from its definition-site, i.e. right
/// after `include!`-ing the generated module.
///
/// ```compile_fail
/// struct MyAnyProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : path) => {
        impl AnyProvider for $provider {
            fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
                const HELLOWORLDV1MARKER: ::icu_provider::DataKeyHash = ::icu_provider::hello_world::HelloWorldV1Marker::KEY.hashed();
                match key.hashed() {
                    HELLOWORLDV1MARKER => core::helloworld_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    _ => return Err(DataErrorKind::MissingDataKey.with_req(key, req)),
                }
                .map(|payload| AnyResponse {
                    payload: Some(payload),
                    metadata: Default::default(),
                })
                .ok_or_else(|| DataErrorKind::MissingLocale.with_req(key, req))
            }
        }
    };
}
struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
