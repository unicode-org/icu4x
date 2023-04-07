import { i32 } from "./diplomat-runtime"

/**

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIteratorUtf16.html Rust documentation for `WordBreakIteratorUtf16`} for more information.
 */
export class ICU4XWordBreakIteratorUtf16 {

  /**

   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIteratorUtf16.html#method.next Rust documentation for `next`} for more information.
   */
  next(): i32;
}
