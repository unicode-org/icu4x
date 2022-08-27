import { char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html Rust documentation for `properties`} for more information.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetData.html Rust documentation for `CodePointSetData`} for more information.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html Rust documentation for `CodePointSetDataBorrowed`} for more information.
 */
export class ICU4XCodePointSetData {

  /**

   * Gets a set for Unicode property ascii_hex_digit from a {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ascii_hex_digit.html Rust documentation for `load_ascii_hex_digit`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_get_ascii_hex_digit(provider: ICU4XDataProvider): ICU4XCodePointSetData | never;

  /**

   * Checks whether the code point is in the set.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains Rust documentation for `contains`} for more information.
   */
  contains(cp: char): boolean;
}
