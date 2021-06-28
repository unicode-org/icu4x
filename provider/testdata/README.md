# icu_testdata [![crates.io](http://meritbadge.herokuapp.com/icu_testdata)](https://crates.io/crates/icu_testdata)

`icu_testdata` is a unit testing package for [`ICU4X`].

The package exposes a `DataProvider` with stable data useful for unit testing. The data is
based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.

The list of locales and the current CLDR tag can be found in [Cargo.toml](./Cargo.toml).

The output data can be found in the [data](./data/) subdirectory. There, you will find:

- `json` for the ICU4X JSON test data
- `cldr` for the source CLDR JSON

### Pointing to custom test data

If you wish to run ICU4X tests with custom test data, you may do so by setting the "ICU4X_TESTDATA_DIR" environment variable:

```bash
$ ICU4X_TESTDATA_DIR=/path/to/custom/testdata cargo test
```

### Re-generating the data

From the top level directory of the `icu4x` metapackage, run:

```bash
$ cargo make testdata
```

The following commands are also available:

- `cargo make testdata-download` downloads fresh CLDR JSON without overwriting ICU4X JSON
- `cargo make testdata-build-json` re-generates the ICU4X JSON
- `cargo make bincode-gen-testdata` generates Bincode testdata

## Examples

```rust
use std::borrow::Cow;
use icu_provider::prelude::*;
use icu_locid_macros::langid;

let data_provider = icu_testdata::get_provider();

let data: DataPayload<icu_plurals::provider::PluralRuleStringsV1Marker> = data_provider
    .load_payload(&DataRequest {
        resource_path: ResourcePath {
            key: icu_plurals::provider::key::CARDINAL_V1,
            options: ResourceOptions {
                langid: Some(langid!("ru")),
                variant: None,
            },
        },
    })
    .unwrap()
    .take_payload()
    .unwrap();
assert_eq!(data.get().few, Some(Cow::Borrowed("v = 0 and i % 10 = 2..4 and i % 100 != 12..14")));
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
