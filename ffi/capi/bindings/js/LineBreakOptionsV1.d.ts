import { LineBreakStrictness } from "./LineBreakStrictness";
import { LineBreakWordOption } from "./LineBreakWordOption";

/**

 * See the {@link https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakOptions.html Rust documentation for `LineBreakOptions`} for more information.
 */
export class LineBreakOptionsV1 {
  strictness: LineBreakStrictness;
  word_option: LineBreakWordOption;
  ja_zh: boolean;
}
