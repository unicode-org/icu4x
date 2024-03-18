// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[doc = include_str!("../../../README.md")]
mod readme {}

mod tutorials {
    #[doc = include_str!("../../../tutorials/intro.md")]
    mod intro_md {}
    #[doc = include_str!("../../../tutorials/intro_interactive.md")]
    mod intro_interactive_md {}
    #[doc = include_str!("../../../tutorials/data_provider.md")]
    mod data_provider_md {}
    #[doc = include_str!("../../../tutorials/data_management.md")]
    mod data_management_md {}
    #[doc = include_str!("../../../tutorials/js.md")]
    mod js_md {}
}

mod documents {
    #[doc = include_str!("../../../documents/process/writing_a_new_data_struct.md")]
    mod writing_a_new_data_struct_md {}
    #[doc = include_str!("../../../documents/process/markdown_tips.md")]
    mod markdown_tips_md {}
    #[doc = include_str!("../../../documents/design/data_safety.md")]
    mod data_safety_md {}
}
