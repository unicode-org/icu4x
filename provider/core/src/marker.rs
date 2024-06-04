// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Marker types and traits for DataProvider.

use core::marker::PhantomData;

use crate::{data_key, DataKey, DataProvider, DataProviderWithKey};
use yoke::Yokeable;

/// Trait marker for data structs. All types delivered by the data provider must be associated with
/// something implementing this trait.
///
/// Structs implementing this trait are normally generated with the [`data_struct`] macro.
///
/// By convention, the non-standard `Marker` suffix is used by types implementing DynamicDataMarker.
///
/// In addition to a marker type implementing DynamicDataMarker, the following impls must also be present
/// for the data struct:
///
/// - `impl<'a> Yokeable<'a>` (required)
/// - `impl ZeroFrom<Self>`
///
/// Also see [`DataMarker`].
///
/// Note: `DynamicDataMarker`s are quasi-const-generic compile-time objects, and as such are expected
/// to be unit structs. As this is not something that can be enforced by the type system, we
/// currently only have a `'static` bound on them (which is needed by a lot of our code).
///
/// # Examples
///
/// Manually implementing DynamicDataMarker for a custom type:
///
/// ```
/// use icu_provider::prelude::*;
/// use std::borrow::Cow;
///
/// #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
/// struct MyDataStruct<'data> {
///     message: Cow<'data, str>,
/// }
///
/// struct MyDataStructMarker;
///
/// impl DynamicDataMarker for MyDataStructMarker {
///     type Yokeable = MyDataStruct<'static>;
/// }
///
/// // We can now use MyDataStruct with DataProvider:
/// let s = MyDataStruct {
///     message: Cow::Owned("Hello World".into()),
/// };
/// let payload = DataPayload::<MyDataStructMarker>::from_owned(s);
/// assert_eq!(payload.get().message, "Hello World");
/// ```
///
/// [`data_struct`]: crate::data_struct
pub trait DynamicDataMarker: 'static {
    /// A type that implements [`Yokeable`]. This should typically be the `'static` version of a
    /// data struct.
    type Yokeable: for<'a> Yokeable<'a>;
}

/// A [`DynamicDataMarker`] with a [`DataKey`] attached.
///
/// Structs implementing this trait are normally generated with the [`data_struct!`] macro.
///
/// Implementing this trait enables this marker to be used with the main [`DataProvider`] trait.
/// Most markers should be associated with a specific key and should therefore implement this
/// trait.
///
/// [`BufferMarker`] and [`AnyMarker`] are examples of markers that do _not_ implement this trait
/// because they are not specific to a single key.
///
/// Note: `DataMarker`s are quasi-const-generic compile-time objects, and as such are expected
/// to be unit structs. As this is not something that can be enforced by the type system, we
/// currently only have a `'static` bound on them (which is needed by a lot of our code).
///
/// [`data_struct!`]: crate::data_struct
/// [`DataProvider`]: crate::DataProvider
/// [`BufferMarker`]: crate::BufferMarker
/// [`AnyMarker`]: crate::AnyMarker
pub trait DataMarker: DynamicDataMarker {
    /// The single [`DataKey`] associated with this marker.
    const KEY: DataKey;

    /// Binds this [`DataMarker`] to a provider supporting it.
    fn bind<P>(provider: P) -> DataProviderWithKey<Self, P>
    where
        P: DataProvider<Self>,
        Self: Sized,
    {
        DataProviderWithKey::new(provider)
    }
}

/// A [`DynamicDataMarker`] that never returns data.
///
/// All types that have non-blanket impls of `DataProvider<M>` are expected to explicitly
/// implement `DataProvider<NeverMarker<Y>>`, returning [`DataErrorKind::MissingDataKey`].
/// See [`impl_data_provider_never_marker!`].
///
/// [`DataErrorKind::MissingDataKey`]: crate::DataErrorKind::MissingDataKey
/// [`impl_data_provider_never_marker!`]: crate::impl_data_provider_never_marker
///
/// # Examples
///
/// ```
/// use icu_locale_core::langid;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider::NeverMarker;
///
/// let buffer_provider = HelloWorldProvider.into_json_provider();
///
/// let result = DataProvider::<NeverMarker<HelloWorldV1<'static>>>::load(
///     &buffer_provider.as_deserializing(),
///     DataRequest {
///         locale: &langid!("en").into(),
///         ..Default::default()
///     },
/// );
///
/// assert!(matches!(
///     result,
///     Err(DataError {
///         kind: DataErrorKind::MissingDataKey,
///         ..
///     })
/// ));
/// ```
#[derive(Debug, Copy, Clone)]
pub struct NeverMarker<Y>(PhantomData<Y>);

impl<Y> DynamicDataMarker for NeverMarker<Y>
where
    for<'a> Y: Yokeable<'a>,
{
    type Yokeable = Y;
}

impl<Y> DataMarker for NeverMarker<Y>
where
    for<'a> Y: Yokeable<'a>,
{
    const KEY: DataKey = data_key!("_never@1");
}

/// Implements `DataProvider<NeverMarker<Y>>` on a struct.
///
/// For more information, see [`NeverMarker`].
///
/// # Examples
///
/// ```
/// use icu_locale_core::langid;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider::NeverMarker;
///
/// struct MyProvider;
///
/// icu_provider::impl_data_provider_never_marker!(MyProvider);
///
/// let result = DataProvider::<NeverMarker<HelloWorldV1<'static>>>::load(
///     &MyProvider,
///     DataRequest {
///         locale: &langid!("und").into(),
///         ..Default::default()
///     },
/// );
///
/// assert!(matches!(
///     result,
///     Err(DataError {
///         kind: DataErrorKind::MissingDataKey,
///         ..
///     })
/// ));
/// ```
#[macro_export]
macro_rules! impl_data_provider_never_marker {
    ($ty:path) => {
        impl<Y> $crate::DataProvider<$crate::NeverMarker<Y>> for $ty
        where
            for<'a> Y: $crate::prelude::yoke::Yokeable<'a>,
        {
            fn load(
                &self,
                req: $crate::DataRequest,
            ) -> Result<$crate::DataResponse<$crate::NeverMarker<Y>>, $crate::DataError> {
                Err($crate::DataErrorKind::MissingDataKey
                    .with_req(<$crate::NeverMarker<Y> as $crate::DataMarker>::KEY, req))
            }
        }
    };
}
