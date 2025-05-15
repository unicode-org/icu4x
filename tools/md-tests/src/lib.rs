// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::needless_doctest_main)]

#[doc = include_str!("../../../README.md")]
mod readme {}

mod tutorials {
    #[doc = include_str!("../../../tutorials/quickstart.md")]
    mod quickstart_md {}
    #[doc = include_str!("../../../tutorials/data-packs.md")]
    mod data_packs_md {}
    #[doc = include_str!("../../../tutorials/data-provider-runtime.md")]
    mod data_provider_runtime_md {}
    #[doc = include_str!("../../../tutorials/data-slimming.md")]
    mod data_slimming_md {}
}

mod documents {
    #[doc = include_str!("../../../documents/process/writing_a_new_data_struct.md")]
    mod writing_a_new_data_struct_md {}
    #[doc = include_str!("../../../documents/process/markdown_tips.md")]
    mod markdown_tips_md {}
    #[doc = include_str!("../../../documents/design/data_safety.md")]
    mod data_safety_md {}
}
