// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::{self, Display, Formatter};
use core::str::FromStr;
use tinystr::TinyAsciiStr;

pub mod formatter;
pub mod options;

/// An error indicating that a currency code is invalid under ISO 4217.
#[derive(displaydoc::Display, Debug, Copy, Clone, PartialEq, Eq)]
#[displaydoc("Invalid currency code, expected 3 uppercase ASCII letters (A-Z)")]
#[non_exhaustive]
pub struct CurrencyCodeError;

impl core::error::Error for CurrencyCodeError {}

/// A currency code conforming to ISO 4217, such as "USD" or "EUR".
///
/// ISO 4217 specifies that alphabetic currency codes consist of exactly
/// three uppercase ASCII letters (`A`-`Z`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(transparent)]
pub struct CurrencyCode(pub(crate) TinyAsciiStr<3>);

#[cfg(feature = "datagen")]
impl databake::Bake for CurrencyCode {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        env.insert("icu_experimental");
        let string = self.as_str();
        databake::quote! {
            icu_experimental::dimension::currency::CurrencyCode::from_tinystr_unvalidated(
                tinystr::tinystr!(3, #string)
            )
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::BakeSize for CurrencyCode {
    fn borrows_size(&self) -> usize {
        0
    }
}

impl CurrencyCode {
    /// Parses a [`CurrencyCode`] from a UTF-8 byte slice.
    ///
    /// ISO 4217 currency codes must be exactly 3 uppercase ASCII letters (`A`-`Z`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_experimental::dimension::currency::CurrencyCode;
    ///
    /// assert!(CurrencyCode::try_from_utf8(b"USD").is_ok());
    /// assert!(CurrencyCode::try_from_utf8(b"usd").is_err());
    /// assert!(CurrencyCode::try_from_utf8(b"US").is_err());
    /// assert!(CurrencyCode::try_from_utf8(b"US1").is_err());
    /// assert!(CurrencyCode::try_from_utf8(b"USDD").is_err());
    /// ```
    pub const fn try_from_utf8(code_units: &[u8]) -> Result<Self, CurrencyCodeError> {
        if code_units.len() != 3 {
            return Err(CurrencyCodeError);
        }
        if code_units[0].is_ascii_uppercase()
            && code_units[1].is_ascii_uppercase()
            && code_units[2].is_ascii_uppercase()
        {
            match TinyAsciiStr::try_from_utf8(code_units) {
                Ok(s) => Ok(Self(s)),
                Err(_) => Err(CurrencyCodeError),
            }
        } else {
            Err(CurrencyCodeError)
        }
    }

    /// Parses a [`CurrencyCode`] from a string slice.
    ///
    /// ISO 4217 currency codes must be exactly 3 uppercase ASCII letters (`A`-`Z`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_experimental::dimension::currency::CurrencyCode;
    ///
    /// assert!(CurrencyCode::try_from_str("USD").is_ok());
    /// assert!(CurrencyCode::try_from_str("usd").is_err());
    /// assert!(CurrencyCode::try_from_str("123").is_err());
    /// assert!(CurrencyCode::try_from_str("US").is_err());
    /// assert!(CurrencyCode::try_from_str("USDD").is_err());
    /// ```
    #[inline]
    pub const fn try_from_str(s: &str) -> Result<Self, CurrencyCodeError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// Creates a [`CurrencyCode`] from a [`TinyAsciiStr<3>`], validating that it contains
    /// exactly 3 uppercase ASCII letters (`A`-`Z`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_experimental::dimension::currency::CurrencyCode;
    /// use tinystr::tinystr;
    ///
    /// assert!(CurrencyCode::try_from_tinystr(tinystr!(3, "USD")).is_ok());
    /// assert!(CurrencyCode::try_from_tinystr(tinystr!(3, "usd")).is_err());
    /// assert!(CurrencyCode::try_from_tinystr(tinystr!(3, "123")).is_err());
    /// ```
    pub const fn try_from_tinystr(s: TinyAsciiStr<3>) -> Result<Self, CurrencyCodeError> {
        if s.len() == 3 && s.is_ascii_alphabetic_uppercase() {
            Ok(Self(s))
        } else {
            Err(CurrencyCodeError)
        }
    }

    /// Creates a [`CurrencyCode`] from a [`TinyAsciiStr<3>`] without validation.
    ///
    /// # Safety / Invariant
    ///
    /// The caller must ensure that `s` contains 3 uppercase ASCII letters (`A`-`Z`).
    #[inline]
    pub const fn from_tinystr_unvalidated(s: TinyAsciiStr<3>) -> Self {
        Self(s)
    }

    /// Returns the currency code as a string slice.
    #[inline]
    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the currency code as a [`TinyAsciiStr<3>`].
    #[inline]
    pub const fn to_tinystr(self) -> TinyAsciiStr<3> {
        self.0
    }
}

impl FromStr for CurrencyCode {
    type Err = CurrencyCodeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = CurrencyCodeError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from_str(s)
    }
}

impl TryFrom<&[u8]> for CurrencyCode {
    type Error = CurrencyCodeError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_utf8(bytes)
    }
}

impl TryFrom<TinyAsciiStr<3>> for CurrencyCode {
    type Error = CurrencyCodeError;

