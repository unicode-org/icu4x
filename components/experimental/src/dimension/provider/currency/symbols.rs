// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for currency symbols.
//!
//! Read more about data providers: [`icu_provider`]

use icu_locale_core::subtags::Subtag;
use icu_provider::prelude::*;
use tinystr::{TinyAsciiStr, tinystr};
use zerovec::VarZeroCow;
use zerovec::ule::vartuple::{VarTuple, VarTupleULE};

use crate::dimension::currency::CurrencyType;

icu_provider::data_marker!(
    /// Currency symbol data needed for short and narrow currency formatting.
    CurrencySymbolsV1,
    CurrencySymbol<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "currency",
);

icu_provider::data_marker!(
    /// Decimal symbols for currency formatting, where they differ from the locale's
    /// standard decimal symbols.
    ///
    /// The marker attributes are `[<numbering system>/]<currency>` for the symbols of
    /// one currency (CLDR's `<decimal>` and `<group>` in `<currency>`), and
    /// `[<numbering system>]` for the symbols for all other currencies (CLDR's
    /// `<currencyDecimal>` and `<currencyGroup>`). The numbering system is left out
    /// for the locale's default one.
    CurrencyDecimalSymbolsV1,
    "currency/decimal/symbols/v1",
    icu_decimal::provider::DecimalSymbols<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "currency",
);

impl CurrencyDecimalSymbolsV1 {
    /// Creates attributes for this marker from parts.
    ///
    /// Without a currency, the attributes are for the symbols for all currencies that
    /// have no symbols of their own.
    pub fn make_attributes(
        nu: Option<Subtag>,
        currency: Option<CurrencyType>,
        x: &mut TinyAsciiStr<12>,
    ) -> &DataMarkerAttributes {
        *x = match (nu, currency) {
            (Some(nu), Some(currency)) => nu
                .to_tinystr()
                .concat::<1, 9>(tinystr::tinystr!(1, "/"))
                .concat::<3, 12>(currency.iso_code()),
            (Some(nu), None) => nu.to_tinystr().resize(),
            (None, Some(currency)) => currency.iso_code().resize(),
            (None, None) => TinyAsciiStr::EMPTY,
        };
        DataMarkerAttributes::from_str_or_panic(x.as_str())
    }

    /// Parses attributes for this marker into parts.
    #[cfg(feature = "datagen")]
    pub fn parse_attributes(
        attrs: &DataMarkerAttributes,
    ) -> Option<(Option<Subtag>, Option<CurrencyType>)> {
        let (nu, currency) = match attrs.as_str().split_once('/') {
            Some((nu, currency)) => (nu, currency),
            // On their own, currencies are in upper case and numbering systems in lower case.
            None if attrs.starts_with(|c: char| c.is_ascii_uppercase()) => ("", attrs.as_str()),
            None => (attrs.as_str(), ""),
        };
        let nu = match nu {
            "" => None,
            nu => Some(Subtag::try_from_str(nu).ok()?),
        };
        let currency = match currency {
            "" => None,
            currency => Some(CurrencyType::try_from_str(currency).ok()?),
        };

        Some((nu, currency))
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[derive(Debug, Clone, PartialEq, Eq, zerofrom::ZeroFrom, yoke::Yokeable)]

pub struct CurrencySymbol<'a>(
    #[doc(hidden)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub VarZeroCow<'a, VarTupleULE<u8, str>>,
);

impl CurrencySymbol<'_> {
    pub fn new(symbol: &str, starts_with_letter: bool, ends_with_letter: bool) -> Self {
        let sized = (starts_with_letter as u8) << 1 | (ends_with_letter as u8);
        let variable = VarZeroCow::from(symbol);
        Self(VarZeroCow::from_encodeable(&VarTuple { sized, variable }))
    }

    /// Returns true if the symbol starts with a letter.
    pub fn starts_with_letter(&self) -> bool {
        self.0.sized & 0b10 != 0
    }

    /// Returns true if the symbol ends with a letter.
    pub fn ends_with_letter(&self) -> bool {
        self.0.sized & 0b01 != 0
    }

    /// Returns the symbol as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0.variable
    }
}

