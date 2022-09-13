import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XLocaleFallbackIterator } from "./ICU4XLocaleFallbackIterator";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html Rust documentation for `LocaleFallbackerWithConfig`} for more information.
 */
export class ICU4XLocaleFallbackerWithConfig {

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html#method.fallback_for Rust documentation for `fallback_for`} for more information.
   */
  fallback_for_locale(locale: ICU4XLocale): ICU4XLocaleFallbackIterator;
}
