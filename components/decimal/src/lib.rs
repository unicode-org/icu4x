// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
//! ## Format a number with Bangla digits
//!
//! ```
//! use fixed_decimal::SignedFixedDecimal;
//! use icu::decimal::FixedDecimalFormatter;
//! use icu::locale::locale;
//! use writeable::assert_writeable_eq;
//!
//! let fdf = FixedDecimalFormatter::try_new(
//!     locale!("bn").into(),
//!     Default::default(),
//! )
//! .expect("locale should be present");
//!
//! let fixed_decimal = SignedFixedDecimal::from(1000007);
//!
//! assert_writeable_eq!(fdf.format(&fixed_decimal), "১০,০০,০০৭");
//! ```
//!
//! ## Format a number with digits after the decimal separator
//!
//! ```
//! use fixed_decimal::SignedFixedDecimal;
//! use icu::decimal::FixedDecimalFormatter;
//! use icu::locale::Locale;
//! use writeable::assert_writeable_eq;
//!
//! let fdf =
//!     FixedDecimalFormatter::try_new(Default::default(), Default::default())
//!         .expect("locale should be present");
//!
//! let fixed_decimal = {
//!     let mut decimal = SignedFixedDecimal::from(200050);
//!     decimal.multiply_pow10(-2);
//!     decimal
//! };
//!
//! assert_writeable_eq!(fdf.format(&fixed_decimal), "2,000.50");
//! ```
//!
//! ### Format a number using an alternative numbering system
//!
//! Numbering systems specified in the `-u-nu` subtag will be followed.
//!
//! ```
//! use fixed_decimal::SignedFixedDecimal;
//! use icu::decimal::FixedDecimalFormatter;
//! use icu::locale::locale;
//! use writeable::assert_writeable_eq;
//!
//! let fdf = FixedDecimalFormatter::try_new(
//!     locale!("th-u-nu-thai").into(),
//!     Default::default(),
//! )
//! .expect("locale should be present");
//!
//! let fixed_decimal = SignedFixedDecimal::from(1000007);
//!
//! assert_writeable_eq!(fdf.format(&fixed_decimal), "๑,๐๐๐,๐๐๗");
//! ```
//!
//! ### Get the resolved numbering system
//!
//! Inspect the data request to get the resolved numbering system (public but unstable):
//!
//! ```
//! use icu_provider::prelude::*;
//! use icu::decimal::FixedDecimalFormatter;
//! use icu::decimal::provider::DecimalDigitsV1Marker;
//! use icu::locale::locale;
//! use std::any::TypeId;
//! use std::cell::RefCell;
//!
//! struct NumberingSystemInspectionProvider<P> {
//!     inner: P,
//!     numbering_system: RefCell<Option<Box<DataMarkerAttributes>>>,
//! }
//!
//! impl<M, P> DataProvider<M> for NumberingSystemInspectionProvider<P>
//! where
//!     M: DataMarker,
//!     P: DataProvider<M>,
//! {
//!     fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
//!         if TypeId::of::<M>() == TypeId::of::<DecimalDigitsV1Marker>() {
//!             *self.numbering_system.try_borrow_mut().unwrap() = Some(req.id.marker_attributes.to_owned());
//!         }
//!         self.inner.load(req)
//!     }
//! }
//!
//! let provider = NumberingSystemInspectionProvider {
//!     inner: icu::decimal::provider::Baked,
//!     numbering_system: RefCell::new(None),
//! };
//!
//! let fdf = FixedDecimalFormatter::try_new_unstable(
//!     &provider,
//!     locale!("th").into(),
//!     Default::default(),
//! )
//! .unwrap();
//!
//! assert_eq!(provider.numbering_system.borrow().as_ref().map(|x| x.as_str()), Some("latn"));
//!
//! let fdf = FixedDecimalFormatter::try_new_unstable(
//!     &provider,
//!     locale!("th-u-nu-thai").into(),
//!     Default::default(),
//! )
//! .unwrap();
//!
//! assert_eq!(provider.numbering_system.borrow().as_ref().map(|x| x.as_str()), Some("thai"));
//!
//! let fdf = FixedDecimalFormatter::try_new_unstable(
//!     &provider,
//!     locale!("th-u-nu-adlm").into(),
//!     Default::default(),
//! )
//! .unwrap();
//!
//! assert_eq!(provider.numbering_system.borrow().as_ref().map(|x| x.as_str()), Some("adlm"));
//! ```
//!
//! [`FixedDecimalFormatter`]: FixedDecimalFormatter

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
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
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod format;
mod grouper;
pub mod options;
pub mod parts;
pub mod provider;
pub(crate) mod size_test_macro;

