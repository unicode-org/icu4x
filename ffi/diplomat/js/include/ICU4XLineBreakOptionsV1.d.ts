import { ICU4XLineBreakRule } from "./ICU4XLineBreakRule";
import { ICU4XWordBreakRule } from "./ICU4XWordBreakRule";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineBreakOptions.html Rust documentation for `LineBreakOptions`} for more information.
 */
export class ICU4XLineBreakOptionsV1 {
  line_break_rule: ICU4XLineBreakRule;
  word_break_rule: ICU4XWordBreakRule;
  ja_zh: boolean;
}
