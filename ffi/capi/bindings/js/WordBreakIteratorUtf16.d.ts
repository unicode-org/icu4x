// generated by diplomat-tool
import type { SegmenterWordType } from "./SegmenterWordType"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



/**
 * See the [Rust documentation for `WordBreakIterator`](https://docs.rs/icu/2.0.0/icu/segmenter/iterators/struct.WordBreakIterator.html) for more information.
 */
export class WordBreakIteratorUtf16 {
    /** @internal */
    get ffiValue(): pointer;
    /** @internal */
    constructor();


    /**
     * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
     * out of range of a 32-bit signed integer.
     *
     * See the [Rust documentation for `next`](https://docs.rs/icu/2.0.0/icu/segmenter/iterators/struct.WordBreakIterator.html#method.next) for more information.
     */
    next(): number;

    /**
     * Return the status value of break boundary.
     *
     * See the [Rust documentation for `word_type`](https://docs.rs/icu/2.0.0/icu/segmenter/iterators/struct.WordBreakIterator.html#method.word_type) for more information.
     */
    get wordType(): SegmenterWordType;

    /**
     * Return true when break boundary is word-like such as letter/number/CJK
     *
     * See the [Rust documentation for `is_word_like`](https://docs.rs/icu/2.0.0/icu/segmenter/iterators/struct.WordBreakIterator.html#method.is_word_like) for more information.
     */
    get isWordLike(): boolean;
}