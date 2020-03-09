
# Ecosystem tracking

This document tracks the crates that already exist in the ecosystem that cover functionality that we wish to cover in OmnICU.

Where multiple maintainers are listed, the first name is the primary maintainer: e.g. Manish is a member of the Servo org but does not primarily maintain some of these crates.

| **API**               | **Rust Eqivalent**                                             | **Maintainer**      | **CLDR-Aware**   | **Notes**                               |
| --------------------- | -------------------------------------------------------------- | ------------------- | ---------------- | --------------------------------------- |
| icu::DateTimeFormat   | [unic-datetime](https://github.com/zbraniecki/unic-datetime)   | Zibi                | ?                |                                         |
| icu::Locale           | [unic-locale](https://github.com/zbraniecki/unic-locale)       | Zibi                | ?                |                                         |
| icu::Bidi             | [unicode-bidi](http://docs.rs/unicode-bidi)                    | Servo / Manish      | N/A              |                                         |
| icu::Normalization    | [unicode-normalization](http://docs.rs/unicode-normalization/) | Manish / unicode-rs | No               |                                         |
| icu::Script           | [unicode-script](http://docs.rs/unicode-script/)               | Manish              | N/A              |                                         |
| icu::IDNa             | [idna](http://docs.rs/idna/)                                   | Servo / Manish      | CLDR confusables |                                         |
| icu::PluralRules      | [intl-pluralrules](https://github.com/zbraniecki/pluralrules)  | Zibi                | Yes              |                                         |
| icu::BreakIterator    | [unicode-segmentation](https://docs.rs/unicode-segmentation/)  | Manish / unicode-rs | No               | Does not support line segmentation      |
| icu::Collator         | Early Google POC (?)                                           | Shane?              | ?                |                                         |
| icu::NumberFormatter  | Early Google POC (?)                                           | Shane?              | ?                |                                         |
| icu::CharConversion   | -                                                              |                     |                  |                                         |
| icu::Char             | -                                                              |                     |                  |                                         |
| icu::TimeZone         | -                                                              |                     |                  |                                         |
| icu::Regex            | -                                                              |                     |                  |                                         |
| icu::Calendar         | -                                                              |                     |                  |                                         |
| icu::ListFormatter    | -                                                              |                     |                  |                                         |
| icu::RelativeDateTime | -                                                              |                     |                  |                                         |



