# icu_testdata [![crates.io](http://meritbadge.herokuapp.com/icu_testdata)](https://crates.io/crates/icu_testdata)

`icu_testdata` is a unit testing package for [`ICU4X`].

The package exposes a `DataProvider` with stable data useful for unit testing. The data is
based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.

See README.md for instructions on re-generating the data from CLDR.

## Examples

```rust
use std::borrow::Cow;
use icu_provider::prelude::*;
use icu_locid_macros::langid;

let data_provider = icu_testdata::get_provider();

let data: Cow<icu_plurals::provider::PluralRuleStringsV1> = data_provider
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
    .payload.take()
    .unwrap();
assert_eq!(data.few, Some(Cow::Borrowed("v = 0 and i % 10 = 2..4 and i % 100 != 12..14")));
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
