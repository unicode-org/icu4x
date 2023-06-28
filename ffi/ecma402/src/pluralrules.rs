// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Implements the traits found in [`ecma402_traits::pluralrules`].

use icu::plurals as ipr;
use icu::plurals::Error;

pub(crate) mod internal {
    use ecma402_traits::pluralrules::options::Type;
    use ecma402_traits::pluralrules::Options;
    use icu::plurals::{PluralCategory, PluralOperands, PluralRuleType};
    use std::cmp::{max, min};

    // The difference between `first` and `second`, clamped on the bottom by zero.
    fn clamp_diff(first: usize, second: usize) -> usize {
        match first > second {
            true => first - second,
            false => 0,
        }
    }

    /// Converts the trait style option to an equivalent ICU4X type.
    pub fn to_icu4x_type(style: &Type) -> PluralRuleType {
        match style {
            Type::Ordinal => PluralRuleType::Ordinal,
            Type::Cardinal => PluralRuleType::Cardinal,
        }
    }

    /// Formats `n` as a string representation of fixed decimal, based on the supplied options.
    // TODO: Push this implementation into FixedDecimal.
    // See: https://github.com/unicode-org/icu4x/issues/340
    pub fn fixed_format(n: f64, opts: &Options) -> String {
        // n = 1234.5

        // 1234.5 -> "1234"
        // -1234.5 -> "1234"
        let int_part = format!("{}", n.abs().floor());

        // 5 -> ""
        // 0.4 -> "4"
        // 0.43 -> "43"
        let raw_frac_part = format!("{}", n.fract()); // 0.5
        let frac_part = if n.fract() == 0.0 {
            ""
        } else {
            #[allow(clippy::indexing_slicing)] // fract output shape
            &raw_frac_part[2..]
        };

        dbg!("--> frac={}, fracf={}", &frac_part, &raw_frac_part);
        dbg!("int_part='{}'; frac_part='{}'", &int_part, &frac_part);
        dbg!("opts={:?}", opts);

        // Limit the min and max display digits first by individual field.
        let display_integer_digits = max(int_part.len(), opts.minimum_integer_digits as usize);
        let display_fraction_digits = max(
            min(frac_part.len(), opts.maximum_fraction_digits as usize),
            opts.minimum_fraction_digits as usize,
        );
        let total_significant_digits = max(
            opts.minimum_significant_digits as usize,
            min(
                opts.maximum_significant_digits as usize,
                frac_part.len() + int_part.len(),
            ),
        );

        let significant_digits_in_fraction = clamp_diff(total_significant_digits, int_part.len());
        dbg!(
            "did={}; dfd={}; sd={}; rsd={}",
            display_integer_digits,
            display_fraction_digits,
            total_significant_digits,
            significant_digits_in_fraction
        );

        // Integer fragment.
        let leading_zeros = clamp_diff(display_integer_digits, int_part.len());
        let trailing_zeros_in_int_part = clamp_diff(int_part.len(), total_significant_digits);
        let i = std::iter::repeat('0')
            .take(leading_zeros)
            .chain(int_part.chars().take(total_significant_digits))
            .chain(std::iter::repeat('0').take(trailing_zeros_in_int_part));

        // Decimal dot is printed only if decimals will follow.
        let dd = match display_fraction_digits == 0 {
            true => "",
            false => ".",
        }
        .chars();

        // Fractional fragment.
        let f = frac_part
            .chars()
            // At most the number of significant digits.
            .take(significant_digits_in_fraction)
            // Fill the rest with zeros.
            .chain(std::iter::repeat('0'))
            // Take at most the number of fraction digits we're required to display.
            .take(display_fraction_digits);
        // "001234.500"
        let nstr = i.chain(dd).chain(f).collect::<String>();
        dbg!("nstr={}", &nstr);
        nstr
    }

    /// Converts the number to format into the operands representation.
    pub fn to_icu4x_operands(n: f64, opts: Options) -> PluralOperands {
        dbg!("n={}", n);
        let nstr = fixed_format(n, &opts);
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let ret = nstr.parse().unwrap();
        dbg!("ret={:?}\n---\n", &ret);
        ret
    }

