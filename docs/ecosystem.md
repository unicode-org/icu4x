
# Ecosystem tracking

This document tracks the crates that already exist in the ecosystem that cover functionality that we may wish to cover in OmnICU.

Where multiple maintainers are listed, the first name is the primary maintainer: e.g. Manish is a member of the Servo org but does not primarily maintain some of these crates.

| **API**               | **Rust Eqivalent**                                             | **Maintainer**      | **CLDR-Aware**   | **Action** | **Notes**                                                          |
|-----------------------|----------------------------------------------------------------|---------------------|------------------|------------|--------------------------------------------------------------------|
| icu::DateTimeFormat   | [unic-datetime](https://github.com/zbraniecki/unic-datetime)   | Zibi                | Yes              | Import     |                                                                    |
| icu::Locale           | [unic-locale](https://github.com/zbraniecki/unic-locale)       | Zibi                | Yes              | Import     |                                                                    |
| icu::Bidi             | [unicode-bidi](http://docs.rs/unicode-bidi)                    | Servo / Manish      | N/A              | No Action  | Bidi and text layout are unlikely to be coming to ECMA-402         |
| icu::Normalization    | [unicode-normalization](http://docs.rs/unicode-normalization/) | Manish / unicode-rs | No               | Uncertain  | Main issue: [#40](https://github.com/unicode-org/omnicu/issues/40) |
| icu::Script           | [unicode-script](http://docs.rs/unicode-script/)               | Manish              | N/A              | No Action  | ICU4X should expose UCD data through its own custom pipeline       |
| icu::IDNa             | [idna](http://docs.rs/idna/)                                   | Servo / Manish      | CLDR confusables | Uncertain  | IDNA is a specific subject area more related to URLs than i18n     |
| icu::PluralRules      | [intl-pluralrules](https://github.com/zbraniecki/pluralrules)  | Zibi                | Yes              | Import     |                                                                    |
| icu::BreakIterator    | [unicode-segmentation](https://docs.rs/unicode-segmentation/)  | Manish / unicode-rs | No               | No Action  | No line segmentation. Wait for clear user demand outside Rust      |
| icu::Collator         | -                                                              |                     |                  |            |                                                                    |
| icu::NumberFormatter  | [Early Google POC](https://github.com/sffc/rust-wasm-i18n)     | Shane               | ?                | Import     |                                                                    |
| icu::CharConversion   | -                                                              |                     |                  |            |                                                                    |
| icu::Char             | -                                                              |                     |                  |            |                                                                    |
| icu::TimeZone         | -                                                              |                     |                  |            |                                                                    |
| icu::Regex            | -                                                              |                     |                  |            |                                                                    |
| icu::Calendar         | -                                                              |                     |                  |            |                                                                    |
| icu::ListFormatter    | -                                                              |                     |                  |            |                                                                    |
| icu::RelativeDateTime | -                                                              |                     |                  |            |                                                                    |


