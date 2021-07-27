// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Marker types and traits for DataProvider.

mod impls;

pub use impls::*;

use crate::yoke::Yokeable;

/// Trait marker for data structs. All types delivered by the data provider must be associated with
/// something implementing this trait.
///
/// By convention, the non-standard `Marker` suffix is used by types implementing DataMarker.
///
/// In addition to a marker type implementing DataMarker, the following impls must also be present
/// for the data struct:
///
/// - `impl<'a> Yokeable<'a>` (required)
/// - `impl ZeroCopyFrom<Cart>` (required for use with some `DataPayload` constructors)
///
/// See also some common pre-made DataMarker impls in this module.
///
/// # Examples
///
/// Implementing DataMarker for a custom type:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::yoke::*;
/// use std::borrow::Cow;
/// use std::rc::Rc;
///
/// #[derive(Yokeable, ZeroCopyFrom)]
/// struct MyDataStruct<'data> {
///     message: Cow<'data, str>,
/// }
///
/// struct MyDataStructMarker;
///
/// impl<'data> DataMarker<'data> for MyDataStructMarker {
///     type Yokeable = MyDataStruct<'static>;
///
///     // Note: the cart could also be just `str` since
///     // MyDataStruct has only one field.
///     type Cart = MyDataStruct<'data>;
/// }
///
/// // We can now use MyDataStruct with DataProvider:
/// let s = Rc::from(MyDataStruct {
///     message: Cow::Borrowed("Hello World")
/// });
/// let payload = DataPayload::<MyDataStructMarker>::from_partial_owned(s);
/// assert_eq!(payload.get().message, "Hello World");
/// ```
pub trait DataMarker<'data> {
    /// A type that implements [`Yokeable`]. This should typically be the `'static` version of a
    /// data struct.
    type Yokeable: for<'a> Yokeable<'a>;

    /// A type that is capable of owning all data necessary for the Yokeable type. This can often
    /// be the `'data` version of the data struct.
    type Cart: 'data + ?Sized;
}
