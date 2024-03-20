
# Ecosystem tracking

This document tracks the crates that already exist in the ecosystem that cover functionality that we may wish to cover in ICU4X.

Where multiple maintainers are listed, the first name is the primary maintainer: e.g. Manish is a member of the Servo org but does not primarily maintain some of these crates.

| **API**                 | **Rust Equivalent**                                            | **Maintainer**      | **CLDR-Aware**   | **Action** | **Notes**                                                          |
|-------------------------|----------------------------------------------------------------|---------------------|------------------|------------|--------------------------------------------------------------------|
| `icu::DateTimeFormat`   | [unic-datetime](https://github.com/zbraniecki/unic-datetime)   | Zibi                | Yes              | Import     |                                                                    |
|                         | [rust_icu_udat](https://crates.io/crates/rust_icu_udat)        | filmil, kpozin      | Yes              | Uncertain  | Rust wrapper around ICU4C                                          |
| `icu::Locale`           | [unic-locale](https://github.com/zbraniecki/unic-locale)       | Zibi                | Yes              | Import     |                                                                    |
|                         | [rust_icu_uloc](https://crates.io/crates/rust_icu_uloc)        | filmil, kpozin      | Yes              | Uncertain  | Rust wrapper around ICU4C                                          |
| `icu::Bidi`             | [unicode-bidi](http://docs.rs/unicode-bidi)                    | Servo / Manish      | N/A              | No Action  | Bidi and text layout are unlikely to be coming to ECMA-402         |
| `icu::Normalization`    | [unicode-normalization](http://docs.rs/unicode-normalization/) | Manish / unicode-rs | No               | Uncertain  | Main issue: [#40](https://github.com/unicode-org/icu4x/issues/40) |
| `icu::Script`           | [unicode-script](http://docs.rs/unicode-script/)               | Manish              | N/A              | No Action  | ICU4X should expose UCD data through its own custom pipeline       |
| `icu::IDNa`             | [idna](http://docs.rs/idna/)                                   | Servo / Manish      | CLDR confusables | Uncertain  | Main issue: [#42](https://github.com/unicode-org/icu4x/issues/42) |
| `icu::PluralRules`      | [intl-pluralrules](https://github.com/zbraniecki/pluralrules)  | Zibi                | Yes              | Import     |                                                                    |
|                         | [rust_icu_intl][1]                                             | filmil, kpozin      | Yes              | Uncertain  |                                                                    |
| `icu::BreakIterator`    | [unicode-segmentation](https://docs.rs/unicode-segmentation/)  | Manish / unicode-rs | No               | No Action  | No line segmentation. Wait for clear user demand outside Rust      |
| `icu::Collator`         | -                                                              |                     |                  |            |                                                                    |
| `icu::NumberFormatter`  | [Early Google POC](https://github.com/sffc/rust-wasm-i18n)     | Shane               | ?                | Import     |                                                                    |
| icu::CharConversion     | [encoding_rs](https://docs.rs/crate/encoding_rs/)              | Henri               | Encoding Standard| No Action  | Out of scope for ICU4X                                             |
| `icu::Char`             | [rust_icu_ustring](https://crates.io/crates/rust_icu_ustring)  | filmil, kpozin      | Yes              | Uncertain  |                                                                    |
| `icu::TimeZone`         | -                                                              |                     |                  |            |                                                                    |
| `icu::Regex`            | [regex](https://docs.rs/regex/1.3.7/regex/)                    | Core Rust Team      | ?                | No Action  | Main issue: [#37](https://github.com/unicode-org/icu4x/issues/37) |
| `icu::Calendar`         | [rust_icu_ucal](https://crates.io/crates/rust_icu_ucal)        | filmil, kpozin      | Yes              | Uncertain  | Rust wrapper around ICU4C                                          |
| `icu::ListFormatter`    | -                                                              |                     |                  |            |                                                                    |
| `icu::RelativeDateTime` | -                                                              |                     |                  |            |                                                                    |
| `icu::MessageFormat`    | [rust_icu_umsg][2]                                             | filmil, kpozin      | Yes              | Uncertain  |                                                                    |

[1]: https://github.com/google/rust_icu/tree/master/rust_icu_intl
[2]: https://crates.io/crates/rust_icu_umsg
