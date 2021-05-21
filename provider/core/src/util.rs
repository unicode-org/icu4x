// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Implement [`ToOwned`](std::borrow::ToOwned) on a trait object, enabling it to be used in a [`Cow`](std::borrow::Cow).
/// Requires the trait to have a method named `clone_into_box()`.
macro_rules! impl_dyn_clone {
    ($trait:path) => {
        impl_dyn_clone!($trait, 's);
    };
    ($trait:path, $s:lifetime) => {
        impl<$s> ToOwned for dyn $trait + $s {
            type Owned = Box<dyn $trait + $s>;
            fn to_owned(&self) -> Self::Owned {
                <dyn $trait + $s>::clone_into_box(self)
            }
        }
        impl<$s> Clone for Box<(dyn $trait + $s)> {
            fn clone(&self) -> Self {
                <dyn $trait + $s>::clone_into_box(self.as_ref())
            }
        }
    };
}

/// Implement [`From`](std::convert::From)`<`[`DataPayload<T>`]`>` for [DataPayload<dyn S>`] where `T` implements the trait `S`.
macro_rules! impl_dyn_from_payload {
    ($trait:path, $d:lifetime, $s:lifetime) => {
        impl<$d, $s: $d, T> From<$crate::prelude::DataPayload<$d, T>>
            for $crate::prelude::DataPayload<$d, dyn $trait + 's>
        where
            T: $trait + Clone,
        {
            fn from(
                other: $crate::prelude::DataPayload<$d, T>,
            ) -> $crate::prelude::DataPayload<$d, dyn $trait + 's> {
                use std::borrow::Cow;
                Self {
                    cow: match other.cow {
                        Cow::Borrowed(v) => Cow::Borrowed(v as &(dyn $trait + 's)),
                        Cow::Owned(v) => {
                            let boxed: Box<dyn $trait + 's> = Box::new(v);
                            Cow::Owned(boxed)
                        }
                    },
                }
            }
        }
    };
}

/// Implement [`DataProvider<dyn S>`](crate::DataProvider) on a type that already implements [`DataProvider<T>`](crate::DataProvider)
/// for one or more `T`, where `T` is a [`Sized`] type that implements the trait `S`.
///
/// Use this macro to add support to your data provider for:
///
/// - [`ErasedDataStruct`](crate::erased::ErasedDataStruct) if your provider can return typed objects as [`Any`](std::any::Any)
/// - [`SerdeSeDataStruct`](crate::serde::SerdeSeDataStruct) if your provider returns objects implementing [`serde::Serialize`]
///
/// The second argument is a match-like construction mapping from resource keys to structs. To map
/// multiple keys to a single data struct, use `_` as the data key.
///
/// The third argument can be either the trait expression, like [SerdeSeDataStruct<'s>`], or the
/// shorthands `ERASED` or `SERDE_SE`.
///
/// Lifetimes:
///
/// - `$d` is the lifetime parameter for [`DataProvider`](crate::DataProvider); usually `'d`
/// - `$s` is the lifetime bound for the struct trait; usually `'s`
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use icu_provider::prelude::*;
/// # use icu_provider::erased::ErasedDataStruct;
/// # use std::borrow::Cow;
/// const DEMO_KEY: ResourceKey = icu_provider::resource_key!(x, "foo", "bar", 1);
///
/// // A small DataProvider that returns owned strings
/// struct MyProvider(pub String);
/// impl<'d> DataProvider<'d, String> for MyProvider {
///     fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, String>, DataError> {
///         req.resource_path.key.match_key(DEMO_KEY)?;
///         Ok(DataResponse {
///             metadata: Default::default(),
///             payload: Some(DataPayload {
///                 cow: Cow::Owned(self.0.to_string())
///             })
///         })
///     }
/// }
///
/// // Since `String` is `'static`, we can implement `DataProvider<dyn ErasedDataStruct>`
/// icu_provider::impl_dyn_provider!(MyProvider, {
///     DEMO_KEY => String,
/// }, ERASED, 'd, 's);
///
/// // Usage example
/// let provider = MyProvider("demo".to_string());
/// let resp: DataResponse<dyn ErasedDataStruct> = provider
///     .load_payload(&DEMO_KEY.into())
///     .expect("Loading should succeed");
/// ```
///
/// Using the wildcard `_` match:
///
/// ```
/// # use icu_provider::prelude::*;
/// # use std::borrow::Cow;
/// # struct MyProvider(pub String);
/// # impl<'d> DataProvider<'d, String> for MyProvider {
/// #   fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, String>, DataError> {
/// #       Ok(DataResponse {
/// #           metadata: Default::default(),
/// #           payload: Some(DataPayload {
/// #               cow: Cow::Owned(self.0.to_string())
/// #           })
/// #       })
/// #   }
/// # }
/// // Send all keys to the `String` provider.
/// icu_provider::impl_dyn_provider!(MyProvider, {
///     _ => String,
/// }, ERASED, 'd, 's);
/// ```
#[macro_export]
macro_rules! impl_dyn_provider {
    ($provider:ty, { $($pat:pat => $struct:ty),+, }, ERASED, $d:lifetime, $s:lifetime) => {
        $crate::impl_dyn_provider!(
            $provider,
            { $($pat => $struct),+, },
            $crate::erased::ErasedDataStruct,
            $d,
            $s
        );
    };
    ($provider:ty, { $($pat:pat => $struct:ty),+, }, SERDE_SE, $d:lifetime, $s:lifetime) => {
        // If this fails to compile, enable the "provider_serde" feature on this crate.
        $crate::impl_dyn_provider!(
            $provider,
            { $($pat => $struct),+, },
            $crate::serde::SerdeSeDataStruct<$s>,
            $d,
            $s
        );
    };
    ($provider:ty, { $($pat:pat => $struct:ty),+, }, $struct_trait:path, $d:lifetime, $s:lifetime) => {
        impl<$d, $s> $crate::prelude::DataProvider<$d, dyn $struct_trait + $s> for $provider
        where
            $s: $d,
        {
            fn load_payload(
                &self,
                req: &$crate::prelude::DataRequest,
            ) -> Result<
                $crate::prelude::DataResponse<$d, dyn $struct_trait + $s>,
                $crate::prelude::DataError,
            > {
                match req.resource_path.key {
                    $(
                        $pat => {
                            let result: $crate::prelude::DataResponse<$struct> =
                                $crate::prelude::DataProvider::load_payload(self, req)?;
                            Ok(DataResponse {
                                metadata: result.metadata,
                                payload: result.payload.map(|p| p.into()),
                            })
                        }
                    )+,
                    // Don't complain if the call site has its own wildcard match
                    #[allow(unreachable_patterns)]
                    _ => Err(DataError::UnsupportedResourceKey(req.resource_path.key))
                }
            }
        }
    }
}
