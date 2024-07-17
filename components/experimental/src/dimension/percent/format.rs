// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;

use writeable::Writeable;

use crate::dimension::provider::percent::PercentEssentialsV1;

pub struct FormattedPercent<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) essential: &'l PercentEssentialsV1<'l>,
}

impl<'l> Writeable for FormattedPercent<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        // Construct the percent sign value.
        let percent_sign_value = format!(
            "{}{}{}",
            self.essential.percent_sign_affixes.prefix,
            self.essential.percent_sign_symbol,
            self.essential.percent_sign_affixes.suffix
        );

        // Determine order of the number and percent sign.
        let formatted = match self.essential.number_index < self.essential.percent_symbol_index {
            true => {
                let number = self.value.to_string();
                format!("{}{}", number, percent_sign_value)
            }
            false => {
                let number = self.value.to_string();
                format!("{}{}", percent_sign_value, number)
            }
        };

        formatted.write_to(sink)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use writeable::Writeable;

    use crate::dimension::percent::formatter::PercentFormatter;

    #[test]
    pub fn test_en_us() {
        let locale = locale!("en-US").into();
        let fmt = PercentFormatter::try_new(&locale).unwrap();
        let value = "12345.67".parse().unwrap();
        let formatted_percent = fmt.format_percent(&value);
        let mut sink = String::new();
        formatted_percent.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "12345.67%");
    }

    #[test]
    pub fn test_fr_fr() {
        let locale = locale!("fr-FR").into();
        let fmt = PercentFormatter::try_new(&locale).unwrap();
        let value = "12345.67".parse().unwrap();
        let formatted_percent = fmt.format_percent(&value);
        let mut sink = String::new();
        formatted_percent.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "12345.67\u{a0}%");
    }

    #[test]
    pub fn test_ar_eg() {
        let locale = locale!("ar-EG").into();
        let fmt = PercentFormatter::try_new(&locale).unwrap();
        let value = "12345.67".parse().unwrap();
        let formatted_percent = fmt.format_percent(&value);
        let mut sink = String::new();
        formatted_percent.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "12345.67\u{200e}%\u{200e}");
    }

    #[test]
    pub fn test_es() {
        let locale = locale!("es").into();
        let fmt = PercentFormatter::try_new(&locale).unwrap();
        let value = "12345.67".parse().unwrap();
        let formatted_percent = fmt.format_percent(&value);
        let mut sink = String::new();
        formatted_percent.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "12345.67\u{a0}%");
    }

    #[test]
    pub fn test_ccp() {
        let locale = locale!("ccp").into();
        let fmt = PercentFormatter::try_new(&locale).unwrap();
        let value = "12345.67".parse().unwrap();
        let formatted_percent = fmt.format_percent(&value);
        let mut sink = String::new();
        formatted_percent.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "12345.67%");
    }

    #[test]
    pub fn test_tr() {
        let locale = locale!("tr").into();
        let fmt = PercentFormatter::try_new(&locale).unwrap();
        let value = "12345.67".parse().unwrap();
        let formatted_percent = fmt.format_percent(&value);
        let mut sink = String::new();
        formatted_percent.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "%12345.67");
    }
}
