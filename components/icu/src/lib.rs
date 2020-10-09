//! `ICU` is the main meta-package of the `ICU4X` project.
//!
//! It provides a comperhensive selection of Unicode Internationalization Components
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
//! # Data Provider
//!
//! Most of Unicode functionality relies on data which has to be provided
//! to the APIs.
//!
//! `ICU4X` project uses a concept of [`DataProvider`] - a service used to
//! handle data management.
//!
//! There can be many different heuristics for handling data management and
//! this meta-package does not supply any default `DataProvider`.
//!
//! When using `ICU4X` users are expected to decide which provider they want to use
//! and instrument it to point at the correct location where the data files are stored.
//!
//! In the following examples the [`FsDataProvider`] is used, which is a local file-system
//! service using synchronous I/O to fetch data stored on the hard drive.
//!
//! # Examples
//!
//! ```
//! use icu::locale::LanguageIdentifier;
//! use icu::datetime::{DateTimeFormat, date::MockDateTime, options::style};
//! use icu_fs_data_provider::FsDataProvider;
//!
//! let provider = FsDataProvider::try_new("./tests/fixtures/data/icu4x")
//!     .expect("Loading file from testdata directory");
//!
//! let langid: LanguageIdentifier = "en".parse()
//!     .expect("Failed to parse a Language Identifier.");
//!
//! let options = style::Bag {
//!     date: Some(style::Date::Long),
//!     time: Some(style::Time::Medium),
//!     ..Default::default()
//! };
//!
//! let dtf = DateTimeFormat::try_new(langid, &provider, &options.into())
//!     .expect("Failed to create DateTimeFormat instance.");
//!
//! let date: MockDateTime = "2020-09-12T12:35:00".parse()
//!     .expect("Failed to parse date.");
//!
//! let formatted_date = dtf.format(&date);
//! assert_eq!(formatted_date.to_string(), "September 12, 2020 \'at\' 12:35:00 PM");
//! ```
//!
//! [`DataProvider`]: ../icu_data_provider/prelude/trait.DataProvider.html
//! [`FsDataProvider`]: ../icu_fs_data_provider/struct.FsDataProvider.html
#[doc(inline)]
pub use icu_datetime as datetime;
#[doc(inline)]
pub use icu_locale as locale;
#[doc(inline)]
pub use icu_pluralrules as plurals;
#[doc(inline)]
pub use icu_unicodeset as uniset;
