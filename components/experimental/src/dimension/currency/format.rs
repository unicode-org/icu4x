// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: add more tests for this module to cover more locales & currencies.
#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{CurrencyCode, formatter::CurrencyFormatter};

    #[test]
    pub fn test_en_us() {
        let locale = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::try_new(locale, Default::default()).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, &currency_code);
        assert_writeable_eq!(formatted_currency, "$12,345.67");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, &currency_code);
        assert_writeable_eq!(formatted_currency, "-$12,345.67");
    }

    #[test]
    pub fn test_fr_fr() {
        let locale = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));
        let fmt = CurrencyFormatter::try_new(locale, Default::default()).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, &currency_code);
        assert_writeable_eq!(formatted_currency, "12\u{202f}345,67\u{a0}€");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, &currency_code);
        assert_writeable_eq!(formatted_currency, "-12\u{202f}345,67\u{a0}€");
    }

    #[test]
    pub fn test_ar_eg() {
        let locale = locale!("ar-EG").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let fmt = CurrencyFormatter::try_new(locale, Default::default()).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, &currency_code);
        // TODO(#6064)
        assert_writeable_eq!(formatted_currency, "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, &currency_code);
        // TODO(#6064)
        assert_writeable_eq!(
            formatted_currency,
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );
    }

    #[test]
    pub fn test_numbering_system_override() {
        let prefs_arab = locale!("ar-EG").into();
        let prefs_latn = locale!("ar-EG-u-nu-latn").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let value = "12345.67".parse().unwrap();

        // 1. Default numbering system (arab)
        let fmt_arab = CurrencyFormatter::try_new(prefs_arab, Default::default()).unwrap();
        assert_writeable_eq!(
            fmt_arab.format_fixed_decimal(&value, &currency_code),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );

        // 2. Locale extension override (latn)
        let fmt_latn = CurrencyFormatter::try_new(prefs_latn, Default::default()).unwrap();
        assert_writeable_eq!(
            fmt_latn.format_fixed_decimal(&value, &currency_code),
            "\u{200f}12,345.67\u{a0}ج.م.\u{200f}"
        );
    }
}
