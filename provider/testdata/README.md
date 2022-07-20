# icu_testdata [![crates.io](https://img.shields.io/crates/v/icu_testdata)](https://crates.io/crates/icu_testdata)

`icu_testdata` is a unit testing crate for [`ICU4X`].

The crate exposes a data provider with stable data useful for unit testing. The data is
based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.

There are four modes of operation, enabled by features:
* `static` (default) exposes [`get_postcard_provider`].
* `fs` exposes [`get_json_provider`]
* `baked` exposes [`get_baked_provider`].
* `metadata` exposes the [`metadata`] module which contains information such as the CLDR Gitref
  and the list of included locales.

However, clients should not generally choose a specific provider, but rather use [`get_provider`].
This is currently an alias for [`get_postcard_provider`], as it is fast and has few dependencies.

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
use icu_locid::locale;
use icu_provider::prelude::*;
use std::borrow::Cow;

let data_provider = icu_testdata::get_provider();

let data: DataPayload<icu_plurals::provider::CardinalV1Marker> = data_provider
    .load(&DataRequest {
        options: locale!("ru").into(),
        metadata: Default::default(),
    })
    .unwrap()
    .take_payload()
    .unwrap();
let rule = "v = 0 and i % 10 = 2..4 and i % 100 != 12..14"
    .parse()
    .expect("Failed to parse plural rule");
assert_eq!(data.get().few, Some(rule));
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