    /// Expresses the [`PluralCategory`] as a `str`.
    pub fn as_str(c: PluralCategory) -> &'static str {
        match c {
            PluralCategory::Few => "few",
            PluralCategory::Many => "many",
            PluralCategory::One => "one",
            PluralCategory::Other => "other",
            PluralCategory::Two => "two",
            PluralCategory::Zero => "zero",
        }
    }

    #[cfg(test)]
    mod testing {
        use super::*;
        use ecma402_traits::pluralrules::options::Type;
        use icu::plurals::rules::RawPluralOperands;

        fn opt(
            minimum_integer_digits: u8,
            minimum_fraction_digits: u8,
            maximum_fraction_digits: u8,
            minimum_significant_digits: u8,
            maximum_significant_digits: u8,
        ) -> Options {
            Options {
                in_type: Type::Cardinal,
                minimum_integer_digits,
                minimum_fraction_digits,
                maximum_fraction_digits,
                minimum_significant_digits,
                maximum_significant_digits,
            }
        }

        #[test]
        fn string_conversion() {
            #[derive(Debug)]
            struct TestCase {
                n: f64,
                opts: Options,
                expected: &'static str,
            }
            let tests = vec![
                TestCase {
                    n: 0.0,
                    opts: opt(3, 2, 3, 3, 4),
                    expected: "000.00",
                },
                TestCase {
                    n: 1.5,
                    opts: opt(3, 2, 3, 3, 4),
                    expected: "001.50",
                },
                TestCase {
                    n: 1.5,
                    opts: opt(5, 5, 8, 3, 4),
                    expected: "00001.50000",
                },
                TestCase {
                    n: 123456.5,
                    opts: opt(5, 5, 8, 3, 4),
                    expected: "123400.00000",
                },
                TestCase {
                    n: 123456.5432112345,
                    opts: opt(5, 5, 8, 3, 4),
                    expected: "123400.00000000",
                },
            ];
            for test in tests {
                let actual = fixed_format(test.n, &test.opts);
                assert_eq!(test.expected, actual.as_str(), "test: {:?}", &test);
            }
        }

        #[test]
        fn format_conversion() {
            #[derive(Debug)]
            struct TestCase {
                n: f64,
                opts: Options,
                expected: PluralOperands,
            }
            let tests = vec![TestCase {
                n: 1.5,
                opts: Options {
                    in_type: Type::Cardinal,
                    minimum_integer_digits: 3,
                    minimum_fraction_digits: 2,
                    maximum_fraction_digits: 3,
                    minimum_significant_digits: 3,
                    maximum_significant_digits: 4,
                },
                expected: RawPluralOperands {
                    i: 1,
                    v: 2,
                    w: 1,
                    f: 50,
                    t: 5,
                    c: 0,
                }
                .into(),
            }];
            for test in tests {
                let actual = to_icu4x_operands(test.n, test.opts.clone());
                assert_eq!(test.expected, actual, "test: {:?}", &test);
            }
        }
    }
}

#[derive(Debug)]
pub struct PluralRules {
    opts: ecma402_traits::pluralrules::Options,
    rep: ipr::PluralRules,
}

impl ecma402_traits::pluralrules::PluralRules for PluralRules {
    type Error = Error;

    fn try_new<L>(l: L, opts: ecma402_traits::pluralrules::Options) -> Result<Self, Self::Error>
    where
        L: ecma402_traits::Locale,
        Self: Sized,
    {
        let rule_type = internal::to_icu4x_type(&opts.in_type);

        let rep = ipr::PluralRules::try_new(&crate::DataLocale::from_ecma_locale(l), rule_type)?;
        Ok(Self { opts, rep })
    }

    fn select<W>(&self, number: f64, writer: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let op = internal::to_icu4x_operands(number, self.opts.clone());
        let result = self.rep.category_for(op);
        write!(writer, "{}", internal::as_str(result))
    }
}

#[cfg(test)]
mod testing {
    use crate::testing::TestLocale;
    use ecma402_traits::pluralrules;
    use ecma402_traits::pluralrules::PluralRules;
    use icu::plurals::Error;

    #[test]
    fn plurals_per_locale() -> Result<(), Error> {
        #[derive(Debug, Clone)]
        struct TestCase {
            locale: TestLocale,
            opts: pluralrules::Options,
            numbers: Vec<f64>,
            expected: Vec<&'static str>,
        }
        let tests = vec![
            TestCase {
                locale: TestLocale("ar"),
                opts: Default::default(),
                numbers: vec![0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: vec!["zero", "one", "two", "few", "few", "many"],
            },
            TestCase {
                locale: TestLocale("ar"),
                opts: pluralrules::Options {
                    in_type: pluralrules::options::Type::Ordinal,
                    ..Default::default()
                },
                numbers: vec![0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: vec!["other", "other", "other", "other", "other", "other"],
            },
            TestCase {
                locale: TestLocale("sr"),
                opts: Default::default(),
                numbers: vec![0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: vec!["other", "one", "few", "other", "other", "other"],
            },
            TestCase {
                locale: TestLocale("sr"),
                opts: pluralrules::Options {
                    in_type: pluralrules::options::Type::Ordinal,
                    ..Default::default()
                },
                numbers: vec![0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: vec!["other", "other", "other", "other", "other", "other"],
            },
        ];
        for (i, test) in tests.into_iter().enumerate() {
            let plr = super::PluralRules::try_new(test.locale, test.opts)?;
            assert_eq!(
                test.numbers
                    .iter()
                    .map(|n| {
                        let mut result = String::new();
                        plr.select(*n, &mut result).unwrap();
                        result
                    })
                    .collect::<Vec<_>>(),
                test.expected,
                "for test case: {}",
                i
            );
        }
        Ok(())
    }
}
