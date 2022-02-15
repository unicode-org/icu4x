// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Marker types and traits for DataProvider.

mod impls;

pub use impls::*;

use crate::yoke::Yokeable;
use crate::ResourceKey;

/// Trait marker for data structs. All types delivered by the data provider must be associated with
/// something implementing this trait.
///
/// By convention, the non-standard `Marker` suffix is used by types implementing DataMarker.
///
/// In addition to a marker type implementing DataMarker, the following impls must also be present
/// for the data struct:
///
/// - `impl<'a> Yokeable<'a>` (required)
/// - `impl ZeroFrom<Self>`
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
///     message: Cow::Owned("Hello World".into())
/// };
/// let payload = DataPayload::<MyDataStructMarker>::from_owned(s);
/// assert_eq!(payload.get().message, "Hello World");
/// ```
pub trait DataMarker {
    /// A type that implements [`Yokeable`]. This should typically be the `'static` version of a
    /// data struct.
    type Yokeable: for<'a> Yokeable<'a>;
}

pub trait ResourceMarker: DataMarker {
    const KEY: ResourceKey;
}
