import { u16, char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. For properties whose values fit into 16 bits.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html Rust documentation} for more information.
 */
export class ICU4XCodePointMapData16 {

  /**

   * Gets a map for Unicode property Script from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_script(provider: ICU4XDataProvider): ICU4XCodePointMapData16 | never;

  /**

   * Gets the value for a code point.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32 Rust documentation} for more information.
   */
  get(cp: char): u16;
}
