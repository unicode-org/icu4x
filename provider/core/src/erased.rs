// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of traits for providers that support type erasure of data structs.

use crate::prelude::*;
use crate::yoke::*;
use alloc::boxed::Box;
use alloc::rc::Rc;

use core::any::Any;

/// Auto-implemented trait allowing for type erasure of data provider structs.
///
/// Requires the static lifetime in order to be convertible to [`Any`].
pub trait ErasedDataStruct: 'static {
    /// Return this boxed trait object as [`Box`]`<dyn `[`Any`]`>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::hello_world::HelloWorldV1;
    ///
    /// // Create type-erased box
    /// let erased: Box<dyn ErasedDataStruct> = Box::new(HelloWorldV1::default());
    ///
    /// // Convert to typed box
    /// let boxed: Box<HelloWorldV1> = erased.into_any().downcast().expect("Types should match");
    /// ```
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Any>;

    /// Return this trait object reference as `&dyn `[`Any`].
    ///
    /// Also see associated method [`downcast_ref()`](trait.ErasedDataStruct.html#method.downcast_ref).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::hello_world::HelloWorldV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloWorldV1::default();
    /// let erased: &dyn ErasedDataStruct = &data;
    ///
    /// // Borrow as typed reference
    /// let borrowed: &HelloWorldV1 = erased.as_any().downcast_ref().expect("Types should match");
    /// ```
    fn as_any(&self) -> &dyn Any;
}

impl ZeroCopyFrom<dyn ErasedDataStruct> for &'static dyn ErasedDataStruct {
    #[allow(clippy::needless_lifetimes)]
    fn zero_copy_from<'b>(this: &'b (dyn ErasedDataStruct)) -> &'b dyn ErasedDataStruct {
        this
    }
}

/// Marker type for [`ErasedDataStruct`].
pub struct ErasedDataStructMarker {}

impl DataMarker for ErasedDataStructMarker {
    type Yokeable = ErasedDataStructBox;
}

#[derive(Yokeable)]
pub struct ErasedDataStructBox(Box<dyn ErasedDataStruct>);

impl<M> crate::dynutil::UpcastDataPayload<M> for ErasedDataStructMarker
where
    M: DataMarker,
{
    /// Upcast for ErasedDataStruct creates a `Box<dyn ErasedDataStruct>` from the current inner
    /// `Yoke` (i.e., `Box::new(yoke)`).
    fn upcast(other: DataPayload<M>) -> DataPayload<ErasedDataStructMarker> {
        let owned: Box<dyn ErasedDataStruct> = Box::new(other.yoke);
        DataPayload::from_owned(ErasedDataStructBox(owned))
    }
}

impl DataPayload<ErasedDataStructMarker> {
    /// Convert this [`DataPayload`] of an [`ErasedDataStruct`] into a [`DataPayload`] of a
    /// concrete type.
    ///
    /// Returns an error if the type is not compatible.
    ///
    /// This is the main way to consume data returned from an [`ErasedDataProvider`].
    ///
    /// Internally, this method reverses the transformation performed by
    /// [`UpcastDataPayload::upcast`](crate::dynutil::UpcastDataPayload::upcast) as implemented
    /// for [`ErasedDataStructMarker`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::*;
    /// use icu_provider::hello_world::*;
    /// use icu_locid_macros::langid;
    ///
    /// let provider = HelloWorldProvider::new_with_placeholder_data();
    ///
    /// let erased_payload: DataPayload<ErasedDataStructMarker> = provider
    ///     .load_payload(&DataRequest {
    ///         resource_path: ResourcePath {
    ///             key: key::HELLO_WORLD_V1,
    ///             options: ResourceOptions {
    ///                 variant: None,
    ///                 langid: Some(langid!("de")),
    ///             }
    ///         }
    ///     })
    ///     .expect("Loading should succeed")
    ///     .take_payload()
    ///     .expect("Data should be present");
    ///
    /// let downcast_payload: DataPayload<HelloWorldV1Marker> = erased_payload
    ///     .downcast()
    ///     .expect("Types should match");
    ///
    /// assert_eq!("Hallo Welt", downcast_payload.get().message);
    /// ```
    pub fn downcast<M>(self) -> Result<DataPayload<M>, DataError>
    where
        M: DataMarker,
    {
        // It is impossible for clients to construct an ErasedDataStruct payload manually
        // since ErasedDataStructBox has a private field.
        let any_box: Box<dyn Any> = self
            .yoke
            .try_into_yokeable()
            .ok()
            .expect("An ErasedDataStruct payload can only be constructed as fully owned")
            .0
            .into_any();
        // `any_box` is the Yoke that was converted into the `dyn ErasedDataStruct`.
        let maybe_yoke = any_box.downcast::<Yoke<M::Yokeable, Option<Rc<[u8]>>>>();
        match maybe_yoke {
            Ok(yoke) => Ok(DataPayload { yoke: *yoke }),
            Err(any_box) => {
                Err(DataErrorKind::MismatchedType(any_box.type_id()).with_type_context::<M>())
            }
        }
    }
}

