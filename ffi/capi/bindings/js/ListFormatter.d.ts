import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { List } from "./List";
import { ListLength } from "./ListLength";
import { Locale } from "./Locale";

/**

 * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html Rust documentation for `ListFormatter`} for more information.
 */
export class ListFormatter {

  /**

   * Construct a new ListFormatter instance for And patterns

   * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_and_with_length Rust documentation for `try_new_and_with_length`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_and_with_length(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter | never;

  /**

   * Construct a new ListFormatter instance for And patterns

   * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_or_with_length Rust documentation for `try_new_or_with_length`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_or_with_length(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter | never;

  /**

   * Construct a new ListFormatter instance for And patterns

   * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_unit_with_length Rust documentation for `try_new_unit_with_length`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_unit_with_length(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format(list: List): string;
}
