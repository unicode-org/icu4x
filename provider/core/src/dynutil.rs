// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Utilities for using trait objects with `DataPayload`.

/// Trait to allow conversion from `DataPayload<T>` to `DataPayload<S>`.
///
/// This trait can be manually implemented in order to enable [`impl_dyn_provider`].
///
/// [`DataPayload::downcast`]: crate::DataPayload::downcast
pub trait UpcastDataPayload<M>
where
    M: crate::DataMarker,
    Self: Sized + crate::DataMarker,
{
    /// Upcast a `DataPayload<T>` to a `DataPayload<S>` where `T` implements trait `S`.
    ///
    /// # Examples
    ///
    /// Upcast and then downcast a data struct of type `Cow<str>` (cart type `String`) via
    /// [`AnyPayload`](crate::any::AnyPayload):
    ///
    /// ```
    /// use icu_provider::dynutil::UpcastDataPayload;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let original = DataPayload::<HelloWorldV1Marker>::from_static_str("foo");
    /// let upcasted = AnyMarker::upcast(original);
    /// let downcasted = upcasted
    ///     .downcast::<HelloWorldV1Marker>()
    ///     .expect("Type conversion");
    /// assert_eq!(downcasted.get().message, "foo");
    /// ```
    fn upcast(other: crate::DataPayload<M>) -> crate::DataPayload<Self>;
}

