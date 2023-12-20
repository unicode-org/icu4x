// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use core::marker::PhantomData;

use crate::format::neo::*;
use crate::input::ExtractedDateTimeInput;
use crate::raw::neo::*;
use crate::CldrCalendar;

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoDateFormatter<C: CldrCalendar> {
    inner: RawNeoDateFormatter,
    _calendar: PhantomData<C>,
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoDate<'a> {
    pattern: DatePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoTimeFormatter<C: CldrCalendar> {
    inner: RawNeoTimeFormatter,
    _calendar: PhantomData<C>,
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoTime<'a> {
    pattern: TimePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoDateTimeFormatter<C: CldrCalendar> {
    inner: RawNeoDateTimeFormatter,
    _calendar: PhantomData<C>,
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoDateTime<'a> {
    pattern: DateTimePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}
