// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Marker types and traits for DataProvider.

use crate::key::DataKey;
use crate::yoke::Yokeable;

/// Trait marker for data structs. All types delivered by the data provider must be associated with
/// something implementing this trait.
///
/// Structs implementing this trait are normally generated with the [`data_struct`] macro.
///
/// By convention, the non-standard `Marker` suffix is used by types implementing DataMarker.
///
/// In addition to a marker type implementing DataMarker, the following impls must also be present
/// for the data struct:
///
/// - `impl<'a> Yokeable<'a>` (required)
/// - `impl ZeroFrom<Self>`
///
/// Also see [`KeyedDataMarker`].
///
/// # Examples
///
/// Manually implementing DataMarker for a custom type:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::yoke::*;
/// use icu_provider::zerofrom::*;
/// use std::borrow::Cow;
/// use std::rc::Rc;
///
/// #[derive(Yokeable, ZeroFrom)]
/// struct MyDataStruct<'data> {
///     message: Cow<'data, str>,
/// }
///
/// struct MyDataStructMarker;
///
/// impl DataMarker for MyDataStructMarker {
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
pub trait DataMarker {
    /// A type that implements [`Yokeable`]. This should typically be the `'static` version of a
    /// data struct.
    type Yokeable: for<'a> Yokeable<'a>;
}

/// A [`DataMarker`] with a [`DataKey`] attached.
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
/// [`data_struct!`]: crate::data_struct
/// [`DataProvider`]: crate::DataProvider
/// [`BufferMarker`]: crate::BufferMarker
/// [`AnyMarker`]: crate::AnyMarker
pub trait KeyedDataMarker: DataMarker {
    /// The single [`DataKey`] associated with this marker.
    const KEY: DataKey;
}
