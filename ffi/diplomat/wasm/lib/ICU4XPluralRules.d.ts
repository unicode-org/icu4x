import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XPluralCategories } from "./ICU4XPluralCategories";
import { ICU4XPluralCategory } from "./ICU4XPluralCategory";
import { ICU4XPluralOperands } from "./ICU4XPluralOperands";

/**

 * FFI version of `PluralRules`.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html Rust documentation} for more information.
 */
export class ICU4XPluralRules {

  /**

   * FFI version of `PluralRules::try_new_cardinal()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_cardinal(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XPluralRules | never;

  /**

   * FFI version of `PluralRules::try_new_ordinal()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_ordinal(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XPluralRules | never;

  /**

   * FFI version of `PluralRules::select()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select Rust documentation} for more information.
   */
  select(op: ICU4XPluralOperands): ICU4XPluralCategory;

  /**

   * FFI version of `PluralRules::categories()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories Rust documentation} for more information.
   */
  categories(): ICU4XPluralCategories;
}
