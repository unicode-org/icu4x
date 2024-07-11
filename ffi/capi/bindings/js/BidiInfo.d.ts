import { u8, usize } from "./diplomat-runtime"
import { BidiParagraph } from "./BidiParagraph";

/**

 * An object containing bidi information for a given string, produced by `for_text()` on `Bidi`

 * See the {@link https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.BidiInfo.html Rust documentation for `BidiInfo`} for more information.
 */
export class BidiInfo {

  /**

   * The number of paragraphs contained here
   */
  paragraph_count(): usize;

  /**

   * Get the nth paragraph, returning `None` if out of bounds
   */
  paragraph_at(n: usize): BidiParagraph | undefined;

  /**

   * The number of bytes in this full text
   */
  size(): usize;

  /**

   * Get the BIDI level at a particular byte index in the full text. This integer is conceptually a `unicode_bidi::Level`, and can be further inspected using the static methods on Bidi.

   * Returns 0 (equivalent to `Level::ltr()`) on error
   */
  level_at(pos: usize): u8;
}
