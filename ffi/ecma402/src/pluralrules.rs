// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Implements the traits found in [`ecma402_traits::pluralrules`].

use icu::plurals as ipr;

pub(crate) mod internal {
    use ecma402_traits::pluralrules::options::Type;
    use ecma402_traits::pluralrules::Options;
    use fixed_decimal::FixedDecimal;
    use icu::plurals::{PluralCategory, PluralOperands, PluralRuleType};

    /// Converts the trait style option to an equivalent ICU4X type.
    pub fn to_icu4x_type(style: &Type) -> PluralRuleType {
        match style {
            Type::Ordinal => PluralRuleType::Ordinal,
            Type::Cardinal => PluralRuleType::Cardinal,
        }
    }

    /// Converts the number to format into the operands representation.
    pub fn to_icu4x_operands(n: f64, _opts: Options) -> PluralOperands {
        dbg!("n={}", n);
        (&FixedDecimal::try_from_f64(n, fixed_decimal::FloatPrecision::Floating).unwrap()).into()
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
}

#[derive(Debug)]
pub struct PluralRules {
    opts: ecma402_traits::pluralrules::Options,
    rep: ipr::PluralRules,
}

impl ecma402_traits::pluralrules::PluralRules for PluralRules {
    type Error = icu_provider::DataError;

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
    use icu_provider::DataError;

    #[test]
    fn plurals_per_locale() -> Result<(), DataError> {
        #[derive(Debug, Clone)]
        struct TestCase {
            locale: TestLocale,
            opts: pluralrules::Options,
            numbers: &'static [f64],
            expected: &'static [&'static str],
        }
        let tests = [
            TestCase {
                locale: TestLocale("ar"),
                opts: Default::default(),
                numbers: &[0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: &["zero", "one", "two", "few", "few", "many"],
            },
            TestCase {
                locale: TestLocale("ar"),
                opts: pluralrules::Options {
                    in_type: pluralrules::options::Type::Ordinal,
                    ..Default::default()
                },
                numbers: &[0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: &["other", "other", "other", "other", "other", "other"],
            },
            TestCase {
                locale: TestLocale("sr"),
                opts: Default::default(),
                numbers: &[0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: &["other", "one", "few", "other", "other", "other"],
            },
            TestCase {
                locale: TestLocale("sr"),
                opts: pluralrules::Options {
                    in_type: pluralrules::options::Type::Ordinal,
                    ..Default::default()
                },
                numbers: &[0.0, 1.0, 2.0, 5.0, 6.0, 18.0],
                expected: &["other", "other", "other", "other", "other", "other"],
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
