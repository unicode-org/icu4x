# icu_locid_transform [![crates.io](https://img.shields.io/crates/v/icu_locid_transform)](https://crates.io/crates/icu_locid_transform)

Canonicalization of locale identifiers based on [`CLDR`] data.

This module is published as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate.

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
use icu::locid::Locale;
use icu::locid_transform::{LocaleCanonicalizer, TransformResult};

let lc = LocaleCanonicalizer::try_new_unstable(&icu_testdata::unstable())
    .expect("create failed");

let mut locale: Locale = "ja-Latn-fonipa-hepburn-heploc"
    .parse()
    .expect("parse failed");
assert_eq!(lc.canonicalize(&mut locale), TransformResult::Modified);
assert_eq!(locale.to_string(), "ja-Latn-alalc97-fonipa");
```

```rust
use icu::locid::Locale;
use icu::locid_transform::{LocaleExpander, TransformResult};

let lc = LocaleExpander::try_new_unstable(&icu_testdata::unstable())
    .expect("create failed");

let mut locale: Locale = "zh-CN".parse().expect("parse failed");
assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
assert_eq!(locale.to_string(), "zh-Hans-CN");

let mut locale: Locale = "zh-Hant-TW".parse().expect("parse failed");
assert_eq!(lc.maximize(&mut locale), TransformResult::Unmodified);
assert_eq!(locale.to_string(), "zh-Hant-TW");
```

```rust
use icu::locid::Locale;
use icu::locid_transform::{LocaleExpander, TransformResult};

let lc = LocaleExpander::try_new_unstable(&icu_testdata::unstable())
    .expect("create failed");

let mut locale: Locale = "zh-Hans-CN".parse().expect("parse failed");
assert_eq!(lc.minimize(&mut locale), TransformResult::Modified);
assert_eq!(locale.to_string(), "zh");

let mut locale: Locale = "zh".parse().expect("parse failed");
assert_eq!(lc.minimize(&mut locale), TransformResult::Unmodified);
assert_eq!(locale.to_string(), "zh");
```

[`ICU4X`]: ../icu/index.html
[`CLDR`]: http://cldr.unicode.org/
[`UTS #35: Unicode LDML 3. Likely Subtags`]: https://www.unicode.org/reports/tr35/#Likely_Subtags.
[`UTS #35: Unicode LDML 3. LocaleId Canonicalization`]: http://unicode.org/reports/tr35/#LocaleId_Canonicalization,

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
