import { u16, u32, char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XCodePointSetData } from "./ICU4XCodePointSetData";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

 * For properties whose values fit into 16 bits.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html Rust documentation for `properties`} for more information.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapData.html Rust documentation for `CodePointMapData`} for more information.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html Rust documentation for `CodePointMapDataBorrowed`} for more information.
 */
export class ICU4XCodePointMapData16 {

  /**

   * Gets the value for a code point.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get Rust documentation for `get`} for more information.
   */
  get(cp: char): u16;

  /**

   * Gets the value for a code point (specified as a 32 bit integer, in UTF-32)
   */
  get32(cp: u32): u16;

  /**

   * Gets a {@link ICU4XCodePointSetData `ICU4XCodePointSetData`} representing all entries in this map that map to the given value

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value Rust documentation for `get_set_for_value`} for more information.
   */
  get_set_for_value(value: u16): ICU4XCodePointSetData;

  /**

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_script.html Rust documentation for `load_script`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static load_script(provider: ICU4XDataProvider): ICU4XCodePointMapData16 | never;
}
