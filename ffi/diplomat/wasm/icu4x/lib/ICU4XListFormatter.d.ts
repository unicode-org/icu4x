import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XList } from "./ICU4XList";
import { ICU4XListStyle } from "./ICU4XListStyle";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/list/struct.ListFormatter.html Rust documentation for `ListFormatter`} for more information.
 */
export class ICU4XListFormatter {

  /**

   * Construct a new ICU4XListFormatter instance for And patterns

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.try_new_and_unstable Rust documentation for `try_new_and_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_and(provider: ICU4XDataProvider, locale: ICU4XLocale, style: ICU4XListStyle): ICU4XListFormatter | never;

  /**

   * Construct a new ICU4XListFormatter instance for And patterns

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.try_new_or_unstable Rust documentation for `try_new_or_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_or(provider: ICU4XDataProvider, locale: ICU4XLocale, style: ICU4XListStyle): ICU4XListFormatter | never;

  /**

   * Construct a new ICU4XListFormatter instance for And patterns

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.try_new_unit_unstable Rust documentation for `try_new_unit_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_unit(provider: ICU4XDataProvider, locale: ICU4XLocale, style: ICU4XListStyle): ICU4XListFormatter | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.format Rust documentation for `format`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  format(list: ICU4XList): string | never;
}
