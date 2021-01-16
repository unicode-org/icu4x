// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::fmt;
use writeable::assert_writeable_eq;
use writeable::Writeable;

/// A sample type implementing Writeable
struct WriteableMessage<'s> {
    message: &'s str,
}

impl Writeable for WriteableMessage<'_> {
    fn write_to(&self, sink: &mut dyn fmt::Write) -> fmt::Result {
        sink.write_str(self.message)
    }

    fn write_len(&self) -> usize {
        self.message.len()
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
