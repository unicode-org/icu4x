// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This example illustrates a very simple type implementing Writeable.
icu_benchmark_macros::static_setup!();

use std::fmt;
use writeable::LengthHint;
use writeable::Writeable;

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

fn main() {
    icu_benchmark_macros::main_setup!();

    let writeable = WriteableMessage {
        message: "hello world",
    };
    assert_eq!("hello world", writeable.writeable_to_string());
}