impl<T> ErasedDataStruct for T
where
    T: Any,
    for<'a> &'a T: Clone,
{
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A type-erased data provider that loads a payload of types implementing [`Any`].
///
/// Note: This trait is redundant with [`DataProvider`]`<dyn `[`ErasedDataStruct`]`>` and auto-implemented
/// for all types implementing that trait. This trait may eventually be removed when the following
/// Rust issues are resolved:
///
/// - [#41517](https://github.com/rust-lang/rust/issues/41517) (trait aliases are not supported)
/// - [#68636](https://github.com/rust-lang/rust/issues/68636) (identical traits can't be auto-implemented)
pub trait ErasedDataProvider {
    /// Query the provider for data, returning the result as an [`ErasedDataStruct`] trait object.
    ///
    /// Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns a
    /// DataError with more information.
    fn load_erased(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<ErasedDataStructMarker>, DataError>;
}

// Auto-implement `ErasedDataProvider` on types implementing `DataProvider<dyn ErasedDataStruct>`
impl<T> ErasedDataProvider for T
where
    T: DataProvider<ErasedDataStructMarker>,
{
    fn load_erased(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<ErasedDataStructMarker>, DataError> {
        DataProvider::<ErasedDataStructMarker>::load_payload(self, req)
    }
}

impl<M> DataProvider<M> for dyn ErasedDataProvider
where
    M: DataMarker,
    <M::Yokeable as Yokeable<'static>>::Output: Clone + Any,
{
    /// Serve [`Sized`] objects from an [`ErasedDataProvider`] via downcasting.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let result = ErasedDataProvider::load_erased(self, req)?;
        Ok(DataResponse {
            metadata: result.metadata,
            payload: result.payload.map(|p| p.downcast()).transpose()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dynutil::UpcastDataPayload;
    use crate::marker::CowStrMarker;
    use alloc::borrow::Cow;

    #[test]
    fn test_erased_case_3() {
        let data = "foo".to_string();
        let original = DataPayload::<CowStrMarker>::from_owned(Cow::Owned(data));
        let upcasted = ErasedDataStructMarker::upcast(original);
        let downcasted = upcasted
            .downcast::<CowStrMarker>()
            .expect("Type conversion");
        assert_eq!(downcasted.get(), "foo");
    }

    #[test]
    fn test_erased_case_4() {
        let data: Rc<[u8]> = "foo".as_bytes().into();
        let original = DataPayload::<CowStrMarker>::try_from_rc_buffer_badly(data, |bytes| {
            core::str::from_utf8(bytes).map(|s| Cow::Borrowed(s))
        })
        .expect("String is valid UTF-8");
        let upcasted = ErasedDataStructMarker::upcast(original);
        let downcasted = upcasted
            .downcast::<CowStrMarker>()
            .expect("Type conversion");
        assert_eq!(downcasted.get(), "foo");
    }
}
