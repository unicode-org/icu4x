import { Locale } from "./Locale";
import { LocaleFallbackIterator } from "./LocaleFallbackIterator";

/**

 * An object that runs the ICU4X locale fallback algorithm with specific configurations.

 * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html Rust documentation for `LocaleFallbacker`} for more information.

 * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbackerWithConfig.html Rust documentation for `LocaleFallbackerWithConfig`} for more information.
 */
export class LocaleFallbackerWithConfig {

  /**

   * Creates an iterator from a locale with each step of fallback.

   * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html#method.fallback_for Rust documentation for `fallback_for`} for more information.
   */
  fallback_for_locale(locale: Locale): LocaleFallbackIterator;
}
