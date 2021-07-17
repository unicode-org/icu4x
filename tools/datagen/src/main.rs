// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen` contains command-line tools to generate and process ICU4X data.
//!
//! The tools include:
//!
//! 1. `icu4x-datagen`: Read source data (CLDR JSON) and dump ICU4X-format data.
//! 2. `icu4x-testdata-download`: Download fresh CLDR JSON for testdata.
//!
//! More details on each tool can be found by running `--help`.
//!
//! # Examples
//!
//! Generate ICU4X JSON file tree:
//!
//!```bash
//!# Run from the icu4x project folder
//!$ cargo run --bin icu4x-datagen -- \
//!    --cldr-tag 39.0.0 \
//!    --all-keys \
//!    --all-locales \
//!    --out /tmp/icu4x_data/json \
//!    -v
//!```
//!
//! Generate ICU4X Bincode blob (single file):
//!
//!```bash
//!# Run from the icu4x project folder
//!$ cargo run --bin icu4x-datagen -- \
//!    --cldr-tag 39.0.0 \
//!    --all-keys \
//!    --all-locales \
//!    --format blob \
//!    --out /tmp/icu4x_data/icu4x_data.bincode \
//!    -v
//!```
//!
//! Generate ICU4X Bincode file tree:
//!
//!```bash
//!# Run from the icu4x project folder
//!$ cargo run --bin icu4x-datagen -- \
//!    --cldr-tag 39.0.0 \
//!    --all-keys \
//!    --all-locales \
//!    --syntax bincode \
//!    --out /tmp/icu4x_data/bincode \
//!    -v
//!```

fn main() {
    panic!("Please run a more specific binary")
}
