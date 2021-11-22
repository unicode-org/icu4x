# ICU4X Design Principles & Decisions

These principles are not cast in stone, but are strong guidelines for developers.

## i18n Best Practices

Above all, ICU4X must provide modern, standards-compliant APIs that encourage best i18n practices and produce correct results for all languages and locales. No language or locale should be at a structural disadvantage.

## Code Style

All ICU4X code must conform to the [style guide](../process/style_guide.md), including the following:

### Safety

All standard Rust practices regarding mutability, lifetimes and safety must be followed.

### No standard library dependencies in the core library

The core icu4x library (the `icu` and `icu_capi` crates and all of their direct and indirect dependencies) should be `#[no_std]`, but may use the `alloc` crate.

### No internal threading

Both Rust and Wasm support multithreading but we don’t have a need for it in the i18n realm.

To simplify our library, and make sure we don’t have cross platform/language compatibility issues, one should avoid using threads in the core library.

*NOTE*: In rare cases where threading is needed in native Rust implementation, multiple code paths can be created/opted in at build time.

### No global mutable data

ICU has an internal cache that optimizes construction of some of the objects. We plan to leave optimization to the user, i.e. reusable objects should be kept around for later use, not discarded and recreated again.

*NOTE*: Memoization is acceptable, as it's not really a global cache, but an object with internal state.  An example implementation used in [fluent-rs](https://github.com/projectfluent/fluent-rs/tree/master/intl-memoizer).

## Stable data across stable library versions

Older library versions should be able to read newer data, and vice-versa to the extent possible. (One of the big problems for existing ICU users is that ICU data cannot be shared among different versions of code, forcing clients to carry hefty duplicates with small deltas.)

## Modular Code and Data with static analysis

Both the code and the data should be written so that you only bring what you need.  Code and data should be modular not only on a "class" level, but also within a class, such that you don't carry code and data for a feature of a class that you aren't using.

Code and data slicing should be able to be determined using static code analysis.  We should be able to look at the functions being called, and from that, build exactly the code and data bundle that the app needs.

## Available Across Programming Languages

ICU4X functionality should be available uniformly in all supported programming languages.  The API shape may differ between programming languages, but the underlying functionality should be uniformly available; it should be an exception for functionality to be available in Rust but not in FFI.
