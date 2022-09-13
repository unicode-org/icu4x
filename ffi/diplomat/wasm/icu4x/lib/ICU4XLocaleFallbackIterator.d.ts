import { ICU4XLocale } from "./ICU4XLocale";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html Rust documentation for `LocaleFallbackIterator`} for more information.
 */
export class ICU4XLocaleFallbackIterator {

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.get Rust documentation for `get`} for more information.
   */
  get(): ICU4XLocale;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.step Rust documentation for `step`} for more information.
   */
  step(): void;
}