    #[inline]
    fn try_from(s: TinyAsciiStr<3>) -> Result<Self, Self::Error> {
        Self::try_from_tinystr(s)
    }
}

impl From<CurrencyCode> for TinyAsciiStr<3> {
    #[inline]
    fn from(code: CurrencyCode) -> Self {
        code.0
    }
}

impl Display for CurrencyCode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl writeable::Writeable for CurrencyCode {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str(self.as_str())
    }
}

impl core::ops::Deref for CurrencyCode {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for CurrencyCode {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CurrencyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = TinyAsciiStr::<3>::deserialize(deserializer)?;
        Self::try_from_tinystr(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
pub mod compact_format;
#[cfg(test)]
pub mod format;

#[cfg(test)]
mod tests {
    use super::*;
    use tinystr::tinystr;

    #[test]
    fn test_valid_currency_codes() {
        let valid = [
            "USD", "EUR", "JPY", "GBP", "CAD", "AUD", "CHF", "CNY", "XYZ", "ZZZ", "AAA",
        ];
        for code in valid {
            let parsed = CurrencyCode::try_from_str(code).unwrap();
            let ts = TinyAsciiStr::<3>::try_from_utf8(code.as_bytes()).unwrap();
            assert_eq!(parsed.as_str(), code);
            assert_eq!(parsed.to_tinystr(), ts);
            assert_eq!(parsed.to_string(), code);
            assert_eq!(&*parsed, code);
            assert_eq!(parsed, code.parse::<CurrencyCode>().unwrap());
            assert_eq!(
                parsed,
                CurrencyCode::try_from_utf8(code.as_bytes()).unwrap()
            );
            assert_eq!(parsed, CurrencyCode::try_from_tinystr(ts).unwrap());
        }

        assert_eq!(
            CurrencyCode::try_from_tinystr(tinystr!(3, "USD"))
                .unwrap()
                .as_str(),
            "USD"
        );
    }

    #[test]
    fn test_invalid_currency_codes() {
        let invalid = [
            "", "U", "US", "USDD", "usd", "Usd", "uSD", "US1", "123", "U$D", " US", "US ", "ÉUR",
        ];
        for code in invalid {
            assert!(
                CurrencyCode::try_from_str(code).is_err(),
                "Expected error for: {code}"
            );
            assert!(
                code.parse::<CurrencyCode>().is_err(),
                "Expected error for parse: {code}"
            );
            assert!(
                CurrencyCode::try_from_utf8(code.as_bytes()).is_err(),
                "Expected error for utf8: {code}"
            );
        }
    }
}
