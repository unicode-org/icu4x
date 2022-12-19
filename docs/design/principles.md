# ICU4X Design Principles & Decisions

These principles are not cast in stone, but are strong guidelines for developers.

## Internationalization Best Practices

Above all, ICU4X must provide modern, standards-compliant APIs that encourage best internationalization practices and produce correct results for all languages and locales. No language or locale should be at a structural disadvantage.

## Code Style

All ICU4X code must conform to the [style guide](../process/style_guide.md), including the following:

### Safety

*What:* All standard Rust practices regarding mutability, lifetimes and safety must be followed. Unsafe code should be avoided in the core library; if it must be used, it must be reviewed by a Rust expert and have an appropriate safety comment.

*Why:* To promote security for our users.

### No standard library dependencies in the core library

*What:* The core icu4x library (the `icu` crate and all of its direct and indirect dependencies, not including dev-dependencies) should be `#[no_std]`, but may use the `alloc` crate.

*Why:* We run ICU4X in resource-constrained environments that don't have access to a standard library.

### No global caches

*What:* ICU has an internal cache that optimizes construction of some of the objects. We plan to leave optimization to the user, i.e. reusable objects should be kept around for later use, not discarded and recreated again. Memoization is acceptable, as it's not really a global cache, but an object with internal state.  An example implementation used in [fluent-rs](https://github.com/projectfluent/fluent-rs/tree/master/intl-memoizer).

*Why:* Experience from ICU suggests that there is no "one-size-fits-all" solution for runtime caching. Some environments are more memory-constrained, others are more space-constrained, and others are more performance-constrained.

## Stable data across stable library versions

*What:* Older library versions should be able to read newer data, and vice-versa to the extent possible.

*Why:* One of the big problems for existing ICU users is that ICU data cannot be shared among different versions of code, forcing clients to carry hefty duplicates with small deltas.

## Modular Code and Data with static analysis

*What:* Both the code and the data should be written so that you only bring what you need.  Code and data should be modular not only on a "class" level, but also within a class, such that you don't carry code and data for a feature of a class that you aren't using. Code and data slicing should be able to be determined using static code analysis. We should be able to look at the functions being called, and from that, build exactly the code and data bundle that the app needs.

*Why:* We want to leverage dead-code elimination (DCE) as much as possible in order to reduce binary size.

## Available Across Programming Languages

*What:* ICU4X functionality should be available uniformly in all supported programming languages.  The API shape may differ between programming languages, but the underlying functionality should be uniformly available in both Rust and FFI.

*Why:* Portability is a core principle of ICU4X.
