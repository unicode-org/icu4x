// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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

    #[diplomat::opaque]
    pub struct ICU4XLineBreakIterator<'a, 'b>(LineBreakIterator<'a, 'b, char>);

    impl ICU4XLineBreakSegmenter {
        /// Construct an [`ICU4XLineBreakSegmenter`] from an locale identifier.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.from_bytes) for more information.
        pub fn try_new() -> DiplomatResult<Box<ICU4XLineBreakSegmenter>, ()> {
            LineBreakSegmenter::try_new()
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
