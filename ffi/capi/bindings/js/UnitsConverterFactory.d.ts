import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { MeasureUnit } from "./MeasureUnit";
import { MeasureUnitParser } from "./MeasureUnitParser";
import { UnitsConverter } from "./UnitsConverter";

/**

 * An ICU4X Units Converter Factory object, capable of creating converters a {@link UnitsConverter `UnitsConverter`} for converting between two {@link MeasureUnit `MeasureUnit`}s. Also, it can parse the CLDR unit identifier (e.g. `meter-per-square-second`) and get the {@link MeasureUnit `MeasureUnit`}.

 * See the {@link https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html Rust documentation for `ConverterFactory`} for more information.
 */
export class UnitsConverterFactory {

  /**

   * Construct a new {@link UnitsConverterFactory `UnitsConverterFactory`} instance.

   * See the {@link https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): UnitsConverterFactory | never;

  /**

   * Creates a new {@link UnitsConverter `UnitsConverter`} from the input and output {@link MeasureUnit `MeasureUnit`}s. Returns nothing if the conversion between the two units is not possible. For example, conversion between `meter` and `second` is not possible.

   * See the {@link https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html#method.converter Rust documentation for `converter`} for more information.
   */
  converter(from: MeasureUnit, to: MeasureUnit): UnitsConverter | undefined;

  /**

   * Creates a parser to parse the CLDR unit identifier (e.g. `meter-per-square-second`) and get the {@link MeasureUnit `MeasureUnit`}.

   * See the {@link https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html#method.parser Rust documentation for `parser`} for more information.
   */
  parser(): MeasureUnitParser;
}
