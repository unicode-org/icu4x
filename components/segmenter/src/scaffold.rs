// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "unstable")]
use crate::GraphemeClusterSegmenterBorrowed;
#[cfg(feature = "unstable")]
use crate::complex::{ComplexIterator, ComplexPayloadBorrowed, ComplexPayloadsBorrowed};
#[cfg(feature = "unstable")]
use crate::provider::ComplexScript;

use crate::indices::*;

/// A trait allowing for `RuleBreakIterator` to be generalized to multiple string
/// encoding methods and granularity such as grapheme cluster, word, etc.
///
/// <div class="stab unstable">
/// 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this
/// trait, please consider using a type from the implementors listed below.
/// </div>
pub trait RuleBreakType: crate::private::Sealed + Sized {
    /// The iterator over characters.
    type IterAttr<'s>: Iterator<Item = (usize, Self::CharType)> + Clone + core::fmt::Debug;

    /// The character type.
    type CharType: Copy + Into<u32> + core::fmt::Debug;

    #[doc(hidden)]
    const CAN_CONTAIN_SA: bool;

    #[doc(hidden)]
    fn char_len(ch: Self::CharType) -> usize;

    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    type ComplexPayloads<'data>: core::fmt::Debug + Clone + Copy;
    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    type ComplexPayload<'data>: core::fmt::Debug;

    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    fn select_complex<'a>(
        data: &Self::ComplexPayloads<'a>,
        complex_script: ComplexScript,
    ) -> Option<Self::ComplexPayload<'a>>;

    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    fn handle_complex<'data, 's>(
        data: &Self::ComplexPayload<'data>,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> ComplexIterator<'data, 's, Self>;

    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    fn offset<'s>(iter: &Self::IterAttr<'s>) -> usize;

    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    fn is_empty<'s>(iter: &Self::IterAttr<'s>) -> bool;
}

#[derive(Debug)]
#[non_exhaustive]
/// [`RuleBreakType`] for UTF-8 strings
pub struct Utf8;

impl crate::private::Sealed for Utf8 {}

impl RuleBreakType for Utf8 {
    type IterAttr<'s> = CharIndices<'s>;
    type CharType = char;

    const CAN_CONTAIN_SA: bool = true;

