// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod capi_datetime;
mod capi_properties;

fn main() {
    capi_datetime::main();
    capi_properties::main();
}
