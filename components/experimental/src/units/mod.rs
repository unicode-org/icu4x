// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

pub mod converter;
pub mod converter_factory;
pub mod convertible;
pub mod provider;
pub mod ratio;

#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[displaydoc("The unit is not valid")]
/// There is no conversion between the two units or the conversion data is missing.
/// In the end, the conversion is not possible.
pub struct InvalidConversionError;
