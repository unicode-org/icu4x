// This file is part of ICU4 and_or: todo!(), polarity: todo!(), operand: todo!(), modulo: todo!(), range_list: todo!()  and_or: todo!(), polarity: todo!(), operand: todo!(), modulo: todo!(), range_list: todo!() X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)]

use crate::rules::reference;
use core::{convert::TryInto, fmt, mem, str::FromStr};
use icu_provider::yoke::{self, *};
use zerovec::{
    ule::{custom::EncodeAsVarULE, AsULE, PlainOldULE, VarULE, ULE},
    {VarZeroVec, ZeroVec},
};

#[derive(Debug, Yokeable, ZeroCopyFrom)]
pub struct Rule<'data>(pub VarZeroVec<'data, RelationULE>);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum AndOr {
    Or,
    And,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Polarity {
    Negative,
    Positive,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Operand {
    N,
    I,
    V,
    W,
    F,
    T,
    C,
    E,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RangeOrValue {
    Range(u32, u32), // XXX: Can we get away from smaller?
    Value(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Relation<'data> {
    pub(crate) and_or: AndOr,
    pub(crate) polarity: Polarity,
    pub(crate) operand: Operand,
    pub(crate) modulo: u32,
    pub(crate) range_list: ZeroVec<'data, RangeOrValue>,
}

/////

impl Operand {
    fn idx(self) -> u8 {
        self as u8
    }

    fn from_idx(idx: u8) -> Self {
        debug_assert!(idx < 8);
        // safe due to the repr(u8)
        unsafe { mem::transmute(idx) }
    }
}

impl From<&reference::ast::Rule> for Rule<'_> {
    fn from(input: &reference::ast::Rule) -> Self {
        let mut relations: alloc::vec::Vec<Relation> = alloc::vec![];

        for (i_or, and_condition) in input.condition.0.iter().enumerate() {
            for (i_and, relation) in and_condition.0.iter().enumerate() {
                let range_list = relation
                    .range_list
                    .0
                    .iter()
                    .map(Into::into)
                    .collect::<alloc::vec::Vec<_>>();

                let and_or = if i_or > 0 && i_and == 0 {
                    AndOr::Or
                } else {
                    AndOr::And
                };

                relations.push(Relation {
                    and_or,
                    polarity: relation.operator.into(),
                    operand: relation.expression.operand.into(),
                    modulo: get_modulo(&relation.expression.modulus),
                    range_list: ZeroVec::clone_from_slice(&range_list),
                })
            }
        }

        Self(VarZeroVec::from(relations.as_slice()))
    }
}

impl From<&Rule<'_>> for reference::ast::Rule {
    fn from(input: &Rule<'_>) -> Self {
        let mut or_conditions: alloc::vec::Vec<reference::ast::AndCondition> = alloc::vec![];
        let mut and_conditions: alloc::vec::Vec<reference::ast::Relation> = alloc::vec![];
        for rel in input.0.iter() {
            let rel = rel.as_relation();
            let list = rel.range_list.iter().map(Into::into).collect();
            let relation = reference::ast::Relation {
                expression: (rel.operand, rel.modulo).into(),
                operator: rel.polarity.into(),
                range_list: reference::ast::RangeList(list),
            };

            if rel.and_or == AndOr::And {
                and_conditions.push(relation);
            } else {
                or_conditions.push(reference::ast::AndCondition(and_conditions));
                and_conditions = alloc::vec![relation];
            }
        }

        if !and_conditions.is_empty() {
            or_conditions.push(reference::ast::AndCondition(and_conditions));
        }

        Self {
            condition: reference::ast::Condition(or_conditions),
            samples: None,
        }
    }
}

impl From<reference::ast::Operator> for Polarity {
    fn from(op: reference::ast::Operator) -> Self {
        match op {
            reference::ast::Operator::Eq => Polarity::Positive,
            reference::ast::Operator::NotEq => Polarity::Negative,
        }
    }
}

impl From<Polarity> for reference::ast::Operator {
    fn from(pol: Polarity) -> Self {
        match pol {
            Polarity::Negative => reference::ast::Operator::NotEq,
            Polarity::Positive => reference::ast::Operator::Eq,
        }
    }
}

impl From<reference::ast::Operand> for Operand {
    fn from(op: reference::ast::Operand) -> Self {
        match op {
            reference::ast::Operand::N => Self::N,
            reference::ast::Operand::I => Self::I,
            reference::ast::Operand::V => Self::V,
            reference::ast::Operand::W => Self::W,
            reference::ast::Operand::F => Self::F,
            reference::ast::Operand::T => Self::T,
            reference::ast::Operand::C => Self::C,
            reference::ast::Operand::E => Self::E,
        }
    }
}

impl From<Operand> for reference::ast::Operand {
    fn from(op: Operand) -> Self {
        match op {
            Operand::N => Self::N,
            Operand::I => Self::I,
            Operand::V => Self::V,
            Operand::W => Self::W,
            Operand::F => Self::F,
            Operand::T => Self::T,
            Operand::C => Self::C,
            Operand::E => Self::E,
        }
    }
}

impl From<(Operand, u32)> for reference::ast::Expression {
    fn from(input: (Operand, u32)) -> Self {
        Self {
            operand: input.0.into(),
            modulus: get_modulus(input.1),
        }
    }
}

fn get_modulo(op: &Option<reference::ast::Value>) -> u32 {
    if let Some(op) = op {
        u32::from(op)
    } else {
        0
    }
}

fn get_modulus(input: u32) -> Option<reference::ast::Value> {
    if input == 0 {
        None
    } else {
        Some(input.into())
    }
}

impl From<&reference::ast::Value> for u32 {
    fn from(v: &reference::ast::Value) -> Self {
        v.0.try_into().expect("Failed to convert u64 into u32")
    }
}

impl From<u32> for reference::ast::Value {
    fn from(input: u32) -> Self {
        Self(input.into())
    }
}

impl From<&reference::ast::RangeListItem> for RangeOrValue {
    fn from(item: &reference::ast::RangeListItem) -> Self {
        match item {
            reference::ast::RangeListItem::Range(range) => {
                RangeOrValue::Range(range.start().into(), range.end().into())
            }
            reference::ast::RangeListItem::Value(value) => RangeOrValue::Value(value.into()),
        }
    }
}

impl From<RangeOrValue> for reference::ast::RangeListItem {
    fn from(item: RangeOrValue) -> Self {
        match item {
            RangeOrValue::Range(min, max) => Self::Range(min.into()..=max.into()),
            RangeOrValue::Value(value) => Self::Value(value.into()),
        }
    }
}

impl FromStr for Rule<'_> {
    type Err = reference::parser::ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule = reference::parser::parse(s.as_bytes())?;
        Ok((&rule).into())
    }
}

