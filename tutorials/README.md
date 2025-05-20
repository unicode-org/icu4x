
# ICU4X Tutorials

> [!WARNING]  
> These tutorials work with code in this Git commit, which might not be a published release. Verify that you are viewing a release branch.

Welcome! We're glad you want to try out ICU4X! This page serves as a landing page for people looking to perform various tasks with ICU4X.

If new to ICU4X, we recommend reading through [the introduction tutorial](quickstart.md): it walks through the process of using ICU4X as a Rust dependency, and some of the basics common to most ICU4X components.

It leads in to the [Data management tutorial](data-management.md), which covers how internationalization data can be generated and loaded into ICU4X. Users needing more control over their flow of locale data can then read [the data provider tutorial](data-provider-runtime.md).

After going through that, you can take a look at [the ICU4X root docs][icu-crate-docs] and check out the various components, each of which covers some area of internationalization and has usage docs for doing so.

For help setting up your Cargo.toml file to pull in everything you need, see [the ICU4X Cargo example](../examples/cargo/).

If you intend to use ICU4X from other languages, check out our examples for [C++](../examples/cpp) and [JS](../examples/npm). It is recommended one read through the main tutorial first to understand the core concepts; the C++ and JS APIs are rather similar to the Rust ones.

We're happy to answer any questions on our [discussions forum]!

Contributors should also check out [CONTRIBUTING.md](../../CONTRIBUTING.md).

 [discussions forum]: https://github.com/unicode-org/icu4x/discussions
 [icu-crate-docs]: https://docs.rs/icu/latest/icu/
