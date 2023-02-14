import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html Rust documentation for `MetazoneCalculator`} for more information.
 */
export class ICU4XMetazoneCalculator {

  /**

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XMetazoneCalculator | never;
}
