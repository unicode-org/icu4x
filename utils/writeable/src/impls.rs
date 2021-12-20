// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

macro_rules! impl_write_num {
    ($u:ty, $i:ty, $test:ident) => {
        impl $crate::Writeable for $u {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
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
                    buf[i] = b'0' + (n % 10) as u8;
                    n /= 10;
                }
                if i == buf.len() {
                    debug_assert_eq!(*self, 0);
                    i -= 1;
                }
                let s = unsafe { core::str::from_utf8_unchecked(&buf[i..]) };
                sink.write_str(s)
            }

            fn write_len(&self) -> $crate::LengthHint {
                // When https://github.com/rust-lang/rust/issues/70887 stabilizes use
                // LengthHint::exact(self.checked_log10().unwrap_or(0) as usize + 1).

                // We need to special-case 0, so we might as well check < 10
                if *self < 10 {
                    return $crate::LengthHint::exact(1);
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
                    $crate::LengthHint::exact(low as usize)
                } else {
                    $crate::LengthHint::exact(high as usize)
                }
            }
        }

        impl $crate::Writeable for $i {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                if self.is_negative() {
                    sink.write_str("-")?;
                }
                self.unsigned_abs().write_to(sink)
            }

            fn write_len(&self) -> $crate::LengthHint {
                $crate::LengthHint::exact(if self.is_negative() { 1 } else { 0 })
                    + self.unsigned_abs().write_len()
            }
        }

        #[test]
        fn $test() {
            use $crate::assert_writeable_eq;
            assert_writeable_eq!(&(0 as $u), "0");
            assert_writeable_eq!(&(0 as $u), "0");
            assert_writeable_eq!(&(-0 as $i), "0");
            assert_writeable_eq!(&(1 as $u), "1");
            assert_writeable_eq!(&(1 as $i), "1");
            assert_writeable_eq!(&(-1 as $i), "-1");
            assert_writeable_eq!(&(10 as $u), "10");
            assert_writeable_eq!(&(10 as $i), "10");
            assert_writeable_eq!(&(-10 as $i), "-10");
            assert_writeable_eq!(&(99 as $u), "99");
            assert_writeable_eq!(&(99 as $i), "99");
            assert_writeable_eq!(&(-99 as $i), "-99");
            assert_writeable_eq!(&(100 as $u), "100");
            assert_writeable_eq!(&(-100 as $i), "-100");
            assert_writeable_eq!(&<$u>::MAX, <$u>::MAX.to_string());
            assert_writeable_eq!(&<$i>::MAX, <$i>::MAX.to_string());
            assert_writeable_eq!(&<$i>::MIN, <$i>::MIN.to_string());

            for _ in 0..1000 {
                let rand = rand::random::<$u>();
                assert_writeable_eq!(rand, rand.to_string()); 
            }
        }
    };
}

impl_write_num!(u8, i8, test_u8);
impl_write_num!(u16, i16, test_u16);
impl_write_num!(u32, i32, test_u32);
impl_write_num!(u64, i64, test_u64);
impl_write_num!(u128, i128, test_u128);

#[test]
fn assert_log10_approximation() {
    for i in 1..u128::BITS {
        assert_eq!(i * 59 / 196, 2f64.powf(i.into()).log10().floor() as u32);
    }
}
