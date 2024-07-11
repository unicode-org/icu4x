import { FFIError } from "./diplomat-runtime"
import { DataProvider } from "./DataProvider";
import { Error } from "./Error";
import { Locale } from "./Locale";
import { TransformResult } from "./TransformResult";

/**

 * A locale expander.

 * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleExpander.html Rust documentation for `LocaleExpander`} for more information.
 */
export class LocaleExpander {

  /**

   * Create a new {@link LocaleExpander `LocaleExpander`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleExpander.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create(provider: DataProvider): LocaleExpander | never;

  /**

   * Create a new {@link LocaleExpander `LocaleExpander`} with extended data.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleExpander.html#method.new_extended Rust documentation for `new_extended`} for more information.
   * @throws {@link FFIError}<{@link Error}>
   */
  static create_extended(provider: DataProvider): LocaleExpander | never;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleExpander.html#method.maximize Rust documentation for `maximize`} for more information.
   */
  maximize(locale: Locale): TransformResult;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleExpander.html#method.minimize Rust documentation for `minimize`} for more information.
   */
  minimize(locale: Locale): TransformResult;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.LocaleExpander.html#method.minimize_favor_script Rust documentation for `minimize_favor_script`} for more information.
   */
  minimize_favor_script(locale: Locale): TransformResult;
}
