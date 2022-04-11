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
//! # Features
//!
//! ICU4X components share a set of common features that control whether core pieces of
//! functionality are compiled. These features are:
//!
//! - `serialize`: Whether to include Serde Deserialize implementations for
//!   ICU4X locale data structs, such as [`SymbolsV1`], and Serialize/Deserialize implementations
//!   for core libary types, such as [`Locale`] (On by default)
//! - `datagen`: Whether to include Serde Serialize and other data generation traits for ICU4X locale data structs.
//! - `bench`: Whether to enable exhaustive benchmarks. This can be enabled on individual crates
//!   when running `cargo bench`.
//! - `experimental`: Whether to enable experimental preview features. Modules enabled with
//!   this feature may not be production-ready and could change at any time.
//!
//! # Example
//!
//! ```
//! use icu::locid::locale;
//! use icu::datetime::{DateTimeFormat, options::length, mock::parse_gregorian_from_str};
//!
//! let provider = icu_testdata::get_provider();
//!
//! let options = length::Bag {
//!     date: Some(length::Date::Long),
//!     time: Some(length::Time::Medium),
//!     ..Default::default()
//! }.into();
//!
//! let dtf = DateTimeFormat::try_new(locale!("en"), &provider, &options)
//!     .expect("Failed to create DateTimeFormat instance.");
//!
//! let date = parse_gregorian_from_str("2020-09-12T12:35:00")
//!     .expect("Failed to parse date.");
//!
//! let formatted_date = dtf.format(&date);
//! assert_eq!(formatted_date.to_string(), "September 12, 2020 at 12:35:00 PM");
//! ```
//!
//! [`DataProvider`]: ../icu_provider/prelude/trait.DataProvider.html
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`icu_testdata`]: ../icu_testdata/index.html
//! [`Locale`]: crate::locid::Locale
//! [`SymbolsV1`]: crate::decimal::provider::DecimalSymbolsV1

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )
)]

pub mod calendar {
    //! Contains the core types used by ICU4X for dealing
    //! with dates, times, and custom calendars.
    //!
    //! The [`types`] module has a lot of common types for dealing with dates and times.
    //!
    //! [`Calendar`] is a trait that allows one to define custom calendars, and [`Date`]
    //! can represent dates for arbitrary calendars.
    //!
    //! The [`iso`] and [`gregorian`] modules contain implementations for the ISO and
    //! Gregorian calendars respectively.
    pub use icu_calendar::*;
}

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
    //! use icu::locid::locale;
    //! use icu::datetime::{DateTimeFormat, options::length, mock::parse_gregorian_from_str};
    //!
    //! let provider = icu_testdata::get_provider();
    //!
    //! let options = length::Bag::new(Some(length::Date::Medium), Some(length::Time::Short)).into();
    //! let dtf = DateTimeFormat::try_new(locale!("en"), &provider, &options)
    //!     .expect("Failed to create DateTimeFormat instance.");
    //!
    //! let date = parse_gregorian_from_str("2020-09-12T12:35:00")
    //!     .expect("Failed to parse date.");
    //!
    //! let formatted_date = dtf.format(&date);
    //! assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
    //! ```
    //! [`DataProvider`]: ../../icu_provider/index.html
    pub use icu_datetime::*;
}

pub mod decimal {
    //! Decimal formatting operations
    //!
    //! This API provides necessary functionality for formatting of numbers with decimal digits.
    //!
    //! [`FixedDecimalFormat`] is the main structure of the component. It formats a
    //! [`FixedDecimal`] to a [`FormattedFixedDecimal`].
    //!
    //! # Examples
    //!
    //! ## Format a number with Bengali digits
    //!
    //! ```
    //! use icu::decimal::FixedDecimalFormat;
    //! use icu::locid::locale;
    //! use writeable::Writeable;
    //!
    //! let provider = icu_testdata::get_provider();
    //! let fdf = FixedDecimalFormat::try_new(locale!("bn"), &provider, Default::default())
    //!     .expect("Data should load successfully");
    //!
    //! let fixed_decimal = 1000007.into();
    //! let formatted_value = fdf.format(&fixed_decimal);
    //! let formatted_str = formatted_value.write_to_string();
    //!
    //! assert_eq!("১০,০০,০০৭", formatted_str);
    //! ```
    //!
    //! ## Format a number with digits after the decimal separator
    //!
    //! ```
    //! use fixed_decimal::FixedDecimal;
    //! use icu::decimal::FixedDecimalFormat;
    //! use icu::locid::Locale;
    //! use writeable::Writeable;
    //!
    //! let provider = icu_provider::inv::InvariantDataProvider;
    //! let fdf = FixedDecimalFormat::try_new(Locale::UND, &provider, Default::default())
    //!     .expect("Data should load successfully");
    //!
    //! let fixed_decimal = FixedDecimal::from(200050)
    //!     .multiplied_pow10(-2)
    //!     .expect("Operation is fully in range");
    //!
    //! assert_eq!("2,000.50", fdf.format(&fixed_decimal).write_to_string());
    //! ```
    //!
    //! [`FixedDecimal`]: fixed_decimal::FixedDecimal
    pub use icu_decimal::*;
}

