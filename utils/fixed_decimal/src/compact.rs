// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::convert::TryFrom;
use core::fmt;

use core::str::FromStr;

use crate::Error;
use crate::FixedDecimal;

/// A struct containing a [`FixedDecimal`] significand together with an exponent, representing a
/// number written in compact notation (such as 1.2M).
/// This represents a _source number_, as defined
/// [in UTS #35](https://www.unicode.org/reports/tr35/tr35-numbers.html#Plural_rules_syntax).
/// The value exponent=0 represents a number in non-compact
/// notation (such as 1 200 000).
///
/// This is distinct from [`crate::ScientificDecimal`] because it does not represent leading 0s
/// nor a sign in the exponent, and behaves differently in pluralization.
#[derive(Debug, Clone, PartialEq)]
pub struct CompactDecimal {
    significand: FixedDecimal,
    exponent: i16,
}

impl CompactDecimal {
    /// Constructs a [`CompactDecimal`] from its significand and exponent.
    pub fn from_significand_and_exponent(significand: FixedDecimal, exponent: u8) -> Self {
        Self {
            significand,
            exponent: exponent.into(),
        }
    }

    /// Returns a reference to the significand of `self`.
    /// ```
    /// # use fixed_decimal::CompactDecimal;
    /// # use fixed_decimal::FixedDecimal;
    /// # use std::str::FromStr;
    /// #
    /// assert_eq!(
    ///     CompactDecimal::from_str("+1.20c6").unwrap().significand(),
    ///     &FixedDecimal::from_str("+1.20").unwrap()
    /// );
    /// ```
    pub fn significand(&self) -> &FixedDecimal {
        &self.significand
    }

    /// Returns the significand of `self`.
    /// ```
    /// # use fixed_decimal::CompactDecimal;
    /// # use fixed_decimal::FixedDecimal;
    /// # use std::str::FromStr;
    /// #
    /// assert_eq!(
    ///     CompactDecimal::from_str("+1.20c6")
    ///         .unwrap()
    ///         .into_significand(),
    ///     FixedDecimal::from_str("+1.20").unwrap()
    /// );
    /// ```
    pub fn into_significand(self) -> FixedDecimal {
        self.significand
    }

    /// Returns the exponent of `self`.
    /// ```
    /// # use fixed_decimal::CompactDecimal;
    /// # use fixed_decimal::FixedDecimal;
    /// # use std::str::FromStr;
    /// #
    /// assert_eq!(CompactDecimal::from_str("+1.20c6").unwrap().exponent(), 6);
    /// assert_eq!(CompactDecimal::from_str("1729").unwrap().exponent(), 0);
    /// ```
    pub fn exponent(&self) -> i16 {
        self.exponent
    }
}

/// Render the [`CompactDecimal`] in sampleValue syntax.
/// The letter c is used, rather than the deprecated e.
///
/// # Examples
///
/// ```
/// # use fixed_decimal::CompactDecimal;
/// # use std::str::FromStr;
/// # use writeable::assert_writeable_eq;
/// #
/// assert_writeable_eq!(
///     CompactDecimal::from_str("+1.20c6").unwrap(),
///     "+1.20c6"
/// );
/// assert_writeable_eq!(CompactDecimal::from_str("+1729").unwrap(), "+1729");
/// ```
impl writeable::Writeable for CompactDecimal {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        self.significand.write_to(sink)?;
        if self.exponent != 0 {
            sink.write_char('c')?;
            self.exponent.write_to(sink)?;
        }
        Ok(())
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        let mut result = self.significand.writeable_length_hint();
        if self.exponent != 0 {
            result += self.exponent.writeable_length_hint() + 1;
        }
        result
    }
}

writeable::impl_display_with_writeable!(CompactDecimal);

impl FromStr for CompactDecimal {
    type Err = Error;
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        Self::try_from(input_str.as_bytes())
    }
}

/// The deprecated letter e is not accepted as a synonym for c.
impl TryFrom<&[u8]> for CompactDecimal {
    type Error = Error;
    fn try_from(input_str: &[u8]) -> Result<Self, Self::Error> {
        if input_str.iter().any(|&c| c == b'e' || c == b'E') {
            return Err(Error::Syntax);
        }
        let mut parts = input_str.split(|&c| c == b'c');
        let significand = FixedDecimal::try_from(parts.next().ok_or(Error::Syntax)?)?;
        match parts.next() {
            None => Ok(CompactDecimal {
                significand,
                exponent: 0,
            }),
            Some(exponent_str) => {
                let exponent_str = core::str::from_utf8(exponent_str).map_err(|_| Error::Syntax)?;
                if parts.next().is_some() {
                    return Err(Error::Syntax);
                }
                if exponent_str.is_empty()
                    || exponent_str.bytes().next() == Some(b'0')
                    || !exponent_str.bytes().all(|c| c.is_ascii_digit())
                {
                    return Err(Error::Syntax);
                }
                let exponent = exponent_str.parse().map_err(|_| Error::Limit)?;
                Ok(CompactDecimal {
                    significand,
                    exponent,
                })
            }
        }
    }
}

#[test]
fn test_compact_syntax_error() {
    #[derive(Debug)]
    struct TestCase {
        pub input_str: &'static str,
        pub expected_err: Option<Error>,
    }
    let cases = [
        TestCase {
            input_str: "-123e4",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-123c",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "1c10",
            expected_err: None,
        },
        TestCase {
            input_str: "1E1c1",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "1e1c1",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "1c1e1",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "1c1E1",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-1c01",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-1c-1",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-1c1",
            expected_err: None,
        },
    ];
    for cas in &cases {
        match CompactDecimal::from_str(cas.input_str) {
            Ok(dec) => {
                assert_eq!(cas.expected_err, None, "{cas:?}");
                assert_eq!(cas.input_str, dec.to_string(), "{cas:?}");
            }
            Err(err) => {
                assert_eq!(cas.expected_err, Some(err), "{cas:?}");
            }
        }
    }
}
