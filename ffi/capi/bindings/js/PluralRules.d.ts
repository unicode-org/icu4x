import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";
import { PluralCategories } from "./PluralCategories";
import { PluralCategory } from "./PluralCategory";
import { PluralOperands } from "./PluralOperands";

/**

 * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html Rust documentation for `PluralRules`} for more information.
 */
export class PluralRules {

  /**

   * Construct an {@link PluralRules `PluralRules`} for the given locale, for cardinal numbers

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_cardinal Rust documentation for `try_new_cardinal`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_cardinal(provider: DataProvider, locale: Locale): PluralRules | never;

  /**

   * Construct an {@link PluralRules `PluralRules`} for the given locale, for ordinal numbers

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_ordinal Rust documentation for `try_new_ordinal`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_ordinal(provider: DataProvider, locale: Locale): PluralRules | never;

  /**

   * Get the category for a given number represented as operands

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.category_for Rust documentation for `category_for`} for more information.
   */
  category_for(op: PluralOperands): PluralCategory;

  /**

   * Get all of the categories needed in the current locale

   * See the {@link https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.categories Rust documentation for `categories`} for more information.
   */
  categories(): PluralCategories;
}