impl fmt::Display for Rule<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reference = reference::ast::Rule::from(self);
        reference::serialize(&reference, f)
    }
}

/////////////////

#[derive(Debug)]
#[repr(packed)]
pub struct RelationULE {
    /// This maps to (AndOr, Polarity, Operand),
    /// with the first bit mapping to AndOr (1 == And), the second bit
    /// to Polarity (1 == Positive), and the remaining bits to Operand
    /// encoded via Operand::encode. It is unsound for the Operand bits to
    /// not be a valid encoded Operand.
    andor_polarity_operand: u8,
    modulo: <u32 as AsULE>::ULE,
    range_list: [RangeOrValueULE],
}

impl RelationULE {
    pub fn as_relation(&self) -> Relation {
        // safe to call because we are operating on a valid RelationULE
        let (and_or, polarity, operand) =
            unsafe { Self::decode_andor_polarity_operand(self.andor_polarity_operand) };
        Relation {
            and_or,
            polarity,
            operand,
            modulo: u32::from_unaligned(self.modulo),
            range_list: ZeroVec::borrowed_from_slice(&self.range_list),
        }
    }

    fn encode_andor_polarity_operand(and_or: AndOr, polarity: Polarity, operand: Operand) -> u8 {
        // XXX: Ensure and_or is one bit, polarity is one bit, and operand is max 6 bits
        (((and_or == AndOr::And) as u8) << 7)
            + (((polarity == Polarity::Positive) as u8) << 6)
            + operand.idx()
    }

