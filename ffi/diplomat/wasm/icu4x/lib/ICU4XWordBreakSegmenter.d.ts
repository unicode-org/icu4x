import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XWordBreakIteratorLatin1 } from "./ICU4XWordBreakIteratorLatin1";
import { ICU4XWordBreakIteratorUtf16 } from "./ICU4XWordBreakIteratorUtf16";
import { ICU4XWordBreakIteratorUtf8 } from "./ICU4XWordBreakIteratorUtf8";

/**

 * An ICU4X word-break segmenter, capable of finding word breakpoints in strings.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html Rust documentation} for more information.
 */
export class ICU4XWordBreakSegmenter {

  /**

   * Construct an {@link ICU4XWordBreakSegmenter `ICU4XWordBreakSegmenter`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider): ICU4XWordBreakSegmenter | never;

  /**

   * Segments a UTF-8 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.segment_str Rust documentation} for more information.
   */
  segment_utf8(input: string): ICU4XWordBreakIteratorUtf8;

  /**

   * Segments a UTF-16 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.segment_utf16 Rust documentation} for more information.
   */
  segment_utf16(input: Uint16Array): ICU4XWordBreakIteratorUtf16;

  /**

   * Segments a Latin-1 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.segment_latin1 Rust documentation} for more information.
   */
  segment_latin1(input: Uint8Array): ICU4XWordBreakIteratorLatin1;
}
