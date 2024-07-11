import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";

/**

 * An object capable of computing the metazone from a timezone.

 * This can be used via `maybe_calculate_metazone()` on {@link crate::timezone::ffi::CustomTimeZone `CustomTimeZone`}.

 * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html Rust documentation for `MetazoneCalculator`} for more information.
 */
export class MetazoneCalculator {

  /**

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): MetazoneCalculator | never;
}
