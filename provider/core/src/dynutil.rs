// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Utilities for using trait objects with `DataPayload`.

/// Trait to allow conversion from `DataPayload<T>` to `DataPayload<S>` where `T` implements `S`.
///
/// This is used internally by [`impl_dyn_provider!`] and is not intended to be called from userland
/// code. You may be looking for [`DataPayload::downcast`], which converts in the other direction.
///
/// [`DataPayload::downcast`]: crate::DataPayload::downcast
pub trait UpcastDataPayload<M>
where
    M: crate::prelude::DataMarker,
    Self: Sized + crate::prelude::DataMarker,
{
    /// Upcast a `DataPayload<T>` to a `DataPayload<S>` where `T` implements trait `S`.
    ///
    /// # Examples
    ///
    /// Upcast and then downcast a data struct of type `Cow<str>` (cart type `String`) via
    /// [`ErasedDataStruct`](crate::erased::ErasedDataStruct):
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::*;
    /// use icu_provider::dynutil::UpcastDataPayload;
    /// use icu_provider::marker::CowStrMarker;
    /// use std::borrow::Cow;
    ///
    /// let data = "foo".to_string();
    /// let original = DataPayload::<CowStrMarker>::from_owned(Cow::Owned(data));
    /// let upcasted = ErasedDataStructMarker::upcast(original);
    /// let downcasted = upcasted
    ///     .downcast::<CowStrMarker>()
    ///     .expect("Type conversion");
    /// assert_eq!(downcasted.get(), "foo");
    /// ```
    fn upcast(other: crate::prelude::DataPayload<M>) -> crate::prelude::DataPayload<Self>;
}

/// Implement [`DataProvider`] for a trait object `S` on a type that already implements [`DataProvider`]
/// for one or more `M`, where `M` is a concrete type that implements the trait `S`.
///
/// Use this macro to add support to your data provider for:
///
/// - [`ErasedDataStruct`] if your provider can return typed objects as [`Any`](core::any::Any)
/// - [`SerializeMarker`] if your provider returns objects implementing [`serde::Serialize`]
///
/// The second argument is a match-like construction mapping from resource keys to structs. To map
/// multiple keys to a single data struct, use `_` as the data key.
///
/// The third argument can be either the trait object marker, like [`SerializeMarker`], or the
/// shorthands `ERASED` or `SERDE_SE`.
///
/// Lifetimes:
///
/// - `$data` is the lifetime parameter for [`DataProvider`](crate::DataProvider); usually `'data`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::erased::ErasedDataStructMarker;
/// use icu_provider::marker::CowStrMarker;
/// use std::borrow::Cow;
/// const DEMO_KEY: ResourceKey = icu_provider::resource_key!(x, "foo", "bar", 1);
///
/// // A small DataProvider that returns owned strings
/// struct MyProvider(pub String);
/// impl DataProvider<CowStrMarker> for MyProvider {
///     fn load_payload(&self, req: &DataRequest)
///             -> Result<DataResponse<CowStrMarker>, DataError> {
///         req.resource_path.key.match_key(DEMO_KEY)?;
///         Ok(DataResponse {
///             metadata: Default::default(),
///             payload: Some(DataPayload::from_owned(Cow::Owned(self.0.to_string())))
///         })
///     }
/// }
///
/// // Implement DataProvider<ErasedDataStructMarker>
/// icu_provider::impl_dyn_provider!(MyProvider, {
///     DEMO_KEY => CowStrMarker,
/// }, ERASED);
///
/// // Usage example
/// let provider = MyProvider("demo".to_string());
/// let resp: DataResponse<ErasedDataStructMarker> = provider
///     .load_payload(&DEMO_KEY.into())
///     .expect("Loading should succeed");
/// let payload: DataPayload<CowStrMarker> = resp
///     .take_payload()
///     .expect("Payload should be present")
///     .downcast()
///     .expect("Type should downcast successfully");
/// assert_eq!("demo", payload.get());
/// ```
///
/// Using the wildcard `_` match:
///
/// ```
/// # use icu_provider::prelude::*;
/// # use icu_provider::marker::CowStrMarker;
/// # use std::borrow::Cow;
/// # struct MyProvider(pub String);
/// # impl DataProvider<CowStrMarker> for MyProvider {
/// #   fn load_payload(&self, req: &DataRequest)
/// #           -> Result<DataResponse<CowStrMarker>, DataError> {
/// #       Ok(DataResponse {
/// #           metadata: Default::default(),
/// #           payload: Some(DataPayload::from_owned(self.0.to_string().into()))
/// #       })
/// #   }
/// # }
/// // Send all keys to the `CowStrMarker` provider.
/// icu_provider::impl_dyn_provider!(MyProvider, {
///     _ => CowStrMarker,
/// }, ERASED);
/// ```
///
/// [`DataProvider`]: crate::DataProvider
/// [`ErasedDataStruct`]: (crate::erased::ErasedDataStruct)
/// [`SerializeMarker`]: (crate::serde::SerializeMarker)
#[macro_export]
macro_rules! impl_dyn_provider {
    ($provider:ty, { $($pat:pat => $struct_m:ty),+, }, ERASED) => {
        $crate::impl_dyn_provider!(
            $provider,
            { $($pat => $struct_m),+, },
            $crate::erased::ErasedDataStructMarker
        );
    };
    ($provider:ty, { $($pat:pat => $struct_m:ty),+, }, SERDE_SE) => {
        // If this fails to compile, enable the "serialize" feature on this crate.
        $crate::impl_dyn_provider!(
            $provider,
            { $($pat => $struct_m),+, },
            $crate::serde::SerializeMarker
        );
    };
    ($provider:ty, { $($pat:pat => $struct_m:ty),+, }, $dyn_m:path) => {
        impl $crate::prelude::DataProvider<$dyn_m> for $provider
        {
            fn load_payload(
                &self,
                req: &$crate::prelude::DataRequest,
            ) -> Result<
                $crate::prelude::DataResponse<$dyn_m>,
                $crate::prelude::DataError,
            > {
                match req.resource_path.key {
                    $(
                        $pat => {
                            let result: $crate::prelude::DataResponse<$struct_m> =
                                $crate::prelude::DataProvider::load_payload(self, req)?;
                            Ok(DataResponse {
                                metadata: result.metadata,
                                payload: result.payload.map(|p| {
                                    $crate::dynutil::UpcastDataPayload::<$struct_m>::upcast(p)
                                }),
                            })
                        }
                    )+,
                    // Don't complain if the call site has its own wildcard match
                    #[allow(unreachable_patterns)]
                    _ => Err($crate::prelude::DataErrorKind::MissingResourceKey.with_req(req))
                }
            }
        }
    }
}
