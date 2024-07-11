import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { Error } from "./Error";
import { Locale } from "./Locale";
import { TransformResult } from "./TransformResult";

/**

 * A locale canonicalizer.

 * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleCanonicalizer.html Rust documentation for `LocaleCanonicalizer`} for more information.
 */
export class LocaleCanonicalizer {

  /**

   * Create a new {@link LocaleCanonicalizer `LocaleCanonicalizer`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleCanonicalizer.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create(provider: DataProvider): LocaleCanonicalizer | never;

  /**

   * Create a new {@link LocaleCanonicalizer `LocaleCanonicalizer`} with extended data.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleCanonicalizer.html#method.new_with_expander Rust documentation for `new_with_expander`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create_extended(provider: DataProvider): LocaleCanonicalizer | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleCanonicalizer.html#method.canonicalize Rust documentation for `canonicalize`} for more information.
   */
  canonicalize(locale: Locale): TransformResult;
}
