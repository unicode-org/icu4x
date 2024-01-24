// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// If no exporter feature is enabled this all doesn't make sense
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

mod args;
mod config;
mod datagen;

// Needed for datagen.rs to used with `include!`
#[allow(clippy::single_component_path_imports)]
use icu_datagen;

pub fn main() -> eyre::Result<()> {
    datagen::main()
}
