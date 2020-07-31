use icu_pluralrules::PluralOperands;
use serde::Deserialize;
use std::convert::TryInto;

#[derive(Deserialize)]
pub struct OperandsTestSet {
    pub string: Vec<OperandsTest<String>>,
    pub int: Vec<OperandsTest<isize>>,
}

#[derive(Clone, Deserialize)]
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
                n: operands.0,
                i: operands.1,
                v: operands.2,
                w: operands.3,
                f: operands.4,
                t: operands.5,
            },
            PluralOperandsInput::Struct { n, i, v, w, f, t } => Self {
                n: n.unwrap_or(0_f64),
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

#[derive(Deserialize)]
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
