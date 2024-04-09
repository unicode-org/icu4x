// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use core::cmp::Ordering;

pub trait TryWriteable {
    type Error;

    fn try_write_to<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<Result<(), Self::Error>, fmt::Error>;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error>;

    fn try_writeable_length_hint(&self) -> Result<LengthHint, (LengthHint, Self::Error)>;

    fn try_write_to_string(&self) -> Result<Cow<str>, Self::Error> {
        let (hint, has_error) = match self.try_writeable_length_hint() {
            Ok(hint) => (hint, false),
            Err((hint, _)) => (hint, true),
        };
        if hint.is_zero() {
            return Ok(Cow::Borrowed(""));
        }
        let mut output = String::with_capacity(hint.capacity());
        let result = match self.try_write_to(&mut output) {
            Ok(result) => result,
            Err(core::fmt::Error) => {
                debug_assert!(false, "String infallible");
                Ok(())
            }
        };
        debug_assert_eq!(
            has_error,
            result.is_err(),
            "try_writeable_length_hint and try_write_to_string should have same error state"
        );
        result.map(|_| Cow::Owned(output))
    }

    fn try_write_cmp_bytes(&self, other: &[u8]) -> Result<Ordering, (Ordering, Self::Error)> {
        let mut wc = cmp::WriteComparator::new(other);
        let result = match self.try_write_to(&mut wc) {
            Ok(result) => result,
            Err(core::fmt::Error) => {
                debug_assert!(false, "WriteComparator infallible");
                Ok(())
            }
        };
        let ordering = wc.finish().reverse();
        result.map(|_| ordering).map_err(|e| (ordering, e))
    }
}

#[macro_export]
macro_rules! assert_try_writeable_eq {
    ($actual_writeable:expr, $expected_str:expr $(,)?) => {
        $crate::assert_writeable_eq!($actual_writeable, $expected_str, Ok(()), "");
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $($arg:tt)+) => {{
        let actual_writeable = &$actual_writeable;
        let (actual_str, _, actual_result) = $crate::try_writeable_to_parts_for_test(actual_writeable);
        assert_eq!(actual_str, $expected_str, $($arg)*);
        assert_eq!(actual_result, $expected_result, $($arg)*);
        let actual_result = match $crate::TryWriteable::try_write_to_string(actual_writeable) {
            Ok(s) => {
                assert_eq!(actual_cow_str, $expected_str, $($arg)+);
                Ok(())
            }
            Err(e) => Err(e)
        };
        assert_eq!(actual_result, $expected_result, $($arg)*);
        let (length_hint, actual_result) = match $crate::TryWriteable::writeable_length_hint(actual_writeable) {
            Ok(length_hint) => (length_hint, Ok(())),
            Err((length_hint, e)) => (length_hint, Err(e)),
        };
        assert!(
            length_hint.0 <= actual_str.len(),
            "hint lower bound {} larger than actual length {}: {}",
            length_hint.0, actual_str.len(), format!($($arg)*),
        );
        if let Some(upper) = length_hint.1 {
            assert!(
                actual_str.len() <= upper,
                "hint upper bound {} smaller than actual length {}: {}",
                length_hint.0, actual_str.len(), format!($($arg)*),
            );
        }
        assert_eq!(actual_result, $expected_result, $($arg)*);
        assert_eq!(actual_writeable.to_string(), $expected_str);
        let (ordering, actual_result) = match $crate::Writeable::try_write_cmp_bytes(actual_writeable, $expected_str.as_bytes()) {
            Ok(ordering) => (ordering, Ok(())),
            Err((ordering, e)) => (ordering, Err(e)),
        };
        assert_eq!(ordering, core::cmp::Ordering::Equal, $($arg)+);
        assert_eq!(actual_result, $expected_result, $($arg)*);
    }};
}
