import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html Rust documentation for `MetaZoneCalculator`} for more information.
 */
export class ICU4XMetaZoneCalculator {

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider): ICU4XMetaZoneCalculator | never;
}
