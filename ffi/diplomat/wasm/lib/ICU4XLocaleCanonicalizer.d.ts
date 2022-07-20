import { FFIError } from "./diplomat-runtime"
import { ICU4XCanonicalizationResult } from "./ICU4XCanonicalizationResult";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * A locale canonicalizer.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html Rust documentation} for more information.
 */
export class ICU4XLocaleCanonicalizer {

  /**

   * Create a new {@link ICU4XLocaleCanonicalizer `ICU4XLocaleCanonicalizer`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.new Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XLocaleCanonicalizer | never;

  /**

   * FFI version of `LocaleCanonicalizer::canonicalize()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.canonicalize Rust documentation} for more information.
   */
  canonicalize(locale: ICU4XLocale): ICU4XCanonicalizationResult;

  /**

   * FFI version of `LocaleCanonicalizer::maximize()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.maximize Rust documentation} for more information.
   */
  maximize(locale: ICU4XLocale): ICU4XCanonicalizationResult;

  /**

   * FFI version of `LocaleCanonicalizer::minimize()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.minimize Rust documentation} for more information.
   */
  minimize(locale: ICU4XLocale): ICU4XCanonicalizationResult;
}