    // Must be called on an `encoded` that is obtained from `encode_andor_polarity_operand`, i.e.
    // the field on a valid RelationULE
    unsafe fn decode_andor_polarity_operand(encoded: u8) -> (AndOr, Polarity, Operand) {
        let and_or = if encoded & 0b1000_0000 != 0 {
            AndOr::And
        } else {
            AndOr::Or
        };

        let polarity = if encoded & 0b0100_0000 != 0 {
            Polarity::Positive
        } else {
            Polarity::Negative
        };

        let operand = Operand::from_idx(encoded & 0b0011_1111);
        (and_or, polarity, operand)
    }
}

unsafe impl VarULE for RelationULE {
    type Error = &'static str;

    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        let ptr = bytes.as_ptr();
        let len = bytes.len();
        // subtract length of andor_polarity_operand and modulo and then convert between a slice of bytes and PlainOldULE<8>
        let len_new = (len - 5) / 8;
        // it's hard constructing custom DSTs, we fake a pointer/length construction
        // eventually we can use the Pointer::Metadata APIs when they stabilize
        let fake_slice = core::ptr::slice_from_raw_parts(ptr as *const RangeOrValueULE, len_new);
        let ret = &*(fake_slice as *const Self);
        debug_assert_eq!(core::mem::size_of_val(ret), core::mem::size_of_val(bytes));
        ret
    }

    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of_val(self),
            )
        }
    }

    //     what it should do is attempt to parse the first 4 bytes as a u32::ULE (POU<4>), and the remaining bytes as a ZV<RangeOrValueULE>
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        let bits = bytes[0]; // XXX: Validate those bits
        <PlainOldULE<4> as ULE>::validate_byte_slice(&bytes[1..5]).map_err(|_| "foo")?;
        let remaining = &bytes[5..];
        RangeOrValueULE::validate_byte_slice(remaining).map_err(|_| "foo")?;
        Ok(())
    }
}

unsafe impl EncodeAsVarULE<RelationULE> for Relation<'_> {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        let encoded =
            RelationULE::encode_andor_polarity_operand(self.and_or, self.polarity, self.operand);
        cb(&[
            &[encoded],
            PlainOldULE::<4>::as_byte_slice(&[self.modulo.as_unaligned()]),
            self.range_list.as_bytes(),
        ])
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RangeOrValueULE(PlainOldULE<8>);

unsafe impl ULE for RangeOrValueULE {
    type Error = zerovec::ule::ULEError<core::convert::Infallible>;

    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        PlainOldULE::<8>::validate_byte_slice(bytes)
    }
}

impl AsULE for RangeOrValue {
    type ULE = RangeOrValueULE;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        let mut result = [0; 8];
        match self {
            Self::Range(start, end) => {
                let start_bytes = start.to_be_bytes();
                let end_bytes = end.to_be_bytes();
                result[..4].copy_from_slice(&start_bytes);
                result[4..].copy_from_slice(&end_bytes);
                RangeOrValueULE(result.into())
            }
            Self::Value(idx) => {
                let bytes = idx.to_be_bytes();
                result[..4].copy_from_slice(&bytes);
                result[4..].copy_from_slice(&bytes);
                RangeOrValueULE(result.into())
            }
        }
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let b = unaligned.0 .0;
        let start = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let end = u32::from_be_bytes([b[4], b[5], b[6], b[7]]);
        if start == end {
            Self::Value(start)
        } else {
            Self::Range(start, end)
        }
    }
}

#[cfg(feature = "provider_serde")]
mod serde {
    use super::*;
    use ::serde::{de, ser, Deserialize, Deserializer, Serialize};
    use alloc::{
        format,
        string::{String, ToString},
    };

