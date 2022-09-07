# icu_testdata [![crates.io](https://img.shields.io/crates/v/icu_testdata)](https://crates.io/crates/icu_testdata)

`icu_testdata` is a unit testing crate for [`ICU4X`].

The crate exposes data providers with stable data useful for unit testing. The data is
based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.

The crate exposes three kinds of providers, corresponding to the three types of constructors
in ICU:
* [`unstable`], [`unstable_no_fallback`]
* [`any`], [`any_no_fallback`]
* [`buffer`], [`buffer_no_fallback`], [`small_buffer`] (`buffer` feature)


Additionally, the `metadata` feature exposes the [`metadata`] module which contains information
such as the CLDR Gitref  and the list of included locales.

## `bin` feature

### Downloading fresh CLDR data

```bash
$ cargo run --bin --features=bin icu4x-testdata-download-sources
```

### Regenerating data

```bash
$ cargo run --bin --features=bin icu4x-testdata-datagen
```

## Examples

```rust
use icu_locid::locale;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;

let req = DataRequest {
    locale: &locale!("en").into(),
    metadata: Default::default(),
};

assert_eq!(
    DataProvider::<HelloWorldV1Marker>::load(
        &icu_testdata::unstable(),
        req
    )
    .and_then(DataResponse::take_payload)
    .unwrap()
    .get()
    .message,
    "Hello World"
);

assert_eq!(
    BufferProvider::load_buffer(
        &icu_testdata::buffer(),
        HelloWorldV1Marker::KEY,
        req
    )
    .and_then(DataResponse::take_payload)
    .unwrap()
    .get(),
    &b"\x0bHello World"
);

assert_eq!(
    AnyProvider::load_any(
        &icu_testdata::any(),
        HelloWorldV1Marker::KEY,
        req
    )
    .and_then(AnyResponse::downcast::<HelloWorldV1Marker>)
    .and_then(DataResponse::take_payload)
    .unwrap()
    .get()
    .message,
    "Hello World"
);
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