    #[cfg(feature = "unstable")]
    type ComplexPayloads<'data> = ComplexPayloadsBorrowed<'data>;
    #[cfg(feature = "unstable")]
    type ComplexPayload<'data> = (
        ComplexPayloadBorrowed<'data>,
        GraphemeClusterSegmenterBorrowed<'data>,
    );

    #[cfg(feature = "unstable")]
    fn select_complex<'a>(
        &data: &Self::ComplexPayloads<'a>,
        complex_script: ComplexScript,
    ) -> Option<Self::ComplexPayload<'a>> {
        data.select(complex_script).map(|d| (d, data.grapheme))
    }

    #[cfg(feature = "unstable")]
    fn handle_complex<'data, 's>(
        &(lang, grapheme): &Self::ComplexPayload<'data>,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> ComplexIterator<'data, 's, Self> {
        let complex_offset = complex.offset();
        #[allow(clippy::indexing_slicing)] // valid offset
        let complex = &complex.as_str()[..(past_complex.offset() - complex_offset)];
        lang.segment_str(complex, grapheme, complex_offset)
    }

    fn char_len(ch: Self::CharType) -> usize {
        ch.len_utf8()
    }

    #[cfg(feature = "unstable")]
    fn offset<'s>(iter: &Self::IterAttr<'s>) -> usize {
        iter.offset()
    }

    #[cfg(feature = "unstable")]
    fn is_empty<'s>(iter: &Self::IterAttr<'s>) -> bool {
        iter.as_str().is_empty()
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// [`RuleBreakType`] for potentially ill-formed UTF-8 strings
pub struct PotentiallyIllFormedUtf8;

impl crate::private::Sealed for PotentiallyIllFormedUtf8 {}

impl RuleBreakType for PotentiallyIllFormedUtf8 {
    type IterAttr<'s> = Utf8CharIndices<'s>;
    type CharType = char;

    const CAN_CONTAIN_SA: bool = true;

    #[cfg(feature = "unstable")]
    type ComplexPayloads<'data> = ComplexPayloadsBorrowed<'data>;
    #[cfg(feature = "unstable")]
    type ComplexPayload<'data> = (
        ComplexPayloadBorrowed<'data>,
        GraphemeClusterSegmenterBorrowed<'data>,
    );

    #[cfg(feature = "unstable")]
    fn select_complex<'a>(
        data: &Self::ComplexPayloads<'a>,
        complex_script: ComplexScript,
    ) -> Option<Self::ComplexPayload<'a>> {
        Utf8::select_complex(data, complex_script)
    }

    #[cfg(feature = "unstable")]
    fn handle_complex<'data, 's>(
        &(lang, grapheme): &Self::ComplexPayload<'data>,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> ComplexIterator<'data, 's, Self> {
        let offset = complex.offset();
        #[allow(clippy::indexing_slicing)] // valid offset
        let complex = &complex.as_slice()[..(past_complex.offset() - offset)];
        lang.segment_utf8(complex, grapheme, offset)
    }

    fn char_len(ch: Self::CharType) -> usize {
        ch.len_utf8()
    }

    #[cfg(feature = "unstable")]
    fn offset<'s>(iter: &Self::IterAttr<'s>) -> usize {
        iter.offset()
    }

    #[cfg(feature = "unstable")]
    fn is_empty<'s>(iter: &Self::IterAttr<'s>) -> bool {
        iter.as_slice().is_empty()
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// [`RuleBreakType`] for Latin-1 strings
pub struct Latin1;

impl crate::private::Sealed for Latin1 {}

impl RuleBreakType for Latin1 {
    type IterAttr<'s> = Latin1Indices<'s>;
    type CharType = u8;

    const CAN_CONTAIN_SA: bool = false;

    #[cfg(feature = "unstable")]
    type ComplexPayloads<'data> = core::convert::Infallible;
    #[cfg(feature = "unstable")]
    type ComplexPayload<'data> = core::convert::Infallible;

    #[cfg(feature = "unstable")]
    fn select_complex<'a>(
        &complex_payloads: &Self::ComplexPayloads<'a>,
        _: ComplexScript,
    ) -> Option<Self::ComplexPayload<'a>> {
        match complex_payloads {}
    }

    #[cfg(feature = "unstable")]
    fn handle_complex<'data, 's>(
        &complex_payload: &Self::ComplexPayloads<'data>,
        _: &Self::IterAttr<'s>,
        _: &Self::IterAttr<'s>,
    ) -> ComplexIterator<'data, 's, Self> {
        match complex_payload {}
    }

    fn char_len(_ch: Self::CharType) -> usize {
        unreachable!()
    }

    #[cfg(feature = "unstable")]
    fn offset<'s>(iter: &Self::IterAttr<'s>) -> usize {
        iter.offset()
    }

    #[cfg(feature = "unstable")]
    fn is_empty<'s>(iter: &Self::IterAttr<'s>) -> bool {
        iter.as_slice().is_empty()
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// [`RuleBreakType`] for UTF-16 strings
pub struct Utf16;

impl crate::private::Sealed for Utf16 {}

impl RuleBreakType for Utf16 {
    type IterAttr<'s> = Utf16Indices<'s>;
    type CharType = u32;

    const CAN_CONTAIN_SA: bool = true;

    #[cfg(feature = "unstable")]
    type ComplexPayloads<'data> = ComplexPayloadsBorrowed<'data>;
    #[cfg(feature = "unstable")]
    type ComplexPayload<'data> = (
        ComplexPayloadBorrowed<'data>,
        GraphemeClusterSegmenterBorrowed<'data>,
    );

    #[cfg(feature = "unstable")]
    fn select_complex<'a>(
        data: &Self::ComplexPayloads<'a>,
        complex_script: ComplexScript,
    ) -> Option<Self::ComplexPayload<'a>> {
        Utf8::select_complex(data, complex_script)
    }

    #[cfg(feature = "unstable")]
    fn handle_complex<'data, 's>(
        &(lang, grapheme): &Self::ComplexPayload<'data>,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> ComplexIterator<'data, 's, Utf16> {
        let complex_offset = complex.offset();
        #[allow(clippy::indexing_slicing)] // valid offset
        let complex = &complex.as_slice()[..(past_complex.offset() - complex_offset)];
        lang.segment_utf16(complex, grapheme, complex_offset)
    }

    fn char_len(ch: Self::CharType) -> usize {
        if ch >= 0x10000 { 2 } else { 1 }
    }

    #[cfg(feature = "unstable")]
    fn offset<'s>(iter: &Self::IterAttr<'s>) -> usize {
        iter.offset()
    }

    #[cfg(feature = "unstable")]
    fn is_empty<'s>(iter: &Self::IterAttr<'s>) -> bool {
        iter.as_slice().is_empty()
    }
}

/// A trait allowing for [`WordBreakIterator`](crate::iterators::WordBreakIterator) to be generalized to multiple
/// string iteration methods.
///
/// This is implemented by ICU4X for several common string types.
///
/// <div class="stab unstable">
/// 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this
/// trait, please consider using a type from the implementors listed below.
/// </div>
pub trait WordBreakType: crate::private::Sealed + Sized + RuleBreakType {}
impl WordBreakType for Utf8 {}
impl WordBreakType for PotentiallyIllFormedUtf8 {}
impl WordBreakType for Latin1 {}
impl WordBreakType for Utf16 {}

/// A trait allowing for `LineBreakIterator` to be generalized to multiple string iteration methods.
///
/// This is implemented by ICU4X for several common string types.
///
/// <div class="stab unstable">
/// 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this
/// trait, please consider using a type from the implementors listed below.
/// </div>
pub trait LineBreakType: crate::private::Sealed + Sized + RuleBreakType {}
impl LineBreakType for Utf8 {}
impl LineBreakType for PotentiallyIllFormedUtf8 {}
impl LineBreakType for Latin1 {}
impl LineBreakType for Utf16 {}
