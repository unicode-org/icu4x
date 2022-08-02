import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLineBreakIteratorLatin1 } from "./ICU4XLineBreakIteratorLatin1";
import { ICU4XLineBreakIteratorUtf16 } from "./ICU4XLineBreakIteratorUtf16";
import { ICU4XLineBreakIteratorUtf8 } from "./ICU4XLineBreakIteratorUtf8";
import { ICU4XLineBreakOptions } from "./ICU4XLineBreakOptions";

/**

 * An ICU4X line-break segmenter, capable of finding breakpoints in strings.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html Rust documentation} for more information.
 */
export class ICU4XLineBreakSegmenter {

  /**

   * Construct a {@link ICU4XLineBreakSegmenter `ICU4XLineBreakSegmenter`} with default options.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider): ICU4XLineBreakSegmenter | never;

  /**

   * Construct a {@link ICU4XLineBreakSegmenter `ICU4XLineBreakSegmenter`} with custom options.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new_with_options Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_with_options(provider: ICU4XDataProvider, options: ICU4XLineBreakOptions): ICU4XLineBreakSegmenter | never;

  /**

   * Segments a UTF-8 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_str Rust documentation} for more information.
   */
  segment_utf8(input: string): ICU4XLineBreakIteratorUtf8;

  /**

   * Segments a UTF-16 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_utf16 Rust documentation} for more information.
   */
  segment_utf16(input: Uint16Array): ICU4XLineBreakIteratorUtf16;

  /**

   * Segments a Latin-1 string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_latin1 Rust documentation} for more information.
   */
  segment_latin1(input: Uint8Array): ICU4XLineBreakIteratorLatin1;
}
