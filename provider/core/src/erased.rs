// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of traits for providers that support type erasure of data structs.

use crate::error::Error;
use crate::prelude::*;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Cow;
use std::fmt::Debug;

/// Auto-implemented trait allowing for type erasure of data provider structs.
///
/// Requires the static lifetime in order to be convertible to [`Any`].
pub trait ErasedDataStruct: 'static + Debug {
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

impl_dyn_from_payload!(ErasedDataStruct, 'd, 's);

impl dyn ErasedDataStruct {
    /// Convenience function: Return a downcast reference, or an error if mismatched types.
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
    /// let borrowed: &HelloWorldV1 = erased.downcast_ref().expect("Types should match");
    /// ```
    pub fn downcast_ref<T: Any>(&self) -> Result<&T, Error> {
        self.as_any()
            .downcast_ref()
            .ok_or_else(|| Error::MismatchedType {
                actual: Some(self.as_any().type_id()),
                generic: Some(TypeId::of::<T>()),
            })
    }
}

impl<'d> DataPayload<'d, dyn ErasedDataStruct> {
    /// Convert this [`DataPayload`] of an [`ErasedDataStruct`] into a [`DataPayload`] of a [`Sized`] type.
    /// Returns an error if the type is not compatible.
    pub fn downcast<T>(self) -> Result<DataPayload<'d, T>, Error>
    where
        T: Clone + Debug + Any,
    {
        let old_cow = match self.cow {
            Some(cow) => cow,
            None => return Ok(DataPayload { cow: None }),
        };
        let new_cow = match old_cow {
            Cow::Borrowed(erased) => {
                let borrowed: &'d T =
                    erased
                        .as_any()
                        .downcast_ref()
                        .ok_or_else(|| Error::MismatchedType {
                            actual: Some(erased.type_id()),
                            generic: Some(TypeId::of::<T>()),
                        })?;
                Cow::Borrowed(borrowed)
            }
            Cow::Owned(erased) => {
                let boxed: Box<T> =
                    erased
                        .into_any()
                        .downcast()
                        .map_err(|any| Error::MismatchedType {
                            actual: Some(any.type_id()),
                            generic: Some(TypeId::of::<T>()),
                        })?;
                Cow::Owned(*boxed)
            }
        };
        Ok(DataPayload { cow: Some(new_cow) })
    }
}

impl<T> ErasedDataStruct for T
where
    T: Clone + Debug + Any,
{
    fn clone_into_box(&self) -> Box<dyn ErasedDataStruct> {
        Box::new(self.clone())
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
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
    ) -> Result<DataResponse<'d, dyn ErasedDataStruct>, Error>;
}

// Auto-implement `ErasedDataProvider` on types implementing `DataProvider<dyn ErasedDataStruct>`
impl<'d, T> ErasedDataProvider<'d> for T
where
    T: DataProvider<'d, dyn ErasedDataStruct>,
{
    fn load_erased(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, dyn ErasedDataStruct>, Error> {
        DataProvider::<dyn ErasedDataStruct>::load_payload(self, req)
    }
}

impl<'d, T> DataProvider<'d, T> for dyn ErasedDataProvider<'d> + 'd
where
    T: Clone + Debug + Any,
{
    /// Serve [`Sized`] objects from an [`ErasedDataProvider`] via downcasting.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        let result = ErasedDataProvider::load_erased(self, req)?;
        Ok(DataResponse {
            metadata: result.metadata,
            payload: result.payload.downcast()?,
        })
    }
}
