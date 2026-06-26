// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: add more tests for this module to cover more locales & currencies.
#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{
        CurrencyCode,
        formatter::{CurrencyFormatter, CurrencyFormatterPreferences, Decimal},
    };

    #[test]
    pub fn test_en_us() {
        let locale: CurrencyFormatterPreferences = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));

        // Short
        let fmt_short = CurrencyFormatter::<Decimal>::try_new_short(locale).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&positive_value, &currency_code),
            "$12,345.67"
        );
        let negative_value = "-12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&negative_value, &currency_code),
            "-$12,345.67"
        );

        // Narrow
        let fmt_narrow = CurrencyFormatter::<Decimal>::try_new_narrow(locale).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&positive_value, &currency_code),
            "$12,345.67"
        );
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&negative_value, &currency_code),
            "-$12,345.67"
        );
    }

    #[test]
    pub fn test_fr_fr() {
        let locale: CurrencyFormatterPreferences = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));

        // Short
        let fmt_short = CurrencyFormatter::<Decimal>::try_new_short(locale).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&positive_value, &currency_code),
            "12\u{202f}345,67\u{a0}€"
        );
        let negative_value = "-12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&negative_value, &currency_code),
            "-12\u{202f}345,67\u{a0}€"
        );

        // Narrow
        let fmt_narrow = CurrencyFormatter::<Decimal>::try_new_narrow(locale).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&positive_value, &currency_code),
            "12\u{202f}345,67\u{a0}€"
        );
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&negative_value, &currency_code),
            "-12\u{202f}345,67\u{a0}€"
        );
    }

    #[test]
    pub fn test_ar_eg() {
        let locale: CurrencyFormatterPreferences = locale!("ar-EG").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));

        // Short
        let fmt_short = CurrencyFormatter::<Decimal>::try_new_short(locale).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&positive_value, &currency_code),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );
        let negative_value = "-12345.67".parse().unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&negative_value, &currency_code),
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );

        // Narrow
        let fmt_narrow = CurrencyFormatter::<Decimal>::try_new_narrow(locale).unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&positive_value, &currency_code),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&negative_value, &currency_code),
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );
    }

    #[test]
    pub fn test_usd_in_fr_fr() {
        let locale: CurrencyFormatterPreferences = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let value = "12345.67".parse().unwrap();

        // Short USD in fr-FR should be US$ or $US
        let fmt_short = CurrencyFormatter::<Decimal>::try_new_short(locale).unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&value, &currency_code),
            "12\u{202f}345,67\u{a0}$US"
        );

        // Narrow USD in fr-FR should be $
        let fmt_narrow = CurrencyFormatter::<Decimal>::try_new_narrow(locale).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&value, &currency_code),
            "12\u{202f}345,67\u{a0}$"
        );
    }

    #[test]
    pub fn test_numbering_system_override() {
        let prefs_arab = locale!("ar-EG").into();
        let prefs_latn = locale!("ar-EG-u-nu-latn").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let value = "12345.67".parse().unwrap();

        // 1. Default numbering system (arab) - Short
        let fmt_arab = CurrencyFormatter::<Decimal>::try_new_short(prefs_arab).unwrap();
        assert_writeable_eq!(
            fmt_arab.format_fixed_decimal(&value, &currency_code),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );

        // 2. Locale extension override (latn) - Short
        let fmt_latn = CurrencyFormatter::<Decimal>::try_new_short(prefs_latn).unwrap();
        assert_writeable_eq!(
            fmt_latn.format_fixed_decimal(&value, &currency_code),
            "\u{200f}12,345.67\u{a0}ج.م.\u{200f}"
        );
    }

    #[test]
    pub fn test_en_us_cad() {
        let locale: CurrencyFormatterPreferences = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "CAD"));
        let value = "12345.67".parse().unwrap();

        // Short
        let fmt_short = CurrencyFormatter::<Decimal>::try_new_short(locale).unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&value, &currency_code),
            "CA$12,345.67"
        );

        // Narrow
        let fmt_narrow = CurrencyFormatter::<Decimal>::try_new_narrow(locale).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&value, &currency_code),
            "$12,345.67"
        );
    }

    #[test]
    pub fn test_en_us_aud() {
        let locale: CurrencyFormatterPreferences = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "AUD"));
        let value = "12345.67".parse().unwrap();

        // Short
        let fmt_short = CurrencyFormatter::<Decimal>::try_new_short(locale).unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&value, &currency_code),
            "A$12,345.67"
        );

        // Narrow
        let fmt_narrow = CurrencyFormatter::<Decimal>::try_new_narrow(locale).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&value, &currency_code),
            "$12,345.67"
        );
    }
}
