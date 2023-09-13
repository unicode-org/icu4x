// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::str::FromStr;

use num_bigint::BigUint;
use num_traits::{Zero, One};

use crate::Error;


pub fn gcd_big_unit(a: BigUint, b: BigUint) -> Result<BigUint, Error> {
    if a.is_zero() || b.is_zero(){
         return Err(Error::InvalidInput);
    }

    let mut a = a.clone();
    let mut b = b.clone();

    while !b.is_zero() {
        let t = b.clone();
        b = a.modpow(&One::one(),  &b);
        a = t;
    }

    Ok(a)
}

/// Returns the greatest common divisor of two numbers.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[test]
fn test_gcd_big_unit() {
   let input1 = BigUint::from(10u32);
   let input2 = BigUint::from(5u32);
   let expected = BigUint::from(5u32);
    assert_eq!(gcd_big_unit(input1, input2).unwrap(), expected);

    let input1 = BigUint::from(1000u32);
    let input2 = BigUint::from(10u32);
    let expected = BigUint::from(10u32);
    assert_eq!(gcd_big_unit(input1, input2).unwrap(), expected);

    let input1 = BigUint::from(0u32);
    let input2 = BigUint::from(10u32);
    assert!(gcd_big_unit(input1, input2).is_err());

    let input1 = BigUint::from_str("100025").unwrap();
    let input2 = BigUint::from_str("500").unwrap();
    let expected = BigUint::from_str("25").unwrap();
    assert_eq!(gcd_big_unit(input1, input2).unwrap(), expected);
}