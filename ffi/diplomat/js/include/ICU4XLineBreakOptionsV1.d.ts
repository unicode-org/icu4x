import { ICU4XLineBreakStrictness } from "./ICU4XLineBreakStrictness";
import { ICU4XWordBreakRule } from "./ICU4XWordBreakRule";

/**

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakOptions.html Rust documentation for `LineBreakOptions`} for more information.
 */
export class ICU4XLineBreakOptionsV1 {
  line_break_strictness: ICU4XLineBreakStrictness;
  word_break_rule: ICU4XWordBreakRule;
  ja_zh: boolean;
}
