// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::LineBreakOptions;
use icu_segmenter::LineBreakRule;
use icu_segmenter::WordBreakRule;

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use core::convert::TryFrom;
    use diplomat_runtime::DiplomatResult;
    use icu_segmenter::LineBreakIterator;
    use icu_segmenter::LineBreakSegmenter;

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
    pub struct ICU4XLineBreakIterator<'a, 'b>(LineBreakIterator<'a, 'b, char>);

    impl ICU4XLineBreakSegmenter {
        /// Construct a [`ICU4XLineBreakSegmenter`] with default options.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new) for more information.
        pub fn try_new() -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ()> {
            LineBreakSegmenter::try_new()
                .map(|o| Box::new(ICU4XLineBreakSegmenter(o)))
                .map_err(|_| ())
                .into()
        }

        /// Construct a [`ICU4XLineBreakSegmenter`] with custom options.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new_with_options) for more information.
        pub fn try_new_with_options(options: ICU4XLineBreakOptions) -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ()> {
            LineBreakSegmenter::try_new_with_options(options.into())
            .map(|o| Box::new(ICU4XLineBreakSegmenter(o)))
            .map_err(|_| ())
            .into()
        }

        pub fn segment_str<'a, 'b>(
            &'a self,
            input: &'b str,
        ) -> Box<ICU4XLineBreakIterator<'a, 'b>> {
            Box::new(ICU4XLineBreakIterator(self.0.segment_str(input)))
        }
    }

    impl<'a, 'b> ICU4XLineBreakIterator<'a, 'b> {
        /// Find the next breakpoint. Returns -1 if at the end of the string or if the index is
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
