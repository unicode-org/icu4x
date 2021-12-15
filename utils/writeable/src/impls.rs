// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::LengthHint;
use crate::Writeable;
use core::convert::TryFrom;
use core::fmt;
use core::str;

impl Writeable for u8 {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut buf = [b'0'; 3];
        let mut n = *self;
        let mut i = 3usize;
        while n != 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10);
            n /= 10;
        }
        if i == 3 {
            debug_assert_eq!(*self, 0);
            i = 2;
        }
        let s = unsafe { str::from_utf8_unchecked(&buf[i..]) };
        sink.write_str(s)
    }

    fn write_len(&self) -> LengthHint {
        if *self < 10 {
            LengthHint::Exact(1)
        } else if *self < 100 {
            LengthHint::Exact(2)
        } else {
            LengthHint::Exact(3)
        }
    }
}

impl Writeable for u16 {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut buf = [b'0'; 5];
        let mut n = *self;
        let mut i = 5usize;
        while n != 0 {
            i -= 1;
            buf[i] = b'0' + u8::try_from(n % 10).expect("<10");
            n /= 10;
        }
        if i == 5 {
            debug_assert_eq!(*self, 0);
            i = 4;
        }
        let s = unsafe { str::from_utf8_unchecked(&buf[i..]) };
        sink.write_str(s)
    }

    fn write_len(&self) -> LengthHint {
        if *self < 10 {
            LengthHint::Exact(1)
        } else if *self < 100 {
            LengthHint::Exact(2)
        } else if *self < 1000 {
            LengthHint::Exact(3)
        } else if *self < 10000 {
            LengthHint::Exact(4)
        } else {
            LengthHint::Exact(5)
        }
    }
}

#[test]
fn test_u8() {
    use crate::assert_writeable_eq;
    assert_writeable_eq!(&0u8, "0",);
    assert_writeable_eq!(&1u8, "1",);
    assert_writeable_eq!(&10u8, "10",);
    assert_writeable_eq!(&99u8, "99",);
    assert_writeable_eq!(&255u8, "255",);
}

#[test]
fn test_u16() {
    use crate::assert_writeable_eq;
    assert_writeable_eq!(&0u16, "0",);
    assert_writeable_eq!(&1u16, "1",);
    assert_writeable_eq!(&10u16, "10",);
    assert_writeable_eq!(&99u16, "99",);
    assert_writeable_eq!(&65535u16, "65535",);
}
