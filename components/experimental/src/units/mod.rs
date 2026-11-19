// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Unit conversion for single and compound measurement units.
//!
//! This module provides APIs for converting values between different units of measurement.
//! It supports proportional, offset, and reciprocal conversions, as well as compound units.
//!
//! The main entry point is [`converter_factory::ConverterFactory`], which loads the necessary conversion data
//! and creates [`converter::UnitsConverter`] instances.
//!
//! # Examples
//!
//! ## Proportional Conversion (`f64`)
//!
//! Converting meters to feet:
//!
//! ```
//! use icu::experimental::measure::measureunit::MeasureUnit;
//! use icu::experimental::units::converter_factory::ConverterFactory;
//!
//! let factory = ConverterFactory::new();
//! let meter = MeasureUnit::try_from_str("meter").unwrap();
//! let foot = MeasureUnit::try_from_str("foot").unwrap();
//! let converter = factory.converter::<f64>(&meter, &foot).unwrap();
//!
//! // 2 meters is approximately 6.56168 feet.
//! assert!(((converter.convert(2.0) - 6.56168) / 6.56168).abs() < 1e-5);
//! ```
//!
//! ## Proportional Conversion (`Ratio`)
//!
//! Using [`num_rational::Ratio`] for high-precision conversions:
//!
//! ```
//! use icu::experimental::measure::measureunit::MeasureUnit;
//! use icu::experimental::units::converter_factory::ConverterFactory;
//! use num_bigint::BigInt;
//! use num_rational::Ratio;
//!
//! let factory = ConverterFactory::new();
//! let meter = MeasureUnit::try_from_str("meter").unwrap();
//! let foot = MeasureUnit::try_from_str("foot").unwrap();
//! let converter = factory.converter::<&Ratio<BigInt>>(&meter, &foot).unwrap();
//!
//! let meters = Ratio::from(BigInt::from(2));
//! let feet = converter.convert(&meters);
//!
//! // 1 foot is 0.3048 meters, so 2 meters is 20000 / 3048 = 2500 / 381 feet.
//! assert_eq!(feet, Ratio::new(BigInt::from(2500), BigInt::from(381)));
//! ```
//!
//! ## Offset Conversion
//!
//! Converting Celsius to Fahrenheit (requires offset handling):
//!
//! ```
//! use icu::experimental::measure::measureunit::MeasureUnit;
//! use icu::experimental::units::converter_factory::ConverterFactory;
//!
//! let factory = ConverterFactory::new();
//! let celsius = MeasureUnit::try_from_str("celsius").unwrap();
//! let fahrenheit = MeasureUnit::try_from_str("fahrenheit").unwrap();
//! let converter = factory.converter::<f64>(&celsius, &fahrenheit).unwrap();
//!
//! // 0°C is 32°F.
//! assert!(((converter.convert(0.0) - 32.0) / 32.0).abs() < 1e-9);
//! // 100°C is 212°F.
//! assert!(((converter.convert(100.0) - 212.0) / 212.0).abs() < 1e-9);
//! ```
//!
//! ## Reciprocal Conversion
//!
//! Converting mile-per-gallon to liter-per-100-kilometer (reciprocal relationship):
//!
//! ```
//! use icu::experimental::measure::measureunit::MeasureUnit;
//! use icu::experimental::units::converter_factory::ConverterFactory;
//!
//! let factory = ConverterFactory::new();
//! let mpg = MeasureUnit::try_from_str("mile-per-gallon").unwrap();
//! let lp100km = MeasureUnit::try_from_str("liter-per-100-kilometer").unwrap();
//! let converter = factory.converter::<f64>(&mpg, &lp100km).unwrap();
//!
//! // 30 mpg is approximately 7.840486 L/100km.
//! // Formula: 235.2145833 / mpg = lp100km
//! assert!(((converter.convert(30.0) - 7.840486) / 7.840486).abs() < 1e-5);
//! ```
//!
//! ## Incompatible Units
//!
//! Attempting to convert incompatible units returns an [`InvalidConversionError`]:
//!
//! ```
//! use icu::experimental::measure::measureunit::MeasureUnit;
//! use icu::experimental::units::converter_factory::ConverterFactory;
//! use icu::experimental::units::InvalidConversionError;
//!
//! let factory = ConverterFactory::new();
//! let meter = MeasureUnit::try_from_str("meter").unwrap();
//! let second = MeasureUnit::try_from_str("second").unwrap();
//!
//! // Cannot convert meters to seconds.
//! let converter = factory.converter::<f64>(&meter, &second);
//! assert!(matches!(converter, Err(InvalidConversionError { .. })));
//! ```

#![allow(missing_docs)] // todo

use displaydoc::Display;

pub mod converter;
pub mod converter_factory;
pub mod convertible;
pub mod provider;
pub mod ratio;

/// There is no conversion between the two units or the conversion data is missing.
/// In the end, the conversion is not possible.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[displaydoc("The unit is not valid")]
#[non_exhaustive]
pub struct InvalidConversionError;
