// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::Field;
use crate::skeleton::reference;
use alloc::vec::Vec;
use core::fmt::{self, Write};
use zerovec::ZeroVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Skeleton<'data>(pub(crate) ZeroVec<'data, Field>);

impl<'data> Skeleton<'data> {
    pub(crate) fn fields_iter(&self) -> impl Iterator<Item = Field> + '_ {
        self.0.iter()
    }
}

impl From<reference::Skeleton> for Skeleton<'_> {
    fn from(input: reference::Skeleton) -> Self {
        let fields = input.fields_iter().copied().collect::<Vec<_>>();
        Self(ZeroVec::alloc_from_slice(&fields))
    }
}

impl<'data> From<ZeroVec<'data, Field>> for Skeleton<'data> {
    fn from(input: ZeroVec<'data, Field>) -> Self {
        Self(input)
    }
}

impl fmt::Display for Skeleton<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for field in self.fields_iter() {
            let ch: char = field.symbol.into();
            for _ in 0..field.length.to_len() {
                formatter.write_char(ch)?;
            }
        }
        Ok(())
    }
}
