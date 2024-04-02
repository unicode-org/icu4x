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

pub type WriteableResult<E> = Result<Result<(), E>, fmt::Error>;

pub trait FalliblePartsWrite: PartsWrite {
    type Error;
    type SubFalliblePartsWrite: FalliblePartsWrite + ?Sized;

    fn try_with_part(
        &mut self,
        part: Part,
        f: impl FnMut(&mut Self::SubPartsWrite) -> WriteableResult<Self::Error>,
    ) -> WriteableResult<Self::Error>;
}

pub trait FallibleWriteable {
    type Error;

    fn try_write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> WriteableResult<Self::Error> {
        self.try_write_to_with_handler(sink, |e, _| Ok(Err(e)))
    }

    fn try_write_to_with_handler<
        E,
        W: fmt::Write + ?Sized,
        F: FnMut(Self::Error, &mut W) -> WriteableResult<E>,
    >(
        &self,
        sink: &mut W,
        handler: F,
    ) -> WriteableResult<E>;

    fn try_write_to_parts<S: FalliblePartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> WriteableResult<Self::Error> {
        self.try_write_to_parts_with_handler(sink, |e, _| Ok(Err(e)))
    }

    fn try_write_to_parts_with_handler<
        E,
        S: FalliblePartsWrite + ?Sized,
        F: FnMut(Self::Error, &mut S) -> WriteableResult<E>,
    >(
        &self,
        sink: &mut S,
        handler: F,
    ) -> WriteableResult<E>;

    fn lossy(&self) -> LossyWriteable<&Self> {
        LossyWriteable(self)
    }

    fn panicky(&self) -> PanickyWriteable<&Self> {
        PanickyWriteable(self)
    }
}

#[derive(Debug)]
pub struct LossyWriteable<T>(pub(crate) T);

impl<T> Writeable for LossyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: Writeable,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let infallible_result = self.0.try_write_to_with_handler(sink, |err, sink| {
            err.write_to(sink)?;
            Ok(Ok::<(), Infallible>(()))
        })?;
        Ok(infallible_result.unwrap_or_else(|never| match never {}))
    }

    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        todo!()
    }

    fn writeable_length_hint(&self) -> LengthHint {
        todo!()
    }

    fn write_to_string(&self) -> Cow<str> {
        todo!()
    }

    fn write_cmp_bytes(&self, other: &[u8]) -> core::cmp::Ordering {
        todo!()
    }
}

#[derive(Debug)]
pub struct PanickyWriteable<T>(pub(crate) T);

impl<T> Writeable for PanickyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: fmt::Debug,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let result = self.0.try_write_to_with_handler(sink, |e, _| Ok(Err(e)))?;
        result.unwrap();
        Ok(())
    }

    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        todo!()
    }

    fn writeable_length_hint(&self) -> LengthHint {
        todo!()
    }

    fn write_to_string(&self) -> Cow<str> {
        todo!()
    }

    fn write_cmp_bytes(&self, other: &[u8]) -> core::cmp::Ordering {
        todo!()
    }
}

impl<T> FallibleWriteable for Option<T>
where
    T: Writeable,
{
    type Error = MissingWriteableError;

    fn try_write_to_with_handler<
        E,
        W: fmt::Write + ?Sized,
        F: FnMut(Self::Error, &mut W) -> WriteableResult<E>,
    >(
        &self,
        sink: &mut W,
        mut handler: F,
    ) -> WriteableResult<E> {
        match self {
            Some(writeable) => writeable.write_to(sink).map(Ok),
            None => handler(MissingWriteableError, sink),
        }
    }

    fn try_write_to_parts_with_handler<
        E,
        S: FalliblePartsWrite + ?Sized,
        F: FnMut(Self::Error, &mut S) -> WriteableResult<E>,
    >(
        &self,
        sink: &mut S,
        mut handler: F,
    ) -> WriteableResult<E> {
        match self {
            Some(writeable) => writeable.write_to_parts(sink).map(Ok),
            None => handler(MissingWriteableError, sink),
        }
    }
}

#[test]
fn test_basic() {
    let mut sink = String::new();

    Some("abc").try_write_to(&mut sink).unwrap().unwrap();
    assert_eq!(sink, "abc");
    assert!(matches!(
        None::<&str>.try_write_to(&mut sink),
        Ok(Err(MissingWriteableError))
    ));

    sink.clear();
    Some("def").lossy().write_to(&mut sink).unwrap();
    assert_eq!(sink, "def");
    sink.clear();
    None::<&str>.lossy().write_to(&mut sink).unwrap();
    assert_eq!(sink, "�");
}
