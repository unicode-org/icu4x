// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)]

use crate::rules::reference;
use core::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};
use icu_provider::{yoke, zerofrom};
use num_enum::{IntoPrimitive, TryFromPrimitive, UnsafeFromPrimitive};
use zerovec::{
    ule::{AsULE, EncodeAsVarULE, PairULE, RawBytesULE, VarULE, ZeroVecError, ULE},
    {VarZeroVec, ZeroVec},
};

#[derive(yoke::Yokeable, zerofrom::ZeroFrom, Clone, PartialEq, Debug)]
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

#[derive(IntoPrimitive, UnsafeFromPrimitive, TryFromPrimitive, Copy, Clone, Debug, PartialEq)]
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
    Range(u32, u32),
    Value(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Relation<'data> {
    pub(crate) aopo: AndOrPolarityOperand,
    pub(crate) modulo: u32,
    pub(crate) range_list: ZeroVec<'data, RangeOrValue>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct AndOrPolarityOperand {
    pub(crate) and_or: AndOr,
    pub(crate) polarity: Polarity,
    pub(crate) operand: Operand,
}

/////

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

                let aopo = AndOrPolarityOperand {
                    and_or,
                    polarity: relation.operator.into(),
                    operand: relation.expression.operand.into(),
                };

                relations.push(Relation {
                    aopo,
                    modulo: get_modulo(&relation.expression.modulus),
                    range_list: ZeroVec::alloc_from_slice(&range_list),
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
                expression: (rel.aopo.operand, rel.modulo).into(),
                operator: rel.aopo.polarity.into(),
                range_list: reference::ast::RangeList(list),
            };

            if rel.aopo.and_or == AndOr::And {
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

/// `RelationULE` is a type optimized for efficent storing and
/// deserialization of `rule::runtime::ast::Rule` `ZeroVec` model.
///
/// The serialization model packages the rule into 4 bytes plus
/// variable length of the list of ranges.
///
/// The first byte is used to store:
///  * One bit to store the state of AndOr.
///  * One bit to store the state of Polarity.
///  * Six bits to store the Operand.
///
/// The following four bytes store the value of the modulo as `u32`.
///
/// Finally, the remaining variable length is used to store a list
/// of `RangeOrValue` elements which have `[u32;2]` layout.
///
/// # Diagram
///
/// ```text
/// ┌───────────────┬───────────────┬───────────────┐
/// │      u8       │    [u8;4]     │     [u8]      │
/// ├─┬─┬─┬─┬─┬─┬─┬─┼───┬───┬───┬───┼───────────────┤
/// ├─┼─┼─┴─┴─┴─┴─┴─┼───┴───┴───┴───┼───────────────┤
/// │ │ │  Operand  │    Modulo     │  RangeOrValue │
/// └─┴─┴───────────┴───────────────┴───────────────┘
///  ▲ ▲
///  │ │
///  │ Polarity
///  │
///  AndOr
/// ```
///
/// # Optimization
///
/// This model is optimized for efficient packaging of the `Relation` elements
/// and performant deserialization from the `RelationULE` to `Relation` type.
///
/// # Constraints
///
/// The model constraints the `Operand` to 64 variants, and `Modulo` to `u32::MAX`.
#[derive(Debug, PartialEq)]
#[repr(packed)]
pub struct RelationULE {
    /// This maps to (AndOr, Polarity, Operand),
    /// with the first bit mapping to AndOr (1 == And), the second bit
    /// to Polarity (1 == Positive), and the remaining bits to Operand
    /// encoded via Operand::encode. It is unsound for the Operand bits to
    /// not be a valid encoded Operand.
    aopo: AndOrPolarityOperandULE,
    modulo: <u32 as AsULE>::ULE,
    range_list: [RangeOrValueULE],
}

impl RelationULE {
    #[inline]
    pub fn as_relation(&self) -> Relation {
        Relation {
            aopo: AndOrPolarityOperand::from_unaligned(self.aopo),
            modulo: u32::from_unaligned(self.modulo),
            range_list: ZeroVec::Borrowed(&self.range_list),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct AndOrPolarityOperandULE(u8);

// Safety (based on the safety checklist on the ULE trait):
//  1. AndOrPolarityOperandULE does not include any uninitialized or padding bytes
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
/// 2. AndOrPolarityOperandULE is aligned to 1 byte
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes
//     (impossible since it is of size 1 byte)
//  5 The other ULE methods use the default impl.
//  6. AndOrPolarityOperandULE byte equality is semantic equality.
unsafe impl ULE for AndOrPolarityOperandULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        for byte in bytes {
            Operand::try_from(byte & 0b0011_1111).map_err(|_| ZeroVecError::parse::<Self>())?;
        }
        Ok(())
    }
}

impl AsULE for AndOrPolarityOperand {
    type ULE = AndOrPolarityOperandULE;
    fn to_unaligned(self) -> AndOrPolarityOperandULE {
        let encoded_operand = u8::from(self.operand);
        debug_assert!(encoded_operand <= 0b0011_1111);
        AndOrPolarityOperandULE(
            (((self.and_or == AndOr::And) as u8) << 7)
                + (((self.polarity == Polarity::Positive) as u8) << 6)
                + encoded_operand,
        )
    }

    fn from_unaligned(other: AndOrPolarityOperandULE) -> Self {
        let encoded = other.0;
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

        let operand = unsafe { Operand::from_unchecked(encoded & 0b0011_1111) };
        Self {
            and_or,
            polarity,
            operand,
        }
    }
}

// Safety (based on the safety checklist on the ULE trait):
//  1. RelationULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(packed)]` on a type that satisfies this invariant)
//  2. RelationULE is aligned to 1 byte
//     (achieved by `#[repr(packed)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. The other VarULE methods use the default impl.
//  7. RelationULE byte equality is semantic equality.
unsafe impl VarULE for RelationULE {
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        let ptr = bytes.as_ptr();
        let len = bytes.len();
        // subtract length of andor_polarity_operand and modulo and then convert between a slice of bytes and RangeOrValueULE
        let len_new = (len - 5) / 8;

        // Keep this in sync with `RelationULE`
        struct RelationULESized {
            _andor_polarity_operand: u8,
            _modulo: <u32 as AsULE>::ULE,
        }
        debug_assert_eq!(core::mem::size_of::<RelationULESized>(), 5);
        debug_assert_eq!(core::mem::size_of::<RangeOrValueULE>(), 8);

        // it's hard constructing custom DSTs, we fake a pointer/length construction
        // eventually we can use the Pointer::Metadata APIs when they stabilize
        let fake_slice = core::ptr::slice_from_raw_parts(ptr as *const RangeOrValueULE, len_new);
        let ret = &*(fake_slice as *const Self);
        debug_assert_eq!(core::mem::size_of_val(ret), core::mem::size_of_val(bytes));
        ret
    }

    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        // Skip bytes 1-4 as they're always valid `u32` for `Modulo`.
        if bytes.len() < 5 {
            return Err(ZeroVecError::parse::<Self>());
        }
        // check byte 0
        AndOrPolarityOperandULE::validate_byte_slice(&bytes[0..1])?;
        let remaining = &bytes[5..];
        RangeOrValueULE::validate_byte_slice(remaining)?;
        Ok(())
    }
}

// Safety
//
// the slices to the callback must, when concatenated,
// are a valid instance of the RelationULE type.
unsafe impl EncodeAsVarULE<RelationULE> for Relation<'_> {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        let encoded = self.aopo.to_unaligned();
        cb(&[
            ULE::as_byte_slice(&[encoded]),
            RawBytesULE::<4>::as_byte_slice(&[self.modulo.to_unaligned()]),
            self.range_list.as_bytes(),
        ])
    }
}

