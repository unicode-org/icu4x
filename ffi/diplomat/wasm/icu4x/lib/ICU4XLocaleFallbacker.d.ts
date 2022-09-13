import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocaleFallbackConfig } from "./ICU4XLocaleFallbackConfig";
import { ICU4XLocaleFallbackerWithConfig } from "./ICU4XLocaleFallbackerWithConfig";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html Rust documentation for `LocaleFallbacker`} for more information.
 */
export class ICU4XLocaleFallbacker {

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XLocaleFallbacker | never;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.new_without_data Rust documentation for `new_without_data`} for more information.
   */
  static create_without_data(): ICU4XLocaleFallbacker;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.for_config Rust documentation for `for_config`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  for_config(config: ICU4XLocaleFallbackConfig): ICU4XLocaleFallbackerWithConfig | never;
}
