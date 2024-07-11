import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";
import { LocaleParseError } from "./LocaleParseError";

/**

 * See the {@link https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html Rust documentation for `RegionDisplayNames`} for more information.
 */
export class RegionDisplayNames {

  /**

   * Creates a new `RegionDisplayNames` from locale data and an options bag.

   * See the {@link https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider, locale: Locale): RegionDisplayNames | never;

  /**

   * Returns the locale specific display name of a region. Note that the function returns an empty string in case the display name for a given region code is not found.

   * See the {@link https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.of Rust documentation for `of`} for more information.
   * @throws {@link FFIError}<{@link LocaleParseError}>
   */
  of(region: string): string | never;
}
