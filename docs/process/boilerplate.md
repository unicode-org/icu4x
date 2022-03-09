# ICU4X Boilerplate Documentation

This file contains explanations of ICU4X-specific conventions on boilerplate (extra code/annotations) in source files.

## Library annotations

In order to assert that library crates conform to the ICU4X style guide, the following annotation should be present in every *lib.rs* file. It forces the crate to be `no_std` and have no unannotated panicky behavior, except in test mode:

    #![cfg_attr(
        not(test),
        no_std,
        deny(
            clippy::indexing_slicing,
            clippy::unwrap_used,
            clippy::expect_used,
            clippy::panic
        )
    )]

Crates whose panics are not yet annotated may have simply:

    #![cfg_attr(not(test), no_std)]
