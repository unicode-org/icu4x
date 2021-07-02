// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Yoke;
use crate::Yokeable;

impl<'s, Y, C> serde::Serialize for Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    for<'a> &'a <Y as Yokeable<'a>>::Output: serde::Serialize,
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}
