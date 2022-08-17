# ICU4X Boilerplate Documentation

This file contains explanations of ICU4X-specific conventions on boilerplate (extra code/annotations) in source files.

## Library annotations

In order to assert that library crates conform to the ICU4X style guide, the following annotation should be present in every *lib.rs* file. The annotations do the following:

1. Set the crate to be `no_std`
2. Enforce no unannotated panicky behavior, except in test mode
3. Require every exported method to be documented, implement `Debug`, and be `non_exhaustive` or annotated as exhaustive

If the crate has no `std` feature:

    // https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
    #![cfg_attr(not(test), no_std)]
    #![cfg_attr(
        not(test),
        deny(
            clippy::indexing_slicing,
            clippy::unwrap_used,
            clippy::expect_used,
            clippy::panic,
            clippy::exhaustive_structs,
            clippy::exhaustive_enums,
            missing_debug_implementations,
        )
    )]
    #![warn(missing_docs)]

Not all crates are yet able to be annotated in this way. Annotations may be omitted and added when able.

If the crate has an `std` feature, specify this in the first line:

    // https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
    #![cfg_attr(not(any(test, feature = "std")), no_std)]
    #![cfg_attr(
        not(test),
        deny(
            clippy::indexing_slicing,
            clippy::unwrap_used,
            clippy::expect_used,
            clippy::panic,
            clippy::exhaustive_structs,
            clippy::exhaustive_enums,
            missing_debug_implementations,
        )
    )]
    #![warn(missing_docs)]
