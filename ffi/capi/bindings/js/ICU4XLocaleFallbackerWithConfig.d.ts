import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XLocaleFallbackIterator } from "./ICU4XLocaleFallbackIterator";

/**

 * An object that runs the ICU4X locale fallback algorithm with specific configurations.

 * See the {@link https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html Rust documentation for `LocaleFallbacker`} for more information.

 * See the {@link https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackerWithConfig.html Rust documentation for `LocaleFallbackerWithConfig`} for more information.
 */
export class ICU4XLocaleFallbackerWithConfig {

  /**

   * Creates an iterator from a locale with each step of fallback.

   * See the {@link https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.fallback_for Rust documentation for `fallback_for`} for more information.
   */
  fallback_for_locale(locale: ICU4XLocale): ICU4XLocaleFallbackIterator;
}
