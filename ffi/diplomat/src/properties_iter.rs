#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use core::ops::RangeInclusive;

    /// Result of a single iteration of [`CodePointRangeIterator`].
    /// Logically can be considered to be an `Option<RangeInclusive<u32>>`,
    ///
    /// `start` and `end` represent an inclusive range of code points [start, end],
    /// and `done` will be true if the iterator has already finished. The last contentful
    /// iteration will NOT produce a range done=true, in other words `start` and `end` are useful
    /// values if and only if `done=false`.
    pub struct CodePointRangeIteratorResult {
        pub start: u32,
        pub end: u32,
        pub done: bool,
    }

    /// An iterator over code point ranges, produced by `ICU4XCodePointSetData` or
    /// one of the `ICU4XCodePointMapData` types
    #[diplomat::opaque]
    pub struct CodePointRangeIterator<'a>(pub Box<dyn Iterator<Item = RangeInclusive<u32>> + 'a>);

    impl<'a> CodePointRangeIterator<'a> {
        pub fn next(&mut self) -> CodePointRangeIteratorResult {
            self.0
                .next()
                .map(|r| CodePointRangeIteratorResult {
                    start: *r.start(),
                    end: *r.end(),
                    done: false,
                })
                .unwrap_or(CodePointRangeIteratorResult {
                    start: 0,
                    end: 0,
                    done: true,
                })
        }
    }
}
