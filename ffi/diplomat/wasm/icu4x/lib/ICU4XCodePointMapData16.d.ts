import { u16, char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
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

   * Gets a map for Unicode property Script from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_script.html Rust documentation for `load_script`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_script(provider: ICU4XDataProvider): ICU4XCodePointMapData16 | never;

  /**

   * Gets the value for a code point.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get Rust documentation for `get`} for more information.
   */
  get(cp: char): u16;
}
