// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::Field;
use crate::skeleton::reference;
use alloc::vec::Vec;
use zerovec::ZeroVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Skeleton<'data>(ZeroVec<'data, Field>);

impl<'data> Skeleton<'data> {
    pub(crate) fn fields_iter<'a>(&'a self) -> impl Iterator<Item = Field> + 'a {
        self.0.iter()
    }

    pub(crate) fn fields_len(&self) -> usize {
        self.0.len()
    }
}

impl From<reference::Skeleton> for Skeleton<'_> {
    fn from(input: reference::Skeleton) -> Self {
        let fields = input.fields_iter().copied().collect::<Vec<_>>();
        Self(ZeroVec::clone_from_slice(&fields))
    }
}
