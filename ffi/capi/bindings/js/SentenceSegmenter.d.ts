import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { SentenceBreakIteratorLatin1 } from "./SentenceBreakIteratorLatin1";
import { SentenceBreakIteratorUtf16 } from "./SentenceBreakIteratorUtf16";
import { SentenceBreakIteratorUtf8 } from "./SentenceBreakIteratorUtf8";

/**

 * An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html Rust documentation for `SentenceSegmenter`} for more information.
 */
export class SentenceSegmenter {

  /**

   * Construct an {@link SentenceSegmenter `SentenceSegmenter`}.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): SentenceSegmenter | never;

  /**

   * Segments a string.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf8 Rust documentation for `segment_utf8`} for more information.
   */
  segment_utf8(input: string): SentenceBreakIteratorUtf8;

  /**

   * Segments a string.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf16 Rust documentation for `segment_utf16`} for more information.
   */
  segment_utf16(input: string): SentenceBreakIteratorUtf16;

  /**

   * Segments a Latin-1 string.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.segment_latin1 Rust documentation for `segment_latin1`} for more information.
   */
  segment_latin1(input: Uint8Array): SentenceBreakIteratorLatin1;
}
