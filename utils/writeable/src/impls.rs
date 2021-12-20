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
                // We need to special-case 0, so we might as well check < 10
                if *self < 10 {
                    return LengthHint::exact(1);
                }
                // We want to compute ⌊log₁₀(self)⌋ + 1, but can't do so directly because
                // we're no_std (so there's no f32::log). However, this approach turns out
                // to be faster also on systems with the latest fancy floating point
                // instructions.
                let b = <$u>::BITS - self.leading_zeros();
                // self ∈ [2ᵇ⁻¹, 2ᵇ-1] ⟹ len ∈ [⌊(b-1) log₁₀(2)⌋ + 1, ⌊b log₁₀(2)⌋ + 1].
                let low = (b - 1) * 59 / 196 + 1; // correct for b < 682
                let high = b * 59 / 196 + 1;

                // If the bounds aren't tight (e.g. 87 ∈ [64, 127] ⟹ len ∈ [2,3]), compare
                // to 10ʰ⁻¹ (100). This shouldn't happen too often as there are more powers
                // of 2 than 10 (it happens for 14% of u32s).
                if low == high || *self < (10 as $u).pow(low) {
                    LengthHint::exact(low as usize)
                } else {
                    LengthHint::exact(high as usize)
                }
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

#[test]
fn assert_log10_approximation() {
    for i in 1..u128::BITS {
        assert_eq!(i * 59 / 196, 2f64.powf(i.into()).log10().floor() as u32);
    }
}
