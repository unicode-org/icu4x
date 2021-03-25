// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt;
use writeable::assert_writeable_eq;
use writeable::LengthHint;
use writeable::Writeable;

/// A sample type implementing Writeable
struct WriteableMessage<'s> {
    message: &'s str,
}

impl Writeable for WriteableMessage<'_> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str(self.message)
    }

    fn write_len(&self) -> LengthHint {
        LengthHint::Exact(self.message.len())
    }
}

#[test]
fn test_basic() {
    let input_string = "hello world";
    let message = WriteableMessage {
        message: input_string,
    };
    assert_writeable_eq!(input_string, &message);
}
