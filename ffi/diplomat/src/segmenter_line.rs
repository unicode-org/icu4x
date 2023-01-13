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
    use icu_provider::DataProvider;
    use icu_segmenter::provider::{
        GraphemeClusterBreakDataV1Marker, LineBreakDataV1Marker, LstmDataV1Marker,
        UCharDictionaryBreakDataV1Marker,
    };
    use icu_segmenter::{
        LineBreakIteratorLatin1, LineBreakIteratorPotentiallyIllFormedUtf8, LineBreakIteratorUtf16,
        LineSegmenter,
    };

    #[diplomat::opaque]
    /// An ICU4X line-break segmenter, capable of finding breakpoints in strings.
    #[diplomat::rust_link(icu::segmenter::LineSegmenter, Struct)]
    pub struct ICU4XLineSegmenter(LineSegmenter);

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
    pub struct ICU4XLineBreakOptionsV1 {
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

    impl ICU4XLineSegmenter {
        /// Construct a [`ICU4XLineSegmenter`] with default options.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::try_new_unstable, FnInStruct)]
        pub fn create(provider: &ICU4XDataProvider) -> Result<Box<ICU4XLineSegmenter>, ICU4XError> {
            Self::try_new_impl(&provider.0)
        }

        fn try_new_impl<D>(provider: &D) -> Result<Box<ICU4XLineSegmenter>, ICU4XError>
        where
            D: DataProvider<LineBreakDataV1Marker>
                + DataProvider<UCharDictionaryBreakDataV1Marker>
                + DataProvider<LstmDataV1Marker>
                + DataProvider<GraphemeClusterBreakDataV1Marker>
                + ?Sized,
        {
            Ok(Box::new(ICU4XLineSegmenter(
                LineSegmenter::try_new_unstable(provider)?,
            )))
        }

        /// Construct a [`ICU4XLineSegmenter`] with custom options.
        #[diplomat::rust_link(
            icu::segmenter::LineSegmenter::try_new_with_options_unstable,
            FnInStruct
        )]
        pub fn create_with_options_v1(
            provider: &ICU4XDataProvider,
            options: ICU4XLineBreakOptionsV1,
        ) -> Result<Box<ICU4XLineSegmenter>, ICU4XError> {
            Self::try_new_with_options_impl(&provider.0, options)
        }

        fn try_new_with_options_impl<D>(
            provider: &D,
            options: ICU4XLineBreakOptionsV1,
        ) -> Result<Box<ICU4XLineSegmenter>, ICU4XError>
        where
            D: DataProvider<LineBreakDataV1Marker>
                + DataProvider<UCharDictionaryBreakDataV1Marker>
                + DataProvider<LstmDataV1Marker>
                + DataProvider<GraphemeClusterBreakDataV1Marker>
                + ?Sized,
        {
            Ok(Box::new(ICU4XLineSegmenter(
                LineSegmenter::try_new_with_options_unstable(provider, options.into())?,
            )))
        }

        /// Segments a (potentially ill-formed) UTF-8 string.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_utf8, FnInStruct)]
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_str, FnInStruct, hidden)]
        pub fn segment_utf8<'a>(&'a self, input: &'a str) -> Box<ICU4XLineBreakIteratorUtf8<'a>> {
            let input = input.as_bytes(); // #2520
            Box::new(ICU4XLineBreakIteratorUtf8(self.0.segment_utf8(input)))
        }

        /// Segments a UTF-16 string.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_utf16, FnInStruct)]
        pub fn segment_utf16<'a>(
            &'a self,
            input: &'a [u16],
        ) -> Box<ICU4XLineBreakIteratorUtf16<'a>> {
            Box::new(ICU4XLineBreakIteratorUtf16(self.0.segment_utf16(input)))
        }

        /// Segments a Latin-1 string.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_latin1, FnInStruct)]
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

impl From<ffi::ICU4XLineBreakOptionsV1> for LineBreakOptions {
    fn from(other: ffi::ICU4XLineBreakOptionsV1) -> Self {
        let mut options = LineBreakOptions::default();
        options.line_break_rule = other.line_break_rule.into();
        options.word_break_rule = other.word_break_rule.into();
        options.ja_zh = other.ja_zh;
        options
    }
}
