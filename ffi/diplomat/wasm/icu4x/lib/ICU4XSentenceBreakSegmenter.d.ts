import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XSentenceBreakIteratorLatin1 } from "./ICU4XSentenceBreakIteratorLatin1";
import { ICU4XSentenceBreakIteratorUtf16 } from "./ICU4XSentenceBreakIteratorUtf16";
import { ICU4XSentenceBreakIteratorUtf8 } from "./ICU4XSentenceBreakIteratorUtf8";

/**

 * An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html Rust documentation} for more information.
 */
export class ICU4XSentenceBreakSegmenter {

  /**

   * Construct an {@link ICU4XSentenceBreakSegmenter `ICU4XSentenceBreakSegmenter`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider): ICU4XSentenceBreakSegmenter | never;

  /**

   * Segments a UTF-8 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_str Rust documentation} for more information.
   */
  segment_utf8(input: string): ICU4XSentenceBreakIteratorUtf8;

  /**

   * Segments a UTF-16 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_utf16 Rust documentation} for more information.
   */
  segment_utf16(input: Uint16Array): ICU4XSentenceBreakIteratorUtf16;

  /**

   * Segments a Latin-1 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_latin1 Rust documentation} for more information.
   */
  segment_latin1(input: Uint8Array): ICU4XSentenceBreakIteratorLatin1;
}
