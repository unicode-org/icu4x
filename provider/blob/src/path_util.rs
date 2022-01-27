// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use icu_provider::prelude::*;
use writeable::Writeable;

pub fn resource_path_to_string(key: ResourceKey, options: &ResourceOptions) -> String {
    let mut output =
        String::with_capacity(key.write_len().capacity() + options.write_len().capacity() + 2);
    output.push('/');
    key.write_to(&mut output)
        .expect("impl Write for String is infallible");
    output.push('/');
    options
        .write_to(&mut output)
        .expect("impl Write for String is infallible");
    output
}
