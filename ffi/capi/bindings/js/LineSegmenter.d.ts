// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { LineBreakIteratorUtf16 } from "./LineBreakIteratorUtf16"
import type { LineBreakOptions } from "./LineBreakOptions"
import type { LineBreakOptions_obj } from "./LineBreakOptions"
import type { Locale } from "./Locale"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



/**
 * An ICU4X line-break segmenter, capable of finding breakpoints in strings.
 *
 * See the [Rust documentation for `LineSegmenter`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html) for more information.
 */
export class LineSegmenter {
    /** @internal */
    get ffiValue(): pointer;
    /** @internal */
    constructor();


    /**
     * Construct a {@link LineSegmenter} with default options (no locale-based tailoring) using compiled data. It automatically loads the best
     * available payload data for Burmese, Khmer, Lao, and Thai.
     *
     * See the [Rust documentation for `new_auto`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
     */
    static createAuto(): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with default options (no locale-based tailoring) and LSTM payload data for
     * Burmese, Khmer, Lao, and Thai, using compiled data.
     *
     * See the [Rust documentation for `new_lstm`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
     */
    static createLstm(): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with default options (no locale-based tailoring) and dictionary payload data for
     * Burmese, Khmer, Lao, and Thai, using compiled data
     *
     * See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
     */
    static createDictionary(): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with custom options using compiled data. It automatically loads the best
     * available payload data for Burmese, Khmer, Lao, and Thai.
     *
     * See the [Rust documentation for `new_auto`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
     */
    static autoWithOptions(contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with custom options. It automatically loads the best
     * available payload data for Burmese, Khmer, Lao, and Thai, using a particular data source.
     *
     * See the [Rust documentation for `new_auto`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
     */
    static autoWithOptionsAndProvider(provider: DataProvider, contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with custom options and LSTM payload data for
     * Burmese, Khmer, Lao, and Thai, using compiled data.
     *
     * See the [Rust documentation for `new_lstm`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
     */
    static lstmWithOptions(contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with custom options and LSTM payload data for
     * Burmese, Khmer, Lao, and Thai, using a particular data source.
     *
     * See the [Rust documentation for `new_lstm`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
     */
    static lstmWithOptionsAndProvider(provider: DataProvider, contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with custom options and dictionary payload data for
     * Burmese, Khmer, Lao, and Thai, using compiled data.
     *
     * See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
     */
    static dictionaryWithOptions(contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    /**
     * Construct a {@link LineSegmenter} with custom options and dictionary payload data for
     * Burmese, Khmer, Lao, and Thai, using a particular data source.
     *
     * See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
     */
    static dictionaryWithOptionsAndProvider(provider: DataProvider, contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    /**
     * Segments a string.
     *
     * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
     * to the WHATWG Encoding Standard.
     *
     * See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/2.0.0/icu/segmenter/struct.LineSegmenterBorrowed.html#method.segment_utf16) for more information.
     */
    segment(input: string): LineBreakIteratorUtf16;
}