    impl Serialize for Rule<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            if serializer.is_human_readable() {
                let string: String = self.to_string();
                serializer.serialize_str(&string)
            } else {
                serializer.serialize_bytes(self.0.get_encoded_slice())
            }
        }
    }

    struct DeserializeRule;

    impl<'de> de::Visitor<'de> for DeserializeRule {
        type Value = Rule<'de>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a valid rule.")
        }

        fn visit_borrowed_str<E>(self, rule_string: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Rule::from_str(rule_string).map_err(|err| {
                de::Error::invalid_value(
                    de::Unexpected::Other(&format!("{}", err)),
                    &"a valid UTS 35 rule string",
                )
            })
        }

        fn visit_borrowed_bytes<E>(self, rule_bytes: &'de [u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let rule = VarZeroVec::parse_byte_slice(rule_bytes).map_err(|err| {
                de::Error::invalid_value(
                    de::Unexpected::Other(&format!("{}", err)),
                    &"a valid UTS 35 rule byte slice",
                )
            })?;
            Ok(Rule(rule))
        }
    }

    impl<'de: 'data, 'data> Deserialize<'de> for Rule<'data> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(DeserializeRule)
            } else {
                deserializer.deserialize_bytes(DeserializeRule)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rules::reference::ast;
    use crate::rules::reference::parse;
    use crate::rules::runtime::test_rule;
    use crate::PluralOperands;

    #[test]
    fn simple_rule_test() {
        let input = "i = 1";
        let full_ast = parse(input.as_bytes()).unwrap();
        assert_eq!(
            full_ast,
            ast::Rule {
                condition: ast::Condition(vec![ast::AndCondition(vec![ast::Relation {
                    expression: ast::Expression {
                        operand: ast::Operand::I,
                        modulus: None,
                    },
                    operator: ast::Operator::Eq,
                    range_list: ast::RangeList(vec![ast::RangeListItem::Value(ast::Value(1))])
                }])]),
                samples: None,
            }
        );

        let rule = Rule::from(&full_ast);
        let relation = rule
            .0
            .iter()
            .next()
            .expect("Should have a relation")
            .as_relation();
        assert_eq!(
            relation,
            Relation {
                and_or: AndOr::And,
                polarity: Polarity::Positive,
                operand: Operand::I,
                modulo: 0,
                range_list: ZeroVec::Borrowed(&[RangeOrValue::Value(1).as_unaligned()])
            }
        );

        let fd = fixed_decimal::decimal::FixedDecimal::from(1);
        let operands = PluralOperands::from(&fd);
        assert!(test_rule(&rule, &operands),);
    }

    #[test]
    fn complex_rule_test() {
        let input = "n % 10 = 3..4,9 and n % 100 != 10..19,70..79,90..99 or n = 0";
        let full_ast = parse(input.as_bytes()).unwrap();
        let rule = Rule::from(&full_ast);

        let fd = fixed_decimal::decimal::FixedDecimal::from(0);
        let operands = PluralOperands::from(&fd);
        assert!(test_rule(&rule, &operands),);

        let fd = fixed_decimal::decimal::FixedDecimal::from(13);
        let operands = PluralOperands::from(&fd);
        assert!(!test_rule(&rule, &operands),);

        let fd = fixed_decimal::decimal::FixedDecimal::from(103);
        let operands = PluralOperands::from(&fd);
        assert!(test_rule(&rule, &operands),);

        let fd = fixed_decimal::decimal::FixedDecimal::from(113);
        let operands = PluralOperands::from(&fd);
        assert!(!test_rule(&rule, &operands),);

        let fd = fixed_decimal::decimal::FixedDecimal::from(178);
        let operands = PluralOperands::from(&fd);
        assert!(!test_rule(&rule, &operands),);

        let fd = fixed_decimal::decimal::FixedDecimal::from(0);
        let operands = PluralOperands::from(&fd);
        assert!(test_rule(&rule, &operands),);
    }

    #[test]
    fn range_or_value_ule_test() {
        let rov = RangeOrValue::Value(1);
        let ule = rov.as_unaligned().0;
        let ref_bytes = &[0, 0, 0, 1, 0, 0, 0, 1];
        assert_eq!(ULE::as_byte_slice(&[ule]), *ref_bytes);

        let rov = RangeOrValue::Range(2, 4);
        let ule = rov.as_unaligned().0;
        let ref_bytes = &[0, 0, 0, 2, 0, 0, 0, 4];
        assert_eq!(ULE::as_byte_slice(&[ule]), *ref_bytes);
    }

    #[test]
    fn relation_ule_test() {
        let rov = RangeOrValue::Value(1);
        let relation = Relation {
            and_or: AndOr::And,
            polarity: Polarity::Positive,
            operand: Operand::N,
            modulo: 0,
            range_list: ZeroVec::clone_from_slice(&[rov]),
        };
        let relations = alloc::vec![relation];
        let vzv = VarZeroVec::from(relations.as_slice());
        assert_eq!(
            vzv.get_encoded_slice(),
            &[1, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1]
        );
    }
}
