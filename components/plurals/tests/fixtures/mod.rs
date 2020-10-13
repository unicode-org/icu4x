// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use fixed_decimal::FixedDecimal;
use icu_plurals::PluralOperands;
use serde::Deserialize;
use std::convert::TryInto;

/// Defines the data-driven test sets for the operands.
#[derive(Deserialize)]
pub struct OperandsTestSet {
    pub string: Vec<OperandsTest<String>>,
    pub int: Vec<OperandsTest<isize>>,
    pub floats: Vec<OperandsTest<f64>>,
    pub from_test: Vec<FromTestCase>,
}

/// A single test case verifying the conversion from [FixedDecimal] into
/// [PluralOperands].
#[derive(Debug, Deserialize)]
pub struct FromTestCase {
    /// The [FixedDecimal] input
    pub input: FixedDecimalInput,
    /// The expected value after conversion.
    pub expected: PluralOperandsInput,
}

/// A serialized representation of [FixedDecimal] in the data driven tests.
///
/// Use the `From` trait to convert into [FixedDecimal] in tests.
#[derive(Debug, Deserialize)]
pub struct FixedDecimalInput {
    /// Value supplied to [FixedDecimal::from] when constructing.
    from: i64,
    /// Value supplied to [FixedDecimal::multiplied_pow10] when constructing.
    pow10: i16,
}

impl From<&FixedDecimalInput> for FixedDecimal {
    fn from(f: &FixedDecimalInput) -> Self {
        FixedDecimal::from(f.from)
            .multiplied_pow10(f.pow10)
            .unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PluralOperandsInput {
    List((f64, u64, usize, usize, u64, u64)),
    Struct {
        n: Option<f64>,
        i: Option<u64>,
        v: Option<usize>,
        w: Option<usize>,
        f: Option<u64>,
        t: Option<u64>,
    },
    String(String),
    Number(isize),
}

impl From<PluralOperandsInput> for PluralOperands {
    fn from(input: PluralOperandsInput) -> Self {
        match input {
            PluralOperandsInput::List(operands) => Self {
                i: operands.1,
                v: operands.2,
                w: operands.3,
                f: operands.4,
                t: operands.5,
            },
            PluralOperandsInput::Struct { n, i, v, w, f, t } => Self {
                i: i.unwrap_or_else(|| n.unwrap_or(0_f64) as u64),
                v: v.unwrap_or(0),
                w: w.unwrap_or(0),
                f: f.unwrap_or(0),
                t: t.unwrap_or(0),
            },
            PluralOperandsInput::String(num) => num
                .parse()
                .expect("Failed to parse a number into operands."),
            PluralOperandsInput::Number(num) => num
                .try_into()
                .expect("Failed to parse a number into operands."),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OperandsTest<T> {
    pub input: T,
    pub output: PluralOperandsInput,
}

#[derive(Deserialize)]
pub struct RuleTest {
    pub rule: String,
    pub input: PluralOperandsInput,
    pub output: RuleTestOutput,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RuleTestOutput {
    Value(bool),
    Error(String),
}

#[derive(Deserialize)]
pub struct RuleTestSet(pub Vec<RuleTest>);
