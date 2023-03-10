import { ICU4XFallback } from "./ICU4XFallback";
import { ICU4XLanguageDisplay } from "./ICU4XLanguageDisplay";
import { ICU4XStyle } from "./ICU4XStyle";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/struct.DisplayNamesOptions.html Rust documentation for `DisplayNamesOptions`} for more information.
 */
export class ICU4XDisplayNamesOptions {
  style: ICU4XStyle;
  fallback: ICU4XFallback;
  language_display: ICU4XLanguageDisplay;
}
