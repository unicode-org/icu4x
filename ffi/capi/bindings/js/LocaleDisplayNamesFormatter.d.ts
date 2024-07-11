import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { DisplayNamesOptionsV1 } from "./DisplayNamesOptionsV1";
import { Locale } from "./Locale";

/**

 * See the {@link https://docs.rs/icu/latest/icu/displaynames/struct.LocaleDisplayNamesFormatter.html Rust documentation for `LocaleDisplayNamesFormatter`} for more information.
 */
export class LocaleDisplayNamesFormatter {

  /**

   * Creates a new `LocaleDisplayNamesFormatter` from locale data and an options bag.

   * See the {@link https://docs.rs/icu/latest/icu/displaynames/struct.LocaleDisplayNamesFormatter.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider, locale: Locale, options: DisplayNamesOptionsV1): LocaleDisplayNamesFormatter | never;

  /**

   * Returns the locale-specific display name of a locale.

   * See the {@link https://docs.rs/icu/latest/icu/displaynames/struct.LocaleDisplayNamesFormatter.html#method.of Rust documentation for `of`} for more information.
   */
  of(locale: Locale): string;
}
