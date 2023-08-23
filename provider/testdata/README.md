# icu_testdata [![crates.io](https://img.shields.io/crates/v/icu_testdata)](https://crates.io/crates/icu_testdata)

<!-- cargo-rdme start -->

`icu_testdata` is a unit testing crate for [`ICU4X`].

The crate exposes data providers with stable data useful for unit testing. The data is
based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.

The crate exposes three kinds of providers, corresponding to the three types of constructors
in ICU:
* [`unstable`], [`unstable_no_fallback`]
* [`any`], [`any_no_fallback`]
* [`buffer`], [`buffer_no_fallback`] (`buffer` Cargo feature)

## Examples

```rust
use icu::locid::locale;
use icu_provider::hello_world::HelloWorldFormatter;

// Unstable constructor
HelloWorldFormatter::try_new_unstable(
    &icu_testdata::unstable(),
    &locale!("en-CH").into(),
).unwrap();

// AnyProvider constructor
HelloWorldFormatter::try_new_with_any_provider(
    &icu_testdata::any(),
    &locale!("en-CH").into(),
).unwrap();

// BufferProvider constructor (`icu` with `serde` feature, `icu_testdata` with `buffer` feature)
HelloWorldFormatter::try_new_with_buffer_provider(
    &icu_testdata::buffer(),
    &locale!("en-CH").into(),
).unwrap();

// Without fallback the locale match needs to be exact
HelloWorldFormatter::try_new_unstable(
    &icu_testdata::unstable_no_fallback(),
    &locale!("en-CH").into(),
).is_err();

HelloWorldFormatter::try_new_unstable(
    &icu_testdata::unstable_no_fallback(),
    &locale!("en").into(),
).unwrap();
```

[`ICU4X`]: ../icu/index.html

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
