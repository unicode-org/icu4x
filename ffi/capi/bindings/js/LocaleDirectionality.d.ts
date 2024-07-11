import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";
import { LocaleDirection } from "./LocaleDirection";
import { LocaleExpander } from "./LocaleExpander";

/**

 * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html Rust documentation for `LocaleDirectionality`} for more information.
 */
export class LocaleDirectionality {

  /**

   * Construct a new LocaleDirectionality instance

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): LocaleDirectionality | never;

  /**

   * Construct a new LocaleDirectionality instance with a custom expander

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html#method.new_with_expander Rust documentation for `new_with_expander`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_with_expander(provider: DataProvider, expander: LocaleExpander): LocaleDirectionality | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html#method.get Rust documentation for `get`} for more information.
   */
  get(locale: Locale): LocaleDirection;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html#method.is_left_to_right Rust documentation for `is_left_to_right`} for more information.
   */
  is_left_to_right(locale: Locale): boolean;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html#method.is_right_to_left Rust documentation for `is_right_to_left`} for more information.
   */
  is_right_to_left(locale: Locale): boolean;
}
