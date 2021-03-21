// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of traits for providers that support *serializing* data via Serde.
//!
//! See the `crate::serde` mod for APIs involving *deserializing* data via Serde.

use crate::error::Error;
use crate::iter::IterableDataProvider;
use crate::prelude::*;
use std::fmt::Debug;
use std::borrow::Cow;

pub trait SerdeDataStruct<'s>: Debug {
    fn clone_into_box(&self) -> Box<dyn SerdeDataStruct<'s> + 's>;
    fn as_serialize(&self) -> &dyn erased_serde::Serialize;
}

impl<'s> ToOwned for dyn SerdeDataStruct<'s> + 's {
    type Owned = Box<dyn SerdeDataStruct<'s> + 's>;

    fn to_owned(&self) -> Self::Owned {
        self.clone_into_box()
    }
}

impl<'s> Clone for Box<dyn SerdeDataStruct<'s> + 's> {
    fn clone(&self) -> Box<dyn SerdeDataStruct<'s> + 's> {
        self.clone_into_box()
    }
}

impl<'s, T> SerdeDataStruct<'s> for T
where
    T: 's + serde::Serialize + Clone + Debug,
{
    fn clone_into_box(&self) -> Box<dyn SerdeDataStruct<'s> + 's> {
        Box::new(self.clone())
    }
    fn as_serialize(&self) -> &dyn erased_serde::Serialize {
        self
    }
}

impl<'d, 's: 'd, T> DataResponse<'d, T>
where
    T: 's + SerdeDataStruct<'s> + Clone,
{
    /// Convert this DataResponse of a Sized type into a DataResponse of an SerdeDataStruct.
    pub fn into_serde(self) -> DataResponse<'d, dyn SerdeDataStruct<'s> + 's> {
        DataResponse {
            metadata: self.metadata,
            payload: self.payload.map(|p| match p {
                Cow::Borrowed(v) => Cow::Borrowed(v as &dyn SerdeDataStruct),
                Cow::Owned(v) => {
                    let boxed: Box<dyn SerdeDataStruct> = Box::new(v);
                    Cow::Owned(boxed)
                }
            }),
        }
    }
}
