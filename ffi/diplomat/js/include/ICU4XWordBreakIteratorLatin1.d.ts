import { i32 } from "./diplomat-runtime"

/**

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIteratorLatin1.html Rust documentation for `WordBreakIteratorLatin1`} for more information.
 */
export class ICU4XWordBreakIteratorLatin1 {

  /**

   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.
   */
  next(): i32;
}
