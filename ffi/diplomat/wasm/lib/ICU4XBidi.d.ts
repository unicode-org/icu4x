import { u8 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XBidiInfo } from "./ICU4XBidiInfo";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X Bidi object, containing loaded bidi data

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html Rust documentation} for more information.
 */
export class ICU4XBidi {

  /**

   * Creates a new {@link ICU4XBidi `ICU4XBidi`} from locale data.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html#method.new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider): ICU4XBidi | never;

  /**

   * Use the data loaded in this object to process a string and calculate bidi information

   * Takes in a Level for the default level, if it is an invalid value it will default to LTR

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.BidiInfo.html#method.new_with_data_source Rust documentation} for more information.
   */
  for_text(text: string, default_level: u8): ICU4XBidiInfo;

  /**

   * Check if a Level returned by level_at is an RTL level.

   * Invalid levels (numbers greater than 125) will be assumed LTR

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_rtl Rust documentation} for more information.
   */
  static level_is_rtl(level: u8): boolean;

  /**

   * Check if a Level returned by level_at is an LTR level.

   * Invalid levels (numbers greater than 125) will be assumed LTR

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_ltr Rust documentation} for more information.
   */
  static level_is_ltr(level: u8): boolean;

  /**

   * Get a basic RTL Level value

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.rtl Rust documentation} for more information.
   */
  static level_rtl(): u8;

  /**

   * Get a simple LTR Level value

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.ltr Rust documentation} for more information.
   */
  static level_ltr(): u8;
}
