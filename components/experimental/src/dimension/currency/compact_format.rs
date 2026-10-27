// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(test)]
mod tests {
    use icu::locale::locale;
    use icu::locale::preferences::extensions::unicode::keywords::currency;
    use icu_decimal::CompactDecimalFormatter;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{
        formatter::CurrencyFormatter,
        options::{CurrencyFormatterOptions, CurrencyUsage},
    };

    /// Formats `value` with `formatter` and checks the result.
    #[track_caller]
    fn check(formatter: &CurrencyFormatter<CompactDecimalFormatter>, value: &str, expected: &str) {
        let decimal = value.parse().unwrap();
        assert_writeable_eq!(formatter.format_fixed_decimal(&decimal), expected);
    }

    fn accounting() -> CurrencyFormatterOptions {
        CurrencyFormatterOptions {
            usage: CurrencyUsage::Accounting,
            ..Default::default()
        }
    }

    #[test]
    pub fn test_en_us() {
        let prefs = locale!("en-US").into();
        let usd = currency!("USD");
        let fmt =
            CurrencyFormatter::try_new_compact_symbol(prefs, usd, Default::default()).unwrap();
        let fmt_accounting =
            CurrencyFormatter::try_new_compact_symbol(prefs, usd, accounting()).unwrap();
        let fmt_narrow =
            CurrencyFormatter::try_new_compact_symbol_narrow(prefs, usd, Default::default())
                .unwrap();
        let fmt_narrow_accounting =
            CurrencyFormatter::try_new_compact_symbol_narrow(prefs, usd, accounting()).unwrap();

        check(&fmt, "12345.67", "$12K");
        check(&fmt, "-12345.67", "-$12K");
        check(&fmt, "1299", "$1.3K");
        check(&fmt_accounting, "12345.67", "$12K");
        check(&fmt_accounting, "-12345.67", "($12K)");
        check(&fmt_narrow, "12345.67", "$12K");
        check(&fmt_narrow, "-12345.67", "-$12K");
        check(&fmt_narrow_accounting, "12345.67", "$12K");
        check(&fmt_narrow_accounting, "-12345.67", "($12K)");
    }

    #[test]
    pub fn test_fr_fr() {
        let fmt = CurrencyFormatter::try_new_compact_symbol(
            locale!("fr-FR").into(),
            currency!("EUR"),
            Default::default(),
        )
        .unwrap();
        check(&fmt, "12345.67", "12\u{a0}k\u{a0}€");
        check(&fmt, "-12345.67", "-12\u{a0}k\u{a0}€");
    }

    #[test]
    pub fn test_zh_cn() {
        let fmt = CurrencyFormatter::try_new_compact_symbol(
            locale!("zh-CN").into(),
            currency!("CNY"),
            Default::default(),
        )
        .unwrap();
        check(&fmt, "12345.67", "¥1.2万");
        check(&fmt, "-12345.67", "-¥1.2万");
    }

    #[test]
    pub fn test_ar_eg() {
        let fmt = CurrencyFormatter::try_new_compact_symbol(
            locale!("ar-EG").into(),
            currency!("EGP"),
            Default::default(),
        )
        .unwrap();
        // TODO(#6064)
        check(&fmt, "12345.67", "\u{200f}١٢\u{a0}ألف\u{a0}ج.م.\u{200f}"); // "١٢ ألف ج.م."
        // TODO(#6064)
        check(
            &fmt,
            "-12345.67",
            "\u{61c}-\u{200f}١٢\u{a0}ألف\u{a0}ج.م.\u{200f}",
        );
    }

    #[test]
    pub fn test_code() {
        let prefs = locale!("en-US").into();
        let usd = currency!("USD");
        let fmt = CurrencyFormatter::try_new_compact_code(prefs, usd, Default::default()).unwrap();
        let fmt_accounting =
            CurrencyFormatter::try_new_compact_code(prefs, usd, accounting()).unwrap();
        let fmt_long =
            CurrencyFormatter::try_new_compact_long_code(prefs, usd, Default::default()).unwrap();
        let fmt_long_accounting =
            CurrencyFormatter::try_new_compact_long_code(prefs, usd, accounting()).unwrap();

        check(&fmt, "12345.67", "USD\u{a0}12K");
        check(&fmt, "-12345.67", "-USD\u{a0}12K");
        check(&fmt_accounting, "12345.67", "USD\u{a0}12K");
        check(&fmt_accounting, "-12345.67", "(USD\u{a0}12K)");
        check(&fmt_long, "12345.67", "USD\u{a0}12 thousand");
        check(&fmt_long, "-12345.67", "-USD\u{a0}12 thousand");
        check(&fmt_long_accounting, "12345.67", "USD\u{a0}12 thousand");
        check(&fmt_long_accounting, "-12345.67", "(USD\u{a0}12 thousand)");
    }

    #[test]
    pub fn test_name() {
        let fmt =
            CurrencyFormatter::try_new_compact_name(locale!("en-US").into(), currency!("USD"))
                .unwrap();
        check(&fmt, "12345.67", "12K US dollars");
    }

