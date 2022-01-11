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
enum AnyPayloadInner {
    /// A reference to `M::Yokeable`
    StructRef(&'static dyn Any),
    /// A boxed `DataPayload<M>`.
    PayloadRc(Rc<dyn Any>),
}

/// A type-erased data payload.
pub struct AnyPayload {
    inner: AnyPayloadInner,
}

impl AnyPayload {
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
                // TODO: ZCC
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
}
