// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "ICU4X{0}"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::provider::ffi::DataProvider;

    #[diplomat::opaque]
    /// An ICU4X line-break segmenter, capable of finding breakpoints in strings.
    #[diplomat::rust_link(icu::segmenter::LineSegmenter, Struct)]
    pub struct LineSegmenter(icu_segmenter::LineSegmenter);

    #[diplomat::rust_link(icu::segmenter::LineBreakStrictness, Enum)]
    pub enum LineBreakStrictness {
        Loose,
        Normal,
        Strict,
        Anywhere,
    }

    #[diplomat::rust_link(icu::segmenter::LineBreakWordOption, Enum)]
    pub enum LineBreakWordOption {
        Normal,
        BreakAll,
        KeepAll,
    }

    #[diplomat::rust_link(icu::segmenter::LineBreakOptions, Struct)]
    #[diplomat::attr(dart, rename = "LineBreakOptions")]
    pub struct LineBreakOptionsV1 {
        pub strictness: LineBreakStrictness,
        pub word_option: LineBreakWordOption,
        pub ja_zh: bool,
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::segmenter::LineBreakIterator, Struct)]
    #[diplomat::rust_link(
        icu::segmenter::LineBreakIteratorPotentiallyIllFormedUtf8,
        Typedef,
        compact
    )]
    #[diplomat::rust_link(icu::segmenter::LineBreakIteratorUtf8, Typedef, hidden)]
    pub struct LineBreakIteratorUtf8<'a>(
        icu_segmenter::LineBreakIteratorPotentiallyIllFormedUtf8<'a, 'a>,
    );

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::segmenter::LineBreakIterator, Struct)]
    #[diplomat::rust_link(icu::segmenter::LineBreakIteratorUtf16, Typedef, compact)]
    pub struct LineBreakIteratorUtf16<'a>(icu_segmenter::LineBreakIteratorUtf16<'a, 'a>);

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::segmenter::LineBreakIterator, Struct)]
    #[diplomat::rust_link(icu::segmenter::LineBreakIteratorLatin1, Typedef, compact)]
    pub struct LineBreakIteratorLatin1<'a>(icu_segmenter::LineBreakIteratorLatin1<'a, 'a>);

    impl LineSegmenter {
        /// Construct a [`LineSegmenter`] with default options. It automatically loads the best
        /// available payload data for Burmese, Khmer, Lao, and Thai.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::new_auto, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "auto")]
        pub fn create_auto(provider: &DataProvider) -> Result<Box<LineSegmenter>, DataError> {
            Ok(Box::new(LineSegmenter(call_constructor!(
                icu_segmenter::LineSegmenter::new_auto [r => Ok(r)],
                icu_segmenter::LineSegmenter::try_new_auto_with_any_provider,
                icu_segmenter::LineSegmenter::try_new_auto_with_buffer_provider,
                provider
            )?)))
        }

        /// Construct a [`LineSegmenter`] with default options and LSTM payload data for
        /// Burmese, Khmer, Lao, and Thai.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::new_lstm, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "lstm")]
        pub fn create_lstm(provider: &DataProvider) -> Result<Box<LineSegmenter>, DataError> {
            Ok(Box::new(LineSegmenter(call_constructor!(
                icu_segmenter::LineSegmenter::new_lstm [r => Ok(r)],
                icu_segmenter::LineSegmenter::try_new_lstm_with_any_provider,
                icu_segmenter::LineSegmenter::try_new_lstm_with_buffer_provider,
                provider,
            )?)))
        }

        /// Construct a [`LineSegmenter`] with default options and dictionary payload data for
        /// Burmese, Khmer, Lao, and Thai..
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::new_dictionary, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "dictionary")]
        pub fn create_dictionary(provider: &DataProvider) -> Result<Box<LineSegmenter>, DataError> {
            Ok(Box::new(LineSegmenter(call_constructor!(
                icu_segmenter::LineSegmenter::new_dictionary [r => Ok(r)],
                icu_segmenter::LineSegmenter::try_new_dictionary_with_any_provider,
                icu_segmenter::LineSegmenter::try_new_dictionary_with_buffer_provider,
                provider,
            )?)))
        }

        /// Construct a [`LineSegmenter`] with custom options. It automatically loads the best
        /// available payload data for Burmese, Khmer, Lao, and Thai.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::new_auto_with_options, FnInStruct)]
        #[diplomat::attr(dart, rename = "auto_with_options")]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "auto_with_options_v1")]
        pub fn create_auto_with_options_v1(
            provider: &DataProvider,
            options: LineBreakOptionsV1,
        ) -> Result<Box<LineSegmenter>, DataError> {
            Ok(Box::new(LineSegmenter(call_constructor!(
                icu_segmenter::LineSegmenter::new_auto_with_options [r => Ok(r)],
                icu_segmenter::LineSegmenter::try_new_auto_with_options_with_any_provider,
                icu_segmenter::LineSegmenter::try_new_auto_with_options_with_buffer_provider,
                provider,
                options.into(),
            )?)))
        }

        /// Construct a [`LineSegmenter`] with custom options and LSTM payload data for
        /// Burmese, Khmer, Lao, and Thai.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::new_lstm_with_options, FnInStruct)]
        #[diplomat::attr(dart, rename = "lstm_with_options")]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "lstm_with_options_v1")]
        pub fn create_lstm_with_options_v1(
            provider: &DataProvider,
            options: LineBreakOptionsV1,
        ) -> Result<Box<LineSegmenter>, DataError> {
            Ok(Box::new(LineSegmenter(call_constructor!(
                icu_segmenter::LineSegmenter::new_lstm_with_options [r => Ok(r)],
                icu_segmenter::LineSegmenter::try_new_lstm_with_options_with_any_provider,
                icu_segmenter::LineSegmenter::try_new_lstm_with_options_with_buffer_provider,
                provider,
                options.into(),
            )?)))
        }

        /// Construct a [`LineSegmenter`] with custom options and dictionary payload data for
        /// Burmese, Khmer, Lao, and Thai.
        #[diplomat::rust_link(
            icu::segmenter::LineSegmenter::new_dictionary_with_options,
            FnInStruct
        )]
        #[diplomat::attr(dart, rename = "dictionary_with_options")]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "dictionary_with_options_v1")]
        pub fn create_dictionary_with_options_v1(
            provider: &DataProvider,
            options: LineBreakOptionsV1,
        ) -> Result<Box<LineSegmenter>, DataError> {
            Ok(Box::new(LineSegmenter(call_constructor!(
                icu_segmenter::LineSegmenter::new_dictionary_with_options [r => Ok(r)],
                icu_segmenter::LineSegmenter::try_new_dictionary_with_options_with_any_provider,
                icu_segmenter::LineSegmenter::try_new_dictionary_with_options_with_buffer_provider,
                provider,
                options.into(),
            )?)))
        }

        /// Segments a string.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_utf8, FnInStruct)]
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_str, FnInStruct, hidden)]
        #[diplomat::attr(dart, disable)]
        pub fn segment_utf8<'a>(
            &'a self,
            input: &'a DiplomatStr,
        ) -> Box<LineBreakIteratorUtf8<'a>> {
            Box::new(LineBreakIteratorUtf8(self.0.segment_utf8(input)))
        }

        /// Segments a string.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_utf16, FnInStruct)]
        #[diplomat::attr(dart, rename = "segment")]
        pub fn segment_utf16<'a>(
            &'a self,
            input: &'a DiplomatStr16,
        ) -> Box<LineBreakIteratorUtf16<'a>> {
            Box::new(LineBreakIteratorUtf16(self.0.segment_utf16(input)))
        }

        /// Segments a Latin-1 string.
        #[diplomat::rust_link(icu::segmenter::LineSegmenter::segment_latin1, FnInStruct)]
        #[diplomat::attr(dart, disable)]
        pub fn segment_latin1<'a>(&'a self, input: &'a [u8]) -> Box<LineBreakIteratorLatin1<'a>> {
            Box::new(LineBreakIteratorLatin1(self.0.segment_latin1(input)))
        }
    }

    impl<'a> LineBreakIteratorUtf8<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[diplomat::rust_link(icu::segmenter::LineBreakIterator::next, FnInStruct)]
        #[diplomat::rust_link(
            icu::segmenter::LineBreakIterator::Item,
            AssociatedTypeInStruct,
            hidden
        )]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> LineBreakIteratorUtf16<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[diplomat::rust_link(icu::segmenter::LineBreakIterator::next, FnInStruct)]
        #[diplomat::rust_link(
            icu::segmenter::LineBreakIterator::Item,
            AssociatedTypeInStruct,
            hidden
        )]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> LineBreakIteratorLatin1<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[diplomat::rust_link(icu::segmenter::LineBreakIterator::next, FnInStruct)]
        #[diplomat::rust_link(
            icu::segmenter::LineBreakIterator::Item,
            AssociatedTypeInStruct,
            hidden
        )]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }
}

impl From<ffi::LineBreakStrictness> for icu_segmenter::LineBreakStrictness {
    fn from(other: ffi::LineBreakStrictness) -> Self {
        match other {
            ffi::LineBreakStrictness::Loose => Self::Loose,
            ffi::LineBreakStrictness::Normal => Self::Normal,
            ffi::LineBreakStrictness::Strict => Self::Strict,
            ffi::LineBreakStrictness::Anywhere => Self::Anywhere,
        }
    }
}

impl From<ffi::LineBreakWordOption> for icu_segmenter::LineBreakWordOption {
    fn from(other: ffi::LineBreakWordOption) -> Self {
        match other {
            ffi::LineBreakWordOption::Normal => Self::Normal,
            ffi::LineBreakWordOption::BreakAll => Self::BreakAll,
            ffi::LineBreakWordOption::KeepAll => Self::KeepAll,
        }
    }
}

impl From<ffi::LineBreakOptionsV1> for icu_segmenter::LineBreakOptions {
    fn from(other: ffi::LineBreakOptionsV1) -> Self {
        let mut options = icu_segmenter::LineBreakOptions::default();
        options.strictness = other.strictness.into();
        options.word_option = other.word_option.into();
        options.ja_zh = other.ja_zh;
        options
    }
}
