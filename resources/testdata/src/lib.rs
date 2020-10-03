//! `icu-testdata` is a unit testing package for [`ICU4X`].
//!
//! The package exposes a DataProvider with stable data useful for unit testing. The data is
//! based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.
//!
//! See README.md for instructions on re-generating the data from CLDR.
//!
//! # Examples
//!
//! ```
//! use std::borrow::Cow;
//! use icu_data_provider::prelude::*;
//! use icu_data_provider::structs::plurals::PluralRuleStringsV1;
//!
//! let data_provider = icu_testdata::get_provider();
//!
//! let data: Cow<PluralRuleStringsV1> = data_provider
//!     .load(&DataRequest {
//!         data_entry: DataEntry {
//!             langid: "be".parse().unwrap(),
//!             variant: None,
//!         },
//!         data_key: icu_data_key!(plurals: cardinal@1),
//!     })
//!     .unwrap()
//!     .take_payload()
//!     .unwrap();
//! assert_eq!(data.few, Some(Cow::Borrowed("n % 10 = 2..4 and n % 100 != 12..14")));
//! ```
//!
//! [`ICU4X`]: https://github.com/unicode-org/icu4x

#[cfg(feature = "metadata")]
pub mod metadata;

mod test_data_provider;

pub use test_data_provider::get_provider;
