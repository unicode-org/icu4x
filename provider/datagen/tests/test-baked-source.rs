// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

#[path = "testutil.rs"]
mod testutil;

struct Baked;

const _: () = {
    #[allow(unused_imports)]
    #[path = "data/baked/mod.rs"]
    mod baked_data;
    mod icu {
        pub use icu_datetime as datetime;
    }
    baked_data::make_provider!(Baked);
    baked_data::impl_datetime_patterns_gregory_skeleton_v1!(Baked);
    baked_data::impliterable_datetime_patterns_gregory_skeleton_v1!(Baked);
};
