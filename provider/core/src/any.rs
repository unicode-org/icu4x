// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for data providers that produce `Any` objects.

use crate::prelude::*;
use alloc::rc::Rc;
use core::any::Any;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;
use yoke::ZeroCopyFrom;

/// Representations of the `Any` trait object.
///
/// **Important Note:** The types enclosed by `StructRef` and `PayloadRc` are NOT the same!
/// The first refers to the struct itself, whereas the second refers to a `DataPayload`.
#[derive(Debug, Clone)]
enum AnyPayloadInner {
    /// A reference to `M::Yokeable`
    StructRef(&'static dyn Any),
    /// A boxed `DataPayload<M>`.
    ///
    /// Note: This needs to be an `Rc`, not a `Box`, so that `AnyPayload` is cloneable.
    /// If an `AnyPayload` is cloned, the actual cloning of the data is delayed until
    /// `downcast()` is invoked (at which point we have the concrete type).
    PayloadRc(Rc<dyn Any>),
}

/// A type-erased data payload.
///
/// The only useful method on this type is [`AnyPayload::downcast()`], which transforms this into
/// a normal `DataPayload` which you can subsequently access or mutate.
///
/// As with `DataPayload`, cloning is designed to be cheap.
#[derive(Debug, Clone)]
pub struct AnyPayload {
    inner: AnyPayloadInner,
    type_name: &'static str,
}

impl AnyPayload {
    /// Transforms a type-erased `AnyPayload` into a concrete `DataPayload`.
    ///
    /// Because it is expected that the call site knows the identity of the AnyPayload (e.g., from
    /// the data request), this function returns a `DataError` if the generic type does not match
    /// the type stored in the `AnyPayload`.
    pub fn downcast<M>(self) -> Result<DataPayload<M>, DataError>
    where
        M: DataMarker + 'static,
        // For the StructRef case:
        M::Yokeable: ZeroCopyFrom<'static, M::Yokeable>,
        // For the PayloadRc case:
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    {
        use AnyPayloadInner::*;
        let type_name = self.type_name;
        match self.inner {
            StructRef(any_ref) => {
                let down_ref: &'static M::Yokeable = any_ref.downcast_ref().ok_or_else(|| {
                    DataError::for_type::<M>().with_str_context(type_name)
                })?;
                Ok(DataPayload::from_owned(M::Yokeable::zero_copy_from(
                    down_ref,
                )))
            }
            PayloadRc(any_rc) => {
                let down_rc: Rc<DataPayload<M>> = any_rc.downcast().map_err(|_| {
                    DataError::for_type::<M>().with_str_context(type_name)
                })?;
                Ok(Rc::try_unwrap(down_rc).unwrap_or_else(|down_rc| (*down_rc).clone()))
            }
        }
    }

    /// Creates an `AnyPayload` from a static reference to a data struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::borrow::Cow;
    ///
    /// const HELLO_DATA: HelloWorldV1<'static> = HelloWorldV1 {
    ///     message: Cow::Borrowed("Custom Hello World")
    /// };
    ///
    /// let any_payload = AnyPayload::from_static_ref(&HELLO_DATA);
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = any_payload.downcast()
    ///     .expect("TypeId matches");
    /// assert_eq!("Custom Hello World", payload.get().message);
    /// ```
    pub fn from_static_ref<Y>(static_ref: &'static Y) -> Self
    where
        Y: for<'a> Yokeable<'a>,
    {
        AnyPayload {
            inner: AnyPayloadInner::StructRef(static_ref),
            // Note: This records the Yokeable type rather than the DataMarker type,
            // but that is okay since this is only for debugging
            type_name: core::any::type_name::<Y>(),
        }
    }

    /// Creates an `AnyPayload` from a [`DataPayload`] stored in an [`Rc`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::borrow::Cow;
    /// use std::rc::Rc;
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> =
    ///     DataPayload::from_owned(HelloWorldV1 {
    ///         message: Cow::Borrowed("Custom Hello World")
    ///     });
    /// let rc_payload = Rc::from(payload);
    ///
    /// let any_payload = AnyPayload::from_rc_payload(rc_payload);
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = any_payload.downcast()
    ///     .expect("TypeId matches");
    /// assert_eq!("Custom Hello World", payload.get().message);
    /// ```
    pub fn from_rc_payload<M>(rc_payload: Rc<DataPayload<M>>) -> Self
    where
        M: DataMarker + 'static,
    {
        AnyPayload {
            inner: AnyPayloadInner::PayloadRc(rc_payload),
            type_name: core::any::type_name::<M>(),
        }
    }
}

impl<M> DataPayload<M>
where
    M: DataMarker + 'static,
{
    /// Wraps this DataPayload in an `Rc` and returns it as an `AnyPayload`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::borrow::Cow;
    /// use std::rc::Rc;
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> =
    ///     DataPayload::from_owned(HelloWorldV1 {
    ///         message: Cow::Borrowed("Custom Hello World")
    ///     });
    ///
    /// let any_payload = payload.into_any_payload();
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = any_payload.downcast()
    ///     .expect("TypeId matches");
    /// assert_eq!("Custom Hello World", payload.get().message);
    /// ```
    pub fn into_any_payload(self) -> AnyPayload {
        AnyPayload {
            inner: AnyPayloadInner::PayloadRc(Rc::from(self)),
            type_name: core::any::type_name::<M>(),
        }
    }
}

/// A response object containing an object as payload and metadata about it.
#[derive(Debug, Clone)]
pub struct AnyResponse {
    /// Metadata about the returned object.
    pub metadata: DataResponseMetadata,

    /// The object itself; None if it was not loaded.
    pub payload: Option<AnyPayload>,
}

impl AnyResponse {
    /// Transforms a type-erased `AnyResponse` into a concrete `DataResponse`.
    pub fn downcast<M>(self) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker + 'static,
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
        M::Yokeable: ZeroCopyFrom<'static, M::Yokeable>,
    {
        Ok(DataResponse {
            metadata: self.metadata,
            payload: self.payload.map(|p| p.downcast()).transpose()?,
        })
    }
}

/// An object-safe data provider that returns Rust objects cast to `dyn Any` trait objects.
pub trait AnyProvider {
    fn load_any(&self, req: &DataRequest) -> Result<AnyResponse, DataError>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hello_world::*;
    use crate::marker::CowStrMarker;
    use alloc::borrow::Cow;

    #[test]
    fn test_debug() {
        let payload: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
            message: Cow::Borrowed("Custom Hello World"),
        });

        let any_payload = payload.into_any_payload();
        assert_eq!(
            "AnyPayload { inner: PayloadRc(Any { .. }), type_name: \"icu_provider::hello_world::HelloWorldV1Marker\" }",
            format!("{:?}", any_payload)
        );

        let err = any_payload.downcast::<CowStrMarker>().unwrap_err();
        assert_eq!(
            "ICU4X data error: Mismatched types: tried to downcast with icu_provider::marker::impls::CowStrMarker, but actual type is different: icu_provider::hello_world::HelloWorldV1Marker",
            format!("{}", err)
        );
    }
}
