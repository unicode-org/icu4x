import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XPluralCategories } from "./ICU4XPluralCategories";
import { ICU4XPluralCategory } from "./ICU4XPluralCategory";
import { ICU4XPluralOperands } from "./ICU4XPluralOperands";

/**

 * FFI version of `PluralRulesWithRanges`.

 * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html Rust documentation for `PluralRulesWithRanges`} for more information.
 */
export class ICU4XPluralRulesWithRanges {

  /**

   * Construct an {@link ICU4XPluralRulesWithRanges `ICU4XPluralRulesWithRanges`} for the given locale, for cardinal numbers

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_cardinal Rust documentation for `try_new_cardinal`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_cardinal(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XPluralRulesWithRanges | never;

  /**

   * Construct an {@link ICU4XPluralRulesWithRanges `ICU4XPluralRulesWithRanges`} for the given locale, for ordinal numbers

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_ordinal Rust documentation for `try_new_ordinal`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_ordinal(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XPluralRulesWithRanges | never;

  /**

   * Get the category for a given number represented as operands

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.category_for Rust documentation for `category_for`} for more information.
   */
  category_for(op: ICU4XPluralOperands): ICU4XPluralCategory;

  /**

   * Get all of the categories needed in the current locale

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.categories Rust documentation for `categories`} for more information.
   */
  categories(): ICU4XPluralCategories;

  /**

   * Get the appropriate category for a numeric range.

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.category_for_range Rust documentation for `category_for_range`} for more information.
   */
  category_for_range(start: ICU4XPluralOperands, end: ICU4XPluralOperands): ICU4XPluralCategory;

  /**

   * Get the appropriate category for a numeric range from the categories of its endpoints.

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.resolve_range Rust documentation for `resolve_range`} for more information.
   */
  resolve_range(start: ICU4XPluralCategory, end: ICU4XPluralCategory): ICU4XPluralCategory;
}
