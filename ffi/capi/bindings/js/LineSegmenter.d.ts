import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { LineBreakIteratorLatin1 } from "./LineBreakIteratorLatin1";
import { LineBreakIteratorUtf16 } from "./LineBreakIteratorUtf16";
import { LineBreakIteratorUtf8 } from "./LineBreakIteratorUtf8";
import { LineBreakOptionsV1 } from "./LineBreakOptionsV1";

/**

 * An ICU4X line-break segmenter, capable of finding breakpoints in strings.

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html Rust documentation for `LineSegmenter`} for more information.
 */
export class LineSegmenter {

  /**

   * Construct a {@link LineSegmenter `LineSegmenter`} with default options. It automatically loads the best available payload data for Burmese, Khmer, Lao, and Thai.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_auto Rust documentation for `new_auto`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_auto(provider: DataProvider): LineSegmenter | never;

  /**

   * Construct a {@link LineSegmenter `LineSegmenter`} with default options and LSTM payload data for Burmese, Khmer, Lao, and Thai.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_lstm Rust documentation for `new_lstm`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_lstm(provider: DataProvider): LineSegmenter | never;

  /**

   * Construct a {@link LineSegmenter `LineSegmenter`} with default options and dictionary payload data for Burmese, Khmer, Lao, and Thai..

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary Rust documentation for `new_dictionary`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_dictionary(provider: DataProvider): LineSegmenter | never;

  /**

   * Construct a {@link LineSegmenter `LineSegmenter`} with custom options. It automatically loads the best available payload data for Burmese, Khmer, Lao, and Thai.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_auto_with_options Rust documentation for `new_auto_with_options`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_auto_with_options_v1(provider: DataProvider, options: LineBreakOptionsV1): LineSegmenter | never;

  /**

   * Construct a {@link LineSegmenter `LineSegmenter`} with custom options and LSTM payload data for Burmese, Khmer, Lao, and Thai.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_lstm_with_options Rust documentation for `new_lstm_with_options`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_lstm_with_options_v1(provider: DataProvider, options: LineBreakOptionsV1): LineSegmenter | never;

  /**

   * Construct a {@link LineSegmenter `LineSegmenter`} with custom options and dictionary payload data for Burmese, Khmer, Lao, and Thai.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary_with_options Rust documentation for `new_dictionary_with_options`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_dictionary_with_options_v1(provider: DataProvider, options: LineBreakOptionsV1): LineSegmenter | never;

  /**

   * Segments a string.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.segment_utf8 Rust documentation for `segment_utf8`} for more information.
   */
  segment_utf8(input: string): LineBreakIteratorUtf8;

  /**

   * Segments a string.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.segment_utf16 Rust documentation for `segment_utf16`} for more information.
   */
  segment_utf16(input: string): LineBreakIteratorUtf16;

  /**

   * Segments a Latin-1 string.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.segment_latin1 Rust documentation for `segment_latin1`} for more information.
   */
  segment_latin1(input: Uint8Array): LineBreakIteratorLatin1;
}
