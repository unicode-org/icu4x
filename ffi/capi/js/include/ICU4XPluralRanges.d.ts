import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XPluralCategory } from "./ICU4XPluralCategory";

/**

 * FFI version of `PluralRanges`.

 * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRanges.html Rust documentation for `PluralRanges`} for more information.
 */
export class ICU4XPluralRanges {

  /**

   * Construct an {@link ICU4XPluralRanges `ICU4XPluralRanges`} for the given locale.

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRanges.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XPluralRanges | never;

  /**

   * Get the appropriate category for a numeric range from the categories of its endpoints.

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRanges.html#method.category_for_range Rust documentation for `category_for_range`} for more information.
   */
  category_for_range(start: ICU4XPluralCategory, end: ICU4XPluralCategory): ICU4XPluralCategory;
}
