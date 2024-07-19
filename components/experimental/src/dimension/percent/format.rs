// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::{FixedDecimal, Sign};

use crate::alloc::borrow::ToOwned;
use crate::alloc::string::ToString;
use alloc::string::String;
use writeable::Writeable;

use crate::dimension::provider::percent::PercentEssentialsV1;

use super::options::{Display, PercentFormatterOptions};

pub struct FormattedPercent<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) essential: &'l PercentEssentialsV1<'l>,
    pub(crate) options: &'l PercentFormatterOptions,
}

impl<'l> Writeable for FormattedPercent<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        match self.options.display {
            // In the Standard display, we take the
            Display::Standard => self
                .essential
                .positive_pattern
                .interpolate([self.value])
                .write_to(sink)?,
            Display::Approximate => {
                // Removing the sign from the value
                let abs_value = match self.value.sign() {
                    Sign::Negative => self.value.clone().with_sign(Sign::None),
                    _ => self.value.to_owned(),
                }
                .to_string();

                // The approximate sign gets pre-pended
                let sign = match self.value.sign() {
                    Sign::Negative => {
                        String::new()
                            + &self.essential.approximately_sign
                            + &self.essential.minus_sign
                    }
                    _ => self.essential.approximately_sign.to_string(),
                };

                self.essential
                    .negative_pattern
                    .interpolate([abs_value, sign])
                    .write_to(sink)?;
            }
            // TODO: Determine if we throw an error when explicit plus is called and the value is negative.
            Display::ExplicitPlus => self
                .essential
                .negative_pattern
                .interpolate([self.value.to_string(), self.essential.plus_sign.to_string()])
                .write_to(sink)?,
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fixed_decimal::FixedDecimal;
    use icu_locale_core::locale;
    use icu_provider::DataLocale;
    use writeable::Writeable;

    use crate::dimension::percent::{
        formatter::PercentFormatter,
        options::{Display, PercentFormatterOptions},
    };

    pub fn format_percent(
        locale: &DataLocale,
        options: PercentFormatterOptions,
        value: FixedDecimal,
    ) -> String {
        let default_fmt = PercentFormatter::try_new(&locale, options).unwrap();
        let formatted_percent = default_fmt.format_percent(&value);
        let mut sink = String::new();
        formatted_percent.write_to(&mut sink).unwrap();
        sink
    }

    #[test]
    pub fn test_en_us() {
        let locale = locale!("en-US").into();
        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), positive_value);
        assert_eq!(result.as_str(), "12345.67%");

        // Negative case
        let neg_value = "-12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), neg_value);
        assert_eq!(result.as_str(), "-12345.67%");

        // Approximate Case
        let approx_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::Approximate,
        };
        let result = format_percent(&locale, options, approx_value);
        assert_eq!(result.as_str(), "~12345.67%");

        // ExplicitPlus Case
        let neg_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::ExplicitPlus,
        };
        let result = format_percent(&locale, options, neg_value);
        assert_eq!(result.as_str(), "+12345.67%");
    }

    #[test]
    pub fn test_fr_fr() {
        let locale = locale!("fr-FR").into();
        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), positive_value);
        assert_eq!(result.as_str(), "12345.67\u{a0}%");

        // Negative case
        let neg_value = "-12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), neg_value);
        assert_eq!(result.as_str(), "-12345.67\u{a0}%");

        // Approximate Case
        let approx_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::Approximate,
        };
        let result = format_percent(&locale, options, approx_value);
        assert_eq!(result.as_str(), "≃12345.67\u{a0}%");

        // ExplicitPlus Case
        let neg_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::ExplicitPlus,
        };
        let result = format_percent(&locale, options, neg_value);
        assert_eq!(result.as_str(), "+12345.67\u{a0}%");
    }

    #[test]
    pub fn test_ar_eg() {
        let locale = locale!("ar-EG").into();
        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), positive_value);
        assert_eq!(result.as_str(), "12345.67\u{200e}%\u{200e}");

        // Negative case
        let neg_value = "-12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), neg_value);
        assert_eq!(result.as_str(), "-12345.67\u{200e}%\u{200e}");

        // Approximate Case
        let approx_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::Approximate,
        };
        let result = format_percent(&locale, options, approx_value);
        assert_eq!(result.as_str(), "~12345.67\u{200e}%\u{200e}");

        // ExplicitPlus Case
        let neg_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::ExplicitPlus,
        };
        let result = format_percent(&locale, options, neg_value);
        assert_eq!(result.as_str(), "\u{200e}+12345.67\u{200e}%\u{200e}");
    }

    #[test]
    pub fn test_es() {
        let locale = locale!("es").into();
        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), positive_value);
        assert_eq!(result.as_str(), "12345.67\u{a0}%");

        // Negative case
        let neg_value = "-12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), neg_value);
        assert_eq!(result.as_str(), "-12345.67\u{a0}%");

        // Approximate Case
        let approx_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::Approximate,
        };
        let result = format_percent(&locale, options, approx_value);
        assert_eq!(result.as_str(), "~12345.67\u{a0}%");

        // ExplicitPlus Case
        let neg_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::ExplicitPlus,
        };
        let result = format_percent(&locale, options, neg_value);
        assert_eq!(result.as_str(), "+12345.67\u{a0}%");
    }

    #[test]
    pub fn test_ccp() {
        let locale = locale!("ccp").into();
        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), positive_value);
        assert_eq!(result.as_str(), "12345.67%");

        // Negative case
        let neg_value = "-12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), neg_value);
        assert_eq!(result.as_str(), "-12345.67%");

        // Approximate Case
        let approx_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::Approximate,
        };
        let result = format_percent(&locale, options, approx_value);
        assert_eq!(result.as_str(), "~12345.67%");

        // ExplicitPlus Case
        let neg_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::ExplicitPlus,
        };
        let result = format_percent(&locale, options, neg_value);
        assert_eq!(result.as_str(), "+12345.67%");
    }

    #[test]
    pub fn test_tr() {
        let locale = locale!("tr").into();
        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), positive_value);
        assert_eq!(result.as_str(), "%12345.67");

        // Negative case
        let neg_value = "-12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), neg_value);
        assert_eq!(result.as_str(), "%-12345.67");

        // Approximate Case
        let approx_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::Approximate,
        };
        let result = format_percent(&locale, options, approx_value);
        assert_eq!(result.as_str(), "~%12345.67");

        // ExplicitPlus Case
        let neg_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::ExplicitPlus,
        };
        let result = format_percent(&locale, options, neg_value);
        assert_eq!(result.as_str(), "+%12345.67");
    }

    #[test]
    pub fn test_blo() {
        let locale = locale!("blo").into();
        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), positive_value);
        assert_eq!(result.as_str(), "%\u{a0}12345.67");

        // Negative case
        let neg_value = "-12345.67".parse().unwrap();
        let result = format_percent(&locale, Default::default(), neg_value);
        assert_eq!(result.as_str(), "%\u{a0}-12345.67");

        // Approximate Case
        let approx_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::Approximate,
        };
        let result = format_percent(&locale, options, approx_value);
        assert_eq!(result.as_str(), "%\u{a0}~12345.67");

        // ExplicitPlus Case
        let neg_value = "12345.67".parse().unwrap();
        let options = PercentFormatterOptions {
            display: Display::ExplicitPlus,
        };
        let result = format_percent(&locale, options, neg_value);
        assert_eq!(result.as_str(), "%\u{a0}+12345.67");
    }
}
