// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_decimal` offers localized decimal number formatting.
//!
//! It will eventually support:
//!
//! - Plain decimal numbers
//! - Currencies
//! - Measurement units
//! - Compact notation
//!
//! Currently, the crate is a work in progress with extremely limited functionality. To track
//! progress, follow this issue:
//!
//! https://github.com/unicode-org/icu4x/issues/275

pub mod error;
pub mod format;
pub mod options;
pub mod provider;

pub use error::Error as FixedDecimalFormatError;
pub use format::FormattedFixedDecimal;

use fixed_decimal::FixedDecimal;
use icu_locid::Locale;
use icu_provider::prelude::*;
use std::borrow::Cow;

pub struct FixedDecimalFormat<'d> {
    options: options::FixedDecimalFormatOptions,
    symbols: Cow<'d, provider::DecimalSymbolsV1>,
}

impl<'d> FixedDecimalFormat<'d> {
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

    pub fn format<'l>(&'l self, value: &'l FixedDecimal) -> FormattedFixedDecimal<'l> {
        FormattedFixedDecimal {
            value,
            options: &self.options,
            symbols: &self.symbols,
        }
    }
}
