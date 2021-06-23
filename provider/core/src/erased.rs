// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of traits for providers that support type erasure of data structs.

use crate::error::Error;
use crate::prelude::*;
use std::any::Any;
use std::any::TypeId;
use std::rc::Rc;
use yoke::*;

/// Auto-implemented trait allowing for type erasure of data provider structs.
///
/// Requires the static lifetime in order to be convertible to [`Any`].
pub trait ErasedDataStruct: 'static {
    /// Clone this trait object reference, returning a boxed trait object.
    fn clone_into_box(&self) -> Box<dyn ErasedDataStruct>;

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

impl_dyn_clone!(ErasedDataStruct);

impl<'s> ZeroCopyFrom<dyn ErasedDataStruct> for &'static dyn ErasedDataStruct {
    #[allow(clippy::needless_lifetimes)]
    fn zero_copy_from<'b>(this: &'b (dyn ErasedDataStruct)) -> &'b dyn ErasedDataStruct {
        this
    }
}

/// Marker type for [`ErasedDataStruct`].
pub struct ErasedDataStructMarker {}

impl<'s> DataMarker<'s> for ErasedDataStructMarker {
    type Yokeable = &'static dyn ErasedDataStruct;
    type Cart = dyn ErasedDataStruct;
}

impl<'d, M> crate::dynutil::UpcastDataPayload<'d, 'static, M> for ErasedDataStructMarker
where
    M: DataMarker<'static>,
    M::Cart: Sized,
{
    /// Upcast for ErasedDataStruct performs the following mapping of the data payload variants,
    /// where `Y` is the concrete Yokeable and `S` is ErasedDataStruct Yokeable:
    ///
    /// - `Yoke<Y, &'d C>` => `Yoke<S, &'d dyn ErasedDataStruct>`, where the trait object in the
    ///   cart is obtained by calling `.into_backing_cart()` on the input Yoke
    /// - `Yoke<Y, Rc<C>>` => `Yoke<S, Rc<dyn ErasedDataStruct>`, where the trait object in the
    ///   cart is the result of casting the whole input Yoke to `ErasedDataStruct`
    /// - `Yoke<Y, _>` (fully owned) => `Yoke<S, Rc<dyn ErasedDataStruct>>`, by casting the
    ///   whole input Yoke to `ErasedDataStruct` as above
    fn upcast(
        other: DataPayload<'d, 'static, M>,
    ) -> DataPayload<'d, 'static, ErasedDataStructMarker> {
        use crate::data_provider::DataPayloadInner::*;
        match other.inner {
            Borrowed(yoke) => {
                // Case 1: Cast the cart of the Borrowed Yoke to the trait object.
                // TODO(#752): This is not completely sound, because calling `.into_backing_cart()`
                // throws away overrides stored in the Yokeable, such as those from `.with_mut()`.
                let cart: &'d dyn ErasedDataStruct = yoke.into_backing_cart();
                DataPayload::from_borrowed(cart)
            }
            RcStruct(yoke) => {
                // Case 2: Cast the whole RcStruct Yoke to the trait object.
                let cart: Rc<dyn ErasedDataStruct> = Rc::from(yoke);
                DataPayload::from_partial_owned(cart)
            }
            Owned(yoke) => {
                // Case 3: Cast the whole Owned Yoke to the trait object.
                let cart: Rc<dyn ErasedDataStruct> = Rc::from(yoke);
                DataPayload::from_partial_owned(cart)
            }
            RcBuf(yoke) => {
                // Case 4: Cast the whole RcBuf Yoke to the trait object.
                let cart: Rc<dyn ErasedDataStruct> = Rc::from(yoke);
                DataPayload::from_partial_owned(cart)
            }
        }
    }
}

