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
        let prefs = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, &currency_code).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&positive_value),
            "$12,345.67"
        );
        let negative_value = "-12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&negative_value),
            "-$12,345.67"
        );

        // TODO(#8151): This should format to 2 decimal places ($123.46 or $123.00) once we use currency patterns.
        // Currently it uses decimal patterns which do not pad '123' to 2 decimal places, and do not round '123.4567'.
        let value_no_decimals = "123".parse().unwrap();
        assert_writeable_eq!(fmt_symbol.format_fixed_decimal(&value_no_decimals), "$123");
        let value_4_decimals = "123.4567".parse().unwrap();
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&value_4_decimals),
            "$123.4567"
        );

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&positive_value),
            "$12,345.67"
        );
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&negative_value),
            "-$12,345.67"
        );

        // TODO(#8151): This should format to 2 decimal places ($123.46 or $123.00) once we use currency patterns.
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&value_no_decimals),
            "$123"
        );
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&value_4_decimals),
            "$123.4567"
        );

        // Long / Name
        let fmt_name = CurrencyFormatter::try_new_name(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&positive_value),
            "12,345.67 US dollars"
        );
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&negative_value),
            "-12,345.67 US dollars"
        );

        // TODO(#8151): This should format to 2 decimal places ("123.46 US dollars" or "123.00 US dollars") once we use currency patterns.
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&value_no_decimals),
            "123 US dollars"
        );
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&value_4_decimals),
            "123.4567 US dollars"
        );
    }

    #[test]
    pub fn test_fr_fr() {
        let prefs = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, &currency_code).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&positive_value),
            "12\u{202f}345,67\u{a0}€"
        );
        let negative_value = "-12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&negative_value),
            "-12\u{202f}345,67\u{a0}€"
        );

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&positive_value),
            "12\u{202f}345,67\u{a0}€"
        );
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&negative_value),
            "-12\u{202f}345,67\u{a0}€"
        );

        // Long / Name
        let fmt_name = CurrencyFormatter::try_new_name(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&positive_value),
            "12\u{202f}345,67 euros"
        );
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&negative_value),
            "-12\u{202f}345,67 euros"
        );
    }

    #[test]
    pub fn test_ar_eg() {
        let prefs = locale!("ar-EG").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, &currency_code).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&positive_value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );
        let negative_value = "-12345.67".parse().unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&negative_value),
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, &currency_code).unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&positive_value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&negative_value),
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );

        // Long / Name
        let fmt_name = CurrencyFormatter::try_new_name(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&positive_value),
            "١٢٬٣٤٥٫٦٧ جنيه مصري"
        );
        assert_writeable_eq!(
            fmt_name.format_fixed_decimal(&negative_value),
            "\u{61c}-١٢٬٣٤٥٫٦٧ جنيه مصري"
        );
    }

    #[test]
    pub fn test_usd_in_fr_fr() {
        let prefs = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let value = "12345.67".parse().unwrap();

        // Short / Symbol USD in fr-FR should be US$ or $US
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_symbol.format_fixed_decimal(&value),
            "12\u{202f}345,67\u{a0}$US"
        );

        // Narrow / Symbol Narrow USD in fr-FR should be $
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_symbol_narrow.format_fixed_decimal(&value),
            "12\u{202f}345,67\u{a0}$"
        );
    }

    #[test]
    pub fn test_numbering_system_override() {
        let prefs_arab = locale!("ar-EG").into();
        let prefs_latn = locale!("ar-EG-u-nu-latn").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let value = "12345.67".parse().unwrap();

        // 1. Default numbering system (arab) - Symbol
        let fmt_arab_symbol =
            CurrencyFormatter::try_new_symbol(prefs_arab, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_arab_symbol.format_fixed_decimal(&value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );

        // 2. Locale extension override (latn) - Symbol
        let fmt_latn_symbol =
            CurrencyFormatter::try_new_symbol(prefs_latn, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_latn_symbol.format_fixed_decimal(&value),
            "\u{200f}12,345.67\u{a0}ج.م.\u{200f}"
        );

        // 3. Default numbering system (arab) - Symbol Narrow
        let fmt_arab_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs_arab, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_arab_symbol_narrow.format_fixed_decimal(&value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );

        // 4. Locale extension override (latn) - Symbol Narrow
        let fmt_latn_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs_latn, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_latn_symbol_narrow.format_fixed_decimal(&value),
            "\u{200f}12,345.67\u{a0}E£"
        );

        // 5. Default numbering system (arab) - Name
        let fmt_arab_name = CurrencyFormatter::try_new_name(prefs_arab, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_arab_name.format_fixed_decimal(&value),
            "١٢٬٣٤٥٫٦٧ جنيه مصري"
        );

        // 6. Locale extension override (latn) - Name
        let fmt_latn_name = CurrencyFormatter::try_new_name(prefs_latn, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_latn_name.format_fixed_decimal(&value),
            "12,345.67 جنيه مصري"
        );
    }

    #[test]
    pub fn test_en_us_cad() {
        let prefs = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "CAD"));
        let value = "12345.67".parse().unwrap();

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_symbol.format_fixed_decimal(&value), "CA$12,345.67");

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_symbol_narrow.format_fixed_decimal(&value), "$12,345.67");
    }

    #[test]
    pub fn test_code() {
        let prefs_en = locale!("en-US").into();
        let currency_usd = CurrencyCode(tinystr!(3, "USD"));
        let value = "12345.67".parse().unwrap();

        let fmt_code_en = CurrencyFormatter::try_new_code(prefs_en, &currency_usd).unwrap();
        assert_writeable_eq!(
            fmt_code_en.format_fixed_decimal(&value),
            "USD\u{a0}12,345.67"
        );

        let prefs_fr = locale!("fr-FR").into();
        let currency_eur = CurrencyCode(tinystr!(3, "EUR"));
        let fmt_code_fr = CurrencyFormatter::try_new_code(prefs_fr, &currency_eur).unwrap();
        assert_writeable_eq!(
            fmt_code_fr.format_fixed_decimal(&value),
            "12\u{202f}345,67\u{a0}EUR"
        );
    }

    #[test]
    pub fn test_name_fallback_to_iso_name() {
        let prefs_en = locale!("en-US").into();
        // Unknown currency code should gracefully fall back to IsoName instead of DataError(IdentifierNotFound)
        let currency_xyz = CurrencyCode(tinystr!(3, "XYZ"));
        let value = "12345.67".parse().unwrap();

        let fmt_name = CurrencyFormatter::try_new_name(prefs_en, &currency_xyz).unwrap();
        assert_writeable_eq!(fmt_name.format_fixed_decimal(&value), "12,345.67 XYZ");
    }
}
