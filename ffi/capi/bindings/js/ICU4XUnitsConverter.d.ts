import { f64 } from "./diplomat-runtime"

/**

 * An ICU4X Units Converter object, capable of converting between two {@link ICU4XMeasureUnit `ICU4XMeasureUnit`}s.

 * See the {@link https://docs.rs/icu/latest/icu/experimental/units/converter/struct.UnitsConverter.html Rust documentation for `UnitsConverter`} for more information.
 */
export class ICU4XUnitsConverter {

  /**

   * Converts the input value from the input unit to the output unit. NOTE: The conversion using float is not as accurate as the conversion using ratios.
   */
  convert_f64(input: f64): f64;
}
