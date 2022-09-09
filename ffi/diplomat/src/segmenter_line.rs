// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::LineBreakOptions;
use icu_segmenter::LineBreakRule;
use icu_segmenter::WordBreakRule;

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::convert::TryFrom;
    use diplomat_runtime::DiplomatResult;
    use icu_provider::DataProvider;
    use icu_segmenter::provider::{
        LineBreakDataV1Marker, LstmDataV1Marker, UCharDictionaryBreakDataV1Marker,
    };
    use icu_segmenter::{
        LineBreakIteratorLatin1, LineBreakIteratorPotentiallyIllFormedUtf8, LineBreakIteratorUtf16,
        LineBreakSegmenter,
    };

    #[diplomat::opaque]
    /// An ICU4X line-break segmenter, capable of finding breakpoints in strings.
    #[diplomat::rust_link(icu::segmenter::LineBreakSegmenter, Struct)]
    pub struct ICU4XLineBreakSegmenter(LineBreakSegmenter);

    #[diplomat::rust_link(icu::segmenter::LineBreakRule, Enum)]
    pub enum ICU4XLineBreakRule {
        Loose,
        Normal,
        Strict,
        Anywhere,
    }

    #[diplomat::rust_link(icu::segmenter::WordBreakRule, Enum)]
    pub enum ICU4XWordBreakRule {
        Normal,
        BreakAll,
        KeepAll,
    }

    #[diplomat::rust_link(icu::segmenter::LineBreakOptions, Struct)]
    pub struct ICU4XLineBreakOptions {
        pub line_break_rule: ICU4XLineBreakRule,
        pub word_break_rule: ICU4XWordBreakRule,
        pub ja_zh: bool,
    }

    #[diplomat::opaque]
    pub struct ICU4XLineBreakIteratorUtf8<'a>(LineBreakIteratorPotentiallyIllFormedUtf8<'a, 'a>);

    #[diplomat::opaque]
    pub struct ICU4XLineBreakIteratorUtf16<'a>(LineBreakIteratorUtf16<'a, 'a>);

    #[diplomat::opaque]
    pub struct ICU4XLineBreakIteratorLatin1<'a>(LineBreakIteratorLatin1<'a, 'a>);

    impl ICU4XLineBreakSegmenter {
        /// Construct a [`ICU4XLineBreakSegmenter`] with default options.
        #[diplomat::rust_link(icu::segmenter::LineBreakSegmenter::try_new, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ICU4XError> {
            Self::try_new_impl(&provider.0)
        }

        fn try_new_impl<D>(provider: &D) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ICU4XError>
        where
            D: DataProvider<LineBreakDataV1Marker>
                + DataProvider<UCharDictionaryBreakDataV1Marker>
                + DataProvider<LstmDataV1Marker>
                + ?Sized,
        {
            LineBreakSegmenter::try_new(provider)
                .map(|o| Box::new(ICU4XLineBreakSegmenter(o)))
                .map_err(Into::into)
                .into()
        }

        /// Construct a [`ICU4XLineBreakSegmenter`] with custom options.
        #[diplomat::rust_link(icu::segmenter::LineBreakSegmenter::try_new_with_options, FnInStruct)]
        pub fn try_new_with_options(
            provider: &ICU4XDataProvider,
            options: ICU4XLineBreakOptions,
        ) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ICU4XError> {
            Self::try_new_with_options_impl(&provider.0, options)
        }

        fn try_new_with_options_impl<D>(
            provider: &D,
            options: ICU4XLineBreakOptions,
        ) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ICU4XError>
        where
            D: DataProvider<LineBreakDataV1Marker>
                + DataProvider<UCharDictionaryBreakDataV1Marker>
                + DataProvider<LstmDataV1Marker>
                + ?Sized,
        {
            LineBreakSegmenter::try_new_with_options(provider, options.into())
                .map(|o| Box::new(ICU4XLineBreakSegmenter(o)))
                .map_err(Into::into)
                .into()
        }

        /// Segments a (potentially ill-formed) UTF-8 string.
        #[diplomat::rust_link(icu::segmenter::LineBreakSegmenter::segment_utf8, FnInStruct)]
        #[diplomat::rust_link(icu::segmenter::LineBreakSegmenter::segment_str, FnInStruct, hidden)]
        pub fn segment_utf8<'a>(&'a self, input: &'a str) -> Box<ICU4XLineBreakIteratorUtf8<'a>> {
            let input = input.as_bytes(); // #2520
            Box::new(ICU4XLineBreakIteratorUtf8(self.0.segment_utf8(input)))
        }

        /// Segments a UTF-16 string.
        #[diplomat::rust_link(icu::segmenter::LineBreakSegmenter::segment_utf16, FnInStruct)]
        pub fn segment_utf16<'a>(
            &'a self,
            input: &'a [u16],
        ) -> Box<ICU4XLineBreakIteratorUtf16<'a>> {
            Box::new(ICU4XLineBreakIteratorUtf16(self.0.segment_utf16(input)))
        }

        /// Segments a Latin-1 string.
        #[diplomat::rust_link(icu::segmenter::LineBreakSegmenter::segment_latin1, FnInStruct)]
        pub fn segment_latin1<'a>(
            &'a self,
            input: &'a [u8],
        ) -> Box<ICU4XLineBreakIteratorLatin1<'a>> {
            Box::new(ICU4XLineBreakIteratorLatin1(self.0.segment_latin1(input)))
        }
    }

    impl<'a> ICU4XLineBreakIteratorUtf8<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> ICU4XLineBreakIteratorUtf16<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> ICU4XLineBreakIteratorLatin1<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }
}

impl From<ffi::ICU4XLineBreakRule> for LineBreakRule {
    fn from(other: ffi::ICU4XLineBreakRule) -> Self {
        match other {
            ffi::ICU4XLineBreakRule::Loose => Self::Loose,
            ffi::ICU4XLineBreakRule::Normal => Self::Normal,
            ffi::ICU4XLineBreakRule::Strict => Self::Strict,
            ffi::ICU4XLineBreakRule::Anywhere => Self::Anywhere,
        }
    }
}

impl From<ffi::ICU4XWordBreakRule> for WordBreakRule {
    fn from(other: ffi::ICU4XWordBreakRule) -> Self {
        match other {
            ffi::ICU4XWordBreakRule::Normal => Self::Normal,
            ffi::ICU4XWordBreakRule::BreakAll => Self::BreakAll,
            ffi::ICU4XWordBreakRule::KeepAll => Self::KeepAll,
        }
    }
}

impl From<ffi::ICU4XLineBreakOptions> for LineBreakOptions {
    fn from(other: ffi::ICU4XLineBreakOptions) -> Self {
        let mut options = LineBreakOptions::default();
        options.line_break_rule = other.line_break_rule.into();
        options.word_break_rule = other.word_break_rule.into();
        options.ja_zh = other.ja_zh;
        options
    }
}
