// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Writeable, assert_writeable_eq, concat::Concat, impl_display_with_writeable};

#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // designed for nesting
pub struct Replace<A, B, C> {
    pub source: A,
    pub needle: B,
    pub replacement: C,
}

impl<A, B, C> Writeable for Replace<A, B, C> where A: Writeable, B: Writeable, C: Writeable {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        todo!()
    }

    fn write_to_parts<S: crate::PartsWrite + ?Sized>(&self, sink: &mut S) -> core::fmt::Result {
        todo!()
    }

    fn writeable_length_hint(&self) -> crate::LengthHint {
        todo!() // is this even possible to do efficiently?
    }
}

impl_display_with_writeable!(Replace<A, B, C>, #[cfg(feature = "alloc")], where A: Writeable, B: Writeable, C: Writeable);

#[test]
fn test_replace() {
    let source = Concat("Hello", " 10 22 1101 33");
    let needle = Concat("1", "0");
    let replacement = Concat("4", "4");

    let replace = Replace { source, needle, replacement };

    assert_writeable_eq!(replace, "Hello 44 22 1441 33");
}
