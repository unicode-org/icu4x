import { FFIError } from "./diplomat-runtime"
import { ICU4XDataError } from "./ICU4XDataError";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XLocaleFallbackConfig } from "./ICU4XLocaleFallbackConfig";
import { ICU4XLocaleFallbackerWithConfig } from "./ICU4XLocaleFallbackerWithConfig";
import { ICU4XLocaleParseError } from "./ICU4XLocaleParseError";

/**

 * An object that runs the ICU4X locale fallback algorithm.

 * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html Rust documentation for `LocaleFallbacker`} for more information.
 */
export class ICU4XLocaleFallbacker {

  /**

   * Creates a new `ICU4XLocaleFallbacker` from a data provider.

   * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link ICU4XDataError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XLocaleFallbacker | never;

  /**

   * Creates a new `ICU4XLocaleFallbacker` without data for limited functionality.

   * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html#method.new_without_data Rust documentation for `new_without_data`} for more information.
   */
  static create_without_data(): ICU4XLocaleFallbacker;

  /**

   * Associates this `ICU4XLocaleFallbacker` with configuration options.

   * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html#method.for_config Rust documentation for `for_config`} for more information.
   * @throws {@link FFIError}<{@link ICU4XLocaleParseError}>
   */
  for_config(config: ICU4XLocaleFallbackConfig): ICU4XLocaleFallbackerWithConfig | never;
}
