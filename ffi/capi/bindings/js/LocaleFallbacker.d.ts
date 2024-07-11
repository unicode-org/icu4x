import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { LocaleFallbackConfig } from "./LocaleFallbackConfig";
import { LocaleFallbackerWithConfig } from "./LocaleFallbackerWithConfig";
import { LocaleParseError } from "./LocaleParseError";

/**

 * An object that runs the ICU4X locale fallback algorithm.

 * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html Rust documentation for `LocaleFallbacker`} for more information.
 */
export class LocaleFallbacker {

  /**

   * Creates a new `LocaleFallbacker` from a data provider.

   * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): LocaleFallbacker | never;

  /**

   * Creates a new `LocaleFallbacker` without data for limited functionality.

   * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html#method.new_without_data Rust documentation for `new_without_data`} for more information.
   */
  static create_without_data(): LocaleFallbacker;

  /**

   * Associates this `LocaleFallbacker` with configuration options.

   * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbacker.html#method.for_config Rust documentation for `for_config`} for more information.
   * @throws {@link FFIError}<{@link LocaleParseError}>
   */
  for_config(config: LocaleFallbackConfig): LocaleFallbackerWithConfig | never;
}