/// The width of a currency symbol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CurrencySymbolWidth {
    /// Standard or short currency symbol (e.g. `"$"` or `"CA$"`).
    Short,
    /// Narrow currency symbol (e.g. `"$"`).
    ///
    /// Data for this width falls back to the standard symbol at datagen time: if CLDR
    /// defines no narrow symbol for a currency, the narrow entry carries the standard
    /// symbol instead (UTS #35 lateral inheritance). Consumers therefore never need to
    /// issue a second request for the standard symbol.
    Narrow,
}

impl CurrencySymbolsV1 {
    pub const SHORT: CurrencySymbolWidth = CurrencySymbolWidth::Short;
    pub const NARROW: CurrencySymbolWidth = CurrencySymbolWidth::Narrow;

    pub fn make_attributes(
        currency: CurrencyType,
        width: CurrencySymbolWidth,
        buffer: &mut TinyAsciiStr<4>,
    ) -> &DataMarkerAttributes {
        *buffer = match width {
            CurrencySymbolWidth::Short => currency.iso_code().resize(),
            CurrencySymbolWidth::Narrow => currency.iso_code().concat(tinystr!(1, "n")),
        };
        // All valid
        DataMarkerAttributes::try_from_str(buffer).unwrap()
    }

    /// Parses attributes for this marker into parts.
    #[cfg(feature = "datagen")]
    pub fn parse_attributes(
        attrs: &DataMarkerAttributes,
    ) -> Option<(CurrencyType, CurrencySymbolWidth)> {
        let attrs = attrs.as_str();
        if let Ok(currency) = CurrencyType::try_from_str(attrs) {
            return Some((currency, CurrencySymbolWidth::Short));
        }
        let currency = CurrencyType::try_from_str(attrs.strip_suffix('n')?).ok()?;

        Some((currency, CurrencySymbolWidth::Narrow))
    }
}

icu_provider::data_struct!(
    CurrencySymbol<'_>,
    varule: VarTupleULE<u8, str>,
    #[cfg(feature = "datagen")]
    encode_as_varule: |v: &CurrencySymbol<'_>| &v.0
);

impl<'zf> zerofrom::ZeroFrom<'zf, VarTupleULE<u8, str>> for CurrencySymbol<'zf> {
    fn zero_from(source: &'zf VarTupleULE<u8, str>) -> Self {
        Self(VarZeroCow::zero_from(source))
    }
}

#[test]
fn test_currency_attributes_roundtrip() {
    use icu_locale::preferences::extensions::unicode::keywords::currency;
    use icu_locale::subtags::subtag;

    let currency = currency!("PTE");
    let arab = subtag!("Arab");

    // With and without a numbering system, for one currency and for all currencies.
    for (nu, currency, expected) in [
        (Some(arab), Some(currency), "arab/PTE"),
        (None, Some(currency), "PTE"),
        (Some(arab), None, "arab"),
        (None, None, ""),
    ] {
        let mut buf = TinyAsciiStr::EMPTY;
        let attrs = CurrencyDecimalSymbolsV1::make_attributes(nu, currency, &mut buf);
        assert_eq!(attrs.as_str(), expected);
        assert_eq!(
            CurrencyDecimalSymbolsV1::parse_attributes(attrs),
            Some((nu, currency))
        );
    }

    // `USDn` is an attribute of `CurrencySymbolsV1`, which shares the attributes domain.
    for invalid in ["EU", "EURx", "USDn", "latn/EU"] {
        assert_eq!(
            CurrencyDecimalSymbolsV1::parse_attributes(DataMarkerAttributes::from_str_or_panic(
                invalid
            )),
            None
        );
    }

    let mut buf = TinyAsciiStr::EMPTY;
    for width in [CurrencySymbolsV1::SHORT, CurrencySymbolsV1::NARROW] {
        assert_eq!(
            CurrencySymbolsV1::parse_attributes(CurrencySymbolsV1::make_attributes(
                currency, width, &mut buf
            ))
            .unwrap(),
            (currency, width)
        );
    }

    for invalid in ["EU", "EURx", "EURnn", "latn/EUR"] {
        assert_eq!(
            CurrencySymbolsV1::parse_attributes(DataMarkerAttributes::from_str_or_panic(invalid)),
            None
        );
    }
}
