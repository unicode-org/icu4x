import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { DataStruct } from "./DataStruct";
import { FixedDecimal } from "./FixedDecimal";
import { FixedDecimalGroupingStrategy } from "./FixedDecimalGroupingStrategy";
import { Locale } from "./Locale";

/**

 * An ICU4X Fixed Decimal Format object, capable of formatting a {@link FixedDecimal `FixedDecimal`} as a string.

 * See the {@link https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html Rust documentation for `FixedDecimalFormatter`} for more information.
 */
export class FixedDecimalFormatter {

  /**

   * Creates a new {@link FixedDecimalFormatter `FixedDecimalFormatter`} from locale data.

   * See the {@link https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_with_grouping_strategy(provider: DataProvider, locale: Locale, grouping_strategy: FixedDecimalGroupingStrategy): FixedDecimalFormatter | never;

  /**

   * Creates a new {@link FixedDecimalFormatter `FixedDecimalFormatter`} from preconstructed locale data in the form of an {@link DataStruct `DataStruct`} constructed from `DataStruct::create_decimal_symbols()`.

   * The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed. Passing a consumed struct to this method will return an error.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_with_decimal_symbols_v1(data_struct: DataStruct, grouping_strategy: FixedDecimalGroupingStrategy): FixedDecimalFormatter | never;

  /**

   * Formats a {@link FixedDecimal `FixedDecimal`} to a string.

   * See the {@link https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.format Rust documentation for `format`} for more information.
   */
  format(value: FixedDecimal): string;
}
