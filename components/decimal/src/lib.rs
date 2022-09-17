// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![warn(missing_docs)]

//! Formatting basic decimal numbers.
//!
//! This module is published as its own crate ([`icu_decimal`](https://docs.rs/icu_decimal/latest/icu_decimal/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! Support for currencies, measurement units, and compact notation is planned. To track progress,
//! follow [icu4x#275](https://github.com/unicode-org/icu4x/issues/275).
//!
//! # Examples
//!
//! ## Format a number with Bengali digits
//!
//! ```
//! use icu::decimal::FixedDecimalFormatter;
//! use icu::locid::locale;
//! use writeable::assert_writeable_eq;
//!
//! let fdf = FixedDecimalFormatter::try_new_unstable(
//!     &icu_testdata::unstable(),
//!     &locale!("bn").into(),
//!     Default::default(),
//! )
//! .expect("Data should load successfully");
//!
//! let fixed_decimal = 1000007.into();
//!
//! assert_writeable_eq!(fdf.format(&fixed_decimal), "১০,০০,০০৭");
//! ```
//!
//! ## Format a number with digits after the decimal separator
//!
//! ```
//! use fixed_decimal::FixedDecimal;
//! use icu::decimal::FixedDecimalFormatter;
//! use icu::locid::Locale;
//! use writeable::assert_writeable_eq;
//!
//! let fdf = FixedDecimalFormatter::try_new_unstable(
//!     &icu_testdata::unstable(),
//!     &Locale::UND.into(),
//!     Default::default(),
//! )
//! .expect("Data should load successfully");
//!
//! let fixed_decimal = FixedDecimal::from(200050).multiplied_pow10(-2);
//!
//! assert_writeable_eq!(fdf.format(&fixed_decimal), "2,000.50");
//! ```
//!
//! ### Format a number using an alternative numbering system
//!
//! Numbering systems specified in the `-u-nu` subtag will be followed as long as the locale has
//! symbols for that numbering system.
//!
//! ```
//! use icu::decimal::FixedDecimalFormatter;
//! use icu::locid::locale;
//! use writeable::assert_writeable_eq;
//!
//! let fdf = FixedDecimalFormatter::try_new_unstable(
//!     &icu_testdata::unstable(),
//!     &locale!("th-u-nu-thai").into(),
//!     Default::default(),
//! )
//! .expect("Data should load successfully");
//!
//! let fixed_decimal = 1000007.into();
//!
//! assert_writeable_eq!(fdf.format(&fixed_decimal), "๑,๐๐๐,๐๐๗");
//! ```
//!
//! [`FixedDecimalFormatter`]: FixedDecimalFormatter

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod error;
pub mod format;
mod grouper;
pub mod options;
pub mod provider;

pub use error::Error as FixedDecimalFormatterError;
pub use format::FormattedFixedDecimal;

use alloc::string::String;
use fixed_decimal::FixedDecimal;
use icu_provider::prelude::*;
use writeable::Writeable;

/// A formatter for [`FixedDecimal`], rendering decimal digits in an i18n-friendly way.
///
/// [`FixedDecimalFormatter`] supports:
///
/// 1. Rendering in the local numbering system
/// 2. Locale-sensitive grouping separator positions
/// 3. Locale-sensitive plus and minus signs
///
/// Read more about the options in the [`options`] module.
///
/// See the crate-level documentation for examples.
pub struct FixedDecimalFormatter {
    options: options::FixedDecimalFormatterOptions,
    symbols: DataPayload<provider::DecimalSymbolsV1Marker>,
}

impl FixedDecimalFormatter {
    /// Creates a new [`FixedDecimalFormatter`] from locale data and an options bag.
    pub fn try_new_unstable<D: DataProvider<provider::DecimalSymbolsV1Marker> + ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: options::FixedDecimalFormatterOptions,
    ) -> Result<Self, FixedDecimalFormatterError> {
        let symbols = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { options, symbols })
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: options::FixedDecimalFormatterOptions,
        error: FixedDecimalFormatterError
    );

    /// Formats a [`FixedDecimal`], returning a [`FormattedFixedDecimal`].
    pub fn format<'l>(&'l self, value: &'l FixedDecimal) -> FormattedFixedDecimal<'l> {
        FormattedFixedDecimal {
            value,
            options: &self.options,
            symbols: self.symbols.get(),
        }
    }

    /// Formats a [`FixedDecimal`], returning a [`String`].
    pub fn format_to_string(&self, value: &FixedDecimal) -> String {
        self.format(value).write_to_string().into_owned()
    }
}
