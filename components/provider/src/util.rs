// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Implement ToOwned on a trait object, enabling it to be used in a Cow. Requires the trait to
/// have a method named `clone_into_boxed()`.
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
            fn clone(&self) -> Box<(dyn $trait + $s)> {
                <dyn $trait + $s>::clone_into_box(self.as_ref())
            }
        }
    };
}

/// Implement `From<DataPayload<T>>` for `DataPayload<dyn S>` where `T` implements the trait `S`.
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
                    cow: other.cow.map(|p| match p {
                        Cow::Borrowed(v) => Cow::Borrowed(v as &(dyn $trait + 's)),
                        Cow::Owned(v) => {
                            let boxed: Box<dyn $trait + 's> = Box::new(v);
                            Cow::Owned(boxed)
                        }
                    }),
                }
            }
        }
    };
}

/// Implement `DataProvider<dyn S>` on a type that already implements `DataProvider<T>` where `T`
/// is a `Sized` type that implements the trait `S`.
///
/// Use this macro to add support to your data provider for:
///
/// - `ErasedDataStruct` if your provider can return typed objects as `Any`
/// - `SerdeSeDataStruct` if your provider returns objects implementing `serde::Serialize`
///
/// The third argument can be either the trait expression, like `SerdeSeDataStruct<'s>`, or the
/// shorthands `ERASED` or `SERDE_SE`.
///
/// Lifetimes:
///
/// - `$d` is the lifetime parameter for `DataProvider`; usually `'d`
/// - `$s` is the lifetime bound for the struct trait; usually `'s`
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::erased::ErasedDataStruct;
/// use std::borrow::Cow;
///
/// const DEMO_KEY: ResourceKey = icu_provider::resource_key!(x, "foo", "bar", 1);
///
/// // A small DataProvider that returns owned strings
/// struct MyProvider(pub String);
/// impl<'d> DataProvider<'d, String> for MyProvider {
///     fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, String>, DataError> {
///         req.resource_path.key.match_key(DEMO_KEY)?;
///         Ok(DataResponse {
///             metadata: Default::default(),
///             payload: DataPayload {
///                 cow: Some(Cow::Owned(self.0.to_string()))
///             }
///         })
///     }
/// }
///
/// // Since `String` is `'static`, we can implement `DataProvider<dyn ErasedDataStruct>`
/// icu_provider::impl_dyn_provider!(MyProvider, String, ERASED, 'd, 's);
///
/// // Usage example
/// let provider = MyProvider("demo".to_string());
/// let resp: DataResponse<dyn ErasedDataStruct> = provider
///     .load_payload(&DEMO_KEY.into())
///     .expect("Loading should succeed");
/// ```
#[macro_export]
macro_rules! impl_dyn_provider {
    ($provider:ty, $struct:ty, ERASED, $d:lifetime, $s:lifetime) => {
        $crate::impl_dyn_provider!($provider, $struct, $crate::erased::ErasedDataStruct, $d, $s);
    };
    ($provider:ty, $struct:ty, SERDE_SE, $d:lifetime, $s:lifetime) => {
        $crate::impl_dyn_provider!(
            $provider,
            $struct,
            $crate::serde::SerdeSeDataStruct<$s>,
            $d,
            $s
        );
    };
    ($provider:ty, $struct:ty, $struct_trait:path, $d:lifetime, $s:lifetime) => {
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
                let result: $crate::prelude::DataResponse<$struct> =
                    $crate::prelude::DataProvider::load_payload(self, req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
        }
    };
}
