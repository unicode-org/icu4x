// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
        let fmt = CurrencyFormatter::try_new_compact_symbol(prefs, &currency_code).unwrap();

        // Positive cases
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "$12K");

        let val_1299 = "1299".parse().unwrap();
        assert_writeable_eq!(fmt.format_fixed_decimal(&val_1299), "$1.3K");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-$12K");
    }

    #[test]
    pub fn test_fr_fr() {
        let prefs = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));
        let fmt = CurrencyFormatter::try_new_compact_symbol(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "12\u{a0}k\u{a0}€");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-12\u{a0}k\u{a0}€");
    }

    #[test]
    pub fn test_zh_cn() {
        let prefs = locale!("zh-CN").into();
        let currency_code = CurrencyCode(tinystr!(3, "CNY"));
        let fmt = CurrencyFormatter::try_new_compact_symbol(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "¥1.2万");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-¥1.2万");
    }

    #[test]
    pub fn test_ar_eg() {
        let prefs = locale!("ar-EG").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let fmt = CurrencyFormatter::try_new_compact_symbol(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        // TODO(#6064)
        assert_writeable_eq!(formatted_currency, "\u{200f}١٢\u{a0}ألف\u{a0}ج.م.\u{200f}"); //  "ج.م.١٢ألف"

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        // TODO(#6064)
        assert_writeable_eq!(
            formatted_currency,
            "\u{61c}-\u{200f}١٢\u{a0}ألف\u{a0}ج.م.\u{200f}"
        );
    }
    #[test]
    pub fn test_en_us_long() {
        let prefs = locale!("en-US").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::try_new_compact_long_name(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "12 thousand US dollars");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-12 thousand US dollars");
    }

    #[test]
    pub fn test_en_us_millions_long() {
        let prefs = locale!("en-US").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::try_new_compact_long_name(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "12 million US dollars");

        // Negative case
        let negative_value = "-12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-12 million US dollars");
    }

    #[test]
    pub fn test_fr_fr_long() {
        let prefs = locale!("fr-FR").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::try_new_compact_long_name(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "12 mille dollars des États-Unis");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-12 mille dollars des États-Unis");
    }

    #[test]
    pub fn test_fr_fr_millions_long() {
        let prefs = locale!("fr-FR").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::try_new_compact_long_name(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "12 millions dollars des États-Unis");

        // Negative case
        let negative_value = "-12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-12 millions dollars des États-Unis");
    }

    #[test]
    pub fn test_alpha_next_to_number_and_small_numbers() {
        let prefs = locale!("en-US").into();
        let usd = CurrencyCode(tinystr!(3, "USD"));
        let sek = CurrencyCode(tinystr!(3, "SEK"));

        let fmt_usd = CurrencyFormatter::try_new_compact_symbol(prefs, &usd).unwrap();
        let fmt_sek = CurrencyFormatter::try_new_compact_symbol(prefs, &sek).unwrap();

        // Small number (magnitude < 3, no compact suffix): should fall back cleanly
        let small_value = "123".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&small_value), "$123");
        assert_writeable_eq!(fmt_sek.format_fixed_decimal(&small_value), "SEK\u{a0}123");

        // Compact number with alphabetical currency code: should use alpha_next_to_number pattern with non-breaking space
        let compact_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(fmt_sek.format_fixed_decimal(&compact_value), "SEK\u{a0}12K");
    }

    #[test]
    pub fn test_tr35_rule_3_below_1000() {
        // Per UTS #35 (LDML Part 3: Numbers, Section 3.8, Rule 3 in Compact Number Formatting):
        // "If the element value of P is '0', then use the corresponding non-compact number formatting instead,"
        //
        // Specifically, for numbers below 1,000 (which map to the uncompacted pattern "0"):
        // * Per UTS #35: Suffix division and compact abbreviation steps (like "K" or "M") are skipped in favor
        //   of standard non-compact pattern structures.
        // * Per current ICU4X `CompactDecimalFormatter` implementation: Maximum fractional digits are trimmed to 0
        //   by default, stripping trailing zeros (so 12 USD formats as "$12" instead of "$12.00", and 990 USD as "$990").
        let prefs_en = locale!("en-US").into();
        let usd = CurrencyCode(tinystr!(3, "USD"));
        let fmt_usd = CurrencyFormatter::try_new_compact_symbol(prefs_en, &usd).unwrap();

        // Positive cases
        let val_12 = "12".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&val_12), "$12");

        let val_990 = "990".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&val_990), "$990");

        let val_1200 = "1200".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&val_1200), "$1.2K");

        // Negative cases (pattern selection and Rule 3 fallback operate on absolute magnitude)
        let val_neg_12 = "-12".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&val_neg_12), "-$12");

        let val_neg_990 = "-990".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&val_neg_990), "-$990");

        let val_neg_1200 = "-1200".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&val_neg_1200), "-$1.2K");
    }

    #[test]
    pub fn test_compact_name_and_compact_long_symbol() {
        let prefs = locale!("en-US").into();
        let usd = CurrencyCode(tinystr!(3, "USD"));

        let fmt_compact_name = CurrencyFormatter::try_new_compact_name(prefs, &usd).unwrap();
        let fmt_compact_long_symbol =
            CurrencyFormatter::try_new_compact_long_symbol(prefs, &usd).unwrap();

        let val = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_compact_name.format_fixed_decimal(&val),
            "12K US dollars"
        );
        assert_writeable_eq!(
            fmt_compact_long_symbol.format_fixed_decimal(&val),
            "$12 thousand"
        );
    }
}
