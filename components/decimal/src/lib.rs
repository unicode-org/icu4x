// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_decimal` offers localized decimal number formatting.
//!
//! Currently, `icu_decimal` provides [`FixedDecimalFormat`], which renders basic decimal numbers
//! in a locale-sensitive way.
//!
//! Support for currencies, measurement units, and compact notation is planned. To track progress,
//! follow this issue:
//!
//! https://github.com/unicode-org/icu4x/issues/275
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "provider_serde")] {
//! use icu_decimal::FixedDecimalFormat;
//! use icu_decimal::FormattedFixedDecimal;
//! use icu_locid::Locale;
//! use icu_locid_macros::langid;
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
//! # } // feature = "provider_serde"
//! ```
//!
//! [`FixedDecimalFormat`]: FixedDecimalFormat

mod digit_char;
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
use std::borrow::Cow;

/// A formatter for [`FixedDecimal`], rendering decimal digits in an i18n-friendly way.
///
/// `FixedDecimalFormat` supports:
///
/// 1. Rendering in the local numbering system
/// 2. Locale-sensitive grouping separator positions
/// 3. Locale-sensitive plus and minus signs
///
/// Read more about the options in the [`options`] module.
///
/// See the crate-level documentation for examples.
///
/// [`FixedDecimal`]: FixedDecimal
/// [`options`]: options
pub struct FixedDecimalFormat<'d> {
    options: options::FixedDecimalFormatOptions,
    symbols: Cow<'d, provider::DecimalSymbolsV1>,
}

impl<'d> FixedDecimalFormat<'d> {
    /// Creates a new `FixedDecimalFormat` from locale data and an options bag.
    pub fn try_new<T: Into<Locale>, D: DataProvider<'d, provider::DecimalSymbolsV1> + ?Sized>(
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
            .payload
            .take()?;
        Ok(Self { options, symbols })
    }

    /// Formats a FixedDecimal, returning a FormattedFixedDecimal.
    pub fn format<'l>(&'l self, value: &'l FixedDecimal) -> FormattedFixedDecimal<'l> {
        FormattedFixedDecimal {
            value,
            options: &self.options,
            symbols: &self.symbols,
        }
    }
}
