// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::preferences::extensions::unicode::errors::PreferencesParseError;
use crate::preferences::extensions::unicode::struct_keyword;
use crate::{extensions::unicode::Value, subtags::Subtag};
use tinystr::TinyAsciiStr;

struct_keyword!(
    /// A Unicode Currency Identifier defines a type of currency.
    ///
    /// The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeCurrencyIdentifier).
    [Copy]
    CurrencyType,
    "cu",
    TinyAsciiStr<3>,
    |input: &Value| {
        if let Some(subtag) = input.as_single_subtag() {
            return Self::try_from_tinystr(subtag.as_tinystr());
        }
        Err(PreferencesParseError::InvalidKeywordValue)
    },
    |input: &CurrencyType| {
        Value::from_subtag(Some(
            Subtag::from_tinystr_unvalidated(input.0.resize()),
        ))
    }
);

impl CurrencyType {
    /// Parses a [`CurrencyType`] from a UTF-8 byte slice.
    ///
    /// Valid currency identifiers consist of exactly 3 ASCII alphabetic characters (case-insensitive).
    ///
    /// The parsed currency identifier is stored in lower case.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::preferences::extensions::unicode::keywords::CurrencyType;
    ///
    /// assert!(CurrencyType::try_from_utf8(b"USD").is_ok());
    /// assert!(CurrencyType::try_from_utf8(b"usd").is_ok());
    /// assert!(CurrencyType::try_from_utf8(b"uSd").is_ok());
    /// assert!(CurrencyType::try_from_utf8(b"US").is_err());
    /// assert!(CurrencyType::try_from_utf8(b"US1").is_err());
    /// assert!(CurrencyType::try_from_utf8(b"USDDD").is_err());
    /// ```
    pub const fn try_from_utf8(code_units: &[u8]) -> Result<Self, PreferencesParseError> {
        if let Ok(ts) = TinyAsciiStr::<3>::try_from_utf8(code_units) {
            Self::try_from_tinystr(ts)
        } else {
            Err(PreferencesParseError::InvalidKeywordValue)
        }
    }

    /// Parses a [`CurrencyType`] from a string slice.
    ///
    /// Valid currency identifiers consist of exactly 3 ASCII alphabetic characters (case-insensitive).
    ///
    /// The parsed currency identifier is stored in lower case.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::preferences::extensions::unicode::keywords::CurrencyType;
    ///
    /// assert!(CurrencyType::try_from_str("USD").is_ok());
    /// assert!(CurrencyType::try_from_str("uSd").is_ok());
    /// assert!(CurrencyType::try_from_str("usd").is_ok());
    /// assert!(CurrencyType::try_from_str("123").is_err());
    /// assert!(CurrencyType::try_from_str("US").is_err());
    /// assert!(CurrencyType::try_from_str("USDDD").is_err());
    /// ```
    #[inline]
    pub const fn try_from_str(s: &str) -> Result<Self, PreferencesParseError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// Creates a [`CurrencyType`] from a [`TinyAsciiStr<N>`], validating that it contains
    /// exactly 3 ASCII alphabetic characters (case-insensitive).
    ///
    /// The parsed currency identifier is stored in lower case.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::preferences::extensions::unicode::keywords::CurrencyType;
    /// use tinystr::tinystr;
    ///
    /// assert!(CurrencyType::try_from_tinystr(tinystr!(3, "USD")).is_ok());
    /// assert!(CurrencyType::try_from_tinystr(tinystr!(3, "usd")).is_ok());
    /// assert!(CurrencyType::try_from_tinystr(tinystr!(3, "123")).is_err());
    /// assert!(CurrencyType::try_from_tinystr(tinystr!(3, "US")).is_err());
    /// ```
    pub const fn try_from_tinystr<const N: usize>(
        s: TinyAsciiStr<N>,
    ) -> Result<Self, PreferencesParseError> {
        if s.len() == 3 && s.is_ascii_alphabetic() {
            Ok(Self(s.resize().to_ascii_lowercase()))
        } else {
            Err(PreferencesParseError::InvalidKeywordValue)
        }
    }

    /// Returns the currency identifier as a lower case string slice.
    #[inline]
    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the ISO 4217 3-letter upper case currency code as a [`TinyAsciiStr<3>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::preferences::extensions::unicode::keywords::CurrencyType;
    /// use tinystr::tinystr;
    ///
    /// let currency = CurrencyType::try_from_str("usd").unwrap();
    /// assert_eq!(currency.iso_code(), tinystr!(3, "USD"));
    /// ```
    #[inline]
    pub const fn iso_code(self) -> TinyAsciiStr<3> {
        self.0.to_ascii_uppercase()
    }
}

impl core::str::FromStr for CurrencyType {
    type Err = PreferencesParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinystr::tinystr;

    #[test]
    fn test_valid_currency_types() {
        let valid = [
            ("USD", "usd", "USD"),
            ("uSd", "usd", "USD"),
            ("usd", "usd", "USD"),
            ("EUR", "eur", "EUR"),
            ("JPY", "jpy", "JPY"),
        ];
        for (input, expected_subtag, expected_iso) in valid {
            let parsed = CurrencyType::try_from_str(input).unwrap();
            let expected_ts_iso = TinyAsciiStr::<3>::try_from_str(expected_iso).unwrap();
            assert_eq!(parsed.as_str(), expected_subtag);
            assert_eq!(parsed.iso_code(), expected_ts_iso);
            assert_eq!(parsed, input.parse::<CurrencyType>().unwrap());
        }
    }

    #[test]
    fn test_invalid_currency_types() {
        let invalid = [
            "", "U", "US", "USDDD", "US1", "123", "U$D", " US", "US ", "ÉUR",
        ];
        for input in invalid {
            assert!(CurrencyType::try_from_str(input).is_err());
            assert!(input.parse::<CurrencyType>().is_err());
        }
    }
}
