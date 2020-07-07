use std::convert::TryInto;

/// An iterator over the decimal digits of an integer, from least to most significant
pub(crate) struct IntIterator<T> {
    /// Digits remaining to be returned from the iterator
    unum: T,
    /// Whether the number is negative
    pub is_negative: bool,
}

macro_rules! impl_from_signed_integer_type {
    ($itype:ident, $utype:ident) => {
        impl From<$itype> for IntIterator<$utype> {
            fn from(value: $itype) -> Self {
                IntIterator {
                    unum: {
                        if value == std::$itype::MIN {
                            std::$itype::MAX as $utype + 1
                        } else {
                            value.abs() as $utype
                        }
                    },
                    is_negative: value.is_negative(),
                }
            }
        }
    };
}

macro_rules! impl_from_unsigned_integer_type {
    ($utype:ident) => {
        impl From<$utype> for IntIterator<$utype> {
            fn from(value: $utype) -> Self {
                IntIterator {
                    unum: value,
                    is_negative: false,
                }
            }
        }
    };
}

impl_from_signed_integer_type!(isize, usize);
impl_from_signed_integer_type!(i128, u128);
impl_from_signed_integer_type!(i64, u64);
impl_from_signed_integer_type!(i32, u32);
impl_from_signed_integer_type!(i16, u16);
impl_from_signed_integer_type!(i8, u8);

impl_from_unsigned_integer_type!(usize);
impl_from_unsigned_integer_type!(u128);
impl_from_unsigned_integer_type!(u64);
impl_from_unsigned_integer_type!(u32);
impl_from_unsigned_integer_type!(u16);
impl_from_unsigned_integer_type!(u8);

impl<T> Iterator for IntIterator<T>
where
    T: Copy
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + From<u8>
        + TryInto<u8>
        + PartialEq<T>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.unum == 0.into() {
            None
        } else {
            let div: T = self.unum / 10.into();
            let rem: T = self.unum % 10.into();
            self.unum = div;
            rem.try_into().ok()
        }
    }
}

#[test]
fn test_basic() {
    let mut it = IntIterator {
        unum: 123,
        is_negative: false,
    };
    assert_eq!(Some(3), it.next());
    assert_eq!(Some(2), it.next());
    assert_eq!(Some(1), it.next());
    assert_eq!(None, it.next());
}

#[test]
fn test_zeros() {
    let mut it = IntIterator {
        unum: 9080,
        is_negative: false,
    };
    assert_eq!(Some(0), it.next());
    assert_eq!(Some(8), it.next());
    assert_eq!(Some(0), it.next());
    assert_eq!(Some(9), it.next());
    assert_eq!(None, it.next());
}
