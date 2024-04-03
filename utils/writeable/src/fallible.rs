// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{convert::Infallible, fmt};

use crate::*;

#[derive(Debug)]
pub struct MissingWriteableError;

impl Writeable for MissingWriteableError {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        // Substitute with the Unicode replacement character
        sink.write_char('\u{FFFD}')
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::exact(3)
    }
}

impl_display_with_writeable!(MissingWriteableError);

#[test]
fn test_missing_writeable_error() {
    assert_writeable_eq!(MissingWriteableError, "�");
}

#[derive(Debug)]
pub enum NeverWriteable {}

impl Writeable for NeverWriteable {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, _sink: &mut W) -> fmt::Result {
        match *self {}
    }
    #[inline]
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, _sink: &mut S) -> fmt::Result {
        match *self {}
    }
    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        match *self {}
    }
    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        match *self {}
    }
    #[inline]
    fn write_cmp_bytes(&self, _other: &[u8]) -> core::cmp::Ordering {
        match *self {}
    }
}

pub trait FalliblePartsWrite: PartsWrite {
    type Error;
    type SubFalliblePartsWrite: FalliblePartsWrite + ?Sized;

    fn try_with_part(
        &mut self,
        part: Part,
        f: impl FnMut(&mut Self::SubPartsWrite) -> Result<Result<(), Self::Error>, fmt::Error>,
    ) -> Result<Result<(), Self::Error>, fmt::Error>;
}

pub trait FallibleWriteable {
    type Error;

    fn fallible_write_to<
        W: fmt::Write + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut W,
        handler: F,
    ) -> Result<Result<(), E>, fmt::Error>;

    fn fallible_write_to_parts<
        S: FalliblePartsWrite + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut S,
        handler: F,
    ) -> Result<Result<(), E>, fmt::Error>;

    fn fallible_writeable_length_hint<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        _handler: F,
    ) -> Result<LengthHint, E> {
        Ok(LengthHint::undefined())
    }

    fn fallible_write_to_string<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        mut handler: F,
    ) -> Result<Cow<str>, E> {
        let hint = self.fallible_writeable_length_hint(&mut handler)?;
        if hint.is_zero() {
            return Ok(Cow::Borrowed(""));
        }
        let mut output = String::with_capacity(hint.capacity());
        match self.fallible_write_to(&mut output, handler) {
            Ok(Ok(())) => Ok(Cow::Owned(output)),
            Ok(Err(err)) => Err(err),
            Err(core::fmt::Error) => unreachable!("writing to string is infallible"),
        }
    }

    fn fallible_write_cmp_bytes<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        other: &[u8],
        handler: F,
    ) -> Result<core::cmp::Ordering, E> {
        let mut wc = cmp::WriteComparator::new(other);
        match self.fallible_write_to(&mut wc, handler) {
            Ok(Ok(())) => (),
            Ok(Err(e)) => return Err(e),
            Err(core::fmt::Error) => unreachable!("writing to string is infallible"),
        }
        Ok(wc.finish().reverse())
    }

    #[inline]
    fn resulty(&self) -> ResultyWriteable<&Self> {
        ResultyWriteable(self)
    }

    #[inline]
    fn lossy(&self) -> LossyWriteable<&Self> {
        LossyWriteable(self)
    }

    #[inline]
    fn panicky(&self) -> PanickyWriteable<&Self> {
        PanickyWriteable(self)
    }
}

#[derive(Debug)]
pub struct ResultyWriteable<T>(pub(crate) T);

impl<T> ResultyWriteable<T>
where
    T: FallibleWriteable,
{
    #[inline]
    pub fn try_write_to<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<Result<(), T::Error>, fmt::Error> {
        self.0
            .fallible_write_to(sink, |e| Err::<NeverWriteable, _>(e))
    }

    #[inline]
    pub fn try_write_to_parts<S: FalliblePartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), T::Error>, fmt::Error> {
        self.0
            .fallible_write_to_parts(sink, |e| Err::<NeverWriteable, _>(e))
    }
}

#[derive(Debug)]
pub struct LossyWriteable<T>(pub(crate) T);

