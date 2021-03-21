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

pub trait SerdeDataStruct<'s>: Debug {
    fn clone_into_box(&self) -> Box<dyn SerdeDataStruct<'s>>;
    fn as_serialize(&self) -> &dyn erased_serde::Serialize;
}

impl<'s> ToOwned for dyn SerdeDataStruct<'s> {
    type Owned = Box<dyn SerdeDataStruct<'s>>;

    fn to_owned(&self) -> Self::Owned {
        self.clone_into_box()
    }
}

impl<'s> Clone for Box<dyn SerdeDataStruct<'s>> {
    fn clone(&self) -> Box<dyn SerdeDataStruct<'s>> {
        self.clone_into_box()
    }
}
