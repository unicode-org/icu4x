// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu` is the main meta-package of the `ICU4X` project.
//!
//! It provides a comprehensive selection of
//! [Unicode Internationalization Components](http://site.icu-project.org/)
//! in their canonical configurations intended to enable software
//! internationalization capabilities.
//!
//! The package is provided for convenience and users are encouraged
//! to fine-tune components with the features they need.
//!
//! The package does not bring any unique functionality. Users
//! can achieve the exact same by manually including the dependent
//! components with pre-selected features.
//!
//! # Data Management
//!
//! Most of Unicode functionality relies on data which has to be provided
//! to the APIs.
//!
//! `ICU4X` project uses a concept of [`DataProvider`] - a service used to
//! handle data management.
//!
//! There can be many different heuristics for handling data management and
//! this meta-package does not supply any default [`DataProvider`].
//!
//! When using `ICU4X` users are expected to decide which provider they want to use
//! and instrument it to point at the correct location where the data files are stored.
//!
//! In the following examples an [`icu_testdata`] package is used which wraps
//! an [`FsDataProvider`] with locally available subset of data.
//!
//! # Examples
//!
//! ```
//! use icu::locid::Locale;
//! use icu::locid::macros::langid;
//! use icu::datetime::{DateTimeFormat, mock::datetime::MockDateTime, options::length};
//!
//! let provider = icu_testdata::get_provider();
//!
//! let locale: Locale = langid!("en").into();
//!
//! let options = length::Bag {
//!     date: Some(length::Date::Long),
//!     time: Some(length::Time::Medium),
//!     ..Default::default()
//! }.into();
//!
//! let dtf = DateTimeFormat::try_new(locale, &provider, &options)
//!     .expect("Failed to create DateTimeFormat instance.");
//!
//! let date: MockDateTime = "2020-09-12T12:35:00".parse()
//!     .expect("Failed to parse date.");
//!
//! let formatted_date = dtf.format(&date);
//! assert_eq!(formatted_date.to_string(), "September 12, 2020 at 12:35:00 PM");
//! ```
//!
//! [`icu_testdata`]: ../icu_testdata/index.html
//! [`DataProvider`]: ../icu_provider/prelude/trait.DataProvider.html
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
pub mod datetime {
    //! Date and Time operations
    //!
    //! This API provides necessary functionality for formatting date and time to user readable textual representation.
    //!
    //! [`DateTimeFormat`] is the main structure of the component. It accepts a set of arguments which
    //! allow it to collect necessary data from the [`DataProvider`], and once instantiated, can be
    //! used to quickly format any date and time provided.
    //!
    //! # Examples
    //!
    //! ```
    //! use icu::locid::Locale;
    //! use icu::locid::macros::langid;
    //! use icu::datetime::{DateTimeFormat, mock::datetime::MockDateTime, options::length};
    //!
    //! let provider = icu_testdata::get_provider();
    //!
    //! let locale: Locale = langid!("en").into();
    //!
    //! let options = length::Bag {
    //!     date: Some(length::Date::Medium),
    //!     time: Some(length::Time::Short),
    //!     ..Default::default()
    //! }.into();
    //!
    //! let dtf = DateTimeFormat::try_new(locale, &provider, &options)
    //!     .expect("Failed to create DateTimeFormat instance.");
    //!
    //! let date: MockDateTime = "2020-09-12T12:35:00".parse()
    //!     .expect("Failed to parse date.");
    //!
    //! let formatted_date = dtf.format(&date);
    //! assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
    //! ```
    //! [`DataProvider`]: ../../icu_provider/index.html
    pub use icu_datetime::*;
}

pub mod locid {
    //! Language and Locale operations
    //!
    //! This API provides necessary functionality for parsing, manipulating, and serializing Unicode Language
    //! and Locale Identifiers.
    //!
    //! The crate provides algorithms for parsing a string into a well-formed language or locale identifier
    //! as defined by [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`].
    //!
    //! [`Locale`] is the most common structure to use for storing information about a language,
    //! script, region, variants and extensions. In almost all cases, this struct should be used as the
    //! base unit for all locale management operations.
    //!
    //! [`LanguageIdentifier`] is a strict subset of [`Locale`] which can be useful in a narrow range of
    //! cases where [`Unicode Extensions`] are not relevant.
    //!
    //! If in doubt, use [`Locale`].
    //!
    //! # Examples
    //!
    //! ```
    //! use icu::locid::Locale;
    //! use icu::locid::subtags::{Language, Region};
    //!
    //! let mut loc: Locale = "en-US".parse()
    //!     .expect("Parsing failed.");
    //!
    //! let lang: Language = "en".parse()
    //!     .expect("Parsing failed.");
    //! let region: Region = "US".parse()
    //!     .expect("Parsing failed.");
    //!
    //! assert_eq!(loc.id.language, lang);
    //! assert_eq!(loc.id.script, None);
    //! assert_eq!(loc.id.region, Some(region));
    //! assert_eq!(loc.id.variants.len(), 0);
    //!
    //! let region: Region = "GB".parse().expect("Parsing failed.");
    //! loc.id.region = Some(region);
    //!
    //! assert_eq!(loc.to_string(), "en-GB");
    //! ```
    //!
    //! For more details, see [`Locale`] and [`LanguageIdentifier`].
    //!
    //! [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]: https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers
    //! [`ICU4X`]: https://github.com/unicode-org/icu4x
    //! [`Unicode Extensions`]: extensions
    pub use icu_locid::*;

