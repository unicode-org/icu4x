import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { GraphemeClusterBreakIteratorLatin1 } from "./GraphemeClusterBreakIteratorLatin1";
import { GraphemeClusterBreakIteratorUtf16 } from "./GraphemeClusterBreakIteratorUtf16";
import { GraphemeClusterBreakIteratorUtf8 } from "./GraphemeClusterBreakIteratorUtf8";

/**

 * An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints in strings.

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html Rust documentation for `GraphemeClusterSegmenter`} for more information.
 */
export class GraphemeClusterSegmenter {

  /**

   * Construct an {@link GraphemeClusterSegmenter `GraphemeClusterSegmenter`}.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): GraphemeClusterSegmenter | never;

  /**

   * Segments a string.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.segment_utf8 Rust documentation for `segment_utf8`} for more information.
   */
  segment_utf8(input: string): GraphemeClusterBreakIteratorUtf8;

  /**

   * Segments a string.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.segment_utf16 Rust documentation for `segment_utf16`} for more information.
   */
  segment_utf16(input: string): GraphemeClusterBreakIteratorUtf16;

  /**

   * Segments a Latin-1 string.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.segment_latin1 Rust documentation for `segment_latin1`} for more information.
   */
  segment_latin1(input: Uint8Array): GraphemeClusterBreakIteratorLatin1;
}
