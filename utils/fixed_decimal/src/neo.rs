// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental decimal type

#![allow(missing_docs, dead_code, unused_variables)]

use core::ops::Deref;
use core::ops::DerefMut;

use alloc::vec::Vec;

#[derive(Debug, Copy, Clone)]
pub struct UnsignedInteger<Store> {
    digits: Store,
    magnitude: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Signed<Unsigned> {
    unsigned: Unsigned,
    is_negative: bool,
}

impl<Unsigned> Deref for Signed<Unsigned> {
    type Target = Unsigned;
    fn deref(&self) -> &Unsigned {
        &self.unsigned
    }
}

impl<Unsigned> DerefMut for Signed<Unsigned> {
    fn deref_mut(&mut self) -> &mut Unsigned {
        &mut self.unsigned
    }
}

pub type Integer<Store> = Signed<UnsignedInteger<Store>>;

#[derive(Debug, Copy, Clone)]
pub struct CompactExponent {
    exponent: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Compact<Significand> {
    significant: Significand,
    exponent: CompactExponent,
}

pub mod scaffold {
    pub mod store {
        pub trait Count {
            fn digit_store_count(&self) -> u16;
        }
        pub trait Read: Count {
            fn digit_store_get(&self, position: u16) -> Option<u8>;
        }
        pub trait Truncate: Count {
            fn digit_store_truncate(&mut self, length: u16);
        }

        impl<const N: usize> Count for [u8; N] {
            fn digit_store_count(&self) -> u16 {
                todo!()
            }
        }

        impl<const N: usize> Truncate for [u8; N] {
            fn digit_store_truncate(&mut self, length: u16) {
                todo!()
            }
        }
    }
}

impl<Store> UnsignedInteger<Store> {
    /// Increases the magnitude by 1.
    /// For example, 250 -> 2500 -> 25000
    pub fn increment_magnitude(&mut self) {
        self.magnitude += 1;
    }
}

impl<Store> UnsignedInteger<Store>
where
    Store: scaffold::store::Truncate,
{
    /// Decreases the magnitude by 1, truncating if necessary.
    /// For example, 250 -> 25 -> 2
    pub fn decrement_magnitude(&mut self) {
        self.magnitude -= 1;
        if self.magnitude <= self.digits.digit_store_count() {
            self.digits.digit_store_truncate(self.magnitude + 1);
        }
    }
}

impl<Unsigned> Signed<Unsigned> {
    pub fn negate(&mut self) {
        self.is_negative = !self.is_negative;
    }
}

impl UnsignedInteger<[u8; 10]> {
    pub fn from_u32(value: u32) -> Self {
        todo!()
    }
}

impl Integer<[u8; 10]> {
    pub fn from_i32(value: i32) -> Self {
        todo!()
    }
}

impl UnsignedInteger<[u8; 20]> {
    pub fn from_u64(value: u64) -> Self {
        todo!()
    }
}

impl Integer<[u8; 19]> {
    pub fn from_i64(value: i64) -> Self {
        todo!()
    }
}

impl UnsignedInteger<Vec<u8>> {
    pub fn from_str(value: &str) -> Self {
        todo!()
    }
}

impl Integer<Vec<u8>> {
    pub fn from_str(value: &str) -> Self {
        todo!()
    }
}

#[test]
fn test_no_explicit_generics() {
    let mut integer = Integer::from_i32(12345);
    integer.decrement_magnitude();
    integer.increment_magnitude();
    integer.negate();
    fn copy<T: Copy>(value: T) -> T {
        value
    }
    copy(integer);
}
