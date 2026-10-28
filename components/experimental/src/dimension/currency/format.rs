// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: add more tests for this module to cover more locales & currencies.
#[cfg(test)]
mod tests {
    use icu::locale::locale;
    use icu::locale::preferences::extensions::unicode::keywords::currency;
    use icu_decimal::DecimalFormatter;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{
        formatter::CurrencyFormatter,
        options::{CurrencyFormatterOptions, CurrencyUsage},
    };

    /// Formats `value` with `formatter` and checks the result.
    #[track_caller]
    fn check(formatter: &CurrencyFormatter<DecimalFormatter>, value: &str, expected: &str) {
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

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, usd, Default::default()).unwrap();
        let fmt_symbol_accounting =
            CurrencyFormatter::try_new_symbol(prefs, usd, accounting()).unwrap();
        check(&fmt_symbol, "12345.67", "$12,345.67");
        check(&fmt_symbol_accounting, "12345.67", "$12,345.67");
        check(&fmt_symbol, "-12345.67", "-$12,345.67");
        check(&fmt_symbol_accounting, "-12345.67", "($12,345.67)");
        check(&fmt_symbol, "123", "$123.00");
        check(&fmt_symbol, "123.4567", "$123.46");

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, usd, Default::default()).unwrap();
        let fmt_symbol_narrow_accounting =
            CurrencyFormatter::try_new_symbol_narrow(prefs, usd, accounting()).unwrap();
        check(&fmt_symbol_narrow, "12345.67", "$12,345.67");
        check(&fmt_symbol_narrow_accounting, "12345.67", "$12,345.67");
        check(&fmt_symbol_narrow, "-12345.67", "-$12,345.67");
        check(&fmt_symbol_narrow_accounting, "-12345.67", "($12,345.67)");
        check(&fmt_symbol_narrow, "123", "$123.00");
        check(&fmt_symbol_narrow, "123.4567", "$123.46");

        // Long / Name
        let fmt_name = CurrencyFormatter::try_new_name(prefs, usd).unwrap();
        check(&fmt_name, "12345.67", "12,345.67 US dollars");
        check(&fmt_name, "-12345.67", "-12,345.67 US dollars");
        check(&fmt_name, "123", "123.00 US dollars");
        check(&fmt_name, "123.4567", "123.46 US dollars");
    }

    #[test]
    pub fn test_pt_pt_escudo() {
        let prefs = locale!("pt-PT").into();
        let pte = currency!("PTE");

        // In CLDR, `pt-PT` sets the decimal separator override for PTE to '$' and the
        // currency symbol to U+200B (ZERO WIDTH SPACE), as the cifrão ('$') serves as
        // the decimal separator (e.g. "12,345$67"). Datagen stores that symbol as the
        // empty string.
        //
        // Because the locale's standard currency pattern ("#,##0.00 ¤") contains a
        // non-breaking space (U+00A0) before the symbol placeholder ('¤'), formatting
        // with the raw symbol would produce a trailing space (tracked in CLDR-19771).
        // To resolve this, when the currency symbol is empty, `CurrencyFormatter` uses
        // `noCurrency` formatting so that the amount formats cleanly without any
        // placeholder or trailing space ("12,345$67").
        let escudo = CurrencyFormatter::try_new_symbol(prefs, pte, Default::default()).unwrap();
        check(&escudo, "12345.67", "12,345$67");

        let escudo_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, pte, Default::default()).unwrap();
        // Narrow symbol for PTE is not defined in pt-PT CLDR data, so the narrow data entry
        // carries the standard symbol (#8486), matching `try_new_symbol` above.
        check(&escudo_narrow, "12345.67", "12,345$67");

        let escudo_code = CurrencyFormatter::try_new_code(prefs, pte, Default::default()).unwrap();
        check(&escudo_code, "12345.67", "12,345$67 PTE");

        let escudo_name = CurrencyFormatter::try_new_name(prefs, pte).unwrap();
        check(&escudo_name, "12345.67", "12,345$67 escudos portugueses");

        let escudo_no_currency =
            CurrencyFormatter::try_new_no_currency(prefs, pte, Default::default()).unwrap();
        check(&escudo_no_currency, "12345.67", "12,345$67");

        // Currencies the locale does not single out keep its standard separators.
        let euro =
            CurrencyFormatter::try_new_symbol(prefs, currency!("EUR"), Default::default()).unwrap();
        check(&euro, "12345.67", "12 345,67 €");
    }

    #[test]
    pub fn test_fr_fr() {
        let prefs = locale!("fr-FR").into();
        let eur = currency!("EUR");

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, eur, Default::default()).unwrap();
        check(&fmt_symbol, "12345.67", "12\u{202f}345,67\u{a0}€");
        check(&fmt_symbol, "-12345.67", "-12\u{202f}345,67\u{a0}€");

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, eur, Default::default()).unwrap();
        check(&fmt_symbol_narrow, "12345.67", "12\u{202f}345,67\u{a0}€");
        check(&fmt_symbol_narrow, "-12345.67", "-12\u{202f}345,67\u{a0}€");

        // Long / Name
        let fmt_name = CurrencyFormatter::try_new_name(prefs, eur).unwrap();
        check(&fmt_name, "12345.67", "12\u{202f}345,67 euros");
        check(&fmt_name, "-12345.67", "-12\u{202f}345,67 euros");
    }

    #[test]
    pub fn test_ar_eg() {
        let prefs = locale!("ar-EG").into();
        let egp = currency!("EGP");

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, egp, Default::default()).unwrap();
        // TODO(#6064)
        check(
            &fmt_symbol,
            "12345.67",
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}",
        );
        // TODO(#6064)
        check(
            &fmt_symbol,
            "-12345.67",
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}",
        );

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, egp, Default::default()).unwrap();
        // TODO(#6064)
        check(&fmt_symbol_narrow, "12345.67", "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£");
        // TODO(#6064)
        check(
            &fmt_symbol_narrow,
            "-12345.67",
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£",
        );

        // Long / Name
        let fmt_name = CurrencyFormatter::try_new_name(prefs, egp).unwrap();
        check(&fmt_name, "12345.67", "١٢٬٣٤٥٫٦٧ جنيه مصري");
        check(&fmt_name, "-12345.67", "\u{61c}-١٢٬٣٤٥٫٦٧ جنيه مصري");
    }

    #[test]
    pub fn test_usd_in_fr_fr() {
        let prefs = locale!("fr-FR").into();
        let usd = currency!("USD");

        // Short / Symbol USD in fr-FR should be US$ or $US
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, usd, Default::default()).unwrap();
        check(&fmt_symbol, "12345.67", "12\u{202f}345,67\u{a0}$US");

        // Narrow / Symbol Narrow USD in fr-FR should be $
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, usd, Default::default()).unwrap();
        check(&fmt_symbol_narrow, "12345.67", "12\u{202f}345,67\u{a0}$");
    }

    #[test]
    pub fn test_explicit_negative_pattern() {
        // `ar` with `latn` numbers has an explicit standard negative subpattern:
        // - Escaped: "\u{200f}#,##0.00\u{a0}\u{a4};\u{200f}-#,##0.00\u{a0}\u{a4}"
        // - Literal: "‏#,##0.00 ¤;‏-#,##0.00 ¤"
        // The explicit negative pattern is used as-is for negative values instead of
        // prepending a minus sign to the positive pattern.
        let fmt_symbol = CurrencyFormatter::try_new_symbol(
            locale!("ar-EG-u-nu-latn").into(),
            currency!("EGP"),
            Default::default(),
        )
        .unwrap();
        // Expected output:
        // - Escaped: "\u{200f}\u{200e}-12,345.67\u{a0}\u{62c}.\u{645}.\u{200f}"
        // - Literal: "‏\u{200e}-12,345.67 ج.م.‏"
        check(&fmt_symbol, "-12345.67", "‏\u{200e}-12,345.67 ج.م.‏");

        // `de-CH` places the minus sign between the currency symbol and the number:
        // - Escaped: "\u{a4}\u{a0}#,##0.00;\u{a4}-#,##0.00"
        // - Literal: "¤ #,##0.00;¤-#,##0.00"
        // This behavior can only be expressed via an explicit negative pattern.
        let fmt_symbol = CurrencyFormatter::try_new_symbol(
            locale!("de-CH").into(),
            currency!("CHF"),
            Default::default(),
        )
        .unwrap();
        check(&fmt_symbol, "-12345.67", "CHF-12'345.67");
    }

    #[test]
    pub fn test_numbering_system_override() {
        let prefs_arab = locale!("ar-EG").into();
        let prefs_latn = locale!("ar-EG-u-nu-latn").into();
        let egp = currency!("EGP");

        // 1. Default numbering system (arab) - Symbol
        let fmt_arab_symbol =
            CurrencyFormatter::try_new_symbol(prefs_arab, egp, Default::default()).unwrap();
        check(
            &fmt_arab_symbol,
            "12345.67",
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}",
        );

        // 2. Locale extension override (latn) - Symbol
        let fmt_latn_symbol =
            CurrencyFormatter::try_new_symbol(prefs_latn, egp, Default::default()).unwrap();
        check(
            &fmt_latn_symbol,
            "12345.67",
            "\u{200f}12,345.67\u{a0}ج.م.\u{200f}",
        );

        // 3. Default numbering system (arab) - Symbol Narrow
        let fmt_arab_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs_arab, egp, Default::default()).unwrap();
        check(
            &fmt_arab_symbol_narrow,
            "12345.67",
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£",
        );

        // 4. Locale extension override (latn) - Symbol Narrow
        let fmt_latn_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs_latn, egp, Default::default()).unwrap();
        check(
            &fmt_latn_symbol_narrow,
            "12345.67",
            "\u{200f}12,345.67\u{a0}E£",
        );

        // 5. Default numbering system (arab) - Name
        let fmt_arab_name = CurrencyFormatter::try_new_name(prefs_arab, egp).unwrap();
        check(&fmt_arab_name, "12345.67", "١٢٬٣٤٥٫٦٧ جنيه مصري");

        // 6. Locale extension override (latn) - Name
        let fmt_latn_name = CurrencyFormatter::try_new_name(prefs_latn, egp).unwrap();
        check(&fmt_latn_name, "12345.67", "12,345.67 جنيه مصري");
    }

    #[test]
    pub fn test_en_us_cad() {
        let prefs = locale!("en-US").into();
        let cad = currency!("CAD");

        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, cad, Default::default()).unwrap();
        check(&fmt_symbol, "12345.67", "CA$12,345.67");

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, cad, Default::default()).unwrap();
        check(&fmt_symbol_narrow, "12345.67", "$12,345.67");
    }

    #[test]
    pub fn test_code() {
        let prefs_en = locale!("en-US").into();
        let usd = currency!("USD");

        let fmt_code_en =
            CurrencyFormatter::try_new_code(prefs_en, usd, Default::default()).unwrap();
        let fmt_code_en_accounting =
            CurrencyFormatter::try_new_code(prefs_en, usd, accounting()).unwrap();
        check(&fmt_code_en, "12345.67", "USD\u{a0}12,345.67");
        check(&fmt_code_en_accounting, "12345.67", "USD\u{a0}12,345.67");
        check(&fmt_code_en, "-12345.67", "-USD\u{a0}12,345.67");
        check(&fmt_code_en_accounting, "-12345.67", "(USD\u{a0}12,345.67)");

        let fmt_code_fr = CurrencyFormatter::try_new_code(
            locale!("fr-FR").into(),
            currency!("EUR"),
            Default::default(),
        )
        .unwrap();
        check(&fmt_code_fr, "12345.67", "12\u{202f}345,67\u{a0}EUR");
    }

    #[test]
    pub fn test_name_fallback_to_iso_code_name() {
        // Unknown currency code should gracefully fall back to IsoCodeName instead of DataError(IdentifierNotFound)
        let fmt_name =
            CurrencyFormatter::try_new_name(locale!("en-US").into(), currency!("XYZ")).unwrap();
        check(&fmt_name, "12345.67", "12,345.67 XYZ");
    }

    #[test]
    pub fn test_symbol_fallback_to_iso_code_symbol() {
        let prefs = locale!("en-US").into();
        // Unknown currency code should gracefully fall back to IsoCodeSymbol instead of DataError(IdentifierNotFound)
        let xyz = currency!("XYZ");

        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, xyz, Default::default()).unwrap();
        check(&fmt_symbol, "12345.67", "XYZ\u{a0}12,345.67");

        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, xyz, Default::default()).unwrap();
        check(&fmt_symbol_narrow, "12345.67", "XYZ\u{a0}12,345.67");
    }

    #[test]
    pub fn test_narrow_symbol_fallback_to_standard_symbol() {
        // Test currency where standard symbol is defined, but narrow symbol is not in CLDR.
        // Per UTS #35 lateral inheritance, narrow symbols fall back to standard symbols before
        // the ISO code. The fallback is resolved at datagen time, so the narrow data entry
        // already carries the standard symbol.
        let prefs = locale!("fr-FR").into();
        let frf = currency!("FRF");

        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, frf, Default::default()).unwrap();
        let fmt_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, frf, Default::default()).unwrap();

        // Both use "F" (standard symbol) instead of falling back to ISO code "FRF"
        check(&fmt_symbol, "12345.67", "12\u{202f}345,67\u{a0}F");
        check(&fmt_narrow, "12345.67", "12\u{202f}345,67\u{a0}F");
    }

    #[test]
    pub fn test_jpy() {
        let prefs = locale!("en-US").into();
        let jpy = currency!("JPY");

        // JPY has 0 decimals (defined in global CurrencyFractionsV1)
        // Short / Symbol
        let fmt_symbol = CurrencyFormatter::try_new_symbol(prefs, jpy, Default::default()).unwrap();
        check(&fmt_symbol, "12345.67", "¥12,346");

        // Narrow / Symbol Narrow
        let fmt_symbol_narrow =
            CurrencyFormatter::try_new_symbol_narrow(prefs, jpy, Default::default()).unwrap();
        check(&fmt_symbol_narrow, "12345.67", "¥12,346");

        // Long / Name
        let fmt_name = CurrencyFormatter::try_new_name(prefs, jpy).unwrap();
        check(&fmt_name, "12345.67", "12,346 Japanese yen");
    }

    #[test]
    pub fn test_no_currency() {
        let prefs_en = locale!("en-US").into();
        let usd = currency!("USD");
        let jpy = currency!("JPY");
        let bhd = currency!("BHD");

        // USD: 2 decimal places
        let fmt_usd =
            CurrencyFormatter::try_new_no_currency(prefs_en, usd, Default::default()).unwrap();
        let fmt_usd_acc =
            CurrencyFormatter::try_new_no_currency(prefs_en, usd, accounting()).unwrap();
        check(&fmt_usd, "12345.67", "12,345.67");
        check(&fmt_usd, "-12345.67", "-12,345.67");
        check(&fmt_usd_acc, "-12345.67", "(12,345.67)");
        check(&fmt_usd, "123", "123.00");

        // JPY: 0 decimal places
        let fmt_jpy =
            CurrencyFormatter::try_new_no_currency(prefs_en, jpy, Default::default()).unwrap();
        check(&fmt_jpy, "12345.67", "12,346");

        // BHD: 3 decimal places
        let fmt_bhd =
            CurrencyFormatter::try_new_no_currency(prefs_en, bhd, Default::default()).unwrap();
        check(&fmt_bhd, "12345.67", "12,345.670");

        // French locale EUR: 2 decimal places with French grouping
        let prefs_fr = locale!("fr-FR").into();
        let eur = currency!("EUR");
        let fmt_eur_fr =
            CurrencyFormatter::try_new_no_currency(prefs_fr, eur, Default::default()).unwrap();
        check(&fmt_eur_fr, "12345.67", "12\u{202f}345,67");

        // Arabic locale ar-EG EGP: 2 decimal places with Arabic-Indic digits, BiDi marks and accounting format
        let prefs_ar = locale!("ar-EG").into();
        let egp = currency!("EGP");
        let fmt_egp_ar =
            CurrencyFormatter::try_new_no_currency(prefs_ar, egp, Default::default()).unwrap();
        let fmt_egp_ar_acc =
            CurrencyFormatter::try_new_no_currency(prefs_ar, egp, accounting()).unwrap();
        check(&fmt_egp_ar, "12345.67", "١٢\u{66c}٣٤٥\u{66b}٦٧");
        check(
            &fmt_egp_ar_acc,
            "-12345.67",
            "\u{061c}-١٢\u{66c}٣٤٥\u{66b}٦٧",
        );
    }
}
