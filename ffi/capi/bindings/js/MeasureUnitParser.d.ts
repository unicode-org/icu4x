import { MeasureUnit } from "./MeasureUnit";

/**

 * An ICU4X Measurement Unit parser object which is capable of parsing the CLDR unit identifier (e.g. `meter-per-square-second`) and get the {@link MeasureUnit `MeasureUnit`}.

 * See the {@link https://docs.rs/icu/latest/icu/experimental/units/measureunit/struct.MeasureUnitParser.html Rust documentation for `MeasureUnitParser`} for more information.
 */
export class MeasureUnitParser {

  /**

   * Parses the CLDR unit identifier (e.g. `meter-per-square-second`) and returns the corresponding {@link MeasureUnit `MeasureUnit`}, if the identifier is valid.

   * See the {@link https://docs.rs/icu/latest/icu/experimental/units/measureunit/struct.MeasureUnitParser.html#method.parse Rust documentation for `parse`} for more information.
   */
  parse(unit_id: string): MeasureUnit | undefined;
}
