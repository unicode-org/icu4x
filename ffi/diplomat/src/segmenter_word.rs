// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::convert::TryFrom;
    use icu_provider::DataProvider;
    use icu_segmenter::provider::{
        GraphemeClusterBreakDataV1Marker, LstmDataV1Marker, UCharDictionaryBreakDataV1Marker,
        WordBreakDataV1Marker,
    };
    use icu_segmenter::{
        WordBreakIteratorLatin1, WordBreakIteratorPotentiallyIllFormedUtf8, WordBreakIteratorUtf16,
        WordSegmenter,
    };

    #[diplomat::opaque]
    /// An ICU4X word-break segmenter, capable of finding word breakpoints in strings.
    #[diplomat::rust_link(icu::segmenter::WordSegmenter, Struct)]
    pub struct ICU4XWordSegmenter(WordSegmenter);

    #[diplomat::opaque]
    pub struct ICU4XWordBreakIteratorUtf8<'a>(WordBreakIteratorPotentiallyIllFormedUtf8<'a, 'a>);

    #[diplomat::opaque]
    pub struct ICU4XWordBreakIteratorUtf16<'a>(WordBreakIteratorUtf16<'a, 'a>);

    #[diplomat::opaque]
    pub struct ICU4XWordBreakIteratorLatin1<'a>(WordBreakIteratorLatin1<'a, 'a>);

    impl ICU4XWordSegmenter {
        /// Construct an [`ICU4XWordSegmenter`].
        #[diplomat::rust_link(icu::segmenter::WordSegmenter::try_new_unstable, FnInStruct)]
        pub fn create(provider: &ICU4XDataProvider) -> Result<Box<ICU4XWordSegmenter>, ICU4XError> {
            Self::try_new_impl(&provider.0)
        }

        fn try_new_impl<D>(provider: &D) -> Result<Box<ICU4XWordSegmenter>, ICU4XError>
        where
            D: DataProvider<WordBreakDataV1Marker>
                + DataProvider<UCharDictionaryBreakDataV1Marker>
                + DataProvider<LstmDataV1Marker>
                + DataProvider<GraphemeClusterBreakDataV1Marker>
                + ?Sized,
        {
            Ok(Box::new(ICU4XWordSegmenter(
                WordSegmenter::try_new_unstable(provider)?,
            )))
        }

        /// Segments a (potentially ill-formed) UTF-8 string.
        #[diplomat::rust_link(icu::segmenter::WordSegmenter::segment_utf8, FnInStruct)]
        #[diplomat::rust_link(icu::segmenter::WordSegmenter::segment_str, FnInStruct, hidden)]
        pub fn segment_utf8<'a>(&'a self, input: &'a str) -> Box<ICU4XWordBreakIteratorUtf8<'a>> {
            let input = input.as_bytes(); // #2520
            Box::new(ICU4XWordBreakIteratorUtf8(self.0.segment_utf8(input)))
        }

        /// Segments a UTF-16 string.
        #[diplomat::rust_link(icu::segmenter::WordSegmenter::segment_utf16, FnInStruct)]
        pub fn segment_utf16<'a>(
            &'a self,
            input: &'a [u16],
        ) -> Box<ICU4XWordBreakIteratorUtf16<'a>> {
            Box::new(ICU4XWordBreakIteratorUtf16(self.0.segment_utf16(input)))
        }

        /// Segments a Latin-1 string.
        #[diplomat::rust_link(icu::segmenter::WordSegmenter::segment_latin1, FnInStruct)]
        pub fn segment_latin1<'a>(
            &'a self,
            input: &'a [u8],
        ) -> Box<ICU4XWordBreakIteratorLatin1<'a>> {
            Box::new(ICU4XWordBreakIteratorLatin1(self.0.segment_latin1(input)))
        }
    }

    impl<'a> ICU4XWordBreakIteratorUtf8<'a> {
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

    impl<'a> ICU4XWordBreakIteratorUtf16<'a> {
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

    impl<'a> ICU4XWordBreakIteratorLatin1<'a> {
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
