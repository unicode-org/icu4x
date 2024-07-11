import { u16, u32, char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { CodePointRangeIterator } from "./CodePointRangeIterator";
import { CodePointSetData } from "./CodePointSetData";
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";

/**

 * An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

 * For properties whose values fit into 16 bits.

 * See the {@link https://docs.rs/icu/latest/icu/properties/index.html Rust documentation for `properties`} for more information.

 * See the {@link https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapData.html Rust documentation for `CodePointMapData`} for more information.

 * See the {@link https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html Rust documentation for `CodePointMapDataBorrowed`} for more information.
 */
export class CodePointMapData16 {

  /**

   * Gets the value for a code point.

   * See the {@link https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get Rust documentation for `get`} for more information.
   */
  get(cp: char): u16;

  /**

   * Gets the value for a code point (specified as a 32 bit integer, in UTF-32)
   */
  get32(cp: u32): u16;

  /**

   * Produces an iterator over ranges of code points that map to `value`

   * See the {@link https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value Rust documentation for `iter_ranges_for_value`} for more information.
   */
  iter_ranges_for_value(value: u16): CodePointRangeIterator;

  /**

   * Produces an iterator over ranges of code points that do not map to `value`

   * See the {@link https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value_complemented Rust documentation for `iter_ranges_for_value_complemented`} for more information.
   */
  iter_ranges_for_value_complemented(value: u16): CodePointRangeIterator;

  /**

   * Gets a {@link CodePointSetData `CodePointSetData`} representing all entries in this map that map to the given value

   * See the {@link https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value Rust documentation for `get_set_for_value`} for more information.
   */
  get_set_for_value(value: u16): CodePointSetData;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/maps/fn.script.html Rust documentation for `script`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load_script(provider: DataProvider): CodePointMapData16 | never;
}
