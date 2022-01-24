// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![warn(missing_docs)]

//! [`icu_decimal`](crate) offers localized decimal number formatting.
//!
//! Currently, [`icu_decimal`](crate) provides [`FixedDecimalFormat`], which renders basic decimal numbers
//! in a locale-sensitive way.
//!
//! Support for currencies, measurement units, and compact notation is planned. To track progress,
//! follow [icu4x#275](https://github.com/unicode-org/icu4x/issues/275).
//!
//! # Examples
//!
//! ## Format a number with Bengali digits
//!
//! ```
//! use icu::decimal::FixedDecimalFormat;
//! use icu::locid::Locale;
//! use icu::locid::macros::langid;
//! use writeable::Writeable;
//!
//! let locale: Locale = langid!("bn").into();
//! let provider = icu_testdata::get_provider();
//! let fdf = FixedDecimalFormat::try_new(locale, &provider, Default::default())
//!     .expect("Data should load successfully");
//!
//! let fixed_decimal = 1000007.into();
//! let formatted_value = fdf.format(&fixed_decimal);
//! let formatted_str = formatted_value.writeable_to_string();
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
//! let locale = Locale::und();
//! let provider = icu_provider::inv::InvariantDataProvider;
//! let fdf = FixedDecimalFormat::try_new(locale, &provider, Default::default())
//!     .expect("Data should load successfully");
//!
//! let fixed_decimal = FixedDecimal::from(200050)
//!     .multiplied_pow10(-2)
//!     .expect("Operation is fully in range");
//!
//! assert_eq!("2,000.50", fdf.format(&fixed_decimal).writeable_to_string());
//! ```
//!
//! [`FixedDecimalFormat`]: FixedDecimalFormat

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

pub mod error;
pub mod format;
mod grouper;
pub mod options;
pub mod provider;
mod sign_selector;

pub use error::Error as FixedDecimalFormatError;
pub use format::FormattedFixedDecimal;

use fixed_decimal::FixedDecimal;
use icu_locid::Locale;
use icu_provider::prelude::*;

/// A formatter for [`FixedDecimal`], rendering decimal digits in an i18n-friendly way.
///
/// [`FixedDecimalFormat`] supports:
///
/// 1. Rendering in the local numbering system
/// 2. Locale-sensitive grouping separator positions
/// 3. Locale-sensitive plus and minus signs
///
/// Read more about the options in the [`options`] module.
///
/// See the crate-level documentation for examples.
pub struct FixedDecimalFormat {
    options: options::FixedDecimalFormatOptions,
    symbols: DataPayload<provider::DecimalSymbolsV1Marker>,
}

impl FixedDecimalFormat {
    /// Creates a new [`FixedDecimalFormat`] from locale data and an options bag.
    pub fn try_new<T: Into<Locale>, D: DataProvider<provider::DecimalSymbolsV1Marker> + ?Sized>(
        locale: T,
        data_provider: &D,
        options: options::FixedDecimalFormatOptions,
    ) -> Result<Self, FixedDecimalFormatError> {
        let symbols = data_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::SYMBOLS_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(locale.into().into()),
                    },
                },
            })?
            .take_payload()?;
        Ok(Self { options, symbols })
    }

    /// Formats a [`FixedDecimal`], returning a [`FormattedFixedDecimal`].
    pub fn format<'l>(&'l self, value: &'l FixedDecimal) -> FormattedFixedDecimal<'l> {
        FormattedFixedDecimal {
            value,
            options: &self.options,
            symbols: self.symbols.get(),
        }
    }
}