pub use format::FormattedFixedDecimal;

use alloc::string::String;
use fixed_decimal::SignedFixedDecimal;
use icu_locale_core::locale;
use icu_locale_core::preferences::{
    define_preferences, extensions::unicode::keywords::NumberingSystem,
};
use icu_provider::prelude::*;
use size_test_macro::size_test;
use writeable::Writeable;

size_test!(FixedDecimalFormatter, fixed_decimal_formatter_size, 96);

define_preferences!(
    /// The preferences for fixed decimal formatting.
    [Copy]
    FixedDecimalFormatterPreferences,
    {
        /// The user's preferred numbering system.
        ///
        /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
        numbering_system: NumberingSystem
    }
);

/// A formatter for [`SignedFixedDecimal`], rendering decimal digits in an i18n-friendly way.
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
#[doc = fixed_decimal_formatter_size!()]
#[derive(Debug)]
pub struct FixedDecimalFormatter {
    options: options::FixedDecimalFormatterOptions,
    symbols: DataPayload<provider::DecimalSymbolsV2Marker>,
    digits: DataPayload<provider::DecimalDigitsV1Marker>,
}

impl AsRef<FixedDecimalFormatter> for FixedDecimalFormatter {
    fn as_ref(&self) -> &FixedDecimalFormatter {
        self
    }
}

