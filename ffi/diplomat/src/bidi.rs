// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::{DiplomatResult, DiplomatWriteable};

    use icu_properties::bidi::BidiClassAdapter;
    use icu_properties::maps;
    use icu_properties::provider::BidiClassV1Marker;
    use icu_provider::DataPayload;
    use unicode_bidi::BidiInfo;
    use unicode_bidi::Paragraph;
    use unicode_bidi::Level;
    use core::fmt::Write;

    use crate::provider::ffi::ICU4XDataProvider;

    pub enum ICU4XBidiDirection {
        Ltr,
        Rtl,
        Mixed,
    }

    #[diplomat::opaque]
    /// An ICU4X Bidi object, containing loaded bidi data
    #[diplomat::rust_link(icu::properties::bidi::BidiClassAdapter, Struct)]
    // #[diplomat::rust_link(icu::properties::maps::get_bidi_class, Struct)]
    pub struct ICU4XBidi(pub DataPayload<BidiClassV1Marker>);

    impl ICU4XBidi {
        /// Creates a new [`ICU4XBidi`] from locale data.
        #[diplomat::rust_link(icu::properties::bidi::BidiClassAdapter::new, FnInStruct)]
        pub fn try_new(provider: &ICU4XDataProvider) -> DiplomatResult<Box<ICU4XBidi>, ()> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            if let Result::Ok(bidi) = maps::get_bidi_class(&provider) {
                Ok(Box::new(ICU4XBidi(bidi))).into()
            } else {
                Err(()).into()
            }
        }


        /// Use the data loaded in this object to process a string and calculate bidi information
        #[diplomat::rust_link(unicode_bidi::BidiInfo::new_with_data_source, FnInStruct)]
        pub fn for_text<'text>(&self, text: &'text str) -> Box<ICU4XBidiInfo<'text>> {
            let data = self.0.get();
            let adapter = BidiClassAdapter::new(&data.code_point_trie);

            Box::new(ICU4XBidiInfo(BidiInfo::new_with_data_source(
                &adapter, text, None,
            )))
        }
    }

    /// An object containing bidi information for a given string, produced by `for_text()` on `ICU4XBidi`
    #[diplomat::rust_link(unicode_bidi::BidiInfo, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XBidiInfo<'text>(pub BidiInfo<'text>);

    impl<'text> ICU4XBidiInfo<'text> {
        //. The number of paragraphs contained here
        pub fn paragraph_count(&self) -> usize {
            self.0.paragraphs.len()
        }

        /// Get the nth paragraph, returning None if out of bounds
        pub fn paragraph_at(&'text self, n: usize) -> Option<Box<ICU4XBidiParagraph<'text>>> {
            self.0
                .paragraphs
                .get(n)
                .map(|p| Box::new(ICU4XBidiParagraph(Paragraph::new(&self.0, p))))
        }
    }

    /// Bidi information for a single processed paragraph
    #[diplomat::opaque]
    pub struct ICU4XBidiParagraph<'info>(pub Paragraph<'info, 'info>);

    impl<'info> ICU4XBidiParagraph<'info> {
        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        /// The primary direction of this paragraph
        pub fn direction(&self) -> ICU4XBidiDirection {
            self.0.direction().into()
        }

        /// The number of bytes in this paragraph
        #[diplomat::rust_link(unicode_bidi::ParagraphInfo::len, FnInStruct)]
        pub fn size(&self) -> usize {
            self.0.para.len()
        }

        /// The start index of this paragraph within the source text
        pub fn range_start(&self) -> usize {
            self.0.para.range.start
        }

        /// The end index of this paragraph within the source text
        pub fn range_end(&self) -> usize {
            self.0.para.range.end
        }

        /// Reorder a line based on display order. The ranges are specified relative to the source text and must be contained
        /// within this paragraph's range.
        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        pub fn reorder_line(&self, range_start: usize, range_end: usize, out: &mut DiplomatWriteable) -> DiplomatResult<(), ()> {
            if range_start < self.range_start() || range_end > self.range_end() {
                return Err(()).into()
            }

            let info = self.0.info;
            let para = self.0.para;

            let reordered = info.reorder_line(para, range_start..range_end);

            out.write_str(&reordered).map_err(|_| ()).into()
        }

        /// Get the BIDI level. This integer is conceptually a `unicode_bidi::Level`,
        /// and can be further inspected using the static methods on this class.
        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        pub fn level_at(&self, pos: usize) -> u8 {
            if pos >= self.size() {
                return 0;
            }

            self.0.level_at(pos).number()
        }

        /// Check if a Level returned by level_at is an RTL level.
        ///
        /// Invalid levels (numbers greater than 125) will be assumed LTR
        #[diplomat::rust_link(unicode_bidi::Level::is_rtl, FnInStruct)]
        pub fn level_is_rtl(level: u8) -> bool {
            Level::new(level).unwrap_or(Level::ltr()).is_rtl()
        }

        /// Check if a Level returned by level_at is an LTR level.
        ///
        /// Invalid levels (numbers greater than 125) will be assumed LTR
        #[diplomat::rust_link(unicode_bidi::Level::is_ltr, FnInStruct)]
        pub fn level_is_ltr(level: u8) -> bool {
            Level::new(level).unwrap_or(Level::ltr()).is_ltr()
        }

    }

}

use unicode_bidi::Direction;

impl From<Direction> for ffi::ICU4XBidiDirection {
    fn from(other: Direction) -> Self {
        match other {
            Direction::Ltr => Self::Ltr,
            Direction::Rtl => Self::Rtl,
            Direction::Mixed => Self::Mixed,
        }
    }
}
