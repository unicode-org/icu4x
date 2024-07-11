import { u8 } from "./diplomat-runtime"

/**

 * A generic data struct to be used by ICU4X

 * This can be used to construct a StructDataProvider.
 */
export class DataStruct {

  /**

   * Construct a new DecimalSymbolsV1 data struct.

   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

   * Digits needs to be a 10-character-long slice of valid Unicode characters, otherwise null is returned.

   * See the {@link https://docs.rs/icu/latest/icu/decimal/provider/struct.DecimalSymbolsV1.html Rust documentation for `DecimalSymbolsV1`} for more information.
   */
  static create_decimal_symbols_v1(plus_sign_prefix: string, plus_sign_suffix: string, minus_sign_prefix: string, minus_sign_suffix: string, decimal_separator: string, grouping_separator: string, primary_group_size: u8, secondary_group_size: u8, min_group_size: u8, digits: Uint32Array): DataStruct | undefined;
}
