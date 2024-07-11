import { DisplayNamesFallback } from "./DisplayNamesFallback";
import { DisplayNamesStyle } from "./DisplayNamesStyle";
import { LanguageDisplay } from "./LanguageDisplay";

/**

 * See the {@link https://docs.rs/icu/latest/icu/displaynames/options/struct.DisplayNamesOptions.html Rust documentation for `DisplayNamesOptions`} for more information.
 */
export class DisplayNamesOptionsV1 {
  style: DisplayNamesStyle;
  fallback: DisplayNamesFallback;
  language_display: LanguageDisplay;
}