pub mod locale_canonicalizer {
    //! This API provides functionality to canonicalize locale identifiers based
    //! upon [`CLDR`] data.
    //!
    //! It currently supports the minimize and maximize likely subtags algorithms
    //! as described in [`UTS #35: Unicode LDML 3. Likely Subtags`].
    //!
    //! # Examples
    //!
    //! ```
    //! use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    //! use icu_locid::Locale;
    //!
    //! let provider = icu_testdata::get_provider();
    //! let lc = LocaleCanonicalizer::new(&provider)
    //!     .expect("create failed");
    //!
    //! let mut locale : Locale = "zh-CN".parse()
    //!     .expect("parse failed");
    //! assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Modified);
    //! assert_eq!(locale.to_string(), "zh-Hans-CN");
    //!
    //! let mut locale : Locale = "zh-Hant-TW".parse()
    //!     .expect("parse failed");
    //! assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Unmodified);
    //! assert_eq!(locale.to_string(), "zh-Hant-TW");
    //! ```
    //!
    //! ```
    //! use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    //! use icu_locid::Locale;
    //!
    //! let provider = icu_testdata::get_provider();
    //! let lc = LocaleCanonicalizer::new(&provider)
    //!     .expect("create failed");
    //!
    //! let mut locale : Locale = "zh-Hans-CN".parse()
    //!     .expect("parse failed");
    //! assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Modified);
    //! assert_eq!(locale.to_string(), "zh");
    //!
    //! let mut locale : Locale = "zh".parse()
    //!     .expect("parse failed");
    //! assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Unmodified);
    //! assert_eq!(locale.to_string(), "zh");
    //! ```
    //! [`ICU4X`]: ../icu/index.html
    //! [`CLDR`]: http://cldr.unicode.org/
    //! [`UTS #35: Unicode LDML 3. Likely Subtags`]: https://www.unicode.org/reports/tr35/#Likely_Subtags.
    pub use icu_locale_canonicalizer::*;
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
    //! use icu::locid::{language, locale, region};
    //!
    //! let mut loc = locale!("en-US").clone();
    //!
    //! assert_eq!(loc.id.language, language!("en"));
    //! assert_eq!(loc.id.script, None);
    //! assert_eq!(loc.id.region, Some(region!("US")));
    //! assert_eq!(loc.id.variants.len(), 0);
    //!
    //! let region = region!("GB");
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
    //! use icu::locid::locale;
    //! use icu::plurals::{PluralRules, PluralRuleType, PluralCategory};
    //!
    //! let provider = icu_testdata::get_provider();
    //!
    //! let pr = PluralRules::try_new(locale!("en"), &provider, PluralRuleType::Cardinal)
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

pub mod properties {
    //! Unicode properties
    //!
    //! This API provides definitions of [Unicode Properties] and functions for
    //! retrieving property data in an appropriate data structure.
    //!
    //! APIs that return a [`UnicodeSet`] exist for binary properties and certain enumerated
    //! properties. See the [`sets`] module for more details.
    //!
    //! APIs that return a [`CodePointTrie`] exist for certain enumerated properties. See the
    //! [`maps`] module for more details.
    //!
    //! # Examples
    //!
    //! ## Property data as `UnicodeSet`s
    //!
    //! ```
    //! use icu::properties::{maps, sets, GeneralCategory};
    //!
    //! let provider = icu_testdata::get_provider();
    //!
    //! // A binary property as a `UnicodeSet`
    //!
    //! let payload =
    //!     sets::get_emoji(&provider)
    //!         .expect("The data should be valid");
    //! let data_struct = payload.get();
    //! let emoji = &data_struct.inv_list;
    //!
    //! assert!(emoji.contains('🎃'));  // U+1F383 JACK-O-LANTERN
    //! assert!(!emoji.contains('木'));  // U+6728
    //!
    //! // An individual enumerated property value as a `UnicodeSet`
    //!
    //! let payload = maps::get_general_category(&provider).expect("The data should be valid");
    //! let data_struct = payload.get();
    //! let gc = &data_struct.code_point_trie;
    //! let line_sep = gc.get_set_for_value(GeneralCategory::LineSeparator);
    //!
    //! assert!(line_sep.contains_u32(0x2028));
    //! assert!(!line_sep.contains_u32(0x2029));
    //! ```
    //!
    //! ## Property data as `CodePointTrie`s
    //!
    //! ```
    //! use icu::properties::{maps, Script};
    //!
    //! let provider = icu_testdata::get_provider();
    //!
    //! let payload =
    //!     maps::get_script(&provider)
    //!         .expect("The data should be valid");
    //! let data_struct = payload.get();
    //! let script = &data_struct.code_point_trie;
    //!
    //! assert_eq!(script.get('🎃' as u32), Script::Common);  // U+1F383 JACK-O-LANTERN
    //! assert_eq!(script.get('木' as u32), Script::Han);  // U+6728
    //! ```
    //!
    //! ## Property data for `Script` and `Script_Extensions`
    //!
    //! ```
    //! use icu::properties::{script, Script};
    //!
    //! let provider = icu_testdata::get_provider();
    //!
    //! let payload =
    //!     script::get_script_with_extensions(&provider)
    //!         .expect("The data should be valid");
    //! let data_struct = payload.get();
    //! let swe = &data_struct.data;
    //!
    //! // get the `Script` property value
    //! assert_eq!(swe.get_script_val(0x0650), Script::Inherited); // U+0650 ARABIC KASRA
    //! assert_eq!(swe.get_script_val(0x0660), Script::Arabic); // U+0660 ARABIC-INDIC DIGIT ZERO
    //!
    //! // get the `Script_Extensions` property value
    //! assert_eq!(
    //!     swe.get_script_extensions_val(0x0640) // U+0640 ARABIC TATWEEL
    //!         .iter().collect::<Vec<Script>>(),
    //!     vec![Script::Arabic, Script::Syriac, Script::Mandaic, Script::Manichaean,
    //!          Script::PsalterPahlavi, Script::Adlam, Script::HanifiRohingya, Script::Sogdian,
    //!          Script::OldUyghur]
    //! );
    //! assert_eq!(
    //!     swe.get_script_extensions_val('௫' as u32) // U+0BEB TAMIL DIGIT FIVE
    //!         .iter().collect::<Vec<Script>>(),
    //!     vec![Script::Tamil, Script::Grantha]
    //! );
    //!
    //! // check containment of a `Script` value in the `Script_Extensions` value
    //! // U+0650 ARABIC KASRA
    //! assert!(swe.has_script(0x0650, Script::Arabic));
    //! assert!(swe.has_script(0x0650, Script::Syriac));
    //!
    //! // get a `UnicodeSet` for when `Script` value is contained in `Script_Extensions` value
    //! let syriac = swe.get_script_extensions_set(Script::Syriac);
    //! assert!(syriac.contains_u32(0x0650)); // ARABIC KASRA
    //! assert!(!syriac.contains_u32(0x0660)); // ARABIC-INDIC DIGIT ZERO
    //! ```
    //!
    //! [`ICU4X`]: ../icu/index.html
    //! [Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
    //! [`UnicodeSet`]: ../../icu_uniset/struct.UnicodeSet.html
    //! [`sets`]: sets
    //! [`CodePointTrie`]: ../../icu_codepointtrie/codepointtrie/struct.CodePointTrie.html
    //! [`maps`]: maps
    pub use icu_properties::*;
}

pub mod list {
    //! List formatting
    //!
    //! This API provides the [`ListFormatter`] which renders sequences of [`Writeable`](
    //! writeable::Writeable)s as lists in a locale-sensitive way.
    //!
    //! # Examples
    //!
    //! ## Format a list of strings in Spanish
    //!
    //! ```
    //! use icu::list::{ListFormatter, ListStyle};
    //! use icu::locid::locale;
    //! use writeable::Writeable;
    //!
    //! let provider = icu_testdata::get_provider();
    //! let list_formatter = ListFormatter::try_new_and(locale!("es"), &provider, ListStyle::Wide)
    //!     .expect("Data should load successfully");
    //!
    //! assert_eq!(
    //!     list_formatter.format(["España", "Suiza"].iter())
    //!         .write_to_string(),
    //!     "España y Suiza"
    //! );
    //!
    //! // The Spanish 'y' sometimes becomes an 'e':
    //! assert_eq!(
    //!     list_formatter.format(["España", "Suiza", "Italia"].iter())
    //!         .write_to_string(),
    //!     "España, Suiza e Italia"
    //! );
    //!
    //! // We can use any Writeables as inputs:
    //! assert_eq!(
    //!     list_formatter.format(1..=10).write_to_string(),
    //!     "1, 2, 3, 4, 5, 6, 7, 8, 9 y 10"
    //! );
    //!```
    //!
    //! [`ListFormatter`]: ListFormatter

    pub use icu_list::*;
}
