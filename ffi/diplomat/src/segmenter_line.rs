// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::LineBreakOptions;
use icu_segmenter::LineBreakRule;
use icu_segmenter::WordBreakRule;

#[diplomat::bridge]
pub mod ffi {
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::convert::TryFrom;
    use diplomat_runtime::DiplomatResult;
    use icu_provider::ResourceProvider;
    use icu_segmenter::Latin1Char;
    use icu_segmenter::LineBreakDataV1Marker;
    use icu_segmenter::LineBreakIterator;
    use icu_segmenter::LineBreakSegmenter;
    use icu_segmenter::Utf16Char;

    #[diplomat::opaque]
    /// An ICU4X line-break segmenter, capable of finding breakpoints in strings.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html) for more information.
    pub struct ICU4XLineBreakSegmenter(LineBreakSegmenter);

    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakRule.html) for more information.
    pub enum ICU4XLineBreakRule {
        Loose,
        Normal,
        Strict,
        Anywhere,
    }

    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.WordBreakRule.html) for more information.
    pub enum ICU4XWordBreakRule {
        Normal,
        BreakAll,
        KeepAll,
    }

    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakOptions.html) for more information.
    pub struct ICU4XLineBreakOptions {
        pub line_break_rule: ICU4XLineBreakRule,
        pub word_break_rule: ICU4XWordBreakRule,
        pub ja_zh: bool,
    }

    #[diplomat::opaque]
    pub struct ICU4XLineBreakIteratorUtf8<'a>(LineBreakIterator<'a, 'a, char>);

    #[diplomat::opaque]
    pub struct ICU4XLineBreakIteratorUtf16<'a>(LineBreakIterator<'a, 'a, Utf16Char>);

    #[diplomat::opaque]
    pub struct ICU4XLineBreakIteratorLatin1<'a>(LineBreakIterator<'a, 'a, Latin1Char>);

    impl ICU4XLineBreakSegmenter {
        /// Construct a [`ICU4XLineBreakSegmenter`] with default options.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new) for more information.
        pub fn try_new(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ()> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            Self::try_new_impl(&provider)
        }

        fn try_new_impl<D>(provider: &D) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ()>
        where
            D: ResourceProvider<LineBreakDataV1Marker> + ?Sized,
        {
            LineBreakSegmenter::try_new(provider)
                .map(|o| Box::new(ICU4XLineBreakSegmenter(o)))
                .map_err(|_| ())
                .into()
        }

        /// Construct a [`ICU4XLineBreakSegmenter`] with custom options.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new_with_options) for more information.
        pub fn try_new_with_options(
            provider: &ICU4XDataProvider,
            options: ICU4XLineBreakOptions,
        ) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ()> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            Self::try_new_with_options_impl(&provider, options)
        }

        fn try_new_with_options_impl<D>(
            provider: &D,
            options: ICU4XLineBreakOptions,
        ) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ()>
        where
            D: ResourceProvider<LineBreakDataV1Marker> + ?Sized,
        {
            LineBreakSegmenter::try_new_with_options(provider, options.into())
                .map(|o| Box::new(ICU4XLineBreakSegmenter(o)))
                .map_err(|_| ())
                .into()
        }

        /// Segments a UTF-8 string.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_str) for more information.
        pub fn segment_utf8<'a>(&'a self, input: &'a str) -> Box<ICU4XLineBreakIteratorUtf8<'a>> {
            Box::new(ICU4XLineBreakIteratorUtf8(self.0.segment_str(input)))
        }

        /// Segments a UTF-16 string.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_utf16) for more information.
        pub fn segment_utf16<'a>(
            &'a self,
            input: &'a [u16],
        ) -> Box<ICU4XLineBreakIteratorUtf16<'a>> {
            Box::new(ICU4XLineBreakIteratorUtf16(self.0.segment_utf16(input)))
        }

        /// Segments a Latin-1 string.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_latin1) for more information.
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
