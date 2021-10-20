// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)]

use zerovec::ule::{AsULE, PlainOldULE, VarULE, ULE};
use zerovec::ule::custom::EncodeAsVarULE;
use zerovec::VarZeroVec;
use zerovec::ZeroVec;

use core::mem;

pub struct Rule<'data>(pub VarZeroVec<'data, RelationULE>);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Operand {
    N = 1,
    I = 2,
    V = 3,
    W = 4,
    F = 5,
    T = 6,
    C = 7,
    E = 8,
}

impl Operand {
    fn encode(self) -> u8 {
        self as u8
    }

    /// Safety invariant: must be a valid encoded operand
    unsafe fn decode(encoded: u8) -> Self {
        // safe due to the repr(u8)
        unsafe { mem::transmute(encoded) }
    }
}

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
    pub fn as_relation<'a>(&'a self) -> Relation<'a> {
        // safe to call because we are operating on a valid RelationULE
        let (and_or, polarity, operand) =
            unsafe { Self::decode_andor_polarity_operand(self.andor_polarity_operand) };
        Relation {
            and_or,
            polarity,
            operand,
            modulo: u32::from_unaligned(&self.modulo),
            range_list: ZeroVec::borrowed_from_slice(&self.range_list),
        }
    }

    fn encode_andor_polarity_operand(and_or: AndOr, polarity: Polarity, operand: Operand) -> u8 {
        ((and_or == AndOr::And) as u8)
            << 7 + ((polarity == Polarity::Positive) as u8)
            << 6 + operand.encode()
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

        let operand = Operand::decode(encoded & 0b0010_000);
        (and_or, polarity, operand)
    }
}

unsafe impl VarULE for RelationULE {
    type Error = &'static str;

    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        let ptr = bytes as *const [u8] as *const u8;
        let len = bytes.len();
        let len_new = (len - 5) / 8;
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
        <PlainOldULE<4> as ULE>::validate_byte_slice(&bytes[..4]).map_err(|_| "foo")?;
        let remaining = &bytes[4..];
        RangeOrValueULE::validate_byte_slice(remaining).map_err(|_| "foo")?;
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AndOr {
    Or,
    And,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Polarity {
    Negative,
    Positive,
}

pub struct Relation<'data> {
    pub(crate) and_or: AndOr,
    pub(crate) polarity: Polarity,
    pub(crate) operand: Operand,
    pub(crate) modulo: u32,
    pub(crate) range_list: ZeroVec<'data, RangeOrValue>,
}

unsafe impl EncodeAsVarULE<RelationULE> for Relation<'_> {
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        let encoded =
            RelationULE::encode_andor_polarity_operand(self.and_or, self.polarity, self.operand);

        cb(&[
            &[encoded],
            PlainOldULE::<4>::as_byte_slice(&[self.modulo.as_unaligned()]),
            self.range_list.as_bytes(),
        ])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RangeOrValue {
    Range(u32, u32), // XXX: Can we get away from smaller?
    Value(u32),
}

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
    fn as_unaligned(&self) -> Self::ULE {
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
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
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

impl From<&crate::rules::reference::ast::Rule> for Rule<'_> {
    fn from(input: &crate::rules::reference::ast::Rule) -> Self {
        let mut relations: alloc::vec::Vec<Relation> = alloc::vec![];

        for and_condition in input.condition.0.iter() {
            for relation in and_condition.0.iter() {
                let range_list = alloc::vec![RangeOrValue::Value(1)];

                relations.push(Relation {
                    and_or: AndOr::And,
                    polarity: Polarity::Positive,
                    operand: Operand::N,
                    modulo: 1,
                    range_list: ZeroVec::clone_from_slice(&range_list),
                })
            }
        }

        Self(VarZeroVec::from(relations.as_slice()))
    }
}

mod test {
    use super::*;
    use crate::rules::reference::ast;
    use crate::rules::reference::parse;
    use crate::rules::runtime::test_rule;
    use crate::PluralOperands;

    #[test]
    fn sanity_test() {
        // let input = "n % 10 = 3..4,9 and n % 100 != 10..19,70..79,90..99 or n = 0";
        let input = "n = 1";
        let full_ast = parse(input.as_bytes()).unwrap();
        assert_eq!(
            full_ast,
            ast::Rule {
                condition: ast::Condition(Box::new([ast::AndCondition(Box::new([
                    ast::Relation {
                        expression: ast::Expression {
                            operand: ast::Operand::N,
                            modulus: None,
                        },
                        operator: ast::Operator::Eq,
                        range_list: ast::RangeList(Box::new([ast::RangeListItem::Value(
                            ast::Value(1)
                        )]))
                    }
                ]))])),
                samples: None,
            }
        );

        let rule = Rule::from(&full_ast);

        let fd = fixed_decimal::decimal::FixedDecimal::from(5);
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
            conjunction: true,
            modulo: 0,
            range_list: ZeroVec::clone_from_slice(&[rov]),
        };
        let mut relations = alloc::vec![relation];
        let vzv = VarZeroVec::from(relations.as_slice());
        assert_eq!(
            vzv.get_encoded_slice(),
            &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1]
        );
    }
}