impl<'d> DataPayload<'d, 'static, ErasedDataStructMarker> {
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
    pub fn downcast<M>(self) -> Result<DataPayload<'d, 'static, M>, Error>
    where
        M: DataMarker<'static>,
        M::Cart: Sized,
        M::Yokeable: ZeroCopyFrom<M::Cart>,
    {
        use crate::data_provider::DataPayloadInner::*;
        match self.inner {
            Borrowed(yoke) => {
                // Case 1: The trait object originated from a Borrowed Yoke.
                let any_ref: &dyn Any = yoke.into_backing_cart().as_any();
                let y1 = any_ref.downcast_ref::<M::Cart>();
                match y1 {
                    Some(t_ref) => Ok(DataPayload::from_borrowed(t_ref)),
                    None => Err(Error::MismatchedType {
                        actual: Some(any_ref.type_id()),
                        generic: Some(TypeId::of::<M::Cart>()),
                    }),
                }
            }
            RcStruct(yoke) => {
                let any_rc: Rc<dyn Any> = yoke.into_backing_cart().into_any_rc();
                // `any_rc` is the Yoke that was converted into the `dyn ErasedDataStruct`. It
                // could have been either the RcStruct or the Owned variant of Yoke.
                // Check first for Case 2: an RcStruct Yoke.
                let y1 = any_rc.downcast::<Yoke<M::Yokeable, Rc<M::Cart>>>();
                let any_rc = match y1 {
                    Ok(rc_yoke) => match Rc::try_unwrap(rc_yoke) {
                        Ok(yoke) => {
                            return Ok(DataPayload {
                                inner: RcStruct(yoke),
                            })
                        }
                        // Note: We could consider cloning the Yoke instead of erroring out.
                        Err(_) => return Err(Error::MultipleReferences),
                    },
                    Err(any_rc) => any_rc,
                };
                // Check for Case 3: an Owned Yoke.
                let y2 = any_rc.downcast::<Yoke<M::Yokeable, ()>>();
                let any_rc = match y2 {
                    Ok(rc_yoke) => match Rc::try_unwrap(rc_yoke) {
                        Ok(yoke) => return Ok(DataPayload { inner: Owned(yoke) }),
                        // Note: We could consider cloning the Yoke instead of erroring out.
                        Err(_) => return Err(Error::MultipleReferences),
                    },
                    Err(any_rc) => any_rc,
                };
                // Check for Case 4: an RcBuf Yoke.
                let y2 = any_rc.downcast::<Yoke<M::Yokeable, Rc<[u8]>>>();
                let any_rc = match y2 {
                    Ok(rc_yoke) => match Rc::try_unwrap(rc_yoke) {
                        Ok(yoke) => return Ok(DataPayload { inner: RcBuf(yoke) }),
                        // Note: We could consider cloning the Yoke instead of erroring out.
                        Err(_) => return Err(Error::MultipleReferences),
                    },
                    Err(any_rc) => any_rc,
                };
                // None of the downcasts succeeded; return an error.
                Err(Error::MismatchedType {
                    actual: Some(any_rc.type_id()),
                    generic: Some(TypeId::of::<M::Cart>()),
                })
            }
            // This is unreachable because ErasedDataStruct cannot be fully owned, since it
            // contains a reference.
            Owned(_) => unreachable!(),
            // This is unreachable because ErasedDataStruct needs to reference an object.
            RcBuf(_) => unreachable!(),
        }
    }
}

impl<T> ErasedDataStruct for T
where
    T: Any,
    for<'a> &'a T: Clone,
{
    fn clone_into_box(&self) -> Box<dyn ErasedDataStruct> {
        todo!("#753")
        // Box::new(self.clone())
    }
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
pub trait ErasedDataProvider<'d> {
    /// Query the provider for data, returning the result as an [`ErasedDataStruct`] trait object.
    ///
    /// Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_erased(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 'static, ErasedDataStructMarker>, Error>;
}

// Auto-implement `ErasedDataProvider` on types implementing `DataProvider<dyn ErasedDataStruct>`
impl<'d, T> ErasedDataProvider<'d> for T
where
    T: DataProvider<'d, 'static, ErasedDataStructMarker>,
{
    fn load_erased(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 'static, ErasedDataStructMarker>, Error> {
        DataProvider::<ErasedDataStructMarker>::load_payload(self, req)
    }
}

impl<'d, M> DataProvider<'d, 'static, M> for dyn ErasedDataProvider<'d> + 'd
where
    M: DataMarker<'static>,
    <M::Yokeable as Yokeable<'static>>::Output: Clone + Any,
    M::Yokeable: ZeroCopyFrom<M::Cart>,
    M::Cart: Sized,
{
    /// Serve [`Sized`] objects from an [`ErasedDataProvider`] via downcasting.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 'static, M>, Error> {
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
    use crate::marker::CowStringMarker;
    use std::borrow::Cow;

    #[test]
    fn test_erased_case_1() {
        let data = "foo".to_string();
        let original = DataPayload::<CowStringMarker>::from_borrowed(&data);
        let upcasted = ErasedDataStructMarker::upcast(original);
        let downcasted = upcasted
            .downcast::<CowStringMarker>()
            .expect("Type conversion");
        assert_eq!(downcasted.get(), "foo");
    }

    #[test]
    fn test_erased_case_2() {
        let data = Rc::new("foo".to_string());
        let original = DataPayload::<CowStringMarker>::from_partial_owned(data);
        let upcasted = ErasedDataStructMarker::upcast(original);
        let downcasted = upcasted
            .downcast::<CowStringMarker>()
            .expect("Type conversion");
        assert_eq!(downcasted.get(), "foo");
    }

    #[test]
    fn test_erased_case_3() {
        let data = "foo".to_string();
        let original = DataPayload::<CowStringMarker>::from_owned(Cow::Owned(data));
        let upcasted = ErasedDataStructMarker::upcast(original);
        let downcasted = upcasted
            .downcast::<CowStringMarker>()
            .expect("Type conversion");
        assert_eq!(downcasted.get(), "foo");
    }

    #[test]
    fn test_erased_case_4() {
        let data: Rc<[u8]> = "foo".as_bytes().into();
        let original = DataPayload::<CowStringMarker>::try_from_rc_buffer_badly(data, |bytes| {
            std::str::from_utf8(bytes).map(|s| Cow::Borrowed(s))
        })
        .expect("String is valid UTF-8");
        let upcasted = ErasedDataStructMarker::upcast(original);
        let downcasted = upcasted
            .downcast::<CowStringMarker>()
            .expect("Type conversion");
        assert_eq!(downcasted.get(), "foo");
    }
}
