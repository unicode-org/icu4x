// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use core::cmp::Ordering;

/// A writeable object that can fail while writing.
///
/// The default [`Writeable`] trait returns a [`fmt::Error`], which originates from the sink.
/// In contrast, this trait allows the _writeable itself_ to trigger an error.
/// 
/// Implementations are expected to always make a _best attempt_ at writing to the sink
/// and should write replacement values in the error state. Therefore, the error can be
/// safely ignored to emulate a "lossy" mode.
/// 
/// Any error substrings should be annotated with [`Part::ERROR`].
/// 
/// # Implementor Notes
///
/// This trait requires that implementers make a _best attempt_ at writing to the sink,
/// _even in the error state_, such as with a placeholder or fallback string.
/// 
/// In [`TryWriteable::try_write_to_parts()`], error substrings should be annotated with
/// [`Part::ERROR`]. Becuause of this, writing to parts is not default-implemented like
/// it is on [`Writeable`].
///
/// The trait is implemented on [`Result<T, E>`] where `T` and `E` both implement [`Writeable`];
/// In the `Ok` case, `T` is written, and in the `Err` case, `E` is written as a fallback value.
/// This impl, which writes [`Part::ERROR`], can be used as a basis for more advanced impls.
///
/// # Examples
///
/// Implementing on a custom type:
///
/// ```
/// use core::fmt;
/// use writeable::TryWriteable;
/// use writeable::PartsWrite;
/// use writeable::LengthHint;
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum MyWriteableError {
///     MissingName
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct MyWriteable {
///     pub name: Option<&'static str>
/// }
///
/// impl TryWriteable for MyWriteable {
///     type Error = MyWriteableError;
///
///     fn try_write_to_parts<S: PartsWrite + ?Sized>(
///         &self,
///         sink: &mut S,
///     ) -> Result<Result<(), Self::Error>, fmt::Error> {
///         sink.write_str("Hello, ")?;
///         // Use `impl TryWriteable for Result` to generate the error part:
///         let _ = self.name.ok_or("nobody").try_write_to_parts(sink)?;
///         sink.write_char('!')?;
///         if self.name.is_some() {
///             Ok(Ok(()))
///         } else {
///             Ok(Err(MyWriteableError::MissingName))
///         }
///     }
///
///     fn try_writeable_length_hint(&self) -> (LengthHint, Option<Self::Error>) {
///         let (hint, e) = self.name.ok_or("nobody").try_writeable_length_hint();
///         (hint + 8, e.map(|_| MyWriteableError::MissingName))
///     }
/// }
///
/// writeable::assert_try_writeable_eq!(
///     MyWriteable {
///         name: Some("Alice")
///     },
///     "Hello, Alice!"
/// );
///
/// writeable::assert_try_writeable_eq!(
///     MyWriteable {
///         name: None
///     },
///     "Hello, nobody!",
///     Err::<(), _>(MyWriteableError::MissingName),
/// );
/// ```
pub trait TryWriteable {
    type Error;

    fn try_write_to<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        self.try_write_to_parts(&mut helpers::CoreWriteAsPartsWrite(sink))
    }

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error>;

    fn try_writeable_length_hint(&self) -> (LengthHint, Option<Self::Error>) {
        // TODO: Discuss this function, its signature, and its behavior
        (LengthHint::undefined(), None)
    }

    fn try_write_to_string(&self) -> Result<Cow<str>, Self::Error> {
        let (hint, hint_error) = self.try_writeable_length_hint();
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
            hint_error.is_some(),
            result.is_err(),
            "try_writeable_length_hint and try_write_to_string should have same error state"
        );
        result.map(|_| Cow::Owned(output))
    }

    fn try_write_cmp_bytes(&self, other: &[u8]) -> (Ordering, Option<Self::Error>) {
        let mut wc = cmp::WriteComparator::new(other);
        let result = match self.try_write_to(&mut wc) {
            Ok(result) => result,
            Err(core::fmt::Error) => {
                debug_assert!(false, "WriteComparator infallible");
                Ok(())
            }
        };
        let ordering = wc.finish().reverse();
        (ordering, result.err())
    }
}

///
impl<T, E> TryWriteable for Result<T, E>
where
    T: Writeable,
    E: Writeable + Clone,
{
    type Error = E;

    #[inline]
    fn try_write_to<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        match self {
            Ok(t) => t.write_to(sink).map(Ok),
            Err(e) => e.write_to(sink).map(|_| Err(e.clone())),
        }
    }

    #[inline]
    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        match self {
            Ok(t) => t.write_to_parts(sink).map(Ok),
            Err(e) => sink
                .with_part(Part::ERROR, |sink| e.write_to_parts(sink))
                .map(|_| Err(e.clone())),
        }
    }

    #[inline]
    fn try_writeable_length_hint(&self) -> (LengthHint, Option<Self::Error>) {
        match self {
            Ok(t) => (t.writeable_length_hint(), None),
            Err(e) => (e.writeable_length_hint(), Some(e.clone())),
        }
    }

    #[inline]
    fn try_write_to_string(&self) -> Result<Cow<str>, Self::Error> {
        match self {
            Ok(t) => Ok(t.write_to_string()),
            Err(e) => Err(e.clone()),
        }
    }

    #[inline]
    fn try_write_cmp_bytes(&self, other: &[u8]) -> (Ordering, Option<Self::Error>) {
        match self {
            Ok(t) => (t.write_cmp_bytes(other), None),
            Err(e) => (e.write_cmp_bytes(other), Some(e.clone())),
        }
    }
}

#[macro_export]
macro_rules! assert_try_writeable_eq {
    ($actual_writeable:expr, $expected_str:expr $(,)?) => {
        $crate::assert_try_writeable_eq!($actual_writeable, $expected_str, Ok(()));
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr $(,)?) => {
        $crate::assert_try_writeable_eq!($actual_writeable, $expected_str, $expected_result, "");
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $($arg:tt)+) => {{
        use $crate::TryWriteable;
        let actual_writeable = &$actual_writeable;
        let (actual_str, _, actual_error) = $crate::_internal::try_writeable_to_parts_for_test(actual_writeable);
        assert_eq!(actual_str, $expected_str, $($arg)*);
        assert_eq!(actual_error, $expected_result.err(), $($arg)*);
        let actual_result = match actual_writeable.try_write_to_string() {
            Ok(actual_cow_str) => {
                assert_eq!(actual_cow_str, $expected_str, $($arg)+);
                Ok(())
            }
            Err(e) => Err(e)
        };
        assert_eq!(actual_result, $expected_result, $($arg)*);
        let (length_hint, hint_error) = actual_writeable.try_writeable_length_hint();
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
        assert_eq!(hint_error, $expected_result.err(), $($arg)*);
        let (ordering, ordering_error) = actual_writeable.try_write_cmp_bytes($expected_str.as_bytes());
        assert_eq!(ordering, core::cmp::Ordering::Equal, $($arg)*);
        assert_eq!(ordering_error, $expected_result.err(), $($arg)*);
    }};
}

#[test]
fn test_result_try_writeable() {
    let mut result: Result<&str, usize> = Ok("success");
    assert_try_writeable_eq!(
        result,
        "success"
    );
    result = Err(44);
    assert_try_writeable_eq!(
        result,
        "44",
        Err::<(), _>(44)
    );
}
