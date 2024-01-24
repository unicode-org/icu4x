// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This file builds a version of icu4x-datagen that includes
//! `cfg(test)` components. To use it, run:`
//!
//! ```bash
//! $ RUSTFLAGS=--cfg=icu4x_test_datagen cargo test --lib -- --help
//! ```

// If no exporter feature is enabled this all doesn't make sense
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use crate as icu_datagen;

mod args {
    include!("../bin/datagen/args.rs");
}
mod config {
    include!("../bin/datagen/config.rs");
}
mod datagen {
    include!("../bin/datagen/datagen.rs");
}

#[no_mangle]
fn main() -> eyre::Result<()> {
    datagen::main()
}
