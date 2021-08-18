// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use icu_provider::prelude::*;
use writeable::Writeable;

pub fn resource_path_to_string(resource_path: &ResourcePath) -> String {
    let mut output = String::with_capacity(resource_path.write_len().capacity() + 1);
    output.push('/');
    resource_path
        .write_to(&mut output)
        .expect("impl Write for String is infallible");
    output
}
