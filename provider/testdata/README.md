# icu_testdata [![crates.io](https://img.shields.io/crates/v/icu_testdata)](https://crates.io/crates/icu_testdata)

`icu_testdata` is a unit testing crate for [`ICU4X`].

The crate exposes a data provider with stable data useful for unit testing. The data is
based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.

There are four modes of operation, enabled by features:
* `fs` (default) exposes [`get_json_provider`] with alias [`get_provider`]. In this mode you
  can optionally specify your own test data with the `ICU4X_TESTDATA_DIR` environment variable.
* `static` exposes [`get_postcard_provider`] with alias [`get_provider`] (unless `fs` is
  also enabled).
* `const` exposes [`get_const_provider`] with alias [`get_provider`] (unless `fs` or `static are
  also enabled).
* `metadata` exposes the [`metadata`] module which contains information such as the CLDR Gitref
  and the list of included locales.

## Re-generating the data

### Downloading fresh CLDR data

```bash
$ cargo run --bin --features=bin icu4x-testdata-download-sources
```

### Regenerating JSON and postcard data

```bash
$ cargo run --bin --features=bin icu4x-testdata-datagen
```

## Examples

```rust
use std::borrow::Cow;
use icu_provider::prelude::*;
use icu_locid::locale;

let data_provider = icu_testdata::get_provider();

let data: DataPayload<icu_plurals::provider::CardinalV1Marker> = data_provider
    .load_resource(&DataRequest {
        options: locale!("ru").into(),
        metadata: Default::default(),
    })
    .unwrap()
    .take_payload()
    .unwrap();
let rule = "v = 0 and i % 10 = 2..4 and i % 100 != 12..14".parse()
    .expect("Failed to parse plural rule");
assert_eq!(data.get().few, Some(rule));
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
