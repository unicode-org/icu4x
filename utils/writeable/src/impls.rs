// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::LengthHint;
use crate::Writeable;
use core::convert::TryFrom;
use core::fmt;
use core::str;

macro_rules! impl_write_num {
    ($u:ty, $i:ty) => {
        impl Writeable for $u {
            fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
                // A number x requires ⌊log₁₀(x)⌋ + 1 digits in base 10, so we need a
                // buffer of length ⌊log₁₀($u::MAX)⌋ + 1. As logarithms aren't const,
                // we have to simplify a bit:
                // ⌊log₁₀($u::MAX)⌋ = ⌊log₁₀(2ᵇ - 1)⌋ ≛ ⌊log₁₀(2ᵇ)⌋ = ⌊b log₁₀(2)⌋
                // (*) holds because there is no integer in [log₁₀(2ᵇ - 1), log₁₀(2ᵇ)],
                // if there were, there'd be some 10ⁿ in [2ᵇ - 1, 2ᵇ], but it can't be 
                // 2ᵇ - 1 due to parity nor 2ᵇ due to prime factors.
                let mut buf = [b'0'; (<$u>::BITS as f64 * core::f64::consts::LOG10_2) as usize + 1];
                let mut n = *self;
                let mut i = buf.len();
                while n != 0 {
                    i -= 1;
                    buf[i] = b'0' + u8::try_from(n % 10).expect("<10");
                    n /= 10;
                }
                if i == buf.len() {
                    debug_assert_eq!(*self, 0);
                    i -= 1;
                }
                let s = unsafe { str::from_utf8_unchecked(&buf[i..]) };
                sink.write_str(s)
            }

            fn write_len(&self) -> LengthHint {
                let mut s = *self;
                let mut len = 1;
                while s >= 10 {
                    len += 1;
                    s /= 10;
                }
                LengthHint::exact(len)
            }
        }

        impl Writeable for $i {
            fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
                if self.is_negative() {
                    sink.write_str("-")?;
                }
                self.unsigned_abs().write_to(sink)
            }

            fn write_len(&self) -> LengthHint {
                LengthHint::exact(if self.is_negative() { 1 } else { 0 })
                    + self.unsigned_abs().write_len()
            }
        }
    };
}

impl_write_num!(u8, i8);
impl_write_num!(u16, i16);
impl_write_num!(u32, i32);
impl_write_num!(u64, i64);
impl_write_num!(u128, i128);

#[test]
fn test_ints() {
    use crate::assert_writeable_eq;
    assert_writeable_eq!(&0u128, "0");
    assert_writeable_eq!(&0i128, "0");
    assert_writeable_eq!(&-0i128, "0");
    assert_writeable_eq!(&1u128, "1");
    assert_writeable_eq!(&1i128, "1");
    assert_writeable_eq!(&-1i128, "-1");
    assert_writeable_eq!(&10u128, "10");
    assert_writeable_eq!(&10i128, "10");
    assert_writeable_eq!(&-10i128, "-10");
    assert_writeable_eq!(&99u128, "99");
    assert_writeable_eq!(&99i128, "99");
    assert_writeable_eq!(&-99i128, "-99");
    assert_writeable_eq!(&u128::MAX, u128::MAX.to_string());
    assert_writeable_eq!(&i128::MAX, i128::MAX.to_string());
    assert_writeable_eq!(&i128::MIN, i128::MIN.to_string());
}
