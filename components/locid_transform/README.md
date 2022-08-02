# icu_locid_transform [![crates.io](https://img.shields.io/crates/v/icu_locid_transform)](https://crates.io/crates/icu_locid_transform)

Canonicalization of locale identifiers based on [`CLDR`] data.

This module is published as its own crate ([`icu_locid_transform`](https://docs.rs/icu_locid_transform/latest/icu_locid_transform/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

It currently supports locale canonicalization based upon the canonicalization
algorithm from [`UTS #35: Unicode LDML 3. LocaleId Canonicalization`],
as well as the minimize and maximize likely subtags algorithms
as described in [`UTS #35: Unicode LDML 3. Likely Subtags`].

The maximize method potentially updates a passed in locale in place
depending up the results of running the 'Add Likely Subtags' algorithm
from [`UTS #35: Unicode LDML 3. Likely Subtags`].

This minimize method returns a new Locale that is the result of running the
'Remove Likely Subtags' algorithm from [`UTS #35: Unicode LDML 3. Likely Subtags`].

## Examples

```rust
use icu_locid_transform::{CanonicalizationResult, LocaleCanonicalizer};
use icu_locid::Locale;

let provider = icu_testdata::get_provider();
let lc = LocaleCanonicalizer::new(&provider).expect("create failed");

let mut locale: Locale = "ja-Latn-fonipa-hepburn-heploc"
    .parse()
    .expect("parse failed");
assert_eq!(
    lc.canonicalize(&mut locale),
    CanonicalizationResult::Modified
);
assert_eq!(locale.to_string(), "ja-Latn-alalc97-fonipa");
```

```rust
use icu_locid_transform::{CanonicalizationResult, LocaleCanonicalizer};
use icu_locid::Locale;

let provider = icu_testdata::get_provider();
let lc = LocaleCanonicalizer::new(&provider).expect("create failed");

let mut locale: Locale = "zh-CN".parse().expect("parse failed");
assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Modified);
assert_eq!(locale.to_string(), "zh-Hans-CN");

let mut locale: Locale = "zh-Hant-TW".parse().expect("parse failed");
assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Unmodified);
assert_eq!(locale.to_string(), "zh-Hant-TW");
```

```rust
use icu_locid_transform::{CanonicalizationResult, LocaleCanonicalizer};
use icu_locid::Locale;

let provider = icu_testdata::get_provider();
let lc = LocaleCanonicalizer::new(&provider).expect("create failed");

let mut locale: Locale = "zh-Hans-CN".parse().expect("parse failed");
assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Modified);
assert_eq!(locale.to_string(), "zh");

let mut locale: Locale = "zh".parse().expect("parse failed");
assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Unmodified);
assert_eq!(locale.to_string(), "zh");
```

[`ICU4X`]: ../icu/index.html
[`CLDR`]: http://cldr.unicode.org/
[`UTS #35: Unicode LDML 3. Likely Subtags`]: https://www.unicode.org/reports/tr35/#Likely_Subtags.
[`UTS #35: Unicode LDML 3. LocaleId Canonicalization`]: http://unicode.org/reports/tr35/#LocaleId_Canonicalization,

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