impl FixedDecimalFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        (prefs: FixedDecimalFormatterPreferences, options: options::FixedDecimalFormatterOptions) -> error: DataError,
        /// Creates a new [`FixedDecimalFormatter`] from compiled data and an options bag.
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<
        D: DataProvider<provider::DecimalSymbolsV2Marker>
            + DataProvider<provider::DecimalDigitsV1Marker>
            + ?Sized,
    >(
        provider: &D,
        prefs: FixedDecimalFormatterPreferences,
        options: options::FixedDecimalFormatterOptions,
    ) -> Result<Self, DataError> {
        let locale = DataLocale::from_preferences_locale::<provider::DecimalSymbolsV2Marker>(
            prefs.locale_prefs,
        );
        let provided_nu = prefs.numbering_system.as_ref().map(|s| s.as_str());

        // In case the user explicitly specified a numbering system, use digits from that numbering system. In case of explicitly specified numbering systems,
        // the resolved one may end up being different due to a lack of data or fallback, e.g. attempting to resolve en-u-nu-thai will likely produce en-u-nu-Latn data.
        //
        // This correctly handles the following cases:
        // - Explicitly specified numbering system that is the same as the resolved numbering system: This code effects no change
        // - Explicitly specified numbering system that is different from the resolved one: This code overrides it, but the symbols are still correctly loaded for the locale
        // - No explicitly specified numbering system: The default numbering system for the locale is used.
        // - Explicitly specified numbering system without data for it: this falls back to the resolved numbering system
        //
        // Assuming the provider has symbols for en-u-nu-latn, th-u-nu-thai (default for th), and th-u-nu-latin, this produces the following behavior:
        //
        // | Input Locale | Symbols | Digits | Return value of `numbering_system()` |
        // |--------------|---------|--------|--------------------------------------|
        // | en           | latn    | latn   | latn                                 |
        // | en-u-nu-thai | latn    | thai   | thai                                 |
        // | th           | thai    | thai   | thai                                 |
        // | th-u-nu-latn | latn    | latn   | latn                                 |
        // | en-u-nu-wxyz | latn    | latn   | latn                                 |
        // | th-u-nu-wxyz | thai    | thai   | thai                                 |

        if let Some(provided_nu) = provided_nu {
            // Load symbols for the locale/numsys pair provided
            let symbols: DataPayload<provider::DecimalSymbolsV2Marker> = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::from_str_or_panic(provided_nu),
                        &locale,
                    ),
                    ..Default::default()
                })
                // If it doesn't exist, fall back to the locale
                .or_else(|_err| {
                    provider.load(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::empty(),
                            &locale,
                        ),
                        ..Default::default()
                    })
                })?
                .payload;

            let resolved_nu = symbols.get().numsys();

            // Attempt to load the provided numbering system first
            let digits = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::from_str_or_panic(provided_nu),
                        &locale!("und").into(),
                    ),
                    ..Default::default()
                })
                .or_else(|_err| {
                    provider.load(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::from_str_or_panic(resolved_nu),
                            &locale!("und").into(),
                        ),
                        ..Default::default()
                    })
                })?
                .payload;
            Ok(Self {
                options,
                symbols,
                digits,
            })
        } else {
            let symbols: DataPayload<provider::DecimalSymbolsV2Marker> = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::empty(),
                        &locale,
                    ),
                    ..Default::default()
                })?
                .payload;

            let resolved_nu = symbols.get().numsys();

            let digits = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::from_str_or_panic(resolved_nu),
                        &locale!("und").into(),
                    ),
                    ..Default::default()
                })?
                .payload;
            Ok(Self {
                options,
                symbols,
                digits,
            })
        }
    }

    /// Formats a [`SignedFixedDecimal`], returning a [`FormattedFixedDecimal`].
    pub fn format<'l>(&'l self, value: &'l SignedFixedDecimal) -> FormattedFixedDecimal<'l> {
        FormattedFixedDecimal {
            value,
            options: &self.options,
            symbols: self.symbols.get(),
            digits: self.digits.get(),
        }
    }

    /// Formats a [`SignedFixedDecimal`], returning a [`String`].
    pub fn format_to_string(&self, value: &SignedFixedDecimal) -> String {
        self.format(value).write_to_string().into_owned()
    }
}

#[test]
fn test_numbering_resolution_fallback() {
    fn test_locale(locale: icu_locale_core::Locale, expected_format: &str) {
        let formatter = FixedDecimalFormatter::try_new((&locale).into(), Default::default())
            .expect("Must load");
        let fd = 1234.into();
        writeable::assert_writeable_eq!(
            formatter.format(&fd),
            expected_format,
            "Correct format for {locale}"
        );
    }

    // Loading en with default latn numsys
    test_locale(locale!("en"), "1,234");
    // Loading en with arab numsys not present in symbols data will mix en symbols with arab digits
    test_locale(locale!("en-u-nu-arab"), "١,٢٣٤");
    // Loading ar-EG with default (arab) numsys
    test_locale(locale!("ar-EG"), "١٬٢٣٤");
    // Loading ar-EG with overridden latn numsys, present in symbols data, uses ar-EG-u-nu-latn symbols data
    test_locale(locale!("ar-EG-u-nu-latn"), "1,234");
    // Loading ar-EG with overridden thai numsys, not present in symbols data, uses ar-EG symbols data + thai digits
    test_locale(locale!("ar-EG-u-nu-thai"), "๑٬๒๓๔");
    // Loading with nonexistant numbering systems falls back to default
    test_locale(locale!("en-u-nu-wxyz"), "1,234");
    test_locale(locale!("ar-EG-u-nu-wxyz"), "١٬٢٣٤");
}
