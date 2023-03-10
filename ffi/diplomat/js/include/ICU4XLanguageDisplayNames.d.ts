import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XDisplayNamesOptions } from "./ICU4XDisplayNamesOptions";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html Rust documentation for `LanguageDisplayNames`} for more information.
 */
export class ICU4XLanguageDisplayNames {

  /**

   * Creates a new `LanguageDisplayNames` from locale data and an options bag.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_unstable(provider: ICU4XDataProvider, locale: ICU4XLocale, options: ICU4XDisplayNamesOptions): ICU4XLanguageDisplayNames | never;

  /**
   * @throws {@link FFIError}<void>
   */
  of(code: string): string | never;
}