    pub mod macros {
        pub use icu_locid_macros::*;
    }
}

pub mod plurals {
    //! Plural Rules operations
    //!
    //! This API provides functionality to determine the plural category
    //! appropriate for a given number in a given language.
    //!
    //! For example in English language, when constructing a message
    //! such as `{ num } items`, the user has to prepare
    //! two variants of the message:
    //!
    //! * `1 item`
    //! * `0 items`, `2 items`, `5 items`, `0.5 items` etc.
    //!
    //! The former variant is used when the placeholder variable has value `1`,
    //! while the latter is used for all other values of the variable.
    //!
    //! Unicode defines [`Language Plural Rules`] as a mechanism to codify those
    //! variants and provides data and algorithms to calculate
    //! appropriate [`Plural Category`].
    //!
    //! # Examples
    //!
    //! ```
    //! use icu::locid::macros::langid;
    //! use icu::plurals::{PluralRules, PluralRuleType, PluralCategory};
    //!
    //! let lid = langid!("en");
    //!
    //! let provider = icu_testdata::get_provider();
    //!
    //! let pr = PluralRules::try_new(lid, &provider, PluralRuleType::Cardinal)
    //!     .expect("Failed to construct a PluralRules struct.");
    //!
    //! assert_eq!(pr.select(5_usize), PluralCategory::Other);
    //! ```
    //!
    //! ## Plural Rules
    //!
    //! The crate provides the main struct [`PluralRules`] which handles selection
    //! of the correct [`Plural Category`] for a given language and [`Plural Type`].
    //!
    //! ## Plural Category
    //!
    //! Every number in every language belongs to a certain [`Plural Category`].
    //! For example, Polish language uses four:
    //!
    //! * [`One`](PluralCategory::One): `1 miesiąc`
    //! * [`Few`](PluralCategory::Few): `2 miesiące`
    //! * [`Many`](PluralCategory::Many): `5 miesięcy`
    //! * [`Other`](PluralCategory::Other): `1.5 miesiąca`
    //!
    //! ## Plural Rule Type
    //!
    //! Plural rules depend on the use case. This crate supports two types of plural rules:
    //!
    //! * [`Cardinal`](PluralRuleType::Cardinal): `3 doors`, `1 month`, `10 dollars`
    //! * [`Ordinal`](PluralRuleType::Ordinal): `1st place`, `10th day`, `11th floor`
    //!
    //! [`ICU4X`]: https://github.com/unicode-org/icu4x
    //! [`Plural Type`]: PluralRuleType
    //! [`Plural Category`]: PluralCategory
    //! [`Language Plural Rules`]: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
    //! [`CLDR`]: http://cldr.unicode.org/
    pub use icu_plurals::*;
}

pub mod uniset {
    //! Unicode Set operations
    //!
    //! This API provides necessary functionality for highly efficient querying of sets of Unicode characters.
    //!
    //! It is an implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).
    //!
    //! # Architecture
    //! ICU4X `UnicodeSet` is split up into independent levels, with [`UnicodeSet`] representing the membership/query API,
    //! and [`UnicodeSetBuilder`] representing the builder API. A [Properties API](http://userguide.icu-project.org/strings/properties)
    //! is in future works.
    //!
    //! # Examples:
    //!
    //! ## Creating a `UnicodeSet`
    //!
    //! UnicodeSets are created from either serialized UnicodeSets,
    //! represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
    //! the [`UnicodeSetBuilder`], or from the TBA Properties API.
    //!
    //! ```
    //! use icu::uniset::{UnicodeSet, UnicodeSetBuilder};
    //!
    //! let mut builder = UnicodeSetBuilder::new();
    //! builder.add_range(&('A'..'Z'));
    //! let set: UnicodeSet = builder.build();
    //!
    //! assert!(set.contains('A'));
    //! ```
    //!
    //! ## Querying a `UnicodeSet`
    //!
    //! Currently, you can check if a character/range of characters exists in the UnicodeSet, or iterate through the characters.
    //!
    //! ```
    //! use icu::uniset::{UnicodeSet, UnicodeSetBuilder};
    //!
    //! let mut builder = UnicodeSetBuilder::new();
    //! builder.add_range(&('A'..'Z'));
    //! let set: UnicodeSet = builder.build();
    //!
    //! assert!(set.contains('A'));
    //! assert!(set.contains_range(&('A'..='C')));
    //! assert_eq!(set.iter_chars().next(), Some('A'));
    //! ```
    pub use icu_uniset::*;
}
