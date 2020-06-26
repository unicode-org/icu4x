use num_integer::Integer;
use num_traits::{FromPrimitive, ToPrimitive, Unsigned};

#[cfg(test)]
use num_bigint::BigUint;

/// An iterator over the decimal digits of an unsigned integer, from least to most significant.
pub(crate) struct UintIterator<T: Unsigned + Integer + FromPrimitive + ToPrimitive>(T);

impl<T> From<T> for UintIterator<T>
where
    // TODO(review): How do you de-duplicate the type parameters here and elsewhere?
    T: Unsigned + Integer + FromPrimitive + ToPrimitive,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Iterator for UintIterator<T>
where
    T: Unsigned + Integer + FromPrimitive + ToPrimitive,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == T::from_usize(0).unwrap() {
            None
        } else {
            let (div, rem) = self.0.div_rem(&T::from_usize(10).unwrap());
            self.0 = div;
            Some(rem.to_u8().unwrap())
        }
    }
}

#[test]
fn test_basic() {
    let mut it = UintIterator::from(123u32);
    assert_eq!(Some(3), it.next());
    assert_eq!(Some(2), it.next());
    assert_eq!(Some(1), it.next());
    assert_eq!(None, it.next());
}

#[test]
fn test_biguint() {
    let mut it = UintIterator::from(BigUint::parse_bytes(b"9876543210", 10).unwrap());
    assert_eq!(Some(0), it.next());
    assert_eq!(Some(1), it.next());
    assert_eq!(Some(2), it.next());
    assert_eq!(Some(3), it.next());
    assert_eq!(Some(4), it.next());
    assert_eq!(Some(5), it.next());
    assert_eq!(Some(6), it.next());
    assert_eq!(Some(7), it.next());
    assert_eq!(Some(8), it.next());
    assert_eq!(Some(9), it.next());
    assert_eq!(None, it.next());
}
