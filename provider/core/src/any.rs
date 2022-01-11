// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for data providers that produce `Any` objects.

use crate::prelude::*;
use alloc::rc::Rc;
use core::any::Any;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;

/// Representations of the `Any` trait object.
///
/// **Important Note:** The types enclosed by `StructRef` and `PayloadRc` are NOT the same!
/// The first refers to the struct itself, whereas the second refers to a `DataPayload`.
#[derive(Debug, Clone)]
enum AnyPayloadInner {
    /// A reference to `M::Yokeable`
    StructRef(&'static dyn Any),
    /// A boxed `DataPayload<M>`.
    PayloadRc(Rc<dyn Any>),
}

/// A type-erased data payload.
#[derive(Debug, Clone)]
pub struct AnyPayload {
    inner: AnyPayloadInner,
}

impl AnyPayload {
    /// Transforms a type-erased `AnyPayload` into a concrete `DataPayload`.
    pub fn downcast<M>(self) -> Result<DataPayload<M>, DataError>
    where
        M: DataMarker + 'static,
        M::Yokeable: Clone,
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    {
        use AnyPayloadInner::*;
        match self.inner {
            StructRef(any_ref) => {
                let down_ref: &'static M::Yokeable = any_ref.downcast_ref().ok_or_else(|| {
                    DataErrorKind::MismatchedType(any_ref.type_id()).with_type_context::<M>()
                })?;
                // TODO: Use ZeroCopyFrom here.
                // Although the Clone impl is expected to perform a zero-copy clone since we have
                // a reference to a const-constructed object, (1) there is no guarantee that it
                // does not clone, and (2) ZeroCopyFrom only builds the zero-copy code, not the
                // potentially heavier full-clone code.
                Ok(DataPayload::from_owned(down_ref.clone()))
            }
            PayloadRc(any_rc) => {
                let down_rc: Rc<DataPayload<M>> = any_rc.downcast().map_err(|any_rc| {
                    DataErrorKind::MismatchedType(any_rc.type_id()).with_type_context::<M>()
                })?;
                Ok(Rc::try_unwrap(down_rc).unwrap_or_else(|down_rc| (*down_rc).clone()))
            }
        }
    }

    pub fn from_static_ref<Y>(static_ref: &'static Y) -> Self
    where
        Y: for<'a> Yokeable<'a>,
    {
        AnyPayload {
            inner: AnyPayloadInner::StructRef(static_ref),
        }
    }

    pub fn from_rc_payload<M>(rc_payload: Rc<DataPayload<M>>) -> Self
    where
        M: DataMarker + 'static,
    {
        AnyPayload {
            inner: AnyPayloadInner::PayloadRc(rc_payload),
        }
    }
}

impl<M> DataPayload<M>
where
    M: DataMarker + 'static,
{
    pub fn into_any_payload(self) -> AnyPayload {
        AnyPayload {
            inner: AnyPayloadInner::PayloadRc(Rc::from(self)),
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
        M::Yokeable: Clone,
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
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