type RangeOrValueULE = PairULE<<u32 as AsULE>::ULE, <u32 as AsULE>::ULE>;

impl AsULE for RangeOrValue {
    type ULE = RangeOrValueULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        match self {
            Self::Range(start, end) => PairULE(start.to_unaligned(), end.to_unaligned()),
            Self::Value(idx) => PairULE(idx.to_unaligned(), idx.to_unaligned()),
        }
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let start = u32::from_unaligned(unaligned.0);
        let end = u32::from_unaligned(unaligned.1);
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
                serializer.serialize_bytes(self.0.as_bytes())
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
    use crate::rules::reference;
    use crate::rules::runtime::test_rule;
    use crate::PluralOperands;

    #[test]
    fn simple_rule_test() {
        use reference::ast;

        let input = "i = 1";
        let full_ast = reference::parse(input.as_bytes()).unwrap();
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
                aopo: AndOrPolarityOperand {
                    and_or: AndOr::And,
                    polarity: Polarity::Positive,
                    operand: Operand::I,
                },
                modulo: 0,
                range_list: ZeroVec::Borrowed(&[RangeOrValue::Value(1).to_unaligned()])
            }
        );

        let fd = fixed_decimal::decimal::FixedDecimal::from(1);
        let operands = PluralOperands::from(&fd);
        assert!(test_rule(&rule, &operands),);
    }

    #[test]
    fn complex_rule_test() {
        let input = "n % 10 = 3..4, 9 and n % 100 != 10..19, 70..79, 90..99 or n = 0";
        let ref_rule = reference::parse(input.as_bytes()).unwrap();
        let rule = Rule::from(&ref_rule);

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
    fn complex_rule_ule_roundtrip_test() {
        let input = "n % 10 = 3..4, 9 and n % 100 != 10..19, 70..79, 90..99 or n = 0";

        let ref_rule = reference::parse(input.as_bytes()).unwrap();

        // Create a ZVZ backed Rule from the reference one.
        let rule = Rule::from(&ref_rule);

        // Convert it back to reference Rule and compare.
        assert_eq!(ref_rule, reference::ast::Rule::from(&rule));

        // Verify that the stringified output matches the input.
        assert_eq!(input, rule.to_string(),);
    }

    #[test]
    fn range_or_value_ule_test() {
        let rov = RangeOrValue::Value(1);
        let ule = rov.to_unaligned();
        let ref_bytes = &[1, 0, 0, 0, 1, 0, 0, 0];
        assert_eq!(ULE::as_byte_slice(&[ule]), *ref_bytes);

        let rov = RangeOrValue::Range(2, 4);
        let ule = rov.to_unaligned();
        let ref_bytes = &[2, 0, 0, 0, 4, 0, 0, 0];
        assert_eq!(ULE::as_byte_slice(&[ule]), *ref_bytes);
    }

    #[test]
    fn relation_ule_test() {
        let rov = RangeOrValue::Value(1);
        let aopo = AndOrPolarityOperand {
            and_or: AndOr::And,
            polarity: Polarity::Positive,
            operand: Operand::N,
        };
        let relation = Relation {
            aopo,
            modulo: 0,
            range_list: ZeroVec::alloc_from_slice(&[rov]),
        };
        let relations = alloc::vec![relation];
        let vzv = VarZeroVec::from(relations.as_slice());
        assert_eq!(
            vzv.as_bytes(),
            &[1, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]
        );
    }
}