/// Implements [`DynProvider`] for a marker type `S` on a type that already implements
/// [`DynProvider`] or [`ResourceProvider`] for one or more `M`, where `M` is a concrete type
/// that is convertible to `S` via [`UpcastDataPayload`].
///
/// Use this macro to add support to your data provider for:
///
/// - [`AnyPayload`] if your provider can return typed objects as [`Any`](core::any::Any).
///
/// ## Wrapping ResourceProvider
///
/// If your type implements [`ResourceProvider`], pass a list of markers as the second argument.
/// This results in a `DynProvider` that delegates to a specific marker if the key
/// matches or else returns [`DataErrorKind::MissingResourceKey`].
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// #
/// # // Duplicating HelloWorldProvider because the real one already implements DynProvider<AnyMarker>
/// # struct HelloWorldProvider;
/// # impl ResourceProvider<HelloWorldV1Marker> for HelloWorldProvider {
/// #     fn load_resource(
/// #         &self,
/// #         req: &DataRequest,
/// #     ) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
/// #         icu_provider::hello_world::HelloWorldProvider.load_resource(req)
/// #     }
/// # }
///
/// // Implement DataProvider<AnyMarker> on HelloWorldProvider: ResourceProvider<HelloWorldV1Marker>
/// icu_provider::impl_dyn_provider!(HelloWorldProvider, [HelloWorldV1Marker,], AnyMarker);
///
/// let req = DataRequest {
///     options: icu_locid::locale!("de").into(),
///     metadata: Default::default(),
/// };
///
/// // Successful because the key matches:
/// HelloWorldProvider.load_payload(HelloWorldV1Marker::KEY, &req).unwrap();
///
/// // MissingResourceKey error as the key does not match:
/// assert_eq!(
///     HelloWorldProvider.load_payload(icu_provider::resource_key!("dummy@1"), &req).unwrap_err().kind,
///     DataErrorKind::MissingResourceKey,
/// );
/// ```
///
/// ## Wrapping DynProvider
///
/// It is also possible to wrap a [`DynProvider`] to create another [`DynProvider`]. To do this,
/// pass a match-like statement for keys as the second argument:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// #
/// # struct HelloWorldProvider;
/// # impl DynProvider<HelloWorldV1Marker> for HelloWorldProvider {
/// #     fn load_payload(&self, key: ResourceKey, req: &DataRequest)
/// #             -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
/// #         icu_provider::hello_world::HelloWorldProvider.load_resource(req)
/// #     }
/// # }
///
/// // Implement DataProvider<AnyMarker> on HelloWorldProvider: DynProvider<HelloWorldV1Marker>
/// icu_provider::impl_dyn_provider!(HelloWorldProvider, {
///     // Match HelloWorldV1Marker::KEY and delegate to DynProvider<HelloWorldV1Marker>.
///     HW = HelloWorldV1Marker::KEY => HelloWorldV1Marker,
///     // Send the wildcard match also to DynProvider<HelloWorldV1Marker>.
///     _ => HelloWorldV1Marker,
/// }, AnyMarker);
///
/// let req = DataRequest {
///     options: icu_locid::locale!("de").into(),
///     metadata: Default::default(),
/// };
///
/// // Successful because the key matches:
/// HelloWorldProvider.as_any_provider().load_any(HelloWorldV1Marker::KEY, &req).unwrap();
///
/// // Because of the wildcard, any key actually works:
/// HelloWorldProvider.as_any_provider().load_any(icu_provider::resource_key!("dummy@1"), &req).unwrap();
/// ```
///
/// [`DynProvider`]: crate::DynProvider
/// [`ResourceProvider`]: crate::ResourceProvider
/// [`AnyPayload`]: (crate::any::AnyPayload)
/// [`DataErrorKind::MissingResourceKey`]: (crate::DataErrorKind::MissingResourceKey)
/// [`SerializeMarker`]: (crate::serde::SerializeMarker)
#[macro_export]
macro_rules! impl_dyn_provider {
    // allow passing in multiple things to do and get dispatched
    ($provider:ty, $arms:tt, $one:path, $($rest:path),+) => {
        $crate::impl_dyn_provider!(
            $provider,
            $arms,
            $one
        );

        $crate::impl_dyn_provider!(
            $provider,
            $arms,
            $($rest),+
        );
    };

    ($provider:ty, { $($ident:ident = $key:path => $struct_m:ty),+, $(_ => $struct_d:ty,)?}, $dyn_m:ty) => {
        impl $crate::DynProvider<$dyn_m> for $provider
        {
            fn load_payload(
                &self,
                key: $crate::ResourceKey,
                req: &$crate::DataRequest,
            ) -> Result<
                $crate::DataResponse<$dyn_m>,
                $crate::DataError,
            > {
                $(
                    const $ident: ResourceKeyHash = $key.get_hash();
                )+
                match key.get_hash() {
                    $(
                        $ident => {
                            let result: $crate::DataResponse<$struct_m> =
                                $crate::DynProvider::<$struct_m>::load_payload(self, key, req)?;
                            Ok(DataResponse {
                                metadata: result.metadata,
                                payload: result.payload.map(|p| {
                                    $crate::dynutil::UpcastDataPayload::<$struct_m>::upcast(p)
                                }),
                            })
                        }
                    )+,
                    $(
                        _ => {
                            let result: $crate::DataResponse<$struct_d> =
                                $crate::DynProvider::<$struct_d>::load_payload(self, key, req)?;
                            Ok(DataResponse {
                                metadata: result.metadata,
                                payload: result.payload.map(|p| {
                                    $crate::dynutil::UpcastDataPayload::<$struct_d>::upcast(p)
                                }),
                            })
                        }
                    )?
                    _ => Err($crate::DataErrorKind::MissingResourceKey.with_req(key, req))
                }
            }
        }

    };
    ($provider:ty, [ $($struct_m:ident),+, ], $dyn_m:path) => {
        impl $crate::DynProvider<$dyn_m> for $provider
        {
            fn load_payload(
                &self,
                key: $crate::ResourceKey,
                req: &$crate::DataRequest,
            ) -> Result<
                $crate::DataResponse<$dyn_m>,
                $crate::DataError,
            > {
                #![allow(non_upper_case_globals)]
                // Reusing the struct names as identifiers
                $(
                    const $struct_m: $crate::ResourceKeyHash = $struct_m::KEY.get_hash();
                )+
                match key.get_hash() {
                    $(
                        $struct_m => {
                            let result: $crate::DataResponse<$struct_m> =
                                $crate::ResourceProvider::load_resource(self, req)?;
                            Ok(DataResponse {
                                metadata: result.metadata,
                                payload: result.payload.map(|p| {
                                    $crate::dynutil::UpcastDataPayload::<$struct_m>::upcast(p)
                                }),
                            })
                        }
                    )+,
                    _ => Err($crate::DataErrorKind::MissingResourceKey.with_req(key, req))
                }
            }
        }
    };
}