    #[test]
    pub fn test_long_name() {
        let usd = currency!("USD");

        let en =
            CurrencyFormatter::try_new_compact_long_name(locale!("en-US").into(), usd).unwrap();
        check(&en, "12345.67", "12 thousand US dollars");
        check(&en, "-12345.67", "-12 thousand US dollars");
        check(&en, "12345000.67", "12 million US dollars");
        check(&en, "-12345000.67", "-12 million US dollars");

        let fr =
            CurrencyFormatter::try_new_compact_long_name(locale!("fr-FR").into(), usd).unwrap();
        check(&fr, "12345.67", "12 mille dollars des États-Unis");
        check(&fr, "-12345.67", "-12 mille dollars des États-Unis");
        check(&fr, "12345000.67", "12 millions dollars des États-Unis");
        check(&fr, "-12345000.67", "-12 millions dollars des États-Unis");
    }

    #[test]
    pub fn test_long_symbol() {
        let prefs = locale!("en-US").into();
        let usd = currency!("USD");
        let fmt =
            CurrencyFormatter::try_new_compact_long_symbol(prefs, usd, Default::default()).unwrap();
        let fmt_accounting =
            CurrencyFormatter::try_new_compact_long_symbol(prefs, usd, accounting()).unwrap();

        check(&fmt, "12345.67", "$12 thousand");
        check(&fmt, "-12345.67", "-$12 thousand");
        check(&fmt_accounting, "12345.67", "$12 thousand");
        check(&fmt_accounting, "-12345.67", "($12 thousand)");
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
        let fmt_usd = CurrencyFormatter::try_new_compact_symbol(
            locale!("en-US").into(),
            currency!("USD"),
            Default::default(),
        )
        .unwrap();

        check(&fmt_usd, "12", "$12");
        check(&fmt_usd, "990", "$990");
        check(&fmt_usd, "1200", "$1.2K");

        // Negative cases (pattern selection and Rule 3 fallback operate on absolute magnitude)
        check(&fmt_usd, "-12", "-$12");
        check(&fmt_usd, "-990", "-$990");
        check(&fmt_usd, "-1200", "-$1.2K");
    }

    #[test]
    pub fn test_alpha_next_to_number_and_small_numbers() {
        let prefs = locale!("en-US").into();
        let usd = currency!("USD");
        let sek = currency!("SEK");
        let fmt_usd =
            CurrencyFormatter::try_new_compact_symbol(prefs, usd, Default::default()).unwrap();
        let fmt_sek =
            CurrencyFormatter::try_new_compact_symbol(prefs, sek, Default::default()).unwrap();

        // Small number (magnitude < 3, no compact suffix): should fall back cleanly
        check(&fmt_usd, "123", "$123");
        check(&fmt_sek, "123", "SEK\u{a0}123");

        // Compact number with alphabetical currency code: should use alpha_next_to_number pattern with non-breaking space
        check(&fmt_sek, "12345.67", "SEK\u{a0}12K");
    }

    #[test]
    pub fn test_pt_pt_escudo_compact() {
        let prefs = locale!("pt-PT").into();
        let pte = currency!("PTE");
        let options = Default::default();

        // `PTE` has an empty symbol in `pt-PT` (U+200B in CLDR), so the compact symbol
        // formatters show the ISO code (see `EmptySymbolFallback`, TODO(#8454)).
        let fmt = CurrencyFormatter::try_new_compact_symbol(prefs, pte, options).unwrap();
        let fmt_narrow =
            CurrencyFormatter::try_new_compact_symbol_narrow(prefs, pte, options).unwrap();
        let fmt_long = CurrencyFormatter::try_new_compact_long_symbol(prefs, pte, options).unwrap();
        let fmt_long_narrow =
            CurrencyFormatter::try_new_compact_long_symbol_narrow(prefs, pte, options).unwrap();
        check(&fmt, "1299", "1,3 mil PTE");
        check(&fmt_narrow, "1299", "1,3 mil PTE");
        check(&fmt_long, "1299", "1,3 mil PTE");
        check(&fmt_long_narrow, "1299", "1,3 mil PTE");

        // The code and name formatters don't use the symbol, so they are not affected.
        let fmt_code = CurrencyFormatter::try_new_compact_code(prefs, pte, options).unwrap();
        let fmt_long_code =
            CurrencyFormatter::try_new_compact_long_code(prefs, pte, options).unwrap();
        let fmt_name = CurrencyFormatter::try_new_compact_name(prefs, pte).unwrap();
        let fmt_long_name = CurrencyFormatter::try_new_compact_long_name(prefs, pte).unwrap();
        check(&fmt_code, "1299", "1,3 mil PTE");
        check(&fmt_long_code, "1299", "1,3 mil PTE");
        check(&fmt_name, "1299", "1,3 mil escudos portugueses");
        check(&fmt_long_name, "1299", "1,3 mil escudos portugueses");
    }
}
