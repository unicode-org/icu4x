import { i32 } from "./diplomat-runtime"
import { ICU4XSegmenterRuleStatusType } from "./ICU4XSegmenterRuleStatusType";

/**

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIteratorPotentiallyIllFormedUtf8.html Rust documentation for `WordBreakIteratorPotentiallyIllFormedUtf8`} for more information.
 */
export class ICU4XWordBreakIteratorUtf8 {

  /**

   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIteratorPotentiallyIllFormedUtf8.html#method.next Rust documentation for `next`} for more information.
   */
  next(): i32;

  /**

   * Return the status value of break boundary.

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIteratorPotentiallyIllFormedUtf8.html#method.rule_status Rust documentation for `rule_status`} for more information.
   */
  rule_status(): ICU4XSegmenterRuleStatusType;

  /**

   * Return true when break boundary is word-like such as letter/number/CJK

   * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIteratorPotentiallyIllFormedUtf8.html#method.is_word_like Rust documentation for `is_word_like`} for more information.
   */
  is_word_like(): boolean;
}
