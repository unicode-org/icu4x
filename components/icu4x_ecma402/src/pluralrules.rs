//! TODO: License blurb.

//! Implements the traits found in [ecma402_traits::pluralrules].

use ecma402_traits;
use icu_pluralrules as ipr;
use icu_pluralrules::{PluralOperands, PluralRulesError};

pub(crate) mod internal {
    use ecma402_traits::pluralrules::options::Type;
    use ecma402_traits::pluralrules::Options;
    use icu_pluralrules::{PluralCategory, PluralOperands, PluralRuleType};

    // Converts the trait style option to an equivalent ICU4X type.
    pub fn to_icu4x_type(style: &Type) -> PluralRuleType {
        match style {
            Type::Ordinal => PluralRuleType::Ordinal,
            Type::Cardinal => PluralRuleType::Cardinal,
        }
    }

    pub fn to_icu4x_options(opts: Options) -> PluralOperands {
        PluralOperands {
            n: 0f64,
            // I'll need to understand these conversions a bit better.
            i: opts.minimum_integer_digits as u64,
            v: opts.maximum_fraction_digits as usize,
            w: opts.minimum_fraction_digits as usize,
            f: opts.minimum_integer_digits as u64,
            t: 0,
        }
    }

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

pub struct PluralRules {
    operands: PluralOperands,
    rep: ipr::PluralRules,
}

impl ecma402_traits::pluralrules::PluralRules for PluralRules {
    type Error = PluralRulesError;

    fn try_new<L>(l: L, opts: ecma402_traits::pluralrules::Options) -> Result<Self, Self::Error>
    where
        L: ecma402_traits::Locale,
        Self: Sized,
    {
        let locale: String = format!("{}", l);
        let locale: icu_locale::Locale = locale
            .parse()
            .expect("Converting from locale string to locale should always succeed");
        let rule_type = internal::to_icu4x_type(&opts.in_type);
        let operands = internal::to_icu4x_options(opts);

        // Oops, there is no slot in the ECMA 402 APIs to add the data provider.  What to do?
        let dp = icu_data_provider::InvariantDataProvider;
        let rep = ipr::PluralRules::try_new(locale.into(), rule_type, &dp)?;
        Ok(PluralRules { operands, rep })
    }

    fn select<W>(&self, number: f64, writer: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let mut op = self.operands.clone();
        op.n = number;
        let result = self.rep.select(op);
        write!(writer, "{}", internal::as_str(result))
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use ecma402_traits::pluralrules;
    use ecma402_traits::pluralrules::PluralRules;
    use icu_pluralrules::PluralRulesError;
    use icu_locale::Locale;

    #[test]
    fn plurals_per_locale() -> Result<(), PluralRulesError> {
        #[derive(Debug, Clone)]
        struct TestCase {
            locale: &'static str,
            opts: pluralrules::Options,
            numbers: Vec<f64>,
            expected: Vec<&'static str>,
        };
        let tests = vec![
            // Uncomment once data is available.
            //TestCase {
                //locale: "ar_EG",
                //opts: Default::default(),
                //numbers: vec![0 as f64, 1 as f64, 2 as f64, 6 as f64, 18 as f64],
                //expected: vec!["zero", "one", "two", "few", "many"],
            //},
            //TestCase {
                //locale: "ar_EG",
                //opts: pluralrules::Options {
                    //in_type: pluralrules::options::Type::Ordinal,
                    //..Default::default()
                //},
                //numbers: vec![0 as f64, 1 as f64, 2 as f64, 6 as f64, 18 as f64],
                //expected: vec!["other", "other", "other", "other", "other"],
            //},
            //TestCase {
                //locale: "sr_RS",
                //opts: Default::default(),
                //numbers: vec![0 as f64, 1 as f64, 2 as f64, 4 as f64, 6 as f64, 18 as f64],
                //expected: vec!["other", "one", "few", "few", "other", "other"],
            //},
            TestCase {
                locale: "sr_RS",
                opts: pluralrules::Options {
                    in_type: pluralrules::options::Type::Ordinal,
                    ..Default::default()
                },
                numbers: vec![0 as f64, 1 as f64, 2 as f64, 4 as f64, 6 as f64, 18 as f64],
                expected: vec!["other", "other", "other", "other", "other", "other"],
            },
        ];
        for test in tests {
            let locale: Locale = test.locale.parse().expect("locale exists");
            let plr = super::PluralRules::try_new(locale, test.clone().opts)?;
            let actual = test
                .numbers
                .iter()
                .map(|n| {
                    let mut result = String::new();
                    plr.select(*n, &mut result).unwrap();
                    result
                })
                .collect::<Vec<String>>();
            assert_eq!(test.expected, actual, "for test case: {:?}", &test);
        }
        Ok(())
    }
}
