// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::core::{ParseResult, Parsed};

/// A trait to map only the value of a [`Parsed`] or [`ParseResult`], propagating the source.
pub trait MapValue<ValueIn, ValueOut, Source> {
    /// The resulting type after the value has been mapped.
    type Mapped;

    /// Maps the value and propagates the source.
    fn map_value(self, f: impl FnOnce(ValueIn) -> ValueOut) -> Self::Mapped;
}

impl<ValueIn, ValueOut, Source: Clone> MapValue<ValueIn, ValueOut, Source>
    for Parsed<ValueIn, Source>
{
    type Mapped = Parsed<ValueOut, Source>;

    /// Maps the value of the [`Parsed`] object, propagating the source.
    fn map_value(self, f: impl FnOnce(ValueIn) -> ValueOut) -> Self::Mapped {
        let (value, source) = self.split();
        Self::Mapped::new(f(value), source)
    }
}

impl<Mappable, ValueIn, ValueOut, Source> MapValue<ValueIn, ValueOut, Source>
    for eyre::Result<Mappable>
where
    Mappable: MapValue<ValueIn, ValueOut, Source>,
{
    type Mapped = eyre::Result<<Mappable as MapValue<ValueIn, ValueOut, Source>>::Mapped>;

    /// Maps the value of the [`ParseResult`] object, propagating the source.
    /// If `Self` is already [`Err`], propagates the error.
    fn map_value(self, f: impl FnOnce(ValueIn) -> ValueOut) -> Self::Mapped {
        self.map(|inner| inner.map_value(f))
    }
}

/// An extension trait to map the inner value of an [`Option`].
pub trait MapInner<InnerIn, InnerOut, Source> {
    /// The resulting type after the value has been mapped.
    type Mapped;

    /// Maps the value and propagates the source.
    fn map_inner(self, f: impl FnOnce(InnerIn) -> InnerOut) -> Self::Mapped;
}

impl<InnerIn, InnerOut, Source: Clone> MapInner<InnerIn, InnerOut, Source>
    for Parsed<Option<InnerIn>, Source>
{
    type Mapped = Parsed<Option<InnerOut>, Source>;

    /// Maps the value of the [`Parsed`] object, propagating the source.
    fn map_inner(self, f: impl FnOnce(InnerIn) -> InnerOut) -> Self::Mapped {
        let (value, source) = self.split();
        Self::Mapped::new(value.map(f), source)
    }
}

impl<InnerIn, InnerOut, Source: Clone> MapInner<InnerIn, InnerOut, Source>
    for ParseResult<Option<InnerIn>, Source>
{
    type Mapped = ParseResult<Option<InnerOut>, Source>;

    /// Maps the value of the [`Parsed`] object, propagating the source.
    fn map_inner(self, f: impl FnOnce(InnerIn) -> InnerOut) -> Self::Mapped {
        self.map(|parsed| parsed.map_inner(f))
    }
}

/// A fallible trait to map only the value of a [`Parsed`] or [`ParseResult`], propagating the source.
pub trait TryMapValue<ValueIn, ValueOut, Source> {
    /// The resulting type after the value has been mapped.
    type Mapped;

    /// Maps a fallible function over the value and propagates the source.
    /// Returns an error if the map fails.
    fn try_map_value(
        self,
        f: impl FnOnce(ValueIn) -> eyre::Result<ValueOut>,
    ) -> eyre::Result<Self::Mapped>;
}

impl<ValueIn, ValueOut, Source: Clone> TryMapValue<ValueIn, ValueOut, Source>
    for Parsed<ValueIn, Source>
{
    type Mapped = Parsed<ValueOut, Source>;

    /// Maps a fallible function over the value of the [`Parsed`], propagating the source.
    fn try_map_value(
        self,
        f: impl FnOnce(ValueIn) -> eyre::Result<ValueOut>,
    ) -> eyre::Result<Self::Mapped> {
        let (value, source) = self.split();
        Ok(Self::Mapped::new(f(value)?, source))
    }
}

impl<Mappable, ValueIn, ValueOut, Source> TryMapValue<ValueIn, ValueOut, Source>
    for eyre::Result<Mappable>
where
    Mappable: TryMapValue<ValueIn, ValueOut, Source>,
{
    type Mapped = <Mappable as TryMapValue<ValueIn, ValueOut, Source>>::Mapped;

    /// Maps a fallible function over the value of the [`ParseResult`] object, propagating the source.
    /// If `Self` is already [`Err`], propagates the error.
    fn try_map_value(
        self,
        f: impl FnOnce(ValueIn) -> eyre::Result<ValueOut>,
    ) -> eyre::Result<Self::Mapped> {
        self.and_then(|inner| inner.try_map_value(f))
    }
}

/// A trait to map only the source of a [`Parsed`] or [`ParseResult`], propagating the value.
pub trait MapSource<Value, SourceIn, SourceOut> {
    /// The resulting type after the source has been mapped.
    type Mapped;

    /// Maps the source and propagates the value.
    fn map_source(self, f: impl FnOnce(SourceIn) -> SourceOut) -> Self::Mapped;
}

impl<Value, SourceIn: Clone, SourceOut: Clone> MapSource<Value, SourceIn, SourceOut>
    for Parsed<Value, SourceIn>
{
    type Mapped = Parsed<Value, SourceOut>;

    /// Maps the source of the [`Parsed`], propagating the value.
    fn map_source(self, f: impl FnOnce(SourceIn) -> SourceOut) -> Self::Mapped {
        let (value, source) = self.split();
        Self::Mapped::new(value, f(source))
    }
}

impl<Mappable, Value, SourceIn, SourceOut> MapSource<Value, SourceIn, SourceOut>
    for eyre::Result<Mappable>
where
    Mappable: MapSource<Value, SourceIn, SourceOut>,
{
    type Mapped = eyre::Result<<Mappable as MapSource<Value, SourceIn, SourceOut>>::Mapped>;

    /// Maps the source of the [`Parsed`], propagating the value.
    /// If `Self` is already [`Err`], propagates the error.
    fn map_source(self, f: impl FnOnce(SourceIn) -> SourceOut) -> Self::Mapped {
        self.map(|inner| inner.map_source(f))
    }
}