impl<T> Writeable for LossyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: Writeable,
{
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let infallible_result = self
            .0
            .fallible_write_to(sink, |err| Ok::<_, Infallible>(err))?;
        Ok(infallible_result.unwrap_or_else(|never| match never {}))
    }

    // TODO: for now, use the default impl
    /*
    #[inline]
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        todo!()
    }
    */

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        self.0
            .fallible_writeable_length_hint(|err| Ok::<_, Infallible>(err))
            .unwrap_or_else(|never| match never {})
    }

    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        self.0
            .fallible_write_to_string(|err| Ok::<_, Infallible>(err))
            .unwrap_or_else(|never| match never {})
    }

    #[inline]
    fn write_cmp_bytes(&self, other: &[u8]) -> core::cmp::Ordering {
        self.0
            .fallible_write_cmp_bytes(other, |e| Ok::<_, Infallible>(e))
            .unwrap_or_else(|never| match never {})
    }
}

impl<T> fmt::Display for LossyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: Writeable,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

#[derive(Debug)]
pub struct PanickyWriteable<T>(pub(crate) T);

impl<T> Writeable for PanickyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: fmt::Debug,
{
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let result = self
            .0
            .fallible_write_to(sink, |err| Err::<NeverWriteable, _>(err))?;
        result.unwrap();
        Ok(())
    }

    // TODO: for now, use the default impl
    /*
    #[inline]
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        todo!()
    }
    */

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        let result = self
            .0
            .fallible_writeable_length_hint(|e| Err::<NeverWriteable, _>(e));
        result.unwrap()
    }

    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        let result = self
            .0
            .fallible_write_to_string(|e| Err::<NeverWriteable, _>(e));
        result.unwrap()
    }

    #[inline]
    fn write_cmp_bytes(&self, other: &[u8]) -> core::cmp::Ordering {
        let result = self
            .0
            .fallible_write_cmp_bytes(other, |e| Err::<NeverWriteable, _>(e));
        result.unwrap()
    }
}

impl<T> fmt::Display for PanickyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl<T> FallibleWriteable for Option<T>
where
    T: Writeable,
{
    type Error = MissingWriteableError;

    fn fallible_write_to<
        W: fmt::Write + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut W,
        mut handler: F,
    ) -> Result<Result<(), E>, fmt::Error> {
        match self {
            Some(writeable) => writeable.write_to(sink).map(Ok),
            None => match handler(MissingWriteableError) {
                Ok(l) => l.write_to(sink).map(Ok),
                Err(e) => Ok(Err(e)),
            },
        }
    }

    fn fallible_write_to_parts<
        S: FalliblePartsWrite + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut S,
        mut handler: F,
    ) -> Result<Result<(), E>, fmt::Error> {
        match self {
            Some(writeable) => writeable.write_to_parts(sink).map(Ok),
            None => match handler(MissingWriteableError) {
                Ok(l) => l.write_to_parts(sink).map(Ok),
                Err(e) => Ok(Err(e)),
            },
        }
    }

    fn fallible_writeable_length_hint<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        mut handler: F,
    ) -> Result<LengthHint, E> {
        match self {
            Some(writeable) => Ok(writeable.writeable_length_hint()),
            None => handler(MissingWriteableError).map(|l| l.writeable_length_hint()),
        }
    }

    fn fallible_write_to_string<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        mut handler: F,
    ) -> Result<Cow<str>, E> {
        match self {
            Some(writeable) => Ok(writeable.write_to_string()),
            None => {
                handler(MissingWriteableError).map(|e| Cow::Owned(e.write_to_string().into_owned()))
            }
        }
    }

    fn fallible_write_cmp_bytes<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        other: &[u8],
        mut handler: F,
    ) -> Result<core::cmp::Ordering, E> {
        match self {
            Some(writeable) => Ok(writeable.write_cmp_bytes(other)),
            None => handler(MissingWriteableError).map(|l| l.write_cmp_bytes(other)),
        }
    }
}

#[test]
fn test_basic() {
    let mut sink = String::new();

    Some("abcdefg")
        .resulty()
        .try_write_to(&mut sink)
        .unwrap()
        .unwrap();
    assert_eq!(sink, "abcdefg");
    assert!(matches!(
        None::<&str>.resulty().try_write_to(&mut sink),
        Ok(Err(MissingWriteableError))
    ));

    assert_writeable_eq!(Some("abcdefg").lossy(), "abcdefg");
    assert_eq!(
        Some("abcdefg").lossy().writeable_length_hint(),
        LengthHint::exact(7)
    );

    assert_writeable_eq!(None::<&str>.lossy(), "�");
    assert_eq!(
        None::<&str>.lossy().writeable_length_hint(),
        LengthHint::exact(3)
    );

    assert_writeable_eq!(Some("abcdefg").panicky(), "abcdefg");
    assert_eq!(
        Some("abcdefg").panicky().writeable_length_hint(),
        LengthHint::exact(7)
    );
}